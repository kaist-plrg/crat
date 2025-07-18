use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn msync(
        __addr: *mut libc::c_void,
        __len: size_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn connect(
        __fd: libc::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __errno_location() -> *mut libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pread(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __nbytes: size_t,
        __offset: __off_t,
    ) -> ssize_t;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn strdupn(s: S, k: I) -> S;
    fn getdelim_(s: *mut S, n: *mut I, d: I, f: *mut FILE) -> I;
    fn getline_(s: *mut S, n: *mut I, f: *mut FILE) -> I;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn printAtDepth(u: V, a: K, d: I, x: I, vdep: I, b: I);
    fn kap(a: *mut K, v: V) -> K;
    fn _bd(x: K) -> K;
    fn Ki(x: I) -> K;
    fn wipe_tape(i: I) -> I;
    fn kerr(s: cS) -> K;
    fn spn(s: S, n: I) -> S;
    fn Kv() -> K;
    static mut fbr: I;
    static mut fll: I;
    fn sva(p: V) -> I;
    fn bp(t: I) -> I;
    fn glueSS(c: S, d: S) -> S;
    fn sp(k: S) -> S;
    fn formKfCS(s: S) -> K;
    fn formKiCS(s: S) -> K;
    fn charpos(s: S, c: C) -> I;
    fn stringHasChar(s: S, c: C) -> I;
    fn _n() -> K;
    fn cd(a: K) -> K;
    fn newK(t: I, n: I) -> K;
    fn stat_sz(u: S, n: *mut I) -> I;
    fn CSK(x: K) -> S;
    fn bswapI(n: I) -> I;
    fn membswpI(d: V, s: V, n: I, x: I) -> V;
    fn membswpF(d: V, s: V, n: I, x: I) -> V;
    fn read_tape(i: I, j: I, type_0: I) -> K;
    static mut mMap: F;
    static mut mUsed: F;
    static mut mMax: F;
    static mut PG: I;
    static mut fer: I;
    fn show(a: K) -> K;
    static mut offsetColon: V;
    fn Ks(x: S) -> K;
    fn OOM_CD(g: I, _: ...) -> I;
    fn mrc(x: K, c: I) -> K;
}
pub type __int8_t = libc::c_schar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 256;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
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
pub type L = libc::c_longlong;
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
pub type F = libc::c_double;
pub type C = libc::c_char;
pub type S = *mut C;
pub type cS = *const C;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct k0 {
    pub _c: I,
    pub t: I,
    pub n: I,
    pub k: [*mut k0; 1],
}
pub type K = *mut k0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct m1 {
    pub a: libc::c_char,
    pub b: libc::c_char,
    pub c: [libc::c_char; 5],
    pub d: libc::c_char,
    pub n: I,
}
pub type M1 = m1;
pub type TYPE_SEVEN_MEMBERS = libc::c_uint;
pub const TYPE_SEVEN_SIZE: TYPE_SEVEN_MEMBERS = 8;
pub const CACHE_TREE: TYPE_SEVEN_MEMBERS = 7;
pub const CACHE_WD: TYPE_SEVEN_MEMBERS = 6;
pub const CONJ: TYPE_SEVEN_MEMBERS = 5;
pub const PARAMS: TYPE_SEVEN_MEMBERS = 4;
pub const LOCALS: TYPE_SEVEN_MEMBERS = 3;
pub const CODE: TYPE_SEVEN_MEMBERS = 2;
pub const DEPTH: TYPE_SEVEN_MEMBERS = 1;
pub const CONTeXT: TYPE_SEVEN_MEMBERS = 0;
unsafe extern "C" fn freopen_stdin() -> V {
    return freopen(
        0 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
        stdin,
    ) as V;
}
pub unsafe extern "C" fn _0m(mut a: K) -> K {
    let mut c: I = 0;
    let mut d: I = 0;
    let mut e: I = 0;
    let mut k_0: K = 0 as *mut k0;
    let mut current_block: u64;
    let mut t: I = (*a).t;
    if 4 as libc::c_int as libc::c_longlong != t
        && 3 as libc::c_int as libc::c_longlong
            != (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut b: I = 0 as libc::c_int as I;
    let mut s: I = 0 as libc::c_int as I;
    let mut v: S = 0 as S;
    let mut z: K = 0 as K;
    let mut m: S = 0 as *mut C;
    fll = 0 as libc::c_int as I;
    fbr = fll;
    if 3 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        m = CSK(a);
    }
    let mut sb: stat = stat {
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
    let mut ff: I = 0 as libc::c_int as I;
    if 3 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
        && strcmp(
            m as *const libc::c_char,
            b"/dev/fd/0\0" as *const u8 as *const libc::c_char,
        ) != 0
        && strcmp(
            m as *const libc::c_char,
            b"/dev/stdin\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        if stat(m as *const libc::c_char, &mut sb) == -(1 as libc::c_int) {
            return kerr(b"file\0" as *const u8 as *const libc::c_char);
        }
        if sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint
        {
            ff = 1 as libc::c_int as I;
        }
    }
    if ff != 0 {
        let mut fn_0: I = 0;
        let mut i: I = 0;
        let mut j: I = 0;
        let mut buf: [C; 256] = [0; 256];
        z = newK(0 as libc::c_int as I, 0 as libc::c_int as I);
        fn_0 = open(m as *const libc::c_char, 0 as libc::c_int) as I;
        while read(
            fn_0 as libc::c_int,
            &mut buf as *mut [C; 256] as *mut libc::c_void,
            256 as libc::c_int as size_t,
        ) > 0 as libc::c_int as libc::c_long
        {
            j = 256 as libc::c_int as I;
            let mut y: K = 0 as K;
            i = 0 as libc::c_int as I;
            while i < 256 as libc::c_int as libc::c_longlong {
                if i > j {
                    buf[j as usize] = '\0' as i32 as C;
                    break;
                } else {
                    if buf[i as usize] as libc::c_int == '\n' as i32 {
                        j = i;
                    }
                    i += 1;
                    i;
                }
            }
            let mut n: I = strlen(buf.as_mut_ptr()) as I;
            y = newK(
                (if n < 2 as libc::c_int as libc::c_longlong {
                    3 as libc::c_int
                } else {
                    -(3 as libc::c_int)
                }) as I,
                n,
            );
            memcpy(
                ((*y).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                &mut buf as *mut [C; 256] as *const libc::c_void,
                n as libc::c_ulong,
            );
            kap(&mut z, &mut y as *mut K as V);
            cd(y);
        }
    } else if 4 as libc::c_int as libc::c_longlong == t
        && **(((*a).k).as_mut_ptr() as *mut S) == 0
    {
        let mut ss: [libc::c_char; 300] = [0; 300];
        let mut adr: S = fgets(
            ss.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 300]>() as libc::c_ulong as libc::c_int,
            stdin,
        );
        if adr.is_null() {
            return newK(6 as libc::c_int as I, 1 as libc::c_int as I);
        }
        let mut i_0: I = 0;
        let mut j_0: I = 0;
        i_0 = 0 as libc::c_int as I;
        while i_0 < 300 as libc::c_int as libc::c_longlong {
            if ss[i_0 as usize] as libc::c_int == '\n' as i32 {
                break;
            }
            i_0 += 1;
            i_0;
        }
        let mut k: I = 0 as libc::c_int as I;
        j_0 = 0 as libc::c_int as I;
        while j_0 <= i_0 {
            if ss[j_0 as usize] as libc::c_int != '\u{4}' as i32 {
                let fresh0 = k;
                k = k + 1;
                ss[fresh0 as usize] = ss[j_0 as usize];
            }
            j_0 += 1;
            j_0;
        }
        z = newK(-(3 as libc::c_int) as I, k - 1 as libc::c_int as libc::c_longlong);
        j_0 = 0 as libc::c_int as I;
        while j_0 < k - 1 as libc::c_int as libc::c_longlong {
            *(((*z).k).as_mut_ptr() as *mut C).offset(j_0 as isize) = ss[j_0 as usize];
            j_0 += 1;
            j_0;
        }
    } else {
        if 3 as libc::c_int as libc::c_longlong
            == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
            && (strcmp(
                m as *const libc::c_char,
                b"/dev/fd/0\0" as *const u8 as *const libc::c_char,
            ) == 0
                || strcmp(
                    m as *const libc::c_char,
                    b"/dev/stdin\0" as *const u8 as *const libc::c_char,
                ) == 0)
            || 4 as libc::c_int as libc::c_longlong == t
                && (strcmp(
                    *(((*a).k).as_mut_ptr() as *mut S) as *const libc::c_char,
                    b"/dev/fd/0\0" as *const u8 as *const libc::c_char,
                ) == 0
                    || strcmp(
                        *(((*a).k).as_mut_ptr() as *mut S) as *const libc::c_char,
                        b"/dev/stdin\0" as *const u8 as *const libc::c_char,
                    ) == 0)
        {
            b = getdelim_(&mut v, &mut s, -(1 as libc::c_int) as I, stdin);
            if (freopen_stdin()).is_null() {
                return kerr(b"file\0" as *const u8 as *const libc::c_char);
            }
            if b == -(1 as libc::c_int) as libc::c_longlong {
                z = newK(0 as libc::c_int as I, 0 as libc::c_int as I);
                current_block = 13633966507256000877;
            } else {
                current_block = 9512719473022792396;
            }
        } else {
            let mut f: I = open(CSK(a) as *const libc::c_char, 0 as libc::c_int) as I;
            if f < 0 as libc::c_int as libc::c_longlong {
                return kerr(b"domain\0" as *const u8 as *const libc::c_char);
            }
            if stat_sz(CSK(a), &mut s) != 0 {
                return kerr(strerror(*__errno_location()) as cS);
            }
            v = mmap(
                0 as *mut libc::c_void,
                s as size_t,
                0x1 as libc::c_int,
                0x1 as libc::c_int,
                f as libc::c_int,
                0 as libc::c_int as __off_t,
            ) as S;
            if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
                return kerr(strerror(*__errno_location()) as cS);
            }
            let mut r: I = close(f as libc::c_int) as I;
            if r != 0 {
                return kerr(b"file\0" as *const u8 as *const libc::c_char);
            }
            current_block = 9512719473022792396;
        }
        match current_block {
            13633966507256000877 => {}
            _ => {
                c = (if s != 0 { 1 as libc::c_int } else { 0 as libc::c_int }) as I;
                d = 0 as libc::c_int as I;
                e = 0;
                let mut i_1: I = 0 as libc::c_int as I;
                let mut _i: I = s;
                while i_1 < _i {
                    if '\n' as i32 == *v.offset(i_1 as isize) as libc::c_int
                        && i_1 < s - 1 as libc::c_int as libc::c_longlong
                    {
                        c += 1;
                        c;
                    }
                    i_1 += 1;
                    i_1;
                }
                k_0 = 0 as *mut k0;
                z = newK(0 as libc::c_int as I, c);
                if !z.is_null() {
                    let mut i_2: I = 0 as libc::c_int as I;
                    let mut _i_0: I = s;
                    while i_2 < _i_0 {
                        if '\n' as i32 != *v.offset(i_2 as isize) as libc::c_int {
                            let ref mut fresh1 = *((*z).k)
                                .as_mut_ptr()
                                .offset(d as isize);
                            *fresh1 = (1 as libc::c_int as V)
                                .offset(
                                    *((*z).k).as_mut_ptr().offset(d as isize) as L as isize,
                                ) as *mut k0;
                        } else {
                            d += 1;
                            d;
                        }
                        i_2 += 1;
                        i_2;
                    }
                    let mut i_3: I = 0 as libc::c_int as I;
                    let mut _i_1: I = c;
                    loop {
                        if !(i_3 < _i_1) {
                            current_block = 8464383504555462953;
                            break;
                        }
                        e = *((*z).k).as_mut_ptr().offset(i_3 as isize) as L;
                        k_0 = newK(-(3 as libc::c_int) as I, e);
                        if k_0.is_null() {
                            cd(z);
                            z = 0 as K;
                            current_block = 13633966507256000877;
                            break;
                        } else {
                            let ref mut fresh2 = *((*z).k)
                                .as_mut_ptr()
                                .offset(i_3 as isize);
                            *fresh2 = k_0;
                            i_3 += 1;
                            i_3;
                        }
                    }
                    match current_block {
                        13633966507256000877 => {}
                        _ => {
                            e = 0 as libc::c_int as I;
                            let mut i_4: I = 0 as libc::c_int as I;
                            let mut _i_2: I = c;
                            while i_4 < _i_2 {
                                k_0 = *((*z).k).as_mut_ptr().offset(i_4 as isize);
                                memcpy(
                                    ((*k_0).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                                    v.offset(e as isize) as *const libc::c_void,
                                    (*k_0).n as libc::c_ulong,
                                );
                                e += 1 as libc::c_int as libc::c_longlong + (*k_0).n;
                                i_4 += 1;
                                i_4;
                            }
                        }
                    }
                }
            }
        }
    }
    if !v.is_null() {
        if b != 0 {
            free(v as *mut libc::c_void);
        } else {
            let mut r_0: I = munmap(v as *mut libc::c_void, s as size_t) as I;
            if r_0 != 0 {
                return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
            }
        }
    }
    return z;
}
pub unsafe extern "C" fn _0d(mut a: K, mut b: K) -> K {
    let mut t: I = (*a).t;
    if 4 as libc::c_int as libc::c_longlong == t
        || 3 as libc::c_int as libc::c_longlong
            == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        return _0d_write(a, b);
    }
    if t == 0 {
        return _0d_read(a, b);
    }
    return kerr(b"type\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn ok_0dw(mut b: K) -> I {
    let mut t: I = (*b).t;
    let mut n: I = (*b).n;
    let mut k: K = 0 as *mut k0;
    if 3 as libc::c_int as libc::c_longlong
        != (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        if t == 0 {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = n;
            while i < _i {
                k = *((*b).k).as_mut_ptr().offset(i as isize);
                if 3 as libc::c_int as libc::c_longlong
                    != (if (*k).t < 0 as libc::c_int as libc::c_longlong {
                        -(*k).t
                    } else {
                        (*k).t
                    }) && (t != 0 || (*k).n != 0)
                {
                    return 0 as libc::c_int as I;
                }
                i += 1;
                i;
            }
        } else {
            return 0 as libc::c_int as I
        }
    }
    return 1 as libc::c_int as I;
}
unsafe extern "C" fn _0d_write(mut a: K, mut b: K) -> K {
    let mut t: I = (*b).t;
    let mut n: I = (*b).n;
    let mut k: K = 0 as *mut k0;
    if ok_0dw(b) == 0 {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut m: S = CSK(a);
    let mut s: I = 0 as libc::c_int as I;
    let mut f: I = 0 as libc::c_int as I;
    let mut sb: stat = stat {
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
    if stat(m as *const libc::c_char, &mut sb) != -(1 as libc::c_int)
        && sb.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o10000 as libc::c_int as libc::c_uint
    {
        f = open(m as *const libc::c_char, 0o1 as libc::c_int) as I;
        if f < 0 as libc::c_int as libc::c_longlong {
            return kerr(b"domain\0" as *const u8 as *const libc::c_char);
        }
        if 3 as libc::c_int as libc::c_longlong
            == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
        {
            let mut msg: S = ((*b).k).as_mut_ptr() as *mut C;
            if write(
                f as libc::c_int,
                msg as *const libc::c_void,
                (strlen(msg as *const libc::c_char))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) == -(1 as libc::c_int) as libc::c_long
            {
                return kerr(b"write\0" as *const u8 as *const libc::c_char);
            }
        } else if 0 as libc::c_int as libc::c_longlong == t {
            let mut msg_0: S = 0 as *mut C;
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = n;
            while i < _i {
                if (if (**((*b).k).as_mut_ptr().offset(i as isize)).t
                    < 0 as libc::c_int as libc::c_longlong
                {
                    -(**((*b).k).as_mut_ptr().offset(i as isize)).t
                } else {
                    (**((*b).k).as_mut_ptr().offset(i as isize)).t
                }) != 3 as libc::c_int as libc::c_longlong
                {
                    return kerr(b"domain\0" as *const u8 as *const libc::c_char);
                }
                msg_0 = ((*(*((*b).k).as_mut_ptr().offset(i as isize) as K)).k)
                    .as_mut_ptr() as *mut C;
                if write(
                    f as libc::c_int,
                    msg_0 as *const libc::c_void,
                    (strlen(msg_0 as *const libc::c_char))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) == -(1 as libc::c_int) as libc::c_long
                {
                    return kerr(b"write\0" as *const u8 as *const libc::c_char);
                }
                i += 1;
                i;
            }
        } else {
            let mut r: I = close(f as libc::c_int) as I;
            if r != 0 {
                return kerr(b"file\0" as *const u8 as *const libc::c_char);
            }
            return kerr(b"domain\0" as *const u8 as *const libc::c_char);
        }
        let mut r_0: I = close(f as libc::c_int) as I;
        if r_0 != 0 {
            return kerr(b"file\0" as *const u8 as *const libc::c_char);
        }
        return _n();
    }
    if 3 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        s = n;
    } else {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i_0 < _i_0 {
            s
                += 1 as libc::c_int as libc::c_longlong
                    + (**((*b).k).as_mut_ptr().offset(i_0 as isize)).n;
            i_0 += 1;
            i_0;
        }
    }
    if *m.offset(0 as libc::c_int as isize) == 0
        || strcmp(
            m as *const libc::c_char,
            b"/dev/fd/1\0" as *const u8 as *const libc::c_char,
        ) == 0
        || strcmp(
            m as *const libc::c_char,
            b"/dev/stdout\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut r_1: I = 0;
        f = 1 as libc::c_int as I;
        if 3 as libc::c_int as libc::c_longlong
            == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
        {
            if write(
                f as libc::c_int,
                ((*b).k).as_mut_ptr() as *mut C as *const libc::c_void,
                s as size_t,
            ) == -(1 as libc::c_int) as libc::c_long
            {
                show(kerr(b"write\0" as *const u8 as *const libc::c_char));
            }
        } else {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = n;
            while i_1 < _i_1 {
                k = *((*b).k).as_mut_ptr().offset(i_1 as isize);
                if 3 as libc::c_int as libc::c_longlong
                    == (if (*k).t < 0 as libc::c_int as libc::c_longlong {
                        -(*k).t
                    } else {
                        (*k).t
                    })
                {
                    r_1 = write(
                        f as libc::c_int,
                        ((*k).k).as_mut_ptr() as *mut C as *const libc::c_void,
                        (*k).n as size_t,
                    ) as I;
                }
                r_1 = write(
                    f as libc::c_int,
                    b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                ) as I;
                if r_1 == -(1 as libc::c_int) as libc::c_longlong {
                    show(kerr(b"write\0" as *const u8 as *const libc::c_char));
                }
                i_1 += 1;
                i_1;
            }
        }
    } else {
        if *m.offset(0 as libc::c_int as isize) != 0 {
            f = open(
                m as *const libc::c_char,
                0o2 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
                0o664 as libc::c_int,
            ) as I;
        }
        if f < 0 as libc::c_int as libc::c_longlong {
            return kerr(b"domain\0" as *const u8 as *const libc::c_char);
        }
        if ftruncate(f as libc::c_int, s as __off_t) != 0 {
            return kerr(strerror(*__errno_location()) as cS);
        }
        let mut v: S = 0 as *mut C;
        v = mmap(
            0 as *mut libc::c_void,
            s as size_t,
            0x2 as libc::c_int,
            0x1 as libc::c_int,
            f as libc::c_int,
            0 as libc::c_int as __off_t,
        ) as S;
        if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
            return kerr(strerror(*__errno_location()) as cS);
        }
        let mut r_2: I = close(f as libc::c_int) as I;
        if r_2 != 0 {
            return kerr(b"file\0" as *const u8 as *const libc::c_char);
        }
        let mut c: I = 0 as libc::c_int as I;
        if 3 as libc::c_int as libc::c_longlong
            == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
        {
            memcpy(
                v as *mut libc::c_void,
                ((*b).k).as_mut_ptr() as *mut C as *const libc::c_void,
                s as libc::c_ulong,
            );
        } else {
            let mut i_2: I = 0 as libc::c_int as I;
            let mut _i_2: I = n;
            while i_2 < _i_2 {
                k = *((*b).k).as_mut_ptr().offset(i_2 as isize);
                if 3 as libc::c_int as libc::c_longlong
                    == (if (*k).t < 0 as libc::c_int as libc::c_longlong {
                        -(*k).t
                    } else {
                        (*k).t
                    })
                {
                    memcpy(
                        v.offset(c as isize) as *mut libc::c_void,
                        ((*k).k).as_mut_ptr() as *mut C as *const libc::c_void,
                        (*k).n as libc::c_ulong,
                    );
                    c += (*k).n;
                }
                let fresh3 = c;
                c = c + 1;
                *v.offset(fresh3 as isize) = '\n' as i32 as C;
                i_2 += 1;
                i_2;
            }
        }
        r_2 = munmap(v as *mut libc::c_void, s as size_t) as I;
        if r_2 != 0 {
            return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
        }
    }
    return _n();
}
unsafe extern "C" fn _0d_read(mut a: K, mut b: K) -> K {
    let mut e: I = 0;
    let mut g: C = 0;
    let mut m: S = 0 as *mut C;
    let mut u: I = 0;
    let mut y: I = 0;
    let mut p: I = 0;
    let mut current_block: u64;
    let mut z: K = 0 as K;
    let mut res: I = 0;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if an != 2 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut c: K = *((*a).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    let mut d: K = *((*a).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    let mut cn: I = (*c).n;
    let mut dn: I = (*d).n;
    if 3 as libc::c_int as libc::c_longlong == (*d).t {
        return _0d_rdDsv(a, b);
    }
    if -(3 as libc::c_int) as libc::c_longlong == (*d).t {
        return _0d_rdDsvWc(a, b);
    }
    if 3 as libc::c_int as libc::c_longlong
        != (if (*c).t < 0 as libc::c_int as libc::c_longlong { -(*c).t } else { (*c).t })
        || 1 as libc::c_int as libc::c_longlong
            != (if (*d).t < 0 as libc::c_int as libc::c_longlong {
                -(*d).t
            } else {
                (*d).t
            })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if cn == 0 || cn != dn {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if 3 as libc::c_int as libc::c_longlong
        != (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        && 4 as libc::c_int as libc::c_longlong != bt
        && 0 as libc::c_int as libc::c_longlong != bt
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut fc: I = 0 as libc::c_int as I;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = cn;
    while i < _i {
        if !(' ' as i32
            == *(((*c).k).as_mut_ptr() as *mut C).offset(i as isize) as libc::c_int)
        {
            if stringHasChar(
                b"IFCS\0" as *const u8 as *const libc::c_char as S,
                *(((*c).k).as_mut_ptr() as *mut C).offset(i as isize),
            ) != 0
            {
                fc += 1;
                fc;
            } else {
                return kerr(b"type\0" as *const u8 as *const libc::c_char)
            }
        }
        i += 1;
        i;
    }
    let mut w: I = 1 as libc::c_int as I;
    let mut x: I = 0;
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = dn;
    while i_0 < _i_0 {
        x = *(((*d).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
        if x <= 0 as libc::c_int as libc::c_longlong {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        w += x;
        i_0 += 1;
        i_0;
    }
    let mut ff: K = b;
    let mut k: K = 0 as *mut k0;
    let mut fb: I = 0 as libc::c_int as I;
    let mut fn_0: I = 0 as libc::c_int as I;
    if bt == 0 {
        if 3 as libc::c_int as libc::c_longlong != bn {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        ff = *((*b).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        if 3 as libc::c_int as libc::c_longlong
            != (if (*ff).t < 0 as libc::c_int as libc::c_longlong {
                -(*ff).t
            } else {
                (*ff).t
            }) && 4 as libc::c_int as libc::c_longlong != (*ff).t
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        k = *((*b).k).as_mut_ptr().offset(1 as libc::c_int as isize);
        if 1 as libc::c_int as libc::c_longlong != (*k).t
            && 2 as libc::c_int as libc::c_longlong != (*k).t
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        fb = (if (*k).t - 1 as libc::c_int as libc::c_longlong != 0 {
            *(((*k).k).as_mut_ptr() as *mut F)
        } else {
            *(((*k).k).as_mut_ptr() as *mut I) as libc::c_double
        }) as I;
        k = *((*b).k).as_mut_ptr().offset(2 as libc::c_int as isize);
        if 1 as libc::c_int as libc::c_longlong != (*k).t
            && 2 as libc::c_int as libc::c_longlong != (*k).t
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        fn_0 = (if (*k).t - 1 as libc::c_int as libc::c_longlong != 0 {
            *(((*k).k).as_mut_ptr() as *mut F)
        } else {
            *(((*k).k).as_mut_ptr() as *mut I) as libc::c_double
        }) as I;
    }
    let mut s: I = 0;
    if stat_sz(CSK(ff), &mut s) != 0 {
        return kerr(strerror(*__errno_location()) as cS);
    }
    if bt != 0 {
        fn_0 = s;
    }
    if fn_0 < 0 as libc::c_int as libc::c_longlong {
        fn_0 = 0 as libc::c_int as I;
    }
    if fb < 0 as libc::c_int as libc::c_longlong {
        fb = 0 as libc::c_int as I;
    }
    fb = if fb
        < (if 0 as libc::c_int as libc::c_longlong
            > s - 1 as libc::c_int as libc::c_longlong
        {
            0 as libc::c_int as libc::c_longlong
        } else {
            s - 1 as libc::c_int as libc::c_longlong
        })
    {
        fb
    } else if 0 as libc::c_int as libc::c_longlong
        > s - 1 as libc::c_int as libc::c_longlong
    {
        0 as libc::c_int as libc::c_longlong
    } else {
        s - 1 as libc::c_int as libc::c_longlong
    };
    if fb + fn_0 > s {
        fn_0 = s - fb;
    }
    let mut f: I = open(CSK(ff) as *const libc::c_char, 0 as libc::c_int) as I;
    if f < 0 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut v: S = 0 as *mut C;
    v = mmap(
        0 as *mut libc::c_void,
        fn_0 as size_t,
        0x1 as libc::c_int,
        0x1 as libc::c_int,
        f as libc::c_int,
        fb as __off_t,
    ) as S;
    if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut r: I = close(f as libc::c_int) as I;
    if r != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    r = 0 as libc::c_int as I;
    let mut t: I = 0 as libc::c_int as I;
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_1: I = fn_0;
    while i_1 < _i_1 {
        if *v.offset((fb + i_1) as isize) as libc::c_int == '\n' as i32 {
            if t == w - 1 as libc::c_int as libc::c_longlong {
                r += 1;
                r;
                t = 0 as libc::c_int as I;
            } else {
                t = 0 as libc::c_int as I;
            }
        } else {
            t += 1;
            t;
        }
        i_1 += 1;
        i_1;
    }
    z = newK(0 as libc::c_int as I, fc);
    if !z.is_null() {
        e = 0 as libc::c_int as I;
        g = 0;
        let mut i_2: I = 0 as libc::c_int as I;
        let mut _i_2: I = cn;
        loop {
            if !(i_2 < _i_2) {
                current_block = 16231175055492490595;
                break;
            }
            g = *(((*c).k).as_mut_ptr() as *mut C).offset(i_2 as isize);
            if !(' ' as i32 == g as libc::c_int) {
                let fresh4 = e;
                e = e + 1;
                let ref mut fresh5 = *((*z).k).as_mut_ptr().offset(fresh4 as isize);
                *fresh5 = newK(
                    -charpos(b"CIF S\0" as *const u8 as *const libc::c_char as S, g),
                    r,
                );
                if (*fresh5).is_null() {
                    cd(z);
                    z = 0 as K;
                    current_block = 13357516634469671053;
                    break;
                }
            }
            i_2 += 1;
            i_2;
        }
        match current_block {
            13357516634469671053 => {}
            _ => {
                m = 0 as *mut C;
                u = 0 as libc::c_int as I;
                y = 0;
                p = 0 as libc::c_int as I;
                while u <= fn_0 - w {
                    while u + t < fn_0
                        && '\n' as i32 != *v.offset((u + t) as isize) as libc::c_int
                    {
                        t += 1;
                        t;
                    }
                    if t == w - 1 as libc::c_int as libc::c_longlong
                        && '\n' as i32 == *v.offset((u + t) as isize) as libc::c_int
                    {
                        y = u;
                        x = 0 as libc::c_int as I;
                        e = x;
                        let mut q: K = 0 as K;
                        let mut i_3: I = 0 as libc::c_int as I;
                        let mut _i_3: I = cn;
                        while i_3 < _i_3 {
                            x = *(((*d).k).as_mut_ptr() as *mut I).offset(i_3 as isize);
                            let fresh6 = e;
                            e = e + 1;
                            k = *((*z).k).as_mut_ptr().offset(fresh6 as isize);
                            match *(((*c).k).as_mut_ptr() as *mut C).offset(i_3 as isize)
                                as libc::c_int
                            {
                                32 => {
                                    e -= 1;
                                    e;
                                }
                                73 => {
                                    m = strdupn(v.offset(y as isize), x);
                                    if m.is_null() {
                                        return 0 as K;
                                    }
                                    q = formKiCS(m);
                                    *(((*k).k).as_mut_ptr() as *mut I)
                                        .offset(
                                            p as isize,
                                        ) = if !q.is_null() {
                                        *(((*q).k).as_mut_ptr() as *mut I)
                                    } else {
                                        -(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong
                                    };
                                    free(m as *mut libc::c_void);
                                }
                                70 => {
                                    m = strdupn(v.offset(y as isize), x);
                                    if m.is_null() {
                                        return 0 as K;
                                    }
                                    q = formKfCS(m);
                                    *(((*k).k).as_mut_ptr() as *mut F)
                                        .offset(
                                            p as isize,
                                        ) = if !q.is_null() {
                                        *(((*q).k).as_mut_ptr() as *mut F)
                                    } else {
                                        0 as libc::c_int as libc::c_double / 0.0f64
                                    };
                                    free(m as *mut libc::c_void);
                                }
                                67 => {
                                    q = newK(-(3 as libc::c_int) as I, x);
                                    if q.is_null() {
                                        return 0 as K;
                                    }
                                    memcpy(
                                        ((*q).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                                        v.offset(y as isize) as *const libc::c_void,
                                        x as libc::c_ulong,
                                    );
                                    let ref mut fresh7 = *((*k).k)
                                        .as_mut_ptr()
                                        .offset(p as isize);
                                    *fresh7 = q;
                                    q = 0 as K;
                                }
                                83 => {
                                    m = strdupn(v.offset(y as isize), x);
                                    if m.is_null() {
                                        return 0 as K;
                                    }
                                    let ref mut fresh8 = *(((*k).k).as_mut_ptr() as *mut S)
                                        .offset(p as isize);
                                    *fresh8 = sp(m);
                                    free(m as *mut libc::c_void);
                                }
                                _ => {}
                            }
                            y += x;
                            i_3 += 1;
                            i_3;
                        }
                        p += 1;
                        p;
                    }
                    u += t + 1 as libc::c_int as libc::c_longlong;
                    t = 0 as libc::c_int as I;
                }
            }
        }
    }
    res = munmap(v as *mut libc::c_void, s as size_t) as I;
    if res != 0 {
        return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
    }
    return z;
}
unsafe extern "C" fn _0d_rdDsv(mut a: K, mut b: K) -> K {
    let mut e: I = 0;
    let mut g: C = 0;
    let mut m: S = 0 as *mut C;
    let mut u: I = 0;
    let mut t: I = 0;
    let mut p: I = 0;
    let mut n: I = 0;
    let mut h: I = 0;
    let mut tok: *mut C = 0 as *mut C;
    let mut y: [C; 2] = [0; 2];
    let mut k: K = 0 as *mut k0;
    let mut current_block: u64;
    let mut z: K = 0 as K;
    let mut res: I = 0;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    if an != 2 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut c: K = *((*a).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    let mut d: K = *((*a).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    let mut cn: I = (*c).n;
    if 3 as libc::c_int as libc::c_longlong
        != (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        && 4 as libc::c_int as libc::c_longlong != bt
        && 0 as libc::c_int as libc::c_longlong != bt
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut x: *mut C = ((*d).k).as_mut_ptr() as *mut C;
    let mut w: C = *x;
    let mut fb: I = 0 as libc::c_int as I;
    let mut fn_0: I = 0 as libc::c_int as I;
    let mut s: I = 0;
    if stat_sz(CSK(b), &mut s) != 0 {
        return kerr(strerror(*__errno_location()) as cS);
    }
    if bt != 0 {
        fn_0 = s;
    }
    let mut f: I = open(CSK(b) as *const libc::c_char, 0 as libc::c_int) as I;
    if f < 0 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut v: S = 0 as *mut C;
    v = mmap(
        0 as *mut libc::c_void,
        fn_0 as size_t,
        0x1 as libc::c_int,
        0x1 as libc::c_int,
        f as libc::c_int,
        fb as __off_t,
    ) as S;
    if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
        printf(b"mmap failed\n\0" as *const u8 as *const libc::c_char);
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut r: I = close(f as libc::c_int) as I;
    if r != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    let mut fc: I = 0 as libc::c_int as I;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = cn;
    while i < _i {
        if !(' ' as i32
            == *(((*c).k).as_mut_ptr() as *mut C).offset(i as isize) as libc::c_int)
        {
            if stringHasChar(
                b"IFCS\0" as *const u8 as *const libc::c_char as S,
                *(((*c).k).as_mut_ptr() as *mut C).offset(i as isize),
            ) != 0
            {
                fc += 1;
                fc;
            } else {
                return kerr(b"type\0" as *const u8 as *const libc::c_char)
            }
        }
        i += 1;
        i;
    }
    r = 0 as libc::c_int as I;
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = fn_0;
    while i_0 < _i_0 {
        if *v.offset((fb + i_0) as isize) as libc::c_int == '\n' as i32 {
            r += 1;
            r;
        }
        i_0 += 1;
        i_0;
    }
    if *v.offset((fn_0 - 1 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
        != '\n' as i32
    {
        r += 1;
        r;
    }
    z = newK(0 as libc::c_int as I, fc);
    if !z.is_null() {
        e = 0 as libc::c_int as I;
        g = 0;
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = cn;
        loop {
            if !(i_1 < _i_1) {
                current_block = 2543120759711851213;
                break;
            }
            g = *(((*c).k).as_mut_ptr() as *mut C).offset(i_1 as isize);
            if !(' ' as i32 == g as libc::c_int) {
                let fresh9 = e;
                e = e + 1;
                let ref mut fresh10 = *((*z).k).as_mut_ptr().offset(fresh9 as isize);
                *fresh10 = newK(
                    -charpos(b"CIF S\0" as *const u8 as *const libc::c_char as S, g),
                    r,
                );
                if (*fresh10).is_null() {
                    cd(z);
                    z = 0 as K;
                    current_block = 18420700090356599967;
                    break;
                }
            }
            i_1 += 1;
            i_1;
        }
        match current_block {
            18420700090356599967 => {}
            _ => {
                m = 0 as *mut C;
                u = 0 as libc::c_int as I;
                t = 0 as libc::c_int as I;
                p = 0 as libc::c_int as I;
                n = 0 as libc::c_int as I;
                h = 0 as libc::c_int as I;
                tok = 0 as *mut C;
                y = [0; 2];
                y[0 as libc::c_int as usize] = w;
                k = 0 as *mut k0;
                while u <= fn_0 {
                    while u + t <= fn_0
                        && '\n' as i32 != *v.offset((u + t) as isize) as libc::c_int
                        && *v.offset((u + t) as isize) as libc::c_longlong
                            != 0 as *mut libc::c_void as L
                    {
                        t += 1;
                        t;
                    }
                    if *v.offset((u + t) as isize) as libc::c_int == '\n' as i32
                        || *v.offset((u + t) as isize) as libc::c_longlong
                            == 0 as *mut libc::c_void as L
                    {
                        let mut q: K = 0 as K;
                        h = 0 as libc::c_int as I;
                        e = h;
                        m = strdupn(v.offset(u as isize), t);
                        if m.is_null() {
                            return 0 as K;
                        }
                        if *m.offset(0 as libc::c_int as isize) as libc::c_longlong
                            != 0 as *mut libc::c_void as L
                        {
                            tok = strsep(&mut m, y.as_mut_ptr());
                            let fresh11 = e;
                            e = e + 1;
                            k = *((*z).k).as_mut_ptr().offset(fresh11 as isize);
                            let fresh12 = h;
                            h = h + 1;
                            match *(((*c).k).as_mut_ptr() as *mut C)
                                .offset(fresh12 as isize) as libc::c_int
                            {
                                32 => {
                                    e -= 1;
                                    e;
                                }
                                73 => {
                                    q = formKiCS(tok);
                                    *(((*k).k).as_mut_ptr() as *mut I)
                                        .offset(
                                            p as isize,
                                        ) = if !q.is_null() {
                                        *(((*q).k).as_mut_ptr() as *mut I)
                                    } else {
                                        -(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong
                                    };
                                }
                                70 => {
                                    q = formKfCS(tok);
                                    *(((*k).k).as_mut_ptr() as *mut F)
                                        .offset(
                                            p as isize,
                                        ) = if !q.is_null() {
                                        *(((*q).k).as_mut_ptr() as *mut F)
                                    } else {
                                        0 as libc::c_int as libc::c_double / 0.0f64
                                    };
                                }
                                67 => {
                                    n = strlen(tok) as I;
                                    q = newK(-(3 as libc::c_int) as I, n);
                                    if q.is_null() {
                                        return 0 as K;
                                    }
                                    memcpy(
                                        ((*q).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                                        tok as *const libc::c_void,
                                        n as libc::c_ulong,
                                    );
                                    let ref mut fresh13 = *((*k).k)
                                        .as_mut_ptr()
                                        .offset(p as isize);
                                    *fresh13 = q;
                                    q = 0 as K;
                                }
                                83 => {
                                    let ref mut fresh14 = *(((*k).k).as_mut_ptr() as *mut S)
                                        .offset(p as isize);
                                    *fresh14 = sp(tok);
                                }
                                _ => {}
                            }
                            while !tok.is_null() {
                                tok = strsep(&mut m, y.as_mut_ptr());
                                if !tok.is_null() {
                                    let fresh15 = e;
                                    e = e + 1;
                                    k = *((*z).k).as_mut_ptr().offset(fresh15 as isize);
                                    let fresh16 = h;
                                    h = h + 1;
                                    match *(((*c).k).as_mut_ptr() as *mut C)
                                        .offset(fresh16 as isize) as libc::c_int
                                    {
                                        32 => {
                                            e -= 1;
                                            e;
                                        }
                                        73 => {
                                            q = formKiCS(tok);
                                            *(((*k).k).as_mut_ptr() as *mut I)
                                                .offset(
                                                    p as isize,
                                                ) = if !q.is_null() {
                                                *(((*q).k).as_mut_ptr() as *mut I)
                                            } else {
                                                -(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong
                                            };
                                        }
                                        70 => {
                                            q = formKfCS(tok);
                                            *(((*k).k).as_mut_ptr() as *mut F)
                                                .offset(
                                                    p as isize,
                                                ) = if !q.is_null() {
                                                *(((*q).k).as_mut_ptr() as *mut F)
                                            } else {
                                                0 as libc::c_int as libc::c_double / 0.0f64
                                            };
                                        }
                                        67 => {
                                            n = strlen(tok) as I;
                                            q = newK(-(3 as libc::c_int) as I, n);
                                            if q.is_null() {
                                                return 0 as K;
                                            }
                                            memcpy(
                                                ((*q).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                                                tok as *const libc::c_void,
                                                n as libc::c_ulong,
                                            );
                                            let ref mut fresh17 = *((*k).k)
                                                .as_mut_ptr()
                                                .offset(p as isize);
                                            *fresh17 = q;
                                            q = 0 as K;
                                        }
                                        83 => {
                                            let ref mut fresh18 = *(((*k).k).as_mut_ptr() as *mut S)
                                                .offset(p as isize);
                                            *fresh18 = sp(tok);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        free(m as *mut libc::c_void);
                    }
                    p += 1;
                    p;
                    u += t + 1 as libc::c_int as libc::c_longlong;
                    t = 0 as libc::c_int as I;
                }
            }
        }
    }
    res = munmap(v as *mut libc::c_void, s as size_t) as I;
    if res != 0 {
        return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
    }
    return z;
}
unsafe extern "C" fn _0d_rdDsvWc(mut a: K, mut b: K) -> K {
    let mut e: I = 0;
    let mut g: C = 0;
    let mut m: S = 0 as *mut C;
    let mut u: I = 0;
    let mut t: I = 0;
    let mut p: I = 0;
    let mut n: I = 0;
    let mut h: I = 0;
    let mut tok: *mut C = 0 as *mut C;
    let mut y: [C; 2] = [0; 2];
    let mut k: K = 0 as *mut k0;
    let mut current_block: u64;
    let mut z: K = 0 as K;
    let mut res: I = 0;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    if an != 2 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut c: K = *((*a).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    let mut d: K = *((*a).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    let mut cn: I = (*c).n;
    if 3 as libc::c_int as libc::c_longlong
        != (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        && 4 as libc::c_int as libc::c_longlong != bt
        && 0 as libc::c_int as libc::c_longlong != bt
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut x: *mut C = ((*d).k).as_mut_ptr() as *mut C;
    let mut w: C = *x;
    let mut fb: I = 0 as libc::c_int as I;
    let mut fn_0: I = 0 as libc::c_int as I;
    let mut s: I = 0;
    if stat_sz(CSK(b), &mut s) != 0 {
        return kerr(strerror(*__errno_location()) as cS);
    }
    if bt != 0 {
        fn_0 = s;
    }
    let mut f: I = open(CSK(b) as *const libc::c_char, 0 as libc::c_int) as I;
    if f < 0 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut v: S = 0 as *mut C;
    v = mmap(
        0 as *mut libc::c_void,
        fn_0 as size_t,
        0x1 as libc::c_int,
        0x1 as libc::c_int,
        f as libc::c_int,
        fb as __off_t,
    ) as S;
    if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
        printf(b"mmap failed\n\0" as *const u8 as *const libc::c_char);
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut r: I = close(f as libc::c_int) as I;
    if r != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    let mut fc: I = 0 as libc::c_int as I;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = cn;
    while i < _i {
        if !(' ' as i32
            == *(((*c).k).as_mut_ptr() as *mut C).offset(i as isize) as libc::c_int)
        {
            if stringHasChar(
                b"IFCS\0" as *const u8 as *const libc::c_char as S,
                *(((*c).k).as_mut_ptr() as *mut C).offset(i as isize),
            ) != 0
            {
                fc += 1;
                fc;
            } else {
                return kerr(b"type\0" as *const u8 as *const libc::c_char)
            }
        }
        i += 1;
        i;
    }
    r = 0 as libc::c_int as I;
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = fn_0;
    while i_0 < _i_0 {
        if *v.offset((fb + i_0) as isize) as libc::c_int == '\n' as i32 {
            r += 1;
            r;
        }
        i_0 += 1;
        i_0;
    }
    if *v.offset((fn_0 - 1 as libc::c_int as libc::c_longlong) as isize) as libc::c_int
        != '\n' as i32
    {
        r += 1;
        r;
    }
    z = newK(0 as libc::c_int as I, 2 as libc::c_int as I);
    if !z.is_null() {
        let ref mut fresh19 = *((*z).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        *fresh19 = newK(-(4 as libc::c_int) as I, fc);
        let ref mut fresh20 = *((*z).k).as_mut_ptr().offset(1 as libc::c_int as isize);
        *fresh20 = newK(0 as libc::c_int as I, fc);
        e = 0 as libc::c_int as I;
        g = 0;
        let mut i_1: I = 0 as libc::c_int as I;
        let mut _i_1: I = cn;
        loop {
            if !(i_1 < _i_1) {
                current_block = 1356832168064818221;
                break;
            }
            g = *(((*c).k).as_mut_ptr() as *mut C).offset(i_1 as isize);
            if !(' ' as i32 == g as libc::c_int) {
                let fresh21 = e;
                e = e + 1;
                let ref mut fresh22 = *((*(*((*z).k)
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as K))
                    .k)
                    .as_mut_ptr()
                    .offset(fresh21 as isize);
                *fresh22 = newK(
                    -charpos(b"CIF S\0" as *const u8 as *const libc::c_char as S, g),
                    r - 1 as libc::c_int as libc::c_longlong,
                );
                if (*fresh22).is_null() {
                    cd(z);
                    z = 0 as K;
                    current_block = 407274859743642017;
                    break;
                }
            }
            i_1 += 1;
            i_1;
        }
        match current_block {
            407274859743642017 => {}
            _ => {
                m = 0 as *mut C;
                u = 0 as libc::c_int as I;
                t = 0 as libc::c_int as I;
                p = 0 as libc::c_int as I;
                n = 0 as libc::c_int as I;
                h = 0 as libc::c_int as I;
                tok = 0 as *mut C;
                y = [0; 2];
                y[0 as libc::c_int as usize] = w;
                k = 0 as *mut k0;
                while u <= fn_0 {
                    while u + t <= fn_0
                        && '\n' as i32 != *v.offset((u + t) as isize) as libc::c_int
                        && *v.offset((u + t) as isize) as libc::c_longlong
                            != 0 as *mut libc::c_void as L
                    {
                        t += 1;
                        t;
                    }
                    let fresh23 = n;
                    n = n + 1;
                    if 0 as libc::c_int as libc::c_longlong == fresh23
                        && (*v.offset((u + t) as isize) as libc::c_int == '\n' as i32
                            || *v.offset((u + t) as isize) as libc::c_longlong
                                == 0 as *mut libc::c_void as L)
                    {
                        h = 0 as libc::c_int as I;
                        e = h;
                        m = strdupn(v.offset(u as isize), t);
                        if m.is_null() {
                            return 0 as K;
                        }
                        if *m.offset(0 as libc::c_int as isize) as libc::c_longlong
                            != 0 as *mut libc::c_void as L
                        {
                            tok = strsep(&mut m, y.as_mut_ptr());
                            let fresh24 = e;
                            e = e + 1;
                            k = *((*z).k).as_mut_ptr().offset(fresh24 as isize);
                            let fresh25 = h;
                            h = h + 1;
                            if *(((*c).k).as_mut_ptr() as *mut C)
                                .offset(fresh25 as isize) as libc::c_int == ' ' as i32
                            {
                                e -= 1;
                                e;
                            } else {
                                let fresh26 = p;
                                p = p + 1;
                                let ref mut fresh27 = *(((*(*((*z).k)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize) as K))
                                    .k)
                                    .as_mut_ptr() as *mut S)
                                    .offset(fresh26 as isize);
                                *fresh27 = sp(tok);
                            }
                            while !tok.is_null() {
                                tok = strsep(&mut m, y.as_mut_ptr());
                                if !tok.is_null() {
                                    let fresh28 = e;
                                    e = e + 1;
                                    k = *((*z).k).as_mut_ptr().offset(fresh28 as isize);
                                    let fresh29 = h;
                                    h = h + 1;
                                    if *(((*c).k).as_mut_ptr() as *mut C)
                                        .offset(fresh29 as isize) as libc::c_int == ' ' as i32
                                    {
                                        e -= 1;
                                        e;
                                    } else {
                                        let fresh30 = p;
                                        p = p + 1;
                                        let ref mut fresh31 = *(((*(*((*z).k)
                                            .as_mut_ptr()
                                            .offset(0 as libc::c_int as isize) as K))
                                            .k)
                                            .as_mut_ptr() as *mut S)
                                            .offset(fresh30 as isize);
                                        *fresh31 = sp(tok);
                                    }
                                }
                            }
                        }
                        free(m as *mut libc::c_void);
                        p = 0 as libc::c_int as I;
                    }
                    if n > 1 as libc::c_int as libc::c_longlong
                        && (*v.offset((u + t) as isize) as libc::c_int == '\n' as i32
                            || *v.offset((u + t) as isize) as libc::c_longlong
                                == 0 as *mut libc::c_void as L)
                    {
                        let mut q: K = 0 as K;
                        h = 0 as libc::c_int as I;
                        e = h;
                        m = strdupn(v.offset(u as isize), t);
                        if m.is_null() {
                            return 0 as K;
                        }
                        if *m.offset(0 as libc::c_int as isize) as libc::c_longlong
                            != 0 as *mut libc::c_void as L
                        {
                            tok = strsep(&mut m, y.as_mut_ptr());
                            let fresh32 = e;
                            e = e + 1;
                            k = *((*(*((*z).k)
                                .as_mut_ptr()
                                .offset(1 as libc::c_int as isize) as K))
                                .k)
                                .as_mut_ptr()
                                .offset(fresh32 as isize);
                            let fresh33 = h;
                            h = h + 1;
                            match *(((*c).k).as_mut_ptr() as *mut C)
                                .offset(fresh33 as isize) as libc::c_int
                            {
                                32 => {
                                    e -= 1;
                                    e;
                                }
                                73 => {
                                    q = formKiCS(tok);
                                    *(((*k).k).as_mut_ptr() as *mut I)
                                        .offset(
                                            p as isize,
                                        ) = if !q.is_null() {
                                        *(((*q).k).as_mut_ptr() as *mut I)
                                    } else {
                                        -(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong
                                    };
                                }
                                70 => {
                                    q = formKfCS(tok);
                                    *(((*k).k).as_mut_ptr() as *mut F)
                                        .offset(
                                            p as isize,
                                        ) = if !q.is_null() {
                                        *(((*q).k).as_mut_ptr() as *mut F)
                                    } else {
                                        0 as libc::c_int as libc::c_double / 0.0f64
                                    };
                                }
                                67 => {
                                    n = strlen(tok) as I;
                                    q = newK(-(3 as libc::c_int) as I, n);
                                    if n == 0 {
                                        n += 1;
                                        n;
                                    }
                                    memcpy(
                                        ((*q).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                                        tok as *const libc::c_void,
                                        n as libc::c_ulong,
                                    );
                                    let ref mut fresh34 = *((*k).k)
                                        .as_mut_ptr()
                                        .offset(p as isize);
                                    *fresh34 = q;
                                    q = 0 as K;
                                }
                                83 => {
                                    let ref mut fresh35 = *(((*k).k).as_mut_ptr() as *mut S)
                                        .offset(p as isize);
                                    *fresh35 = sp(tok);
                                }
                                _ => {}
                            }
                            while !tok.is_null() {
                                tok = strsep(&mut m, y.as_mut_ptr());
                                if !tok.is_null() {
                                    let fresh36 = e;
                                    e = e + 1;
                                    k = *((*(*((*z).k)
                                        .as_mut_ptr()
                                        .offset(1 as libc::c_int as isize) as K))
                                        .k)
                                        .as_mut_ptr()
                                        .offset(fresh36 as isize);
                                    let fresh37 = h;
                                    h = h + 1;
                                    match *(((*c).k).as_mut_ptr() as *mut C)
                                        .offset(fresh37 as isize) as libc::c_int
                                    {
                                        32 => {
                                            e -= 1;
                                            e;
                                        }
                                        73 => {
                                            q = formKiCS(tok);
                                            *(((*k).k).as_mut_ptr() as *mut I)
                                                .offset(
                                                    p as isize,
                                                ) = if !q.is_null() {
                                                *(((*q).k).as_mut_ptr() as *mut I)
                                            } else {
                                                -(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong
                                            };
                                        }
                                        70 => {
                                            q = formKfCS(tok);
                                            *(((*k).k).as_mut_ptr() as *mut F)
                                                .offset(
                                                    p as isize,
                                                ) = if !q.is_null() {
                                                *(((*q).k).as_mut_ptr() as *mut F)
                                            } else {
                                                0 as libc::c_int as libc::c_double / 0.0f64
                                            };
                                        }
                                        67 => {
                                            n = strlen(tok) as I;
                                            q = newK(-(3 as libc::c_int) as I, n);
                                            if n == 0 {
                                                n += 1;
                                                n;
                                            }
                                            memcpy(
                                                ((*q).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                                                tok as *const libc::c_void,
                                                n as libc::c_ulong,
                                            );
                                            let ref mut fresh38 = *((*k).k)
                                                .as_mut_ptr()
                                                .offset(p as isize);
                                            *fresh38 = q;
                                            q = 0 as K;
                                        }
                                        83 => {
                                            let ref mut fresh39 = *(((*k).k).as_mut_ptr() as *mut S)
                                                .offset(p as isize);
                                            *fresh39 = sp(tok);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        free(m as *mut libc::c_void);
                        p += 1;
                        p;
                    }
                    u += t + 1 as libc::c_int as libc::c_longlong;
                    t = 0 as libc::c_int as I;
                }
            }
        }
    }
    res = munmap(v as *mut libc::c_void, s as size_t) as I;
    if res != 0 {
        return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
    }
    return z;
}
pub unsafe extern "C" fn _1m(mut x: K) -> K {
    if 4 as libc::c_int as libc::c_longlong != (*x).t
        && 3 as libc::c_int as libc::c_longlong
            != (if (*x).t < 0 as libc::c_int as libc::c_longlong {
                -(*x).t
            } else {
                (*x).t
            })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut m: S = CSK(x);
    let mut sm: I = strlen(m as *const libc::c_char) as I;
    let mut e: S = if sm > 1 as libc::c_int as libc::c_longlong
        && '.' as i32
            == *m.offset((sm - 2 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
        && *(b"K\0" as *const u8 as *const libc::c_char) as libc::c_int
            == *m.offset((sm - 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
    {
        strdupn(m, sm)
    } else {
        glueSS(m, b"K\0" as *const u8 as *const libc::c_char as S)
    };
    if e.is_null() {
        return 0 as K;
    }
    let mut c: stat = stat {
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
    let mut f: I = open(e as *const libc::c_char, 0o2 as libc::c_int) as I;
    if f >= 0 as libc::c_int as libc::c_longlong {
        stat(e as *const libc::c_char, &mut c);
    } else {
        f = open(m as *const libc::c_char, 0o2 as libc::c_int) as I;
        stat(m as *const libc::c_char, &mut c);
    }
    free(e as *mut libc::c_void);
    if f < 0 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut s: I = c.st_size as I;
    let mut r: I = 0;
    if (s as libc::c_ulonglong)
        < (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong)
            as libc::c_ulonglong
    {
        r = close(f as libc::c_int) as I;
        if r != 0 {
            return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
        }
        return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
    }
    let mut v: S = 0 as *mut C;
    v = mmap(
        0 as *mut libc::c_void,
        s as size_t,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x2 as libc::c_int | 0x4000 as libc::c_int,
        f as libc::c_int,
        0 as libc::c_int as __off_t,
    ) as S;
    if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut b: I = 0 as libc::c_int as I;
    let mut z: K = _1m_r(f, v as V, v as V, v.offset(s as isize) as V, &mut b);
    r = close(f as libc::c_int) as I;
    if r != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    r = munmap(v as *mut libc::c_void, s as size_t) as I;
    if r != 0 {
        return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
    }
    return z;
}
unsafe extern "C" fn _1m_r(
    mut f: I,
    mut fixed: V,
    mut v: V,
    mut aft: V,
    mut b: *mut I,
) -> K {
    let mut s: I = aft.offset_from(v) as libc::c_long as I;
    if (s as libc::c_ulonglong)
        < (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong)
            as libc::c_ulonglong
    {
        return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
    }
    let mut w: *mut I = v as *mut I;
    let mut t: I = *w.offset(2 as libc::c_int as isize);
    let mut n: I = *w.offset(3 as libc::c_int as isize);
    if t < -(4 as libc::c_int) as libc::c_longlong
        || t > 7 as libc::c_int as libc::c_longlong
        || n < 0 as libc::c_int as libc::c_longlong
    {
        return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
    }
    if 4 as libc::c_int as libc::c_longlong
        == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
        || 7 as libc::c_int as libc::c_longlong == t
        || 1 as libc::c_int as libc::c_longlong <= t
            && t <= 3 as libc::c_int as libc::c_longlong
        || 6 as libc::c_int as libc::c_longlong == t
    {
        return _2m_r(v, aft, b);
    }
    let mut r: I = (4 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong) as I;
    if 0 as libc::c_int as libc::c_longlong != t
        && 5 as libc::c_int as libc::c_longlong != t
    {
        r = (r as libc::c_ulonglong)
            .wrapping_add(
                ((bp(t) * n
                    + (-(3 as libc::c_int) as libc::c_longlong == t) as libc::c_int
                        as libc::c_longlong) as libc::c_ulonglong)
                    .wrapping_sub(
                        ((t > 0 as libc::c_int as libc::c_longlong) as libc::c_int
                            as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong)
                            as libc::c_ulonglong,
                    ),
            ) as I as I;
    }
    if r % 8 as libc::c_int as libc::c_longlong > 0 as libc::c_int as libc::c_longlong {
        r
            += 8 as libc::c_int as libc::c_longlong
                - r % 8 as libc::c_int as libc::c_longlong;
    }
    if s < r {
        return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = 0 as *mut k0;
    let mut x: K = 0 as *mut k0;
    if 0 as libc::c_int as libc::c_longlong == t
        || 5 as libc::c_int as libc::c_longlong == t
    {
        z = newK(t, n);
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            x = _1m_r(f, fixed, v.offset(r as isize), aft, &mut r);
            if x.is_null() {
                cd(z);
                return 0 as K;
            }
            let ref mut fresh40 = *((*z).k).as_mut_ptr().offset(i as isize);
            *fresh40 = x;
            i += 1;
            i;
        }
    } else {
        let mut u: S = 0 as *mut C;
        let mut length: I = r;
        let mut offset: I = (v.offset_from(fixed) as libc::c_long as libc::c_ulong)
            .wrapping_add(
                ((if t > 0 as libc::c_int as libc::c_longlong {
                    3 as libc::c_int
                } else {
                    4 as libc::c_int
                }) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong),
            ) as I;
        let mut mod_0: I = offset;
        length += mod_0;
        offset -= mod_0;
        u = mmap(
            0 as *mut libc::c_void,
            length as size_t,
            0x1 as libc::c_int | 0x2 as libc::c_int,
            0x2 as libc::c_int | 0x4000 as libc::c_int,
            f as libc::c_int,
            offset as __off_t,
        ) as S;
        if -(1 as libc::c_int) as *mut libc::c_void == u as *mut libc::c_void {
            return kerr(strerror(*__errno_location()) as cS);
        }
        mMap += length as libc::c_double;
        mUsed += length as libc::c_double;
        if mUsed > mMax {
            mMax = mUsed;
        }
        z = (u as V)
            .offset(mod_0 as isize)
            .offset(
                -((3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong) as isize),
            ) as K;
        mrc(z, 1 as libc::c_int as I);
    }
    *b = (*b as libc::c_ulonglong)
        .wrapping_add(
            if r as libc::c_ulonglong
                > (4 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong)
                    as libc::c_ulonglong
            {
                r as libc::c_ulonglong
            } else {
                (4 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong)
                    as libc::c_ulonglong
            },
        ) as I as I;
    return z;
}
pub unsafe extern "C" fn _1d(mut x: K, mut y: K) -> K {
    let mut t: I = (*x).t;
    if 4 as libc::c_int as libc::c_longlong == t
        || -(3 as libc::c_int) as libc::c_longlong == t
    {
        return _1d_write(x, y, 0 as libc::c_int as I);
    }
    if t == 0 {
        return _1d_read(x, y);
    }
    if 3 as libc::c_int as libc::c_longlong == t {
        return _1d_char(x, y);
    }
    return kerr(b"type\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn _1d_write(mut x: K, mut y: K, mut dosync: I) -> K {
    let mut n: I = disk(y);
    let mut m: S = CSK(x);
    let mut sm: I = strlen(m as *const libc::c_char) as I;
    let mut e: S = if sm > 1 as libc::c_int as libc::c_longlong
        && '.' as i32
            == *m.offset((sm - 2 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
        && *(b"K\0" as *const u8 as *const libc::c_char) as libc::c_int
            == *m.offset((sm - 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
    {
        strdupn(m, sm)
    } else {
        glueSS(m, b"K\0" as *const u8 as *const libc::c_char as S)
    };
    if e.is_null() {
        return 0 as K;
    }
    let mut f: I = open(
        e as *const libc::c_char,
        0o2 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
        0o7777 as libc::c_int,
    ) as I;
    free(e as *mut libc::c_void);
    if f < 0 as libc::c_int as libc::c_longlong {
        return kerr(strerror(*__errno_location()) as cS);
    }
    if ftruncate(f as libc::c_int, n as __off_t) != 0 {
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut v: S = 0 as *mut C;
    v = mmap(
        0 as *mut libc::c_void,
        n as size_t,
        0x2 as libc::c_int,
        0x1 as libc::c_int,
        f as libc::c_int,
        0 as libc::c_int as __off_t,
    ) as S;
    if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut r: I = close(f as libc::c_int) as I;
    if r != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    wrep(y, v as V, 1 as libc::c_int as I);
    if dosync != 0 {
        msync(v as *mut libc::c_void, n as size_t, 4 as libc::c_int | 2 as libc::c_int);
    }
    r = munmap(v as *mut libc::c_void, n as size_t) as I;
    if r != 0 {
        return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
    }
    return _n();
}
pub unsafe extern "C" fn wrep(mut x: K, mut v: V, mut y: I) -> I {
    let mut t: I = (*x).t;
    let mut n: I = (*x).n;
    let mut w: *mut I = v as *mut I;
    let mut m: I = (if y != 0 { 2 as libc::c_int } else { 0 as libc::c_int }) as I;
    if y != 0 {
        *w = -(3 as libc::c_int) as I;
        *w.offset(1 as libc::c_int as isize) = 1 as libc::c_int as I;
        *w.offset(2 as libc::c_int as isize) = t;
        *w.offset(3 as libc::c_int as isize) = n;
    } else {
        memcpy(
            w as *mut libc::c_void,
            &mut (*x).t as *mut I as *const libc::c_void,
            (::std::mem::size_of::<I>() as libc::c_ulong)
                .wrapping_add(::std::mem::size_of::<I>() as libc::c_ulong),
        );
    }
    let mut d: V = w.offset(2 as libc::c_int as isize).offset(m as isize) as V;
    let mut e: I = ((2 as libc::c_int as libc::c_longlong + m) as libc::c_ulonglong)
        .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong)
        as I;
    if 0 as libc::c_int as libc::c_longlong != t
        && 5 as libc::c_int as libc::c_longlong != t
        && -(4 as libc::c_int) as libc::c_longlong != t
    {
        e = rep(x, y);
    }
    let mut r: I = 0 as libc::c_int as I;
    let mut s: I = 0;
    if 0 as libc::c_int as libc::c_longlong == t
        || 5 as libc::c_int as libc::c_longlong == t
    {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            let mut point: V = d.offset(r as isize);
            let mut delta: I = wrep(*((*x).k).as_mut_ptr().offset(i as isize), point, y);
            r += delta;
            i += 1;
            i;
        }
    } else if -(4 as libc::c_int) as libc::c_longlong == t {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = n;
        while i_0 < _i_0 {
            s = (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    strlen(
                        *(((*x).k).as_mut_ptr() as *mut S).offset(i_0 as isize)
                            as *const libc::c_char,
                    ),
                ) as I;
            memcpy(
                d.offset(r as isize),
                *(((*x).k).as_mut_ptr() as *mut S).offset(i_0 as isize)
                    as *const libc::c_void,
                s as libc::c_ulong,
            );
            r += s;
            i_0 += 1;
            i_0;
        }
        if r % 8 as libc::c_int as libc::c_longlong
            > 0 as libc::c_int as libc::c_longlong
        {
            r
                += 8 as libc::c_int as libc::c_longlong
                    - r % 8 as libc::c_int as libc::c_longlong;
        }
    } else if '\u{7}' as i32 as libc::c_longlong == t
        || '\u{8}' as i32 as libc::c_longlong == t
    {
        if 1 as libc::c_int as libc::c_longlong == (*x).n
            && 1 as libc::c_int as libc::c_longlong
                == (*(*(((*x).k).as_mut_ptr() as *mut V)
                    .offset(CODE as libc::c_int as isize) as K))
                    .n - 1 as libc::c_int as libc::c_longlong
            && offsetColon
                == *(((*(*((*x).k).as_mut_ptr().offset(CODE as libc::c_int as isize)
                    as K))
                    .k)
                    .as_mut_ptr() as *mut S)
                    .offset(0 as libc::c_int as isize) as V
        {
            let mut k: K = *(((*(*(((*x).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize) as K))
                .k)
                .as_mut_ptr() as *mut S as *mut V) as K;
            let mut s_0: I = sva(k as V);
            *w
                .offset(
                    m as isize,
                ) = (if 1 as libc::c_int as libc::c_longlong == s_0 {
                '\u{7}' as i32
            } else {
                '\u{8}' as i32
            }) as I;
            *w
                .offset(
                    (1 as libc::c_int as libc::c_longlong + m) as isize,
                ) = offsetColon as L;
        } else {
            return kerr(b"syntax\0" as *const u8 as *const libc::c_char) as L
        }
    } else {
        let mut s_1: V = ((*x).k).as_mut_ptr() as V;
        let mut b: I = n * bp(t)
            + (3 as libc::c_int as libc::c_longlong
                == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t }))
                as libc::c_int as libc::c_longlong;
        if t > 0 as libc::c_int as libc::c_longlong {
            d = d.offset(-(::std::mem::size_of::<I>() as libc::c_ulong as isize));
        }
        if 4 as libc::c_int as libc::c_longlong == t {
            s_1 = *(((*x).k).as_mut_ptr() as *mut S) as V;
            b = (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    strlen(*(((*x).k).as_mut_ptr() as *mut S) as *const libc::c_char),
                ) as I;
        }
        if b % 8 as libc::c_int as libc::c_longlong
            > 0 as libc::c_int as libc::c_longlong
        {
            b = 8 as libc::c_int as libc::c_longlong
                * (b / 8 as libc::c_int as libc::c_longlong)
                + (1 as libc::c_int as libc::c_longlong
                    + b % 8 as libc::c_int as libc::c_longlong);
        }
        memcpy(d, s_1 as *const libc::c_void, b as libc::c_ulong);
    }
    return e + r;
}
unsafe extern "C" fn disk(mut x: K) -> I {
    return rep(x, 1 as libc::c_int as I);
}
pub unsafe extern "C" fn rep(mut x: K, mut y: I) -> I {
    let mut m: I = (::std::mem::size_of::<I>() as libc::c_ulong)
        .wrapping_mul(
            (if y != 0 { 4 as libc::c_int } else { 2 as libc::c_int }) as libc::c_ulong,
        ) as I;
    let mut r: I = m;
    let mut n: I = (*x).n;
    let mut q: I = 0 as libc::c_int as I;
    match (*x).t {
        0 | 5 => {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (*x).n;
            while i < _i {
                r += rep(*((*x).k).as_mut_ptr().offset(i as isize), y);
                i += 1;
                i;
            }
        }
        7 | 8 => {
            1 as libc::c_int as libc::c_longlong == (*x).n;
        }
        -4 => {
            let mut i_0: I = 0 as libc::c_int as I;
            let mut _i_0: I = n;
            while i_0 < _i_0 {
                r = (r as libc::c_ulonglong)
                    .wrapping_add(
                        (1 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                strlen(
                                    *(((*x).k).as_mut_ptr() as *mut S).offset(i_0 as isize)
                                        as *const libc::c_char,
                                ),
                            ) as libc::c_ulonglong,
                    ) as I as I;
                i_0 += 1;
                i_0;
            }
            if r % 8 as libc::c_int as libc::c_longlong
                > 0 as libc::c_int as libc::c_longlong
            {
                r
                    += 8 as libc::c_int as libc::c_longlong
                        - r % 8 as libc::c_int as libc::c_longlong;
            }
        }
        -3 => {
            r = (r as libc::c_ulonglong)
                .wrapping_add(
                    ((1 as libc::c_int as libc::c_longlong + n) as libc::c_ulonglong)
                        .wrapping_mul(
                            ::std::mem::size_of::<C>() as libc::c_ulong
                                as libc::c_ulonglong,
                        ),
                ) as I as I;
            if r % 8 as libc::c_int as libc::c_longlong
                > 0 as libc::c_int as libc::c_longlong
            {
                r
                    += 8 as libc::c_int as libc::c_longlong
                        - r % 8 as libc::c_int as libc::c_longlong;
            }
        }
        -2 => {
            r = (r as libc::c_ulonglong)
                .wrapping_add(
                    (n as libc::c_ulonglong)
                        .wrapping_mul(
                            ::std::mem::size_of::<F>() as libc::c_ulong
                                as libc::c_ulonglong,
                        ),
                ) as I as I;
        }
        -1 => {
            r = (r as libc::c_ulonglong)
                .wrapping_add(
                    (n as libc::c_ulonglong)
                        .wrapping_mul(
                            ::std::mem::size_of::<I>() as libc::c_ulong
                                as libc::c_ulonglong,
                        ),
                ) as I as I;
        }
        4 => {
            q = (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    strlen(*(((*x).k).as_mut_ptr() as *mut S) as *const libc::c_char),
                ) as I;
            if q as libc::c_ulonglong
                >= ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong
            {
                r = (r as libc::c_ulonglong)
                    .wrapping_add(
                        (q as libc::c_ulonglong)
                            .wrapping_sub(
                                ::std::mem::size_of::<I>() as libc::c_ulong
                                    as libc::c_ulonglong,
                            ),
                    ) as I as I;
            }
        }
        _ => {}
    }
    return if r > m { r } else { m };
}
pub unsafe extern "C" fn rrep(
    mut v: V,
    mut aft: V,
    mut b: *mut I,
    mut y: I,
    mut x: I,
) -> K {
    let mut m: I = (if y != 0 { 2 as libc::c_int } else { 0 as libc::c_int }) as I;
    let mut s: I = aft.offset_from(v) as libc::c_long as I;
    let mut w: *mut I = v as *mut I;
    let mut r: I = ((2 as libc::c_int as libc::c_longlong + m) as libc::c_ulonglong)
        .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong)
        as I;
    if s < r {
        return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
    }
    if y != 0 {
        if -(3 as libc::c_int) as libc::c_longlong
            != *w.offset(0 as libc::c_int as isize)
        {
            return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
        }
    }
    let mut t: I = 0;
    membswpI(
        &mut t as *mut I as V,
        w.offset(m as isize) as V,
        ::std::mem::size_of::<I>() as libc::c_ulong as I,
        x,
    );
    let mut n: I = 0;
    if t <= 0 as libc::c_int as libc::c_longlong
        || 5 as libc::c_int as libc::c_longlong == t
    {
        membswpI(
            &mut n as *mut I as V,
            w.offset(1 as libc::c_int as isize).offset(m as isize) as V,
            ::std::mem::size_of::<I>() as libc::c_ulong as I,
            x,
        );
    } else if !('\n' as i32 as libc::c_longlong == t) {
        n = 1 as libc::c_int as I;
    }
    if -(1 as libc::c_int) as libc::c_longlong == t {
        r = (r as libc::c_ulonglong)
            .wrapping_add(
                (n as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
                    ),
            ) as I as I;
    } else if -(2 as libc::c_int) as libc::c_longlong == t {
        r = (r as libc::c_ulonglong)
            .wrapping_add(
                (n as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<F>() as libc::c_ulong as libc::c_ulonglong,
                    ),
            ) as I as I;
    } else if -(3 as libc::c_int) as libc::c_longlong == t {
        r = (r as libc::c_ulonglong)
            .wrapping_add(
                ((1 as libc::c_int as libc::c_longlong + n) as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<C>() as libc::c_ulong as libc::c_ulonglong,
                    ),
            ) as I as I;
    }
    if s < r {
        return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
    }
    let mut f: K = 0 as *mut k0;
    let mut z: K = if -(4 as libc::c_int) as libc::c_longlong <= t
        && t <= 6 as libc::c_int as libc::c_longlong
    {
        newK(t, n)
    } else {
        Kv()
    };
    if z.is_null() {
        return 0 as K;
    }
    let mut c: I = 0 as libc::c_int as I;
    match t {
        0 | 5 => {
            while v.offset(r as isize) < aft && c < n {
                let mut k: K = rrep(v.offset(r as isize), aft, &mut r, y, x);
                if OOM_CD(0 as libc::c_int as I, z, k, -(1 as libc::c_int) as V) == 0 {
                    return 0 as K;
                }
                let fresh41 = c;
                c = c + 1;
                memcpy(
                    &mut *((*z).k).as_mut_ptr().offset(fresh41 as isize) as *mut *mut k0
                        as *mut libc::c_void,
                    &mut k as *mut K as *const libc::c_void,
                    ::std::mem::size_of::<K>() as libc::c_ulong,
                );
            }
            if c != n {
                cd(z);
                return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
            }
        }
        -4 => {
            while v.offset(r as isize) < aft && c < n {
                let fresh42 = c;
                c = c + 1;
                r
                    += rrep_4(
                        (((*z).k).as_mut_ptr() as *mut S).offset(fresh42 as isize),
                        v.offset(r as isize) as S,
                        aft as S,
                    );
            }
            if c != n {
                return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
            }
        }
        -3 => {
            memcpy(
                ((*z).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                w.offset(2 as libc::c_int as isize).offset(m as isize)
                    as *const libc::c_void,
                (n as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<C>() as libc::c_ulong as libc::c_ulonglong,
                    ) as libc::c_ulong,
            );
        }
        -2 => {
            membswpF(
                ((*z).k).as_mut_ptr() as *mut F as V,
                w.offset(2 as libc::c_int as isize).offset(m as isize) as V,
                (n as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<F>() as libc::c_ulong as libc::c_ulonglong,
                    ) as I,
                x,
            );
        }
        -1 => {
            membswpI(
                ((*z).k).as_mut_ptr() as *mut I as V,
                w.offset(2 as libc::c_int as isize).offset(m as isize) as V,
                (n as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
                    ) as I,
                x,
            );
        }
        1 => {
            membswpI(
                ((*z).k).as_mut_ptr() as *mut I as V,
                w.offset(1 as libc::c_int as isize).offset(m as isize) as V,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong) as I,
                x,
            );
        }
        2 => {
            membswpF(
                ((*z).k).as_mut_ptr() as *mut F as V,
                w.offset(1 as libc::c_int as isize).offset(m as isize) as V,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<F>() as libc::c_ulong) as I,
                x,
            );
        }
        3 => {
            memcpy(
                ((*z).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                w.offset(1 as libc::c_int as isize).offset(m as isize)
                    as *const libc::c_void,
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<C>() as libc::c_ulong),
            );
        }
        4 => {
            r = (r as libc::c_ulonglong)
                .wrapping_add(
                    (rrep_4(
                        ((*z).k).as_mut_ptr() as *mut S,
                        w.offset(1 as libc::c_int as isize).offset(m as isize) as S,
                        aft as S,
                    ) as libc::c_ulonglong)
                        .wrapping_sub(
                            ::std::mem::size_of::<I>() as libc::c_ulong
                                as libc::c_ulonglong,
                        ),
                ) as I as I;
        }
        6 => {}
        7 | 8 => {
            f = newK(-(4 as libc::c_int) as I, 2 as libc::c_int as I);
            if OOM_CD(0 as libc::c_int as I, z, f, -(1 as libc::c_int) as V) == 0 {
                return 0 as K;
            }
            let ref mut fresh43 = *(((*z).k).as_mut_ptr() as *mut V)
                .offset(CODE as libc::c_int as isize);
            *fresh43 = f as V;
            if x != 0 {
                *w
                    .offset(
                        (1 as libc::c_int as libc::c_longlong + m) as isize,
                    ) = bswapI(
                    *w.offset((1 as libc::c_int as libc::c_longlong + m) as isize),
                );
            }
            let ref mut fresh44 = *((*f).k).as_mut_ptr();
            *fresh44 = *w.offset((1 as libc::c_int as libc::c_longlong + m) as isize)
                as V as *mut k0;
            r += 0 as libc::c_int as libc::c_longlong;
        }
        _ => return kerr(b"nonce\0" as *const u8 as *const libc::c_char),
    }
    *b = (*b as libc::c_ulonglong)
        .wrapping_add(
            if r as libc::c_ulonglong
                > ((2 as libc::c_int as libc::c_longlong + m) as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
                    )
            {
                r as libc::c_ulonglong
            } else {
                ((2 as libc::c_int as libc::c_longlong + m) as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
                    )
            },
        ) as I as I;
    *b = 8 as libc::c_int as libc::c_longlong
        * (*b / 8 as libc::c_int as libc::c_longlong
            + (*b % 8 as libc::c_int as libc::c_longlong
                > 0 as libc::c_int as libc::c_longlong) as libc::c_int
                as libc::c_longlong);
    return z;
}
unsafe extern "C" fn rrep_4(mut z: *mut S, mut a: S, mut t: S) -> I {
    let mut d: S = a;
    while a < t && *a as libc::c_int != 0 {
        a = a.offset(1);
        a;
    }
    let mut c: I = a.offset_from(d) as libc::c_long as I;
    let mut e: S = strdupn(d, c);
    *z = sp(e);
    free(e as *mut libc::c_void);
    return c + (a != t) as libc::c_int as libc::c_longlong;
}
unsafe extern "C" fn _1d_read(mut a: K, mut b: K) -> K {
    let mut types: S = b"cbsijfdmIFCSDZM\0" as *const u8 as *const libc::c_char as S;
    let mut fixed: [I; 8] = [
        ::std::mem::size_of::<C>() as libc::c_ulong as I,
        ::std::mem::size_of::<int8_t>() as libc::c_ulong as I,
        ::std::mem::size_of::<int16_t>() as libc::c_ulong as I,
        ::std::mem::size_of::<int32_t>() as libc::c_ulong as I,
        ::std::mem::size_of::<I>() as libc::c_ulong as I,
        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong as I,
        ::std::mem::size_of::<F>() as libc::c_ulong as I,
        9 as libc::c_int as I,
    ];
    let mut typelist: [I; 15] = [
        -(3 as libc::c_int) as I,
        -(1 as libc::c_int) as I,
        -(1 as libc::c_int) as I,
        -(1 as libc::c_int) as I,
        -(1 as libc::c_int) as I,
        -(2 as libc::c_int) as I,
        -(2 as libc::c_int) as I,
        9 as libc::c_int as I,
        9 as libc::c_int as I,
        9 as libc::c_int as I,
        0 as libc::c_int as I,
        -(4 as libc::c_int) as I,
        9 as libc::c_int as I,
        9 as libc::c_int as I,
        9 as libc::c_int as I,
    ];
    let mut g: C = 0;
    let mut z: K = 0 as K;
    let mut an: I = (*a).n;
    let mut bt: I = (*b).t;
    let mut bn: I = (*b).n;
    if an != 2 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut c: K = *((*a).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    let mut d: K = *((*a).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    let mut cn: I = (*c).n;
    let mut dn: I = (*d).n;
    if 3 as libc::c_int as libc::c_longlong
        != (if (*c).t < 0 as libc::c_int as libc::c_longlong { -(*c).t } else { (*c).t })
        || 1 as libc::c_int as libc::c_longlong
            != (if (*d).t < 0 as libc::c_int as libc::c_longlong {
                -(*d).t
            } else {
                (*d).t
            })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    if cn == 0 || cn != dn {
        return kerr(b"length\0" as *const u8 as *const libc::c_char);
    }
    if 3 as libc::c_int as libc::c_longlong
        != (if bt < 0 as libc::c_int as libc::c_longlong { -bt } else { bt })
        && 4 as libc::c_int as libc::c_longlong != bt
        && 0 as libc::c_int as libc::c_longlong != bt
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut fc: I = 0 as libc::c_int as I;
    let mut i: I = 0 as libc::c_int as I;
    let mut _i: I = cn;
    while i < _i {
        g = *(((*c).k).as_mut_ptr() as *mut C).offset(i as isize);
        if !(' ' as i32 == g as libc::c_int) {
            if stringHasChar(types, g) != 0 {
                fc += 1;
                fc;
            } else {
                return kerr(b"type\0" as *const u8 as *const libc::c_char)
            }
            if *(*__ctype_b_loc()).offset(g as libc::c_int as isize) as libc::c_int
                & _ISlower as libc::c_int as libc::c_ushort as libc::c_int != 0
                && fixed[charpos(types, g) as usize]
                    != *(((*d).k).as_mut_ptr() as *mut I).offset(i as isize)
            {
                return kerr(b"length\0" as *const u8 as *const libc::c_char);
            }
        }
        i += 1;
        i;
    }
    let mut w: I = 0 as libc::c_int as I;
    let mut x: I = 0;
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = dn;
    while i_0 < _i_0 {
        x = *(((*d).k).as_mut_ptr() as *mut I).offset(i_0 as isize);
        if x <= 0 as libc::c_int as libc::c_longlong {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        w += x;
        i_0 += 1;
        i_0;
    }
    let mut ff: K = b;
    let mut k: K = 0 as *mut k0;
    let mut fb: I = 0 as libc::c_int as I;
    let mut fn_0: I = 0 as libc::c_int as I;
    if bt == 0 {
        if 3 as libc::c_int as libc::c_longlong != bn {
            return kerr(b"length\0" as *const u8 as *const libc::c_char);
        }
        ff = *((*b).k).as_mut_ptr().offset(0 as libc::c_int as isize);
        if 3 as libc::c_int as libc::c_longlong
            != (if (*ff).t < 0 as libc::c_int as libc::c_longlong {
                -(*ff).t
            } else {
                (*ff).t
            }) && 4 as libc::c_int as libc::c_longlong != (*ff).t
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        k = *((*b).k).as_mut_ptr().offset(1 as libc::c_int as isize);
        if 1 as libc::c_int as libc::c_longlong != (*k).t
            && 2 as libc::c_int as libc::c_longlong != (*k).t
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        fb = (if (*k).t - 1 as libc::c_int as libc::c_longlong != 0 {
            *(((*k).k).as_mut_ptr() as *mut F)
        } else {
            *(((*k).k).as_mut_ptr() as *mut I) as libc::c_double
        }) as I;
        k = *((*b).k).as_mut_ptr().offset(2 as libc::c_int as isize);
        if 1 as libc::c_int as libc::c_longlong != (*k).t
            && 2 as libc::c_int as libc::c_longlong != (*k).t
        {
            return kerr(b"type\0" as *const u8 as *const libc::c_char);
        }
        fn_0 = (if (*k).t - 1 as libc::c_int as libc::c_longlong != 0 {
            *(((*k).k).as_mut_ptr() as *mut F)
        } else {
            *(((*k).k).as_mut_ptr() as *mut I) as libc::c_double
        }) as I;
    }
    let mut s: I = 0;
    if stat_sz(CSK(ff), &mut s) != 0 {
        return kerr(strerror(*__errno_location()) as cS);
    }
    if bt != 0 {
        fn_0 = s;
    }
    if fn_0 < 0 as libc::c_int as libc::c_longlong {
        fn_0 = 0 as libc::c_int as I;
    }
    if fb < 0 as libc::c_int as libc::c_longlong {
        fb = 0 as libc::c_int as I;
    }
    fb = if fb
        < (if 0 as libc::c_int as libc::c_longlong
            > s - 1 as libc::c_int as libc::c_longlong
        {
            0 as libc::c_int as libc::c_longlong
        } else {
            s - 1 as libc::c_int as libc::c_longlong
        })
    {
        fb
    } else if 0 as libc::c_int as libc::c_longlong
        > s - 1 as libc::c_int as libc::c_longlong
    {
        0 as libc::c_int as libc::c_longlong
    } else {
        s - 1 as libc::c_int as libc::c_longlong
    };
    if fb + fn_0 > s {
        fn_0 = s - fb;
    }
    let mut f: I = open(CSK(ff) as *const libc::c_char, 0 as libc::c_int) as I;
    if f < 0 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut v: S = 0 as *mut C;
    let mut fb_off_by: I = fb % PG;
    let mut map_length: I = fn_0 + fb_off_by;
    let mut map_offset: I = fb - fb_off_by;
    v = mmap(
        0 as *mut libc::c_void,
        map_length as size_t,
        0x1 as libc::c_int,
        0x1 as libc::c_int,
        f as libc::c_int,
        map_offset as __off_t,
    ) as S;
    if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut r: I = close(f as libc::c_int) as I;
    if r != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    r = fn_0 / w;
    z = newK(0 as libc::c_int as I, fc);
    if z.is_null() {
        return 0 as K;
    }
    let mut e: I = 0 as libc::c_int as I;
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_1: I = cn;
    while i_1 < _i_1 {
        g = *(((*c).k).as_mut_ptr() as *mut C).offset(i_1 as isize);
        if !(' ' as i32 == g as libc::c_int) {
            let fresh45 = e;
            e = e + 1;
            let ref mut fresh46 = *((*z).k).as_mut_ptr().offset(fresh45 as isize);
            *fresh46 = newK(typelist[charpos(types, g) as usize], r);
        }
        i_1 += 1;
        i_1;
    }
    let mut i_2: I = 0;
    let mut j: I = 0;
    let mut p: V = v.offset(fb_off_by as isize) as V;
    j = 0 as libc::c_int as I;
    while j < r {
        e = 0 as libc::c_int as I;
        i_2 = 0 as libc::c_int as I;
        while i_2 < cn {
            x = *(((*d).k).as_mut_ptr() as *mut I).offset(i_2 as isize);
            let fresh47 = e;
            e = e + 1;
            let mut q: K = *((*z).k).as_mut_ptr().offset(fresh47 as isize);
            let mut u: S = 0 as *mut C;
            g = *(((*c).k).as_mut_ptr() as *mut C).offset(i_2 as isize);
            match g as libc::c_int {
                32 => {
                    e -= 1;
                    e;
                }
                99 => {
                    *(((*q).k).as_mut_ptr() as *mut C)
                        .offset(j as isize) = *(p as *mut C);
                }
                98 => {
                    *(((*q).k).as_mut_ptr() as *mut I)
                        .offset(j as isize) = *(p as *mut int8_t) as I;
                }
                115 => {
                    *(((*q).k).as_mut_ptr() as *mut I)
                        .offset(j as isize) = *(p as *mut int16_t) as I;
                }
                105 => {
                    *(((*q).k).as_mut_ptr() as *mut I)
                        .offset(j as isize) = *(p as *mut int32_t) as I;
                }
                106 => {
                    *(((*q).k).as_mut_ptr() as *mut I)
                        .offset(j as isize) = *(p as *mut I);
                }
                102 => {
                    *(((*q).k).as_mut_ptr() as *mut F)
                        .offset(j as isize) = *(p as *mut libc::c_float) as F;
                }
                100 => {
                    *(((*q).k).as_mut_ptr() as *mut F)
                        .offset(j as isize) = *(p as *mut F);
                }
                109 => {}
                67 => {
                    k = newK(-(3 as libc::c_int) as I, x);
                    if k.is_null() {
                        return 0 as K;
                    }
                    memcpy(
                        ((*k).k).as_mut_ptr() as *mut C as *mut libc::c_void,
                        p as *const libc::c_void,
                        x as libc::c_ulong,
                    );
                    let ref mut fresh48 = *((*q).k).as_mut_ptr().offset(j as isize);
                    *fresh48 = k;
                }
                83 => {
                    u = spn(p as S, x);
                    if u.is_null() {
                        return 0 as K;
                    }
                    let ref mut fresh49 = *(((*q).k).as_mut_ptr() as *mut S)
                        .offset(j as isize);
                    *fresh49 = u;
                }
                73 | 70 | 68 | 90 | 77 | _ => {}
            }
            p = p.offset(x as isize);
            i_2 += 1;
            i_2;
        }
        j += 1;
        j;
    }
    let mut res: I = munmap(v as *mut libc::c_void, map_length as size_t) as I;
    if res != 0 {
        return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
    }
    return z;
}
unsafe extern "C" fn _1d_char(mut x: K, mut y: K) -> K {
    let mut a: C = *(((*x).k).as_mut_ptr() as *mut C);
    if 'c' as i32 == a as libc::c_int {
        return readVector(y, -(3 as libc::c_int) as I);
    }
    if 'd' as i32 == a as libc::c_int {
        return readVector(y, -(2 as libc::c_int) as I);
    }
    if 'i' as i32 == a as libc::c_int {
        return readVector(y, -(1 as libc::c_int) as I);
    }
    return kerr(b"nonce\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn _2m(mut a: K) -> K {
    let mut t: I = (*a).t;
    if 4 as libc::c_int as libc::c_longlong != t
        && 3 as libc::c_int as libc::c_longlong
            != (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut m: S = CSK(a);
    let mut sm: I = strlen(m as *const libc::c_char) as I;
    let mut e: S = if sm > 1 as libc::c_int as libc::c_longlong
        && '.' as i32
            == *m.offset((sm - 2 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
        && *(b"K\0" as *const u8 as *const libc::c_char) as libc::c_int
            == *m.offset((sm - 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
    {
        strdupn(m, sm)
    } else {
        glueSS(m, b"K\0" as *const u8 as *const libc::c_char as S)
    };
    if e.is_null() {
        return 0 as K;
    }
    let mut s: I = 0;
    let mut f: I = open(e as *const libc::c_char, 0 as libc::c_int) as I;
    if f >= 0 as libc::c_int as libc::c_longlong {
        if stat_sz(e, &mut s) != 0 {
            return kerr(strerror(*__errno_location()) as cS);
        }
    } else {
        f = open(m as *const libc::c_char, 0 as libc::c_int) as I;
        if stat_sz(m, &mut s) != 0 {
            return kerr(strerror(*__errno_location()) as cS);
        }
    }
    free(e as *mut libc::c_void);
    if f < 0 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut v: S = 0 as *mut C;
    v = mmap(
        0 as *mut libc::c_void,
        s as size_t,
        0x1 as libc::c_int,
        0x1 as libc::c_int,
        f as libc::c_int,
        0 as libc::c_int as __off_t,
    ) as S;
    if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut r: I = close(f as libc::c_int) as I;
    if r != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    let mut b: I = 0 as libc::c_int as I;
    let mut z: K = _2m_r(v as V, v.offset(s as isize) as V, &mut b);
    r = munmap(v as *mut libc::c_void, s as size_t) as I;
    if r != 0 {
        return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
    }
    return z;
}
pub unsafe extern "C" fn _2m_r(mut v: V, mut aft: V, mut b: *mut I) -> K {
    return rrep(v, aft, b, 1 as libc::c_int as I, 0 as libc::c_int as I);
}
pub unsafe extern "C" fn _2d(mut a: K, mut b: K) -> K {
    let mut c: K = 0 as *mut k0;
    let mut d: K = 0 as *mut k0;
    if 4 as libc::c_int as libc::c_longlong != (*a).t
        && 3 as libc::c_int as libc::c_longlong
            != (if (*a).t < 0 as libc::c_int as libc::c_longlong {
                -(*a).t
            } else {
                (*a).t
            }) || (*b).t != 0 || (*b).n != 2 as libc::c_int as libc::c_longlong
        || {
            c = *((*b).k).as_mut_ptr().offset(0 as libc::c_int as isize);
            4 as libc::c_int as libc::c_longlong != (*c).t
                && 3 as libc::c_int as libc::c_longlong
                    != (if (*c).t < 0 as libc::c_int as libc::c_longlong {
                        -(*c).t
                    } else {
                        (*c).t
                    })
        }
        || {
            d = *((*b).k).as_mut_ptr().offset(1 as libc::c_int as isize);
            1 as libc::c_int as libc::c_longlong != (*d).t
        }
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut v: L = *(((*d).k).as_mut_ptr() as *mut I);
    if v < 0 as libc::c_int as libc::c_longlong
        || v > 7 as libc::c_int as libc::c_longlong
    {
        return kerr(b"valence\0" as *const u8 as *const libc::c_char);
    }
    let mut e: cS = 0 as *const C;
    let mut x: V = dlopen(
        CSK(a) as *const libc::c_char,
        0x1 as libc::c_int | 0 as libc::c_int,
    );
    let mut y: V = 0 as *mut libc::c_void;
    e = dlerror() as cS;
    if !e.is_null() {
        printf(
            b"error loading %s\nerr=%s\n\0" as *const u8 as *const libc::c_char,
            CSK(a),
            e,
        );
        return 0 as K;
    }
    if x.is_null() {
        printf(b"error loading %s \n\0" as *const u8 as *const libc::c_char, CSK(a));
        return 0 as K;
    }
    y = dlsym(x, CSK(c) as *const libc::c_char);
    e = dlerror() as cS;
    if !e.is_null() && *e as libc::c_int != 0 {
        return kerr(e);
    }
    if y.is_null() {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = Kv();
    let mut w: K = newK(-(4 as libc::c_int) as I, 3 as libc::c_int as I);
    if OOM_CD(0 as libc::c_int as I, z, w, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    (*z).n = 2 as libc::c_int as I;
    let ref mut fresh50 = *((*w).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh50 = v as V as *mut k0;
    let ref mut fresh51 = *((*w).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh51 = y as *mut k0;
    let ref mut fresh52 = *(((*z).k).as_mut_ptr() as *mut V)
        .offset(CODE as libc::c_int as isize);
    *fresh52 = w as V;
    return z;
}
pub unsafe extern "C" fn dm1(mut msg: S, mut m: *mut M1) {
    printf(b"=== %s ===\n\0" as *const u8 as *const libc::c_char, msg);
    printf(b"a: %d\n\0" as *const u8 as *const libc::c_char, (*m).a as libc::c_int);
    printf(b"n: %lld\n\0" as *const u8 as *const libc::c_char, (*m).n);
    printf(b"d: %d\n\0" as *const u8 as *const libc::c_char, (*m).d as libc::c_int);
}
unsafe extern "C" fn sendall(mut s: I, mut b: S, mut k: I) -> I {
    let mut t: I = 0 as libc::c_int as I;
    let mut r: I = k;
    let mut n: I = 0 as libc::c_int as I;
    while t < k {
        n = send(
            s as libc::c_int,
            b.offset(t as isize) as *const libc::c_void,
            r as size_t,
            0 as libc::c_int,
        ) as I;
        if -(1 as libc::c_int) as libc::c_longlong == n {
            break;
        }
        t += n;
        r -= n;
    }
    return if -(1 as libc::c_int) as libc::c_longlong == n {
        n
    } else {
        0 as libc::c_int as libc::c_longlong
    };
}
pub unsafe extern "C" fn ksender(mut sockfd: I, mut y: K, mut t: I) -> I {
    let mut r: I = 0 as libc::c_int as I;
    let mut k: K = 0 as *mut k0;
    k = _bd(y);
    if k.is_null() {
        return 0 as libc::c_int as I;
    }
    let mut m: *mut M1 = ((*k).k).as_mut_ptr() as *mut C as V as *mut M1;
    (*m).d = t as libc::c_char;
    r = sendall(sockfd, ((*k).k).as_mut_ptr() as *mut C, (*k).n);
    if -(1 as libc::c_int) as libc::c_longlong == r {
        perror(b"conn: send error\0" as *const u8 as *const libc::c_char);
    }
    cd(k);
    return r;
}
unsafe extern "C" fn _3d_(mut x: K, mut y: K) -> K {
    let mut res: I = -(1 as libc::c_int) as I;
    if (*y).t == -(3 as libc::c_int) as libc::c_longlong {
        res = ksender(*(((*x).k).as_mut_ptr() as *mut I), y, 0 as libc::c_int as I);
    } else if (*y).t == 0 as libc::c_int as libc::c_longlong
        && (*y).n == 4 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(3 as libc::c_int as isize)).t
            == 7 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(3 as libc::c_int as isize)).n
            == 3 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(1 as libc::c_int as isize)).t
            == 0 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(1 as libc::c_int as isize)).n
            == 0 as libc::c_int as libc::c_longlong
        && *((*(*((*(*((*y).k).as_mut_ptr().offset(2 as libc::c_int as isize) as K)).k)
            .as_mut_ptr()
            .offset(CODE as libc::c_int as isize) as K))
            .k)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) == offsetColon as *mut k0
    {
        let mut sym: S = *(((*(*((*y).k).as_mut_ptr().offset(0 as libc::c_int as isize)
            as K))
            .k)
            .as_mut_ptr() as *mut S);
        let mut lenS: I = strlen(sym as *const libc::c_char) as I;
        let mut cod: S = ((*(*((*(*((*y).k)
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize) as K))
            .k)
            .as_mut_ptr()
            .offset(CODE as libc::c_int as isize) as K))
            .k)
            .as_mut_ptr() as *mut C;
        let mut lenC: I = strlen(cod as *const libc::c_char) as I;
        let vla = (lenS + lenC + 4 as libc::c_int as libc::c_longlong) as usize;
        let mut str: Vec::<C> = ::std::vec::from_elem(0, vla);
        let mut i: I = 0 as libc::c_int as I;
        i = 0 as libc::c_int as I;
        while i < lenS {
            *str.as_mut_ptr().offset(i as isize) = *sym.offset(i as isize);
            i += 1;
            i;
        }
        i = 0 as libc::c_int as I;
        while i < lenC {
            *str
                .as_mut_ptr()
                .offset(
                    (i + lenS + 2 as libc::c_int as libc::c_longlong) as isize,
                ) = *cod.offset(i as isize);
            i += 1;
            i;
        }
        *str.as_mut_ptr().offset(lenS as isize) = ':' as i32 as C;
        *str
            .as_mut_ptr()
            .offset(
                (lenS + 1 as libc::c_int as libc::c_longlong) as isize,
            ) = '{' as i32 as C;
        *str
            .as_mut_ptr()
            .offset(
                (lenS + lenC + 2 as libc::c_int as libc::c_longlong) as isize,
            ) = '}' as i32 as C;
        *str
            .as_mut_ptr()
            .offset(
                (lenS + lenC + 3 as libc::c_int as libc::c_longlong) as isize,
            ) = '\0' as i32 as C;
        let mut q: K = Ks(str.as_mut_ptr());
        res = ksender(*(((*x).k).as_mut_ptr() as *mut I), q, 0 as libc::c_int as I);
        cd(q);
    } else if (*y).t == 0 as libc::c_int as libc::c_longlong
        && (*y).n == 4 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(3 as libc::c_int as isize)).t
            == 7 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(3 as libc::c_int as isize)).n
            == 1 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(1 as libc::c_int as isize)).t
            == 0 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(1 as libc::c_int as isize)).n
            == 0 as libc::c_int as libc::c_longlong
        && *((*(*((*(*((*y).k).as_mut_ptr().offset(2 as libc::c_int as isize) as K)).k)
            .as_mut_ptr()
            .offset(CODE as libc::c_int as isize) as K))
            .k)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) == offsetColon as *mut k0
    {
        let mut sym_0: S = *(((*(*((*y).k).as_mut_ptr().offset(0 as libc::c_int as isize)
            as K))
            .k)
            .as_mut_ptr() as *mut S);
        let mut lenS_0: I = strlen(sym_0 as *const libc::c_char) as I;
        let mut cod_0: S = ((*_5m(
            *((*y).k).as_mut_ptr().offset(3 as libc::c_int as isize),
        ))
            .k)
            .as_mut_ptr() as *mut C;
        let mut lenC_0: I = strlen(cod_0 as *const libc::c_char) as I;
        let vla_0 = (lenS_0 + lenC_0 + 2 as libc::c_int as libc::c_longlong) as usize;
        let mut str_0: Vec::<C> = ::std::vec::from_elem(0, vla_0);
        let mut i_0: I = 0 as libc::c_int as I;
        i_0 = 0 as libc::c_int as I;
        while i_0 < lenS_0 {
            *str_0.as_mut_ptr().offset(i_0 as isize) = *sym_0.offset(i_0 as isize);
            i_0 += 1;
            i_0;
        }
        i_0 = 0 as libc::c_int as I;
        while i_0 < lenC_0 {
            *str_0
                .as_mut_ptr()
                .offset(
                    (i_0 + lenS_0 + 1 as libc::c_int as libc::c_longlong) as isize,
                ) = *cod_0.offset(i_0 as isize);
            i_0 += 1;
            i_0;
        }
        *str_0.as_mut_ptr().offset(lenS_0 as isize) = ':' as i32 as C;
        *str_0
            .as_mut_ptr()
            .offset(
                (lenS_0 + lenC_0 + 1 as libc::c_int as libc::c_longlong) as isize,
            ) = '\0' as i32 as C;
        let mut q_0: K = Ks(str_0.as_mut_ptr());
        res = ksender(*(((*x).k).as_mut_ptr() as *mut I), q_0, 0 as libc::c_int as I);
        cd(q_0);
    } else if (*y).t == 0 as libc::c_int as libc::c_longlong
        && (*y).n == 3 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(2 as libc::c_int as isize)).t
            == 7 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(2 as libc::c_int as isize)).n
            == 3 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(1 as libc::c_int as isize)).t
            == 0 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(1 as libc::c_int as isize)).n
            == 0 as libc::c_int as libc::c_longlong
    {
        let mut sym_1: S = *(((*(*((*y).k).as_mut_ptr().offset(0 as libc::c_int as isize)
            as K))
            .k)
            .as_mut_ptr() as *mut S);
        let mut lenS_1: I = strlen(sym_1 as *const libc::c_char) as I;
        let mut cod_1: S = ((*(*((*(*((*y).k)
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) as K))
            .k)
            .as_mut_ptr()
            .offset(CODE as libc::c_int as isize) as K))
            .k)
            .as_mut_ptr() as *mut C;
        let mut lenC_1: I = strlen(cod_1 as *const libc::c_char) as I;
        let vla_1 = (2 as libc::c_int as libc::c_longlong * lenS_1 + lenC_1
            + 4 as libc::c_int as libc::c_longlong) as usize;
        let mut str_1: Vec::<C> = ::std::vec::from_elem(0, vla_1);
        let mut i_1: I = 0 as libc::c_int as I;
        i_1 = 0 as libc::c_int as I;
        while i_1 < lenS_1 {
            *str_1.as_mut_ptr().offset(i_1 as isize) = *sym_1.offset(i_1 as isize);
            i_1 += 1;
            i_1;
        }
        i_1 = 0 as libc::c_int as I;
        while i_1 < lenC_1 {
            *str_1
                .as_mut_ptr()
                .offset(
                    (i_1 + lenS_1 + 2 as libc::c_int as libc::c_longlong) as isize,
                ) = *cod_1.offset(i_1 as isize);
            i_1 += 1;
            i_1;
        }
        i_1 = 0 as libc::c_int as I;
        while i_1 < lenS_1 {
            *str_1
                .as_mut_ptr()
                .offset(
                    (i_1 + lenS_1 + lenC_1 + 3 as libc::c_int as libc::c_longlong)
                        as isize,
                ) = *sym_1.offset(i_1 as isize);
            i_1 += 1;
            i_1;
        }
        *str_1.as_mut_ptr().offset(lenS_1 as isize) = ':' as i32 as C;
        *str_1
            .as_mut_ptr()
            .offset(
                (lenS_1 + 1 as libc::c_int as libc::c_longlong) as isize,
            ) = '{' as i32 as C;
        *str_1
            .as_mut_ptr()
            .offset(
                (lenS_1 + lenC_1 + 2 as libc::c_int as libc::c_longlong) as isize,
            ) = '}' as i32 as C;
        *str_1
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_longlong * lenS_1 + lenC_1
                    + 3 as libc::c_int as libc::c_longlong) as isize,
            ) = '\0' as i32 as C;
        let mut q_1: K = Ks(str_1.as_mut_ptr());
        res = ksender(*(((*x).k).as_mut_ptr() as *mut I), q_1, 0 as libc::c_int as I);
        cd(q_1);
    } else if (*y).t == 0 as libc::c_int as libc::c_longlong
        && (*y).n == 3 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(2 as libc::c_int as isize)).t
            == 7 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(2 as libc::c_int as isize)).n
            == 1 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(1 as libc::c_int as isize)).t
            == 0 as libc::c_int as libc::c_longlong
        && (**((*y).k).as_mut_ptr().offset(1 as libc::c_int as isize)).n
            == 0 as libc::c_int as libc::c_longlong
    {
        let mut sym_2: S = *(((*(*((*y).k).as_mut_ptr().offset(0 as libc::c_int as isize)
            as K))
            .k)
            .as_mut_ptr() as *mut S);
        let mut lenS_2: I = strlen(sym_2 as *const libc::c_char) as I;
        let mut cod_2: S = ((*_5m(
            *((*y).k).as_mut_ptr().offset(2 as libc::c_int as isize),
        ))
            .k)
            .as_mut_ptr() as *mut C;
        let mut lenC_2: I = strlen(cod_2 as *const libc::c_char) as I;
        let vla_2 = (2 as libc::c_int as libc::c_longlong * lenS_2 + lenC_2
            + 2 as libc::c_int as libc::c_longlong) as usize;
        let mut str_2: Vec::<C> = ::std::vec::from_elem(0, vla_2);
        let mut i_2: I = 0 as libc::c_int as I;
        i_2 = 0 as libc::c_int as I;
        while i_2 < lenS_2 {
            *str_2.as_mut_ptr().offset(i_2 as isize) = *sym_2.offset(i_2 as isize);
            i_2 += 1;
            i_2;
        }
        i_2 = 0 as libc::c_int as I;
        while i_2 < lenC_2 {
            *str_2
                .as_mut_ptr()
                .offset(
                    (i_2 + lenS_2 + 1 as libc::c_int as libc::c_longlong) as isize,
                ) = *cod_2.offset(i_2 as isize);
            i_2 += 1;
            i_2;
        }
        i_2 = 0 as libc::c_int as I;
        while i_2 < lenS_2 {
            *str_2
                .as_mut_ptr()
                .offset(
                    (i_2 + lenS_2 + lenC_2 + 1 as libc::c_int as libc::c_longlong)
                        as isize,
                ) = *sym_2.offset(i_2 as isize);
            i_2 += 1;
            i_2;
        }
        *str_2.as_mut_ptr().offset(lenS_2 as isize) = ':' as i32 as C;
        *str_2
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int as libc::c_longlong * lenS_2 + lenC_2
                    + 1 as libc::c_int as libc::c_longlong) as isize,
            ) = '\0' as i32 as C;
        let mut q_2: K = Ks(str_2.as_mut_ptr());
        res = ksender(*(((*x).k).as_mut_ptr() as *mut I), q_2, 0 as libc::c_int as I);
        cd(q_2);
    } else {
        return kerr(b"nyi\0" as *const u8 as *const libc::c_char)
    }
    if -(1 as libc::c_int) as libc::c_longlong == res {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    return _n();
}
pub unsafe extern "C" fn popen_charvec(mut cmd: S) -> K {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut z: K = 0 as *mut k0;
    let mut l: K = 0 as *mut k0;
    let mut s: S = 0 as S;
    let mut n: I = 0 as libc::c_int as I;
    f = popen(cmd as *const libc::c_char, b"r\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return _n();
    }
    z = newK(0 as libc::c_int as I, 0 as libc::c_int as I);
    while getline_(&mut s, &mut n, f) >= 0 as libc::c_int as libc::c_longlong {
        l = newK(-(3 as libc::c_int) as I, n - 1 as libc::c_int as libc::c_longlong);
        strncpy(
            ((*l).k).as_mut_ptr() as *mut C,
            s as *const libc::c_char,
            (n - 1 as libc::c_int as libc::c_longlong) as libc::c_ulong,
        );
        kap(&mut z, &mut l as *mut K as V);
    }
    free(s as *mut libc::c_void);
    pclose(f);
    return z;
}
unsafe extern "C" fn parse(mut s: S, mut argv: *mut S, mut c: C) {
    while *s as libc::c_int != '\0' as i32 {
        while *s as libc::c_int == c as libc::c_int || *s as libc::c_int == '\t' as i32
            || *s as libc::c_int == '\n' as i32
        {
            let fresh53 = s;
            s = s.offset(1);
            *fresh53 = '\0' as i32 as C;
        }
        let fresh54 = argv;
        argv = argv.offset(1);
        *fresh54 = s;
        while *s as libc::c_int != '\0' as i32 && *s as libc::c_int != c as libc::c_int
            && *s as libc::c_int != '\t' as i32 && *s as libc::c_int != '\n' as i32
        {
            s = s.offset(1);
            s;
        }
    }
    *argv = 0 as S;
}
unsafe extern "C" fn execute(mut argvP: *mut S, mut fWait: I) {
    let mut pid: pid_t = 0;
    let mut status: I = 0;
    pid = fork();
    if pid < 0 as libc::c_int {
        printf(
            b"*** ERROR: forking child process failed\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else if pid == 0 as libc::c_int {
        if execvp(*argvP as *const libc::c_char, argvP as *const *mut libc::c_char)
            < 0 as libc::c_int
        {
            printf(b"*** ERROR: exec failed\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    } else if fWait != 0 {
        while wait(&mut status as *mut I as *mut libc::c_int) != pid {}
    }
}
pub unsafe extern "C" fn _4d_(mut srvr: S, mut port: S, mut y: K) -> K {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut servinfo: *mut addrinfo = 0 as *mut addrinfo;
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    let mut rv: libc::c_int = 0;
    let mut sockfd: libc::c_int = 0;
    let mut errstr: S = 0 as *mut C;
    let mut r: I = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    rv = getaddrinfo(
        srvr as *const libc::c_char,
        port as *const libc::c_char,
        &mut hints,
        &mut servinfo,
    );
    if rv != 0 {
        fprintf(
            stderr,
            b"conn: %s\n\0" as *const u8 as *const libc::c_char,
            gai_strerror(rv),
        );
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    p = servinfo;
    while !p.is_null() {
        sockfd = socket((*p).ai_family, (*p).ai_socktype, (*p).ai_protocol);
        if !(sockfd == -(1 as libc::c_int)) {
            if !(connect(sockfd, (*p).ai_addr, (*p).ai_addrlen) == -(1 as libc::c_int)) {
                break;
            }
            errstr = strerror(*__errno_location());
            r = close(sockfd) as I;
            if r != 0 {
                return kerr(b"file\0" as *const u8 as *const libc::c_char);
            }
        }
        p = (*p).ai_next;
    }
    if p.is_null() {
        fprintf(
            stderr,
            b"conn: failed to connect (%s)\n\0" as *const u8 as *const libc::c_char,
            errstr,
        );
        freeaddrinfo(servinfo);
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut n: I = strlen(((*y).k).as_mut_ptr() as *mut C) as I;
    let vla = (n + 5 as libc::c_int as libc::c_longlong) as usize;
    let mut msg: Vec::<C> = ::std::vec::from_elem(0, vla);
    let mut i: I = 0 as libc::c_int as I;
    i = 0 as libc::c_int as I;
    while i < n + 1 as libc::c_int as libc::c_longlong {
        *msg
            .as_mut_ptr()
            .offset(i as isize) = *(((*y).k).as_mut_ptr() as *mut C).offset(i as isize);
        i += 1;
        i;
    }
    *msg.as_mut_ptr().offset(n as isize) = '\r' as i32 as C;
    *msg
        .as_mut_ptr()
        .offset((n + 1 as libc::c_int as libc::c_longlong) as isize) = '\n' as i32 as C;
    *msg
        .as_mut_ptr()
        .offset((n + 2 as libc::c_int as libc::c_longlong) as isize) = '\r' as i32 as C;
    *msg
        .as_mut_ptr()
        .offset((n + 3 as libc::c_int as libc::c_longlong) as isize) = '\n' as i32 as C;
    *msg
        .as_mut_ptr()
        .offset((n + 4 as libc::c_int as libc::c_longlong) as isize) = '\0' as i32 as C;
    if write(
        sockfd,
        msg.as_mut_ptr() as *mut C as *const libc::c_void,
        strlen(msg.as_mut_ptr()),
    ) == -(1 as libc::c_int) as libc::c_long
    {
        r = close(sockfd) as I;
        if r != 0 {
            return kerr(b"file\0" as *const u8 as *const libc::c_char);
        }
        return kerr(b"write\0" as *const u8 as *const libc::c_char);
    }
    let mut buf: [C; 20000] = [0; 20000];
    let mut size: I = 30000 as libc::c_int as I;
    let mut data: S = malloc(size as libc::c_ulong) as S;
    let mut n1: I = 1 as libc::c_int as I;
    n = 0 as libc::c_int as I;
    loop {
        n1 = read(
            sockfd,
            &mut buf as *mut [C; 20000] as *mut libc::c_void,
            20000 as libc::c_int as size_t,
        ) as I;
        if n1 == 0 as libc::c_int as libc::c_longlong {
            break;
        }
        if n1 < 0 as libc::c_int as libc::c_longlong {
            printf(
                b"errno: %d\n\0" as *const u8 as *const libc::c_char,
                *__errno_location(),
            );
            return 0 as K;
        }
        if n + n1 > size - 1 as libc::c_int as libc::c_longlong {
            size += 20000 as libc::c_int as libc::c_longlong;
            data = realloc(data as *mut libc::c_void, size as libc::c_ulong) as S;
        }
        i = 0 as libc::c_int as I;
        while i < n1 + 1 as libc::c_int as libc::c_longlong {
            *data.offset((n + i) as isize) = buf[i as usize];
            i += 1;
            i;
        }
        n += n1;
        if !(n1 != 0) {
            break;
        }
    }
    r = close(sockfd) as I;
    if r != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = newK(
        (if n == 1 as libc::c_int as libc::c_longlong {
            3 as libc::c_int
        } else {
            -(3 as libc::c_int)
        }) as I,
        n,
    );
    memcpy(
        ((*z).k).as_mut_ptr() as *mut C as *mut libc::c_void,
        data as *const libc::c_void,
        n as libc::c_ulong,
    );
    freeaddrinfo(servinfo);
    free(data as *mut libc::c_void);
    return if n != 0 { z } else { _n() };
}
unsafe extern "C" fn run(mut x: K) -> K {
    let mut line: S = ((*x).k).as_mut_ptr() as *mut C;
    let mut argvL: [S; 64] = [0 as *mut C; 64];
    let mut argvP: [S; 64] = [0 as *mut C; 64];
    let mut i: I = 0;
    let mut fWait: I = 1 as libc::c_int as I;
    parse(line, argvL.as_mut_ptr(), ';' as i32 as C);
    i = 0 as libc::c_int as I;
    while !(argvL[i as usize]).is_null() {
        parse(argvL[i as usize], argvP.as_mut_ptr(), ' ' as i32 as C);
        if (argvL[1 as libc::c_int as usize]).is_null()
            && strcmp(
                argvP[0 as libc::c_int as usize] as *const libc::c_char,
                b"echo\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
        {
            fWait = 0 as libc::c_int as I;
        }
        execute(argvP.as_mut_ptr(), fWait);
        i += 1;
        i;
    }
    return _n();
}
pub unsafe extern "C" fn _3d(mut x: K, mut y: K) -> K {
    if 3 as libc::c_int as libc::c_longlong
        == (if (*x).t < 0 as libc::c_int as libc::c_longlong { -(*x).t } else { (*x).t })
    {
        return _5d_(x, y, 0 as libc::c_int as I);
    }
    if 4 as libc::c_int as libc::c_longlong == (*x).t {
        return if **(((*x).k).as_mut_ptr() as *mut S) == 0 {
            run(y)
        } else {
            _5d_(x, y, 0 as libc::c_int as I)
        };
    }
    if 1 as libc::c_int as libc::c_longlong != (*x).t {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    return _3d_(x, y);
}
pub unsafe extern "C" fn _4d(mut x: K, mut y: K) -> K {
    if 4 as libc::c_int as libc::c_longlong == (*x).t
        && **(((*x).k).as_mut_ptr() as *mut S) == 0
        && -(3 as libc::c_int) as libc::c_longlong == (*y).t
    {
        return popen_charvec(((*y).k).as_mut_ptr() as *mut C);
    }
    if 1 as libc::c_int as libc::c_longlong == (*x).t {
        let mut sockfd: I = *(((*x).k).as_mut_ptr() as *mut I);
        if -(1 as libc::c_int) as libc::c_longlong
            == ksender(sockfd, y, 1 as libc::c_int as I)
        {
            return kerr(b"domain\0" as *const u8 as *const libc::c_char);
        }
        let mut z: K = 0 as K;
        while 2 as libc::c_int as libc::c_longlong != fer
            && {
                z = read_tape(sockfd, sockfd, 1 as libc::c_int as I);
                z.is_null()
            }
        {}
        if z == -(1 as libc::c_int) as K {
            return kerr(b"domain\0" as *const u8 as *const libc::c_char);
        }
        if z.is_null() {
            return 0 as K;
        }
        return z;
    }
    if -(4 as libc::c_int) as libc::c_longlong == (*x).t
        && 2 as libc::c_int as libc::c_longlong == (*x).n
    {
        return _4d_(
            *(((*x).k).as_mut_ptr() as *mut S).offset(0 as libc::c_int as isize),
            *(((*x).k).as_mut_ptr() as *mut S).offset(1 as libc::c_int as isize),
            y,
        );
    }
    if 0 as libc::c_int as libc::c_longlong == (*x).t
        && 2 as libc::c_int as libc::c_longlong == (*x).n
        && 4 as libc::c_int as libc::c_longlong
            == (**((*x).k).as_mut_ptr().offset(0 as libc::c_int as isize)).t
        && 1 as libc::c_int as libc::c_longlong
            == (**((*x).k).as_mut_ptr().offset(1 as libc::c_int as isize)).t
        && 0 as libc::c_int as libc::c_longlong
            <= *(((*(*((*x).k).as_mut_ptr().offset(1 as libc::c_int as isize) as K)).k)
                .as_mut_ptr() as *mut I)
    {
        let mut port: [C; 6] = [0; 6];
        let mut n: libc::c_int = snprintf(
            port.as_mut_ptr(),
            6 as libc::c_int as libc::c_ulong,
            b"%lld\0" as *const u8 as *const libc::c_char,
            *(((*(*((*x).k).as_mut_ptr().offset(1 as libc::c_int as isize) as K)).k)
                .as_mut_ptr() as *mut I),
        );
        if n >= 6 as libc::c_int {
            return kerr(b"write\0" as *const u8 as *const libc::c_char);
        }
        return _4d_(
            *(((*(*((*x).k).as_mut_ptr().offset(0 as libc::c_int as isize) as K)).k)
                .as_mut_ptr() as *mut S),
            port.as_mut_ptr(),
            y,
        );
    }
    return kerr(b"type\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn _4m(mut x: K) -> K {
    return Ki((*x).t);
}
pub unsafe extern "C" fn _5m(mut x: K) -> K {
    let mut z: K = 0 as *mut k0;
    z = newK(-(3 as libc::c_int) as I, 0 as libc::c_int as I);
    if z.is_null() {
        return 0 as K;
    }
    printAtDepth(
        &mut z as *mut K as V,
        x,
        0 as libc::c_int as I,
        0 as libc::c_int as I,
        0 as libc::c_int as I,
        0 as libc::c_int as I,
    );
    return z;
}
unsafe extern "C" fn _5d_(mut x: K, mut y: K, mut dosync: I) -> K {
    if 4 as libc::c_int as libc::c_longlong != (*x).t
        && 3 as libc::c_int as libc::c_longlong
            != (if (*x).t < 0 as libc::c_int as libc::c_longlong {
                -(*x).t
            } else {
                (*x).t
            })
    {
        return 0 as K;
    }
    let mut m: S = CSK(x);
    let mut sm: I = strlen(m as *const libc::c_char) as I;
    let mut e: S = if sm > 1 as libc::c_int as libc::c_longlong
        && '.' as i32
            == *m.offset((sm - 2 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
        && *(b"K\0" as *const u8 as *const libc::c_char) as libc::c_int
            == *m.offset((sm - 1 as libc::c_int as libc::c_longlong) as isize)
                as libc::c_int
    {
        strdupn(m, sm)
    } else {
        glueSS(m, b"K\0" as *const u8 as *const libc::c_char as S)
    };
    if e.is_null() {
        return 0 as K;
    }
    let mut c: stat = stat {
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
    let mut f: I = open(
        e as *const libc::c_char,
        0o2 as libc::c_int,
        0o7777 as libc::c_int,
    ) as I;
    if f >= 0 as libc::c_int as libc::c_longlong {
        stat(e as *const libc::c_char, &mut c);
    } else {
        f = open(m as *const libc::c_char, 0o2 as libc::c_int, 0o7777 as libc::c_int)
            as I;
        stat(m as *const libc::c_char, &mut c);
    }
    free(e as *mut libc::c_void);
    if f < 0 as libc::c_int as libc::c_longlong {
        return _1d_write(x, y, 1 as libc::c_int as I);
    }
    let mut s: I = c.st_size as I;
    if (s as libc::c_ulonglong)
        < (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong)
            as libc::c_ulonglong
    {
        return 0 as K;
    }
    let mut ft: I = 0;
    let mut fn_0: I = 0;
    let mut g: I = 0;
    g = pread(
        f as libc::c_int,
        &mut ft as *mut I as *mut libc::c_void,
        ::std::mem::size_of::<I>() as libc::c_ulong,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong) as __off_t,
    ) as I;
    if g == 0 {
        show(kerr(b"pread\0" as *const u8 as *const libc::c_char));
    }
    g = pread(
        f as libc::c_int,
        &mut fn_0 as *mut I as *mut libc::c_void,
        ::std::mem::size_of::<I>() as libc::c_ulong,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<I>() as libc::c_ulong) as __off_t,
    ) as I;
    if g == 0 {
        show(kerr(b"pread\0" as *const u8 as *const libc::c_char));
    }
    if (*y).t > 0 as libc::c_int as libc::c_longlong
        && (*y).t != 5 as libc::c_int as libc::c_longlong || ft != (*y).t
    {
        return 0 as K;
    }
    let mut b: I = (disk(y) as libc::c_ulonglong)
        .wrapping_sub(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong)
                as libc::c_ulonglong,
        )
        .wrapping_sub(
            (-(3 as libc::c_int) as libc::c_longlong == (*y).t) as libc::c_int
                as libc::c_ulonglong,
        ) as I;
    let mut n: I = s + b;
    if ftruncate(f as libc::c_int, n as __off_t) != 0 {
        return 0 as K;
    }
    let mut v: S = 0 as *mut C;
    v = mmap(
        0 as *mut libc::c_void,
        n as size_t,
        0x2 as libc::c_int,
        0x1 as libc::c_int,
        f as libc::c_int,
        0 as libc::c_int as __off_t,
    ) as S;
    if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut res: I = close(f as libc::c_int) as I;
    if res != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    let mut w: *mut I = v as *mut I;
    *w.offset(3 as libc::c_int as isize) = fn_0 + (*y).n;
    let mut r: I = 0 as libc::c_int as I;
    let mut d: V = v.offset(s as isize) as V;
    if 0 as libc::c_int as libc::c_longlong == (*y).t
        || 5 as libc::c_int as libc::c_longlong == (*y).t
    {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = (*y).n;
        while i < _i {
            r
                += wrep(
                    *((*y).k).as_mut_ptr().offset(i as isize),
                    d.offset(r as isize),
                    1 as libc::c_int as I,
                );
            i += 1;
            i;
        }
    } else if -(4 as libc::c_int) as libc::c_longlong == (*y).t {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = (*y).n;
        while i_0 < _i_0 {
            s = (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    strlen(
                        *(((*y).k).as_mut_ptr() as *mut S).offset(i_0 as isize)
                            as *const libc::c_char,
                    ),
                ) as I;
            memcpy(
                d.offset(r as isize),
                *(((*y).k).as_mut_ptr() as *mut S).offset(i_0 as isize)
                    as *const libc::c_void,
                s as libc::c_ulong,
            );
            r += s;
            i_0 += 1;
            i_0;
        }
    } else if -(3 as libc::c_int) as libc::c_longlong == (*y).t {
        memcpy(
            d.offset(-(1 as libc::c_int as isize)),
            ((*y).k).as_mut_ptr() as *const libc::c_void,
            (n as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<C>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
        *(d as S).offset((*y).n as isize) = 0 as libc::c_int as C;
    } else if -(2 as libc::c_int) as libc::c_longlong == (*y).t {
        memcpy(
            d,
            ((*y).k).as_mut_ptr() as *const libc::c_void,
            ((*y).n as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<F>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
    } else if -(1 as libc::c_int) as libc::c_longlong == (*y).t {
        memcpy(
            d,
            ((*y).k).as_mut_ptr() as *const libc::c_void,
            ((*y).n as libc::c_ulonglong)
                .wrapping_mul(
                    ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
                ) as libc::c_ulong,
        );
    }
    if dosync != 0 {
        msync(v as *mut libc::c_void, n as size_t, 4 as libc::c_int | 2 as libc::c_int);
    }
    res = munmap(v as *mut libc::c_void, n as size_t) as I;
    if res != 0 {
        return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
    }
    return Ki(fn_0 + (*y).n);
}
pub unsafe extern "C" fn _5d(mut x: K, mut y: K) -> K {
    return _5d_(x, y, 1 as libc::c_int as I);
}
pub unsafe extern "C" fn _6m(mut x: K) -> K {
    return readVector(x, -(3 as libc::c_int) as I);
}
unsafe extern "C" fn readVector(mut x: K, mut t: I) -> K {
    if 4 as libc::c_int as libc::c_longlong != (*x).t
        && 3 as libc::c_int as libc::c_longlong
            != (if (*x).t < 0 as libc::c_int as libc::c_longlong {
                -(*x).t
            } else {
                (*x).t
            })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut s: I = 0;
    if stat_sz(CSK(x), &mut s) != 0 {
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut f: I = open(CSK(x) as *const libc::c_char, 0 as libc::c_int) as I;
    if f < 0 as libc::c_int as libc::c_longlong {
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut v: S = 0 as *mut C;
    v = mmap(
        0 as *mut libc::c_void,
        s as size_t,
        0x1 as libc::c_int,
        0x1 as libc::c_int,
        f as libc::c_int,
        0 as libc::c_int as __off_t,
    ) as S;
    if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut r: I = close(f as libc::c_int) as I;
    if r != 0 {
        return kerr(b"file\0" as *const u8 as *const libc::c_char);
    }
    let mut z: K = newK(t, ceil(s as libc::c_double / bp(t) as F) as I);
    memcpy(
        ((*z).k).as_mut_ptr() as *mut libc::c_void,
        v as *const libc::c_void,
        s as libc::c_ulong,
    );
    r = munmap(v as *mut libc::c_void, s as size_t) as I;
    if r != 0 {
        return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
    }
    return z;
}
pub unsafe extern "C" fn _6d(mut a: K, mut b: K) -> K {
    let mut append: I = 0 as libc::c_int as I;
    let mut c: K = a;
    if 0 as libc::c_int as libc::c_longlong == (*a).t
        && 1 as libc::c_int as libc::c_longlong == (*a).n
    {
        append = 1 as libc::c_int as I;
        c = *((*a).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    } else if -(4 as libc::c_int) as libc::c_longlong == (*c).t
        && 1 as libc::c_int as libc::c_longlong == (*c).n
    {
        append = 1 as libc::c_int as I;
    }
    if 4 as libc::c_int as libc::c_longlong != (*c).t
        && 3 as libc::c_int as libc::c_longlong
            != (if (*c).t < 0 as libc::c_int as libc::c_longlong {
                -(*c).t
            } else {
                (*c).t
            }) && (append == 0 && -(4 as libc::c_int) as libc::c_longlong == (*c).t)
    {
        return 0 as K;
    }
    let mut t: I = (*b).t;
    let mut n: I = (*b).n;
    if 3 as libc::c_int as libc::c_longlong
        != (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t })
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char);
    }
    let mut m: S = CSK(c);
    let mut f: I = (if *m.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        open(
            m as *const libc::c_char,
            0o2 as libc::c_int | 0o100 as libc::c_int
                | (if append != 0 { 0 as libc::c_int } else { 0o1000 as libc::c_int }),
            0o7777 as libc::c_int,
        )
    } else {
        1 as libc::c_int
    }) as I;
    let mut e: I = 0 as libc::c_int as I;
    if append != 0 && *m.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        if stat_sz(m, &mut e) != 0 {
            return kerr(strerror(*__errno_location()) as cS);
        }
    }
    if f < 0 as libc::c_int as libc::c_longlong {
        return kerr(strerror(*__errno_location()) as cS);
    }
    let mut r: I = 0;
    if 1 as libc::c_int as libc::c_longlong == f {
        r = write(
            f as libc::c_int,
            ((*b).k).as_mut_ptr() as *mut C as *const libc::c_void,
            n as size_t,
        ) as I;
        if r == 0 {
            show(kerr(b"write\0" as *const u8 as *const libc::c_char));
        }
    } else {
        if ftruncate(f as libc::c_int, (e + n) as __off_t) != 0 {
            return kerr(strerror(*__errno_location()) as cS);
        }
        let mut v: S = 0 as *mut C;
        v = mmap(
            0 as *mut libc::c_void,
            (e + n) as size_t,
            0x2 as libc::c_int,
            0x1 as libc::c_int,
            f as libc::c_int,
            0 as libc::c_int as __off_t,
        ) as S;
        if -(1 as libc::c_int) as *mut libc::c_void == v as *mut libc::c_void {
            return kerr(strerror(*__errno_location()) as cS);
        }
        r = close(f as libc::c_int) as I;
        if r != 0 {
            return kerr(b"file\0" as *const u8 as *const libc::c_char);
        }
        memcpy(
            v.offset(e as isize) as *mut libc::c_void,
            ((*b).k).as_mut_ptr() as *mut C as *const libc::c_void,
            n as libc::c_ulong,
        );
        msync(
            v.offset(e as isize) as *mut libc::c_void,
            n as size_t,
            4 as libc::c_int | 2 as libc::c_int,
        );
        r = munmap(v as *mut libc::c_void, (e + n) as size_t) as I;
        if r != 0 {
            return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
        }
    }
    return _n();
}
pub unsafe extern "C" fn _3m(mut x: K) -> K {
    if 1 as libc::c_int as libc::c_longlong == (*x).t {
        let mut i: I = close(*(((*x).k).as_mut_ptr() as *mut I) as libc::c_int) as I;
        return if i != 0 {
            kerr(b"domain\0" as *const u8 as *const libc::c_char)
        } else {
            _n()
        };
    } else if (*x).t != 0 || (*x).n != 2 as libc::c_int as libc::c_longlong
        || (**((*x).k).as_mut_ptr().offset(0 as libc::c_int as isize)).t
            != 4 as libc::c_int as libc::c_longlong
        || (**((*x).k).as_mut_ptr().offset(1 as libc::c_int as isize)).t
            != 1 as libc::c_int as libc::c_longlong
    {
        return kerr(b"type\0" as *const u8 as *const libc::c_char)
    }
    let mut host: S = CSK(*((*x).k).as_mut_ptr());
    let mut errstr: S = 0 as *mut C;
    let mut port: [libc::c_char; 256] = [0; 256];
    snprintf(
        port.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"%lld\0" as *const u8 as *const libc::c_char,
        *(((*(*((*x).k).as_mut_ptr().offset(1 as libc::c_int as isize) as K)).k)
            .as_mut_ptr() as *mut I),
    );
    let mut sockfd: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut servinfo: *mut addrinfo = 0 as *mut addrinfo;
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    let mut r: I = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    rv = getaddrinfo(
        host as *const libc::c_char,
        port.as_mut_ptr(),
        &mut hints,
        &mut servinfo,
    );
    if rv != 0 {
        fprintf(
            stderr,
            b"conn: %s\n\0" as *const u8 as *const libc::c_char,
            gai_strerror(rv),
        );
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    p = servinfo;
    while !p.is_null() {
        sockfd = socket((*p).ai_family, (*p).ai_socktype, (*p).ai_protocol);
        if !(sockfd == -(1 as libc::c_int)) {
            if !(connect(sockfd, (*p).ai_addr, (*p).ai_addrlen) == -(1 as libc::c_int)) {
                break;
            }
            errstr = strerror(*__errno_location());
            r = close(sockfd) as I;
            if r != 0 {
                return kerr(b"file\0" as *const u8 as *const libc::c_char);
            }
        }
        p = (*p).ai_next;
    }
    if p.is_null() {
        fprintf(
            stderr,
            b"conn: failed to connect (%s)\n\0" as *const u8 as *const libc::c_char,
            errstr,
        );
        freeaddrinfo(servinfo);
        return kerr(b"domain\0" as *const u8 as *const libc::c_char);
    }
    let mut yes: I = 1 as libc::c_int as I;
    setsockopt(
        sockfd,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut yes as *mut I as *const libc::c_void,
        ::std::mem::size_of::<I>() as libc::c_ulong as socklen_t,
    );
    freeaddrinfo(servinfo);
    wipe_tape(sockfd as I);
    return Ki(sockfd as I);
}

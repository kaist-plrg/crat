use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn mremap(
        __addr: *mut libc::c_void,
        __old_len: size_t,
        __new_len: size_t,
        __flags: libc::c_int,
        _: ...
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
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
    static mut kreci: I;
    static mut krec: [V; 1000000];
    fn show(a: K) -> K;
    static mut d_: S;
    fn kerr(s: cS) -> K;
    fn appender(s: *mut S, n: *mut I, t: S, k: I) -> I;
    fn _n() -> K;
    static mut tests: I;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type L = libc::c_longlong;
pub type V = *mut libc::c_void;
pub type I = libc::c_longlong;
pub type uI = libc::c_ulonglong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node {
    pub k: V,
    pub b: I,
    pub c: [*mut node; 2],
}
pub type Node = node;
pub type N = *mut Node;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pda {
    pub i: I,
    pub s: I,
    pub n: I,
    pub c: S,
}
pub type Pda = pda;
pub type PDA = *mut Pda;
pub static mut KP: [V; 27] = [0 as *const libc::c_void as *mut libc::c_void; 27];
pub static mut PG: I = 0;
pub static mut mUsed: F = 0.0f64;
pub static mut mAlloc: F = 0.0f64;
pub static mut mMap: F = 0.0f64;
pub static mut mMax: F = 0.0f64;
pub unsafe extern "C" fn alloc(mut sz_0: size_t) -> V {
    let mut r: V = malloc(sz_0);
    if r.is_null() {
        fputs(b"out of memory\n\0" as *const u8 as *const libc::c_char, stderr);
        exit(1 as libc::c_int);
    }
    return r;
}
pub unsafe extern "C" fn OOM_CD(mut g: I, mut args: ...) -> I {
    let mut a: ::std::ffi::VaListImpl;
    let mut v: V = 0 as *mut libc::c_void;
    let mut o: V = -(1 as libc::c_int) as V;
    a = args.clone();
    loop {
        v = a.arg::<V>();
        if !(o != v) {
            break;
        }
        if v.is_null() {
            g = 1 as libc::c_int as I;
        }
    }
    if g == 0 {
        return 1 as libc::c_int as I;
    }
    a = args.clone();
    loop {
        v = a.arg::<V>();
        if !(o != v) {
            break;
        }
        cd(v as K);
    }
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn rc(mut x: K) -> I {
    return (*x)._c >> 8 as libc::c_int;
}
unsafe extern "C" fn ic(mut x: K) -> K {
    (*x)._c += 256 as libc::c_int as libc::c_longlong;
    return x;
}
unsafe extern "C" fn dc(mut x: K) -> K {
    (*x)._c -= 256 as libc::c_int as libc::c_longlong;
    return x;
}
unsafe extern "C" fn glsz(mut x: K) -> I {
    return 255 as libc::c_int as libc::c_longlong & (*x)._c;
}
unsafe extern "C" fn slsz(mut x: K, mut r: I) -> K {
    (*x)._c = ((*x)._c as libc::c_ulonglong & !(255 as libc::c_int as uI)) as I;
    (*x)._c |= r;
    return x;
}
pub unsafe extern "C" fn mrc(mut x: K, mut c: I) -> K {
    let mut k: I = sz((*x).t, (*x).n);
    let mut r: I = lsz(k);
    (*x)._c = c << 8 as libc::c_int | r;
    return x;
}
pub unsafe extern "C" fn cd(mut x: K) -> K {
    if !x.is_null() && rc(x) <= 0 as libc::c_int as libc::c_longlong {
        fprintf(
            stderr,
            b"%s:%u: %s\n\0" as *const u8 as *const libc::c_char,
            b"src/km.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            b"Tried to cd() already freed item\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"%s:%u: %s=%lld\n\0" as *const u8 as *const libc::c_char,
            b"src/km.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            b"(I)tests\0" as *const u8 as *const libc::c_char,
            tests,
        );
        fprintf(
            stderr,
            b"%s:%u: %s=%lld\n\0" as *const u8 as *const libc::c_char,
            b"src/km.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            b"(I)(L)x\0" as *const u8 as *const libc::c_char,
            x as L,
        );
        fprintf(
            stderr,
            b"%s:%u: %s=%lld\n\0" as *const u8 as *const libc::c_char,
            b"src/km.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            b"(I)rc(x)\0" as *const u8 as *const libc::c_char,
            rc(x),
        );
        fprintf(
            stderr,
            b"%s:%u: %s=%lld\n\0" as *const u8 as *const libc::c_char,
            b"src/km.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            b"(I)x->t\0" as *const u8 as *const libc::c_char,
            (*x).t,
        );
        fprintf(
            stderr,
            b"%s:%u: %s=%lld\n\0" as *const u8 as *const libc::c_char,
            b"src/km.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            b"(I)x->n\0" as *const u8 as *const libc::c_char,
            (*x).n,
        );
        show(x);
    }
    if x.is_null() {
        return 0 as K;
    }
    dc(x);
    match (*x).t {
        5 | 0 => {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (*x).n;
            while i < _i {
                cd(
                    *((*x).k)
                        .as_mut_ptr()
                        .offset(
                            ((*x).n - i - 1 as libc::c_int as libc::c_longlong) as isize,
                        ),
                );
                i += 1;
                i;
            }
        }
        _ => {}
    }
    if (*x)._c > 255 as libc::c_int as libc::c_longlong {
        return x;
    }
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i_0: I = kreci;
    while i_0 < _i_0 {
        if x == krec[i_0 as usize] as K {
            krec[i_0 as usize] = 0 as V;
            break;
        } else {
            i_0 += 1;
            i_0;
        }
    }
    match (*x).t {
        7 => {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = (-(2 as libc::c_int) + TYPE_SEVEN_SIZE as libc::c_int)
                as I;
            while i_1 < _i_1 {
                cd(
                    *(((*x).k).as_mut_ptr() as *mut V)
                        .offset((2 as libc::c_int as libc::c_longlong + i_1) as isize)
                        as K,
                );
                i_1 += 1;
                i_1;
            }
        }
        _ => {}
    }
    let mut o: I = (x as size_t as libc::c_ulonglong
        & (PG - 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong) as I;
    let mut r: I = glsz(x);
    if o == 8 as libc::c_int as libc::c_longlong
        || r > 26 as libc::c_int as libc::c_longlong
    {
        let mut k: I = sz((*x).t, (*x).n);
        let mut s: I = k + o;
        let mut res: I = munmap((x as V).offset(-(o as isize)), s as size_t) as I;
        if res != 0 {
            return kerr(b"munmap\0" as *const u8 as *const libc::c_char);
        }
        if o == 8 as libc::c_int as libc::c_longlong {
            mMap -= s as libc::c_double;
        } else if r > 26 as libc::c_int as libc::c_longlong {
            mAlloc -= s as libc::c_double;
        }
        mUsed -= s as libc::c_double;
    } else {
        repool(x as V, r);
    }
    return 0 as K;
}
pub unsafe extern "C" fn ci(mut x: K) -> K {
    if x.is_null() {
        return 0 as K;
    }
    ic(x);
    match (*x).t {
        5 | 0 => {
            let mut i: I = 0 as libc::c_int as I;
            let mut _i: I = (*x).n;
            while i < _i {
                ci(*((*x).k).as_mut_ptr().offset(i as isize));
                i += 1;
                i;
            }
        }
        _ => {}
    }
    return x;
}
pub unsafe extern "C" fn bp(mut t: I) -> I {
    match if t < 0 as libc::c_int as libc::c_longlong { -t } else { t } {
        1 => return ::std::mem::size_of::<I>() as libc::c_ulong as I,
        2 => return ::std::mem::size_of::<F>() as libc::c_ulong as I,
        3 => return ::std::mem::size_of::<C>() as libc::c_ulong as I,
        _ => return ::std::mem::size_of::<V>() as libc::c_ulong as I,
    };
}
pub unsafe extern "C" fn sz(mut t: I, mut n: I) -> I {
    return ((3 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<I>() as libc::c_ulong) as libc::c_ulonglong)
        .wrapping_add(
            ((if 7 as libc::c_int as libc::c_longlong == t {
                TYPE_SEVEN_SIZE as libc::c_int as libc::c_longlong
            } else {
                n
            }) * bp(t)) as libc::c_ulonglong,
        )
        .wrapping_add(
            (3 as libc::c_int as libc::c_longlong
                == (if t < 0 as libc::c_int as libc::c_longlong { -t } else { t }))
                as libc::c_int as libc::c_ulonglong,
        ) as I;
}
unsafe extern "C" fn nearPG(mut i: I) -> I {
    let mut k: I = (i as size_t as libc::c_ulonglong
        & (PG - 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong) as I;
    return if k != 0 { i + PG - k } else { i };
}
pub unsafe extern "C" fn newK(mut t: I, mut n: I) -> K {
    let mut z: K = 0 as *mut k0;
    if n > 0 as libc::c_int as libc::c_longlong
        && n
            > 9223372036854775807 as libc::c_longlong
                - 1 as libc::c_int as libc::c_longlong
    {
        return kerr(b"wsfull\0" as *const u8 as *const libc::c_char);
    }
    let mut k: I = sz(t, n);
    let mut r: I = 0;
    z = kalloc(k, &mut r) as K;
    if z.is_null() {
        return 0 as K;
    }
    ic(slsz(z, r));
    (*z).t = t;
    (*z).n = n;
    if t == 6 as libc::c_int as libc::c_longlong {
        (*z).n = 0 as libc::c_int as I;
    }
    if (*z)._c == 0 as libc::c_int as libc::c_longlong {
        (*z)._c = 256 as libc::c_int as I;
    }
    let fresh0 = kreci;
    kreci = kreci + 1;
    krec[fresh0 as usize] = z as V;
    return z;
}
unsafe extern "C" fn kallocI(mut k: I, mut r: I) -> V {
    if r > 26 as libc::c_int as libc::c_longlong {
        return amem(k, r);
    }
    return unpool(r);
}
unsafe extern "C" fn kalloc(mut k: I, mut r: *mut I) -> V {
    *r = lsz(k);
    return kallocI(k, *r);
}
unsafe extern "C" fn amem(mut k: I, mut r: I) -> V {
    let mut z: K = 0 as *mut k0;
    z = mmap(
        0 as *mut libc::c_void,
        k as size_t,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x2 as libc::c_int | 0x20 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as __off_t,
    ) as K;
    if -(1 as libc::c_int) as *mut libc::c_void == z as *mut libc::c_void {
        return kerr(b"wsfull\0" as *const u8 as *const libc::c_char) as V;
    }
    mAlloc += (if k < PG { PG } else { k }) as libc::c_double;
    if r > 26 as libc::c_int as libc::c_longlong {
        mUsed += k as libc::c_double;
        if mUsed > mMax {
            mMax = mUsed;
        }
    }
    return z as V;
}
unsafe extern "C" fn unpool(mut r: I) -> V {
    let mut z: *mut V = 0 as *mut V;
    let mut L: *mut V = KP.as_mut_ptr().offset(r as isize);
    let mut k: I = (1 as libc::c_int as I) << r;
    if (*L).is_null() || 0x106 as libc::c_int as V == *L {
        z = amem(k, r) as *mut V;
        if z.is_null() {
            return 0 as V;
        }
        if k < PG {
            let mut y: V = z as V;
            while y < (z as V).offset(PG as isize).offset(-k as isize) {
                let ref mut fresh1 = *(y as *mut V);
                *fresh1 = y.offset(k as isize);
                y = y.offset(k as isize);
            }
        }
        *L = z as V;
    }
    z = *L as *mut V;
    *L = *z;
    *z = 0 as V;
    mUsed += k as libc::c_double;
    if mUsed > mMax {
        mMax = mUsed;
    }
    return z as V;
}
pub unsafe extern "C" fn cl2(mut v: I) -> I {
    if v == 0 {
        return -(1 as libc::c_int) as I;
    }
    let mut e: I = 0 as libc::c_int as I;
    if v as libc::c_ulonglong
        & (v as libc::c_ulonglong).wrapping_sub(1 as libc::c_ulonglong) != 0
    {
        e = 1 as libc::c_int as I;
    }
    if v as libc::c_ulonglong & 0xffff0000 as libc::c_ulonglong != 0 {
        e += 16 as libc::c_int as libc::c_longlong;
        v >>= 16 as libc::c_int;
    }
    if v as libc::c_ulonglong & 0xff00 as libc::c_ulonglong != 0 {
        e += 8 as libc::c_int as libc::c_longlong;
        v >>= 8 as libc::c_int;
    }
    if v as libc::c_ulonglong & 0xf0 as libc::c_ulonglong != 0 {
        e += 4 as libc::c_int as libc::c_longlong;
        v >>= 4 as libc::c_int;
    }
    if v as libc::c_ulonglong & 0xc as libc::c_ulonglong != 0 {
        e += 2 as libc::c_int as libc::c_longlong;
        v >>= 2 as libc::c_int;
    }
    if v as libc::c_ulonglong & 0x2 as libc::c_ulonglong != 0 {
        e += 1 as libc::c_int as libc::c_longlong;
        v >>= 1 as libc::c_int;
    }
    return e;
}
pub unsafe extern "C" fn lsz(mut k: I) -> I {
    return if k <= (1 as libc::c_int as I) << 6 as libc::c_int {
        6 as libc::c_int as libc::c_longlong
    } else {
        cl2(k)
    };
}
pub unsafe extern "C" fn repool(mut v: V, mut r: I) -> I {
    let mut k: I = (1 as libc::c_int as I) << r;
    memset(v, 0 as libc::c_int, k as libc::c_ulong);
    let ref mut fresh2 = *(v as *mut V);
    *fresh2 = KP[r as usize];
    KP[r as usize] = v;
    mUsed -= k as libc::c_double;
    return 0 as libc::c_int as I;
}
unsafe extern "C" fn kexpander(mut p: *mut K, mut n: I) -> I {
    let mut a: K = *p;
    let mut r: I = glsz(a);
    if r > 26 as libc::c_int as libc::c_longlong {
        let mut v: V = 0 as *mut libc::c_void;
        let mut c: I = sz((*a).t, (*a).n);
        let mut d: I = sz((*a).t, n);
        let mut e: I = nearPG(c);
        let mut f: I = d - e;
        if f <= 0 as libc::c_int as libc::c_longlong {
            return 1 as libc::c_int as I;
        }
        let mut w: *mut V = mremap(
            a as *mut libc::c_void,
            c as size_t,
            d as size_t,
            1 as libc::c_int,
        ) as *mut V;
        if -(1 as libc::c_int) as *mut libc::c_void != w as *mut libc::c_void {
            mAlloc += (d - c) as libc::c_double;
            mUsed += (d - c) as libc::c_double;
            if mUsed > mMax {
                mMax = mUsed;
            }
            *p = w as K;
            return 1 as libc::c_int as I;
        }
        v = amem(d, r);
        if v.is_null() {
            return 0 as libc::c_int as I;
        }
        memcpy(v, a as *const libc::c_void, c as libc::c_ulong);
        *p = v as K;
        let mut res: I = munmap(a as *mut libc::c_void, c as size_t) as I;
        if res != 0 {
            show(kerr(b"munmap\0" as *const u8 as *const libc::c_char));
            return 0 as libc::c_int as I;
        }
        mAlloc -= c as libc::c_double;
        mUsed -= c as libc::c_double;
        return 1 as libc::c_int as I;
    }
    let mut d_0: I = sz((*a).t, n);
    if d_0 <= ((1 as libc::c_int) << r) as libc::c_longlong {
        return 1 as libc::c_int as I;
    }
    let mut s: I = lsz(d_0);
    let mut x: K = kallocI(d_0, s) as K;
    if x.is_null() {
        return 0 as libc::c_int as I;
    }
    let mut c_0: I = sz((*a).t, (*a).n);
    memcpy(x as *mut libc::c_void, a as *const libc::c_void, c_0 as libc::c_ulong);
    *p = x;
    slsz(*p, s);
    repool(a as V, r);
    return 1 as libc::c_int as I;
}
unsafe extern "C" fn kap1_(mut a: *mut K, mut v: V) -> K {
    let mut k: K = *a;
    let mut t: I = (*k).t;
    let mut m: I = (*k).n;
    let mut p: I = m + 1 as libc::c_int as libc::c_longlong;
    if kexpander(&mut k, p) == 0 {
        return 0 as K;
    }
    if k != *a {
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = kreci;
        while i < _i {
            if *a == krec[i as usize] as K {
                krec[i as usize] = 0 as V;
                break;
            } else {
                i += 1;
                i;
            }
        }
        *a = k;
    }
    (*k).n = p;
    match -t {
        0 => {
            let ref mut fresh3 = *((*k).k).as_mut_ptr().offset(m as isize);
            *fresh3 = ci(*(v as *mut K).offset(0 as libc::c_int as isize));
        }
        1 => {
            *(((*k).k).as_mut_ptr() as *mut I).offset(m as isize) = *(v as *mut I);
        }
        2 => {
            *(((*k).k).as_mut_ptr() as *mut F).offset(m as isize) = *(v as *mut F);
        }
        3 => {
            *(((*k).k).as_mut_ptr() as *mut C).offset(m as isize) = *(v as *mut C);
            *(((*k).k).as_mut_ptr() as *mut C)
                .offset(p as isize) = 0 as libc::c_int as C;
        }
        4 => {
            let ref mut fresh4 = *(((*k).k).as_mut_ptr() as *mut S).offset(m as isize);
            *fresh4 = *(v as *mut S);
        }
        _ => return 0 as K,
    }
    return k;
}
unsafe extern "C" fn kapn_(mut a: *mut K, mut v: V, mut n: I) -> K {
    if a.is_null() || n == 0 {
        return 0 as K;
    }
    let mut k: K = *a;
    let mut t: I = (*k).t;
    let mut m: I = (*k).n;
    let mut p: I = m + n;
    if 6 as libc::c_int as libc::c_longlong == t {
        let mut z: K = newK(0 as libc::c_int as I, p);
        if z.is_null() {
            return 0 as K;
        }
        let mut zv: *mut K = ((*z).k).as_mut_ptr();
        let fresh5 = zv;
        zv = zv.offset(1);
        *fresh5 = _n();
        let mut i: I = 0 as libc::c_int as I;
        let mut _i: I = n;
        while i < _i {
            let ref mut fresh6 = *zv.offset(i as isize);
            *fresh6 = _n();
            i += 1;
            i;
        }
        cd(k);
        *a = z;
        return z;
    }
    if kexpander(&mut k, p) == 0 {
        return 0 as K;
    }
    if k != *a {
        let mut i_0: I = 0 as libc::c_int as I;
        let mut _i_0: I = kreci;
        while i_0 < _i_0 {
            if *a == krec[i_0 as usize] as K {
                krec[i_0 as usize] = 0 as V;
                break;
            } else {
                i_0 += 1;
                i_0;
            }
        }
        *a = k;
    }
    (*k).n = p;
    match if t < 0 as libc::c_int as libc::c_longlong { -t } else { t } {
        0 | 5 => {
            let mut i_1: I = 0 as libc::c_int as I;
            let mut _i_1: I = n;
            while i_1 < _i_1 {
                let ref mut fresh7 = *((*k).k).as_mut_ptr().offset((i_1 + m) as isize);
                *fresh7 = ci(*(v as *mut K).offset(i_1 as isize));
                i_1 += 1;
                i_1;
            }
        }
        1 => {
            memcpy(
                (((*k).k).as_mut_ptr() as *mut I).offset(m as isize)
                    as *mut libc::c_void,
                v as *const libc::c_void,
                (n as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<I>() as libc::c_ulong as libc::c_ulonglong,
                    ) as libc::c_ulong,
            );
        }
        2 => {
            memcpy(
                (((*k).k).as_mut_ptr() as *mut F).offset(m as isize)
                    as *mut libc::c_void,
                v as *const libc::c_void,
                (n as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<F>() as libc::c_ulong as libc::c_ulonglong,
                    ) as libc::c_ulong,
            );
        }
        3 => {
            strncpy(
                (((*k).k).as_mut_ptr() as *mut C).offset(m as isize),
                v as S as *const libc::c_char,
                n as libc::c_ulong,
            );
            *(((*k).k).as_mut_ptr() as *mut C)
                .offset(p as isize) = 0 as libc::c_int as C;
        }
        4 => {
            memcpy(
                (((*k).k).as_mut_ptr() as *mut S).offset(m as isize)
                    as *mut libc::c_void,
                v as *const libc::c_void,
                (n as libc::c_ulonglong)
                    .wrapping_mul(
                        ::std::mem::size_of::<S>() as libc::c_ulong as libc::c_ulonglong,
                    ) as libc::c_ulong,
            );
        }
        _ => return 0 as K,
    }
    if t > 0 as libc::c_int as libc::c_longlong
        && t < 5 as libc::c_int as libc::c_longlong
        && p > 1 as libc::c_int as libc::c_longlong
    {
        (*k).t *= -(1 as libc::c_int) as libc::c_longlong;
    }
    return *a;
}
pub unsafe extern "C" fn kapn(mut a: *mut K, mut v: V, mut n: I) -> K {
    return kapn_(a, v, n);
}
pub unsafe extern "C" fn kap(mut a: *mut K, mut v: V) -> K {
    if a.is_null() {
        return 0 as K;
    }
    return if (0 as libc::c_int as libc::c_longlong) < (**a).t {
        kapn_(a, v, 1 as libc::c_int as I)
    } else {
        kap1_(a, v)
    };
}
pub unsafe extern "C" fn newN() -> N {
    return unpool(lsz(::std::mem::size_of::<Node>() as libc::c_ulong as I)) as N;
}
pub unsafe extern "C" fn newPDA() -> PDA {
    let mut p: PDA = unpool(lsz(::std::mem::size_of::<Pda>() as libc::c_ulong as I))
        as PDA;
    if p.is_null() {
        return 0 as PDA;
    }
    (*p).c = alloc(1 as libc::c_int as size_t) as S;
    if ((*p).c).is_null() {
        kerr(b"wsfull\0" as *const u8 as *const libc::c_char);
        return 0 as PDA;
    }
    return p;
}
pub unsafe extern "C" fn push(mut p: PDA, mut c: C) -> I {
    return appender(&mut (*p).c, &mut (*p).n, &mut c, 1 as libc::c_int as I);
}
pub unsafe extern "C" fn peek(mut p: PDA) -> C {
    let mut n: I = (*p).n;
    return (if n != 0 {
        *((*p).c).offset((n - 1 as libc::c_int as libc::c_longlong) as isize)
            as libc::c_int
    } else {
        0 as libc::c_int
    }) as C;
}
pub unsafe extern "C" fn pop(mut p: PDA) -> C {
    return (if (*p).n > 0 as libc::c_int as libc::c_longlong {
        (*p).n -= 1;
        *((*p).c).offset((*p).n as isize) as libc::c_int
    } else {
        0 as libc::c_int
    }) as C;
}
pub unsafe extern "C" fn bottom(mut p: PDA) -> C {
    return (if (*p).n > 0 as libc::c_int as libc::c_longlong {
        *((*p).c).offset(0 as libc::c_int as isize) as libc::c_int
    } else {
        0 as libc::c_int
    }) as C;
}
pub unsafe extern "C" fn pdafree(mut p: PDA) {
    free((*p).c as *mut libc::c_void);
    repool(p as V, lsz(::std::mem::size_of::<PDA>() as libc::c_ulong as I));
}
pub unsafe extern "C" fn Ki(mut x: I) -> K {
    let mut z: K = newK(1 as libc::c_int as I, 1 as libc::c_int as I);
    *(((*z).k).as_mut_ptr() as *mut I) = x;
    return z;
}
pub unsafe extern "C" fn Kf(mut x: F) -> K {
    let mut z: K = newK(2 as libc::c_int as I, 1 as libc::c_int as I);
    *(((*z).k).as_mut_ptr() as *mut F) = x;
    return z;
}
pub unsafe extern "C" fn Kc(mut x: C) -> K {
    let mut z: K = newK(3 as libc::c_int as I, 1 as libc::c_int as I);
    *(((*z).k).as_mut_ptr() as *mut C) = x;
    return z;
}
pub unsafe extern "C" fn Ks(mut x: S) -> K {
    if x.is_null() {
        return 0 as K;
    }
    let mut z: K = newK(4 as libc::c_int as I, 1 as libc::c_int as I);
    let ref mut fresh8 = *(((*z).k).as_mut_ptr() as *mut S);
    *fresh8 = x;
    return z;
}
pub unsafe extern "C" fn Kd() -> K {
    return newK(5 as libc::c_int as I, 0 as libc::c_int as I);
}
pub unsafe extern "C" fn Kn() -> K {
    return newK(6 as libc::c_int as I, 1 as libc::c_int as I);
}
pub unsafe extern "C" fn Kv() -> K {
    let mut z: K = newK(7 as libc::c_int as I, TYPE_SEVEN_SIZE as libc::c_int as I);
    if z.is_null() {
        return 0 as K;
    }
    (*z).n = 1 as libc::c_int as I;
    let ref mut fresh9 = *(((*z).k).as_mut_ptr() as *mut V)
        .offset(CONTeXT as libc::c_int as isize);
    *fresh9 = d_ as V;
    let ref mut fresh10 = *(((*z).k).as_mut_ptr() as *mut V)
        .offset(PARAMS as libc::c_int as isize);
    *fresh10 = Kd() as V;
    let ref mut fresh11 = *(((*z).k).as_mut_ptr() as *mut V)
        .offset(LOCALS as libc::c_int as isize);
    *fresh11 = Kd() as V;
    if OOM_CD(0 as libc::c_int as I, z, *fresh10, *fresh11, -(1 as libc::c_int) as V)
        == 0
    {
        return 0 as K;
    }
    return z;
}
pub unsafe extern "C" fn newEntry(mut s: S) -> K {
    return newE(s, _n());
}
pub unsafe extern "C" fn newE(mut s: S, mut k: K) -> K {
    let mut z: K = newK(0 as libc::c_int as I, 3 as libc::c_int as I);
    if z.is_null() {
        return 0 as K;
    }
    let ref mut fresh12 = *((*z).k).as_mut_ptr().offset(0 as libc::c_int as isize);
    *fresh12 = Ks(s);
    let ref mut fresh13 = *((*z).k).as_mut_ptr().offset(1 as libc::c_int as isize);
    *fresh13 = k;
    let ref mut fresh14 = *((*z).k).as_mut_ptr().offset(2 as libc::c_int as isize);
    *fresh14 = _n();
    if OOM_CD(
        0 as libc::c_int as I,
        z,
        *((*z).k).as_mut_ptr().offset(0 as libc::c_int as isize),
        *((*z).k).as_mut_ptr().offset(2 as libc::c_int as isize),
        -(1 as libc::c_int) as V,
    ) == 0
    {
        return 0 as K;
    }
    return z;
}
pub unsafe extern "C" fn rp2(mut v: I) -> I {
    v -= 1;
    v;
    v |= v >> 1 as libc::c_int;
    v |= v >> 2 as libc::c_int;
    v |= v >> 4 as libc::c_int;
    v |= v >> 8 as libc::c_int;
    v |= v >> 16 as libc::c_int;
    if ::std::mem::size_of::<V>() as libc::c_ulong >= 8 as libc::c_int as libc::c_ulong {
        v |= v >> 32 as libc::c_int;
    }
    v += 1;
    v;
    return if 1 as libc::c_int as libc::c_longlong > v {
        1 as libc::c_int as libc::c_longlong
    } else {
        v
    };
}
pub unsafe extern "C" fn mstat() -> K {
    let mut ks: K = newK(-(1 as libc::c_int) as I, 4 as libc::c_int as I);
    if OOM_CD(0 as libc::c_int as I, ks, -(1 as libc::c_int) as V) == 0 {
        return 0 as K;
    }
    let mut s: *mut I = ((*ks).k).as_mut_ptr() as *mut I;
    *s.offset(0 as libc::c_int as isize) = mUsed as I;
    *s.offset(1 as libc::c_int as isize) = mAlloc as I;
    *s.offset(2 as libc::c_int as isize) = mMap as I;
    *s.offset(3 as libc::c_int as isize) = mMax as I;
    return ks;
}

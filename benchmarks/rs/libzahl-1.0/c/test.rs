use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn zperror(_: *const libc::c_char);
    fn zsetup(_: *mut __jmp_buf_tag);
    fn zunsetup();
    fn zfree(_: *mut C2RustUnnamed);
    fn zswap(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsave(_: *mut C2RustUnnamed, _: *mut libc::c_void) -> size_t;
    fn zload(_: *mut C2RustUnnamed, _: *const libc::c_void) -> size_t;
    fn zset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zseti(_: *mut C2RustUnnamed, _: libc::c_longlong);
    fn zsetu(_: *mut C2RustUnnamed, _: libc::c_ulonglong);
    fn zcmp(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> libc::c_int;
    fn zcmpi(_: *mut C2RustUnnamed, _: libc::c_longlong) -> libc::c_int;
    fn zcmpu(_: *mut C2RustUnnamed, _: libc::c_ulonglong) -> libc::c_int;
    fn zcmpmag(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed) -> libc::c_int;
    fn zadd(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsub(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmul(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmodmul(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zdiv(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zdivmod(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zmod(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zsqr(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zneg(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zabs(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zpow(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zmodpow(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zpowu(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: libc::c_ulonglong);
    fn zmodpowu(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: libc::c_ulonglong,
        _: *mut C2RustUnnamed,
    );
    fn zadd_unsigned(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zsub_unsigned(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
    );
    fn zand(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zor(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zxor(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn znot(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zlsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
    fn zrsh(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
    fn ztrunc(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t);
    fn zbtest(_: *mut C2RustUnnamed, _: size_t) -> libc::c_int;
    fn zsplit(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: size_t,
    );
    fn zbits(_: *mut C2RustUnnamed) -> size_t;
    fn zlsb(_: *mut C2RustUnnamed) -> size_t;
    fn zbset(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: size_t, _: libc::c_int);
    fn zgcd(_: *mut C2RustUnnamed, _: *mut C2RustUnnamed, _: *mut C2RustUnnamed);
    fn zptest(
        _: *mut C2RustUnnamed,
        _: *mut C2RustUnnamed,
        _: libc::c_int,
    ) -> zprimality;
    fn zrand(_: *mut C2RustUnnamed, _: zranddev, _: zranddist, _: *mut C2RustUnnamed);
    fn zstr(_: *mut C2RustUnnamed, _: *mut libc::c_char) -> *mut libc::c_char;
    fn zsets(_: *mut C2RustUnnamed, _: *const libc::c_char) -> libc::c_int;
    fn zstr_length(_: *mut C2RustUnnamed, _: libc::c_ulonglong) -> size_t;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint32_t = __uint32_t;
pub type zahl_char_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub sign: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub type z_t = [C2RustUnnamed; 1];
pub type zprimality = libc::c_uint;
pub const PRIME: zprimality = 2;
pub const PROBABLY_PRIME: zprimality = 1;
pub const NONPRIME: zprimality = 0;
pub type zranddev = libc::c_uint;
pub const SECURE_RANDOM: zranddev = 1;
pub const FAST_RANDOM: zranddev = 0;
pub type zranddist = libc::c_uint;
pub const UNIFORM: zranddist = 1;
pub const QUASIUNIFORM: zranddist = 0;
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
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zzero(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zodd(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign != 0
        && *((*a).chars).offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_uint != 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zeven(mut a: *mut C2RustUnnamed) -> libc::c_int {
    return ((*a).sign == 0
        || *((*a).chars).offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_uint == 0) as libc::c_int;
}
#[inline]
unsafe extern "C" fn zinit(mut a: *mut C2RustUnnamed) {
    (*a).alloced = 0 as libc::c_int as size_t;
    (*a).chars = 0 as *mut zahl_char_t;
}
unsafe fn main_0() -> libc::c_int {
    static mut a: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    static mut b: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    static mut c: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    static mut d: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    static mut _0: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    static mut _1: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    static mut _2: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    static mut _3: z_t = [C2RustUnnamed {
        sign: 0,
        used: 0,
        alloced: 0,
        chars: 0 as *const zahl_char_t as *mut zahl_char_t,
    }; 1];
    static mut buf: [libc::c_char; 2000] = [0; 2000];
    static mut ret: libc::c_int = 0 as libc::c_int;
    static mut env: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    static mut env2: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    static mut n: size_t = 0;
    if _setjmp(env.as_mut_ptr()) != 0 {
        zperror(0 as *const libc::c_char);
        ret = 2 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetup(env.as_mut_ptr());
    zinit(a.as_mut_ptr());
    zinit(b.as_mut_ptr());
    zinit(c.as_mut_ptr());
    zinit(d.as_mut_ptr());
    zinit(_0.as_mut_ptr());
    zinit(_1.as_mut_ptr());
    zinit(_2.as_mut_ptr());
    zinit(_3.as_mut_ptr());
    zsetu(_0.as_mut_ptr(), 0 as libc::c_int as libc::c_ulonglong);
    zsetu(_1.as_mut_ptr(), 1 as libc::c_int as libc::c_ulonglong);
    zsetu(_2.as_mut_ptr(), 2 as libc::c_int as libc::c_ulonglong);
    zsetu(_3.as_mut_ptr(), 3 as libc::c_int as libc::c_ulonglong);
    let mut got: libc::c_int = zeven(_0.as_mut_ptr());
    if !(got == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            84 as libc::c_int,
            b"zeven(_0)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_0: libc::c_int = zodd(_0.as_mut_ptr());
    if !(got_0 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            85 as libc::c_int,
            b"zodd(_0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_0,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_1: libc::c_int = zzero(_0.as_mut_ptr());
    if !(got_1 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            86 as libc::c_int,
            b"zzero(_0)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_1,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_2: libc::c_int = zsignum(_0.as_mut_ptr());
    if !(got_2 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            87 as libc::c_int,
            b"zsignum(_0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_2,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_3: libc::c_int = zeven(_1.as_mut_ptr());
    if !(got_3 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            88 as libc::c_int,
            b"zeven(_1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_3,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_4: libc::c_int = zodd(_1.as_mut_ptr());
    if !(got_4 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            89 as libc::c_int,
            b"zodd(_1)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_4,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_5: libc::c_int = zzero(_1.as_mut_ptr());
    if !(got_5 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            90 as libc::c_int,
            b"zzero(_1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_5,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_6: libc::c_int = zsignum(_1.as_mut_ptr());
    if !(got_6 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            91 as libc::c_int,
            b"zsignum(_1)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_6,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_7: libc::c_int = zeven(_2.as_mut_ptr());
    if !(got_7 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            92 as libc::c_int,
            b"zeven(_2)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_7,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_8: libc::c_int = zodd(_2.as_mut_ptr());
    if !(got_8 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            93 as libc::c_int,
            b"zodd(_2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_8,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_9: libc::c_int = zzero(_2.as_mut_ptr());
    if !(got_9 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            94 as libc::c_int,
            b"zzero(_2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_9,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_10: libc::c_int = zsignum(_2.as_mut_ptr());
    if !(got_10 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            95 as libc::c_int,
            b"zsignum(_2)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_10,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zswap(_1.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_11: libc::c_int = zeven(_2.as_mut_ptr());
    if !(got_11 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            98 as libc::c_int,
            b"zeven(_2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_11,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_12: libc::c_int = zodd(_2.as_mut_ptr());
    if !(got_12 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            99 as libc::c_int,
            b"zodd(_2)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_12,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_13: libc::c_int = zzero(_2.as_mut_ptr());
    if !(got_13 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            100 as libc::c_int,
            b"zzero(_2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_13,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_14: libc::c_int = zsignum(_2.as_mut_ptr());
    if !(got_14 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            101 as libc::c_int,
            b"zsignum(_2)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_14,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_15: libc::c_int = zeven(_1.as_mut_ptr());
    if !(got_15 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            102 as libc::c_int,
            b"zeven(_1)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_15,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_16: libc::c_int = zodd(_1.as_mut_ptr());
    if !(got_16 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            103 as libc::c_int,
            b"zodd(_1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_16,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_17: libc::c_int = zzero(_1.as_mut_ptr());
    if !(got_17 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            104 as libc::c_int,
            b"zzero(_1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_17,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_18: libc::c_int = zsignum(_1.as_mut_ptr());
    if !(got_18 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            105 as libc::c_int,
            b"zsignum(_1)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_18,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zswap(_2.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_19: libc::c_int = zeven(_1.as_mut_ptr());
    if !(got_19 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            107 as libc::c_int,
            b"zeven(_1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_19,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_20: libc::c_int = zodd(_1.as_mut_ptr());
    if !(got_20 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            108 as libc::c_int,
            b"zodd(_1)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_20,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_21: libc::c_int = zzero(_1.as_mut_ptr());
    if !(got_21 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            109 as libc::c_int,
            b"zzero(_1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_21,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_22: libc::c_int = zsignum(_1.as_mut_ptr());
    if !(got_22 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            110 as libc::c_int,
            b"zsignum(_1)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_22,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_23: libc::c_int = zeven(_2.as_mut_ptr());
    if !(got_23 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            111 as libc::c_int,
            b"zeven(_2)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_23,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_24: libc::c_int = zodd(_2.as_mut_ptr());
    if !(got_24 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            112 as libc::c_int,
            b"zodd(_2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_24,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_25: libc::c_int = zzero(_2.as_mut_ptr());
    if !(got_25 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            113 as libc::c_int,
            b"zzero(_2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_25,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_26: libc::c_int = zsignum(_2.as_mut_ptr());
    if !(got_26 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            114 as libc::c_int,
            b"zsignum(_2)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_26,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_2.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_27: libc::c_int = zsignum(_2.as_mut_ptr());
    if !(got_27 == -(1 as libc::c_int)) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            116 as libc::c_int,
            b"(zneg(_2, _2), zsignum(_2))\0" as *const u8 as *const libc::c_char,
            b"== -1\0" as *const u8 as *const libc::c_char,
            got_27,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_2.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_28: libc::c_int = zsignum(_2.as_mut_ptr());
    if !(got_28 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            117 as libc::c_int,
            b"zsignum(_2)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_28,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_29: libc::c_int = zcmp(_0.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_29 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            119 as libc::c_int,
            b"zcmp(_0, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_29,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_30: libc::c_int = zcmp(_1.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_30 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            120 as libc::c_int,
            b"zcmp(_1, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_30,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_31: libc::c_int = zcmp(_0.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_31 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            121 as libc::c_int,
            b"zcmp(_0, _1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_31,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_32: libc::c_int = zcmp(_1.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_32 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            122 as libc::c_int,
            b"zcmp(_1, _0)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_32,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_33: libc::c_int = zcmp(_1.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_33 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            123 as libc::c_int,
            b"zcmp(_1, _2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_33,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_34: libc::c_int = zcmp(_2.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_34 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            124 as libc::c_int,
            b"zcmp(_2, _1)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_34,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_35: libc::c_int = zcmp(_0.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_35 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            125 as libc::c_int,
            b"zcmp(_0, _2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_35,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_36: libc::c_int = zcmp(_2.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_36 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            126 as libc::c_int,
            b"zcmp(_2, _0)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_36,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zbset(a.as_mut_ptr(), _0.as_mut_ptr(), 0 as libc::c_int as size_t, 1 as libc::c_int);
    let mut got_37: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_37 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            129 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_37,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zbset(a.as_mut_ptr(), a.as_mut_ptr(), 1 as libc::c_int as size_t, 1 as libc::c_int);
    let mut got_38: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_38 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            131 as libc::c_int,
            b"zcmp(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_38,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zbset(a.as_mut_ptr(), a.as_mut_ptr(), 0 as libc::c_int as size_t, 0 as libc::c_int);
    let mut got_39: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_39 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            133 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_39,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zbset(a.as_mut_ptr(), a.as_mut_ptr(), 0 as libc::c_int as size_t, 0 as libc::c_int);
    let mut got_40: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_40 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            135 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_40,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zbset(
        a.as_mut_ptr(),
        a.as_mut_ptr(),
        0 as libc::c_int as size_t,
        -(1 as libc::c_int),
    );
    let mut got_41: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_41 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            137 as libc::c_int,
            b"zcmp(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_41,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zbset(
        a.as_mut_ptr(),
        a.as_mut_ptr(),
        0 as libc::c_int as size_t,
        -(1 as libc::c_int),
    );
    let mut got_42: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_42 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            139 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_42,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_43: libc::c_int = zsignum(a.as_mut_ptr());
    if !(got_43 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            142 as libc::c_int,
            b"zsignum(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_43,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_44: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_44 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            143 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_44,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_45: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        1 as libc::c_int as libc::c_longlong,
    );
    if !(got_45 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            144 as libc::c_int,
            b"zcmpi(a, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_45,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_46: libc::c_int = zcmpu(
        a.as_mut_ptr(),
        1 as libc::c_int as libc::c_ulonglong,
    );
    if !(got_46 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            145 as libc::c_int,
            b"zcmpu(a, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_46,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(a.as_mut_ptr(), a.as_mut_ptr());
    let mut got_47: libc::c_int = zsignum(a.as_mut_ptr());
    if !(got_47 == -(1 as libc::c_int)) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            147 as libc::c_int,
            b"zsignum(a)\0" as *const u8 as *const libc::c_char,
            b"== -1\0" as *const u8 as *const libc::c_char,
            got_47,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_48: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_48 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            148 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_48,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_49: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        1 as libc::c_int as libc::c_longlong,
    );
    if !(got_49 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            149 as libc::c_int,
            b"zcmpi(a, 1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_49,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_50: libc::c_int = zcmpu(
        a.as_mut_ptr(),
        1 as libc::c_int as libc::c_ulonglong,
    );
    if !(got_50 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            150 as libc::c_int,
            b"zcmpu(a, 1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_50,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(a.as_mut_ptr(), _2.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_51: libc::c_int = zsignum(a.as_mut_ptr());
    if !(got_51 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            152 as libc::c_int,
            b"zsignum(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_51,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_52: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_52 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            153 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_52,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_53: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_longlong,
    );
    if !(got_53 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            154 as libc::c_int,
            b"zcmpi(a, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_53,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_54: libc::c_int = zcmpu(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_ulonglong,
    );
    if !(got_54 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            155 as libc::c_int,
            b"zcmpu(a, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_54,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(a.as_mut_ptr(), a.as_mut_ptr());
    let mut got_55: libc::c_int = zsignum(a.as_mut_ptr());
    if !(got_55 == -(1 as libc::c_int)) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            157 as libc::c_int,
            b"zsignum(a)\0" as *const u8 as *const libc::c_char,
            b"== -1\0" as *const u8 as *const libc::c_char,
            got_55,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_56: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_56 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            158 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_56,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_57: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_longlong,
    );
    if !(got_57 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            159 as libc::c_int,
            b"zcmpi(a, 2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_57,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_58: libc::c_int = zcmpu(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_ulonglong,
    );
    if !(got_58 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            160 as libc::c_int,
            b"zcmpu(a, 2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_58,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_59: libc::c_int = zsignum(_1.as_mut_ptr());
    if !(got_59 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            161 as libc::c_int,
            b"zsignum(_1)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_59,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_60: libc::c_int = zsignum(a.as_mut_ptr());
    if !(got_60 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            163 as libc::c_int,
            b"zsignum(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_60,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_61: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_61 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            164 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_61,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_62: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_longlong,
    );
    if !(got_62 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            165 as libc::c_int,
            b"zcmpi(a, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_62,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_63: libc::c_int = zcmpu(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_ulonglong,
    );
    if !(got_63 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            166 as libc::c_int,
            b"zcmpu(a, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_63,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zset(b.as_mut_ptr(), _1.as_mut_ptr());
    zadd(a.as_mut_ptr(), b.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_64: libc::c_int = zsignum(a.as_mut_ptr());
    if !(got_64 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            169 as libc::c_int,
            b"zsignum(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_64,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_65: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_65 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            170 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_65,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_66: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_longlong,
    );
    if !(got_66 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            171 as libc::c_int,
            b"zcmpi(a, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_66,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_67: libc::c_int = zcmpu(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_ulonglong,
    );
    if !(got_67 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            172 as libc::c_int,
            b"zcmpu(a, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_67,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(a.as_mut_ptr(), a.as_mut_ptr());
    zset(b.as_mut_ptr(), _2.as_mut_ptr());
    zneg(b.as_mut_ptr(), b.as_mut_ptr());
    let mut got_68: libc::c_int = zsignum(a.as_mut_ptr());
    if !(got_68 == -(1 as libc::c_int)) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            176 as libc::c_int,
            b"zsignum(a)\0" as *const u8 as *const libc::c_char,
            b"== -1\0" as *const u8 as *const libc::c_char,
            got_68,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_69: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_69 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            177 as libc::c_int,
            b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_69,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_70: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_70 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            178 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_70,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_71: libc::c_int = zcmpmag(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_71 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            179 as libc::c_int,
            b"zcmpmag(a, b)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_71,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_72: libc::c_int = zcmpmag(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_72 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            180 as libc::c_int,
            b"zcmpmag(a, _2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_72,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_73: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_longlong,
    );
    if !(got_73 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            181 as libc::c_int,
            b"zcmpi(a, 2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_73,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_74: libc::c_int = zcmpu(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_ulonglong,
    );
    if !(got_74 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            182 as libc::c_int,
            b"zcmpu(a, 2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_74,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_75: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        -(2 as libc::c_int) as libc::c_longlong,
    );
    if !(got_75 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            183 as libc::c_int,
            b"zcmpi(a, -2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_75,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_2.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_76: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_76 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            184 as libc::c_int,
            b"(zneg(_2, _2), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_76,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_2.as_mut_ptr(), _2.as_mut_ptr());
    zadd(a.as_mut_ptr(), _1.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_77: libc::c_int = zsignum(a.as_mut_ptr());
    if !(got_77 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            186 as libc::c_int,
            b"zsignum(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_77,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_78: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_78 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            187 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_78,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_79: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_longlong,
    );
    if !(got_79 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            188 as libc::c_int,
            b"zcmpi(a, 2)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_79,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_80: libc::c_int = zcmpu(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_ulonglong,
    );
    if !(got_80 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            189 as libc::c_int,
            b"zcmpu(a, 2)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_80,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(a.as_mut_ptr(), a.as_mut_ptr());
    zset(b.as_mut_ptr(), _2.as_mut_ptr());
    zneg(b.as_mut_ptr(), b.as_mut_ptr());
    let mut got_81: libc::c_int = zsignum(a.as_mut_ptr());
    if !(got_81 == -(1 as libc::c_int)) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            193 as libc::c_int,
            b"zsignum(a)\0" as *const u8 as *const libc::c_char,
            b"== -1\0" as *const u8 as *const libc::c_char,
            got_81,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_82: libc::c_int = zcmpmag(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_82 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            194 as libc::c_int,
            b"zcmpmag(a, _2)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_82,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_83: libc::c_int = zcmpmag(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_83 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            195 as libc::c_int,
            b"zcmpmag(a, b)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_83,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_84: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_84 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            196 as libc::c_int,
            b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_84,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_85: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_85 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            197 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_85,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_86: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_longlong,
    );
    if !(got_86 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            198 as libc::c_int,
            b"zcmpi(a, 2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_86,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_87: libc::c_int = zcmpu(
        a.as_mut_ptr(),
        2 as libc::c_int as libc::c_ulonglong,
    );
    if !(got_87 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            199 as libc::c_int,
            b"zcmpu(a, 2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_87,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_88: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        -(2 as libc::c_int) as libc::c_longlong,
    );
    if !(got_88 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            200 as libc::c_int,
            b"zcmpi(a, -2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_88,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_2.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_89: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_89 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            201 as libc::c_int,
            b"(zneg(_2, _2), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_89,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_2.as_mut_ptr(), _2.as_mut_ptr());
    zneg(b.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_90: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_90 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            203 as libc::c_int,
            b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_90,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zunsetup();
    zsetup(env.as_mut_ptr());
    zsub(a.as_mut_ptr(), _2.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_91: libc::c_int = zcmpmag(_2.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_91 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            209 as libc::c_int,
            b"zcmpmag(_2, _1)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_91,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_92: libc::c_int = zcmpmag(_2.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_92 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            210 as libc::c_int,
            b"zcmpmag(_2, _0)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_92,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_93: libc::c_int = zcmpmag(_1.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_93 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            211 as libc::c_int,
            b"zcmpmag(_1, _0)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_93,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(b.as_mut_ptr(), _1.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_94: libc::c_int = zcmpmag(_2.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_94 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            213 as libc::c_int,
            b"zcmpmag(_2, _0)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_94,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_95: libc::c_int = zcmpmag(_1.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_95 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            214 as libc::c_int,
            b"zcmpmag(_1, _0)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_95,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_96: libc::c_int = zcmpmag(_2.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_96 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            215 as libc::c_int,
            b"zcmpmag(_2, _1)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_96,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_97: libc::c_int = zcmpmag(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_97 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            216 as libc::c_int,
            b"zcmpmag(a, b)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_97,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_98: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_98 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            217 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_98,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_99: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_99 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            218 as libc::c_int,
            b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_99,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_100: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_100 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            219 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_100,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_101: libc::c_int = zcmp(b.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_101 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            220 as libc::c_int,
            b"zcmp(b, _1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_101,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_102: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_102 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            222 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_102,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(b.as_mut_ptr(), 0 as libc::c_int as libc::c_longlong);
    zsetu(c.as_mut_ptr(), 0 as libc::c_int as libc::c_ulonglong);
    zsub(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_103: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_103 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            226 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_103,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_104: libc::c_int = zcmpmag(_2.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_104 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            227 as libc::c_int,
            b"zcmpmag(_2, _1)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_104,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_105: libc::c_int = zcmp(_2.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_105 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            228 as libc::c_int,
            b"zcmp(_2, _1)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_105,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(a.as_mut_ptr(), _2.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_106: libc::c_int = zsignum(a.as_mut_ptr());
    if !(got_106 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            230 as libc::c_int,
            b"zsignum(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_106,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_107: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_107 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            231 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_107,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_108: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_108 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            232 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_108,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(a.as_mut_ptr(), a.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_109: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_109 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            234 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_109,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(a.as_mut_ptr(), a.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_110: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_110 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            236 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_110,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(a.as_mut_ptr(), _1.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_111: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_111 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            238 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_111,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_112: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_112 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            239 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_112,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zabs(a.as_mut_ptr(), a.as_mut_ptr());
    let mut got_113: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_113 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            241 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_113,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zabs(a.as_mut_ptr(), a.as_mut_ptr());
    let mut got_114: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_114 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            243 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_114,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zabs(a.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_115: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_115 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            245 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_115,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zabs(a.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_116: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_116 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            247 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_116,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(b.as_mut_ptr(), -(1 as libc::c_int) as libc::c_longlong);
    zseti(c.as_mut_ptr(), -(2 as libc::c_int) as libc::c_longlong);
    zadd(a.as_mut_ptr(), _0.as_mut_ptr(), b.as_mut_ptr());
    let mut got_117: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_117 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            252 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_117,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_118: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        -(1 as libc::c_int) as libc::c_longlong,
    );
    if !(got_118 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            253 as libc::c_int,
            b"zcmpi(a, -1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_118,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_119: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_119 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            254 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_119,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_120: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_120 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            255 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_120,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(a.as_mut_ptr(), b.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_121: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_121 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            257 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_121,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_122: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        -(1 as libc::c_int) as libc::c_longlong,
    );
    if !(got_122 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            258 as libc::c_int,
            b"zcmpi(a, -1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_122,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_123: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_123 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            259 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_123,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_124: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_124 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            260 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_124,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_125: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_125 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            262 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_125,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_126: libc::c_int = zcmpmag(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_126 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            263 as libc::c_int,
            b"zcmpmag(a, _2)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_126,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(a.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr());
    let mut got_127: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_127 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            265 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_127,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_128: libc::c_int = zcmpmag(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_128 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            266 as libc::c_int,
            b"zcmpmag(a, _2)\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_128,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(a.as_mut_ptr(), b.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_129: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_129 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            268 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_129,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_130: libc::c_int = zcmpmag(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_130 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            269 as libc::c_int,
            b"zcmpmag(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_130,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(a.as_mut_ptr(), _1.as_mut_ptr(), b.as_mut_ptr());
    let mut got_131: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_131 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            271 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_131,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_132: libc::c_int = zcmpmag(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_132 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            272 as libc::c_int,
            b"zcmpmag(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_132,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(b.as_mut_ptr(), _1.as_mut_ptr());
    zneg(c.as_mut_ptr(), _2.as_mut_ptr());
    zsub(a.as_mut_ptr(), _0.as_mut_ptr(), b.as_mut_ptr());
    let mut got_133: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_133 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            277 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_133,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(a.as_mut_ptr(), b.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_134: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_134 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            279 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_134,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_135: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_135 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            280 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_135,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_136: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_136 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            282 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_136,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_137: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_137 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            283 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_137,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(a.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr());
    let mut got_138: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_138 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            285 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_138,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_139: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_139 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            286 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_139,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(a.as_mut_ptr(), b.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_140: libc::c_int = zcmpmag(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_140 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            288 as libc::c_int,
            b"zcmpmag(a, _2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_140,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_141: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_141 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            289 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_141,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_142: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_142 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            290 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_142,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(a.as_mut_ptr(), _1.as_mut_ptr(), b.as_mut_ptr());
    let mut got_143: libc::c_int = zcmp(b.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_143 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            292 as libc::c_int,
            b"zcmp(b, _1)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_143,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_144: libc::c_int = zcmpmag(b.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_144 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            293 as libc::c_int,
            b"zcmpmag(b, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_144,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_145: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_145 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            294 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_145,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000 as libc::c_int as libc::c_ulonglong);
    zsetu(b.as_mut_ptr(), 0 as libc::c_int as libc::c_ulonglong);
    let mut got_146: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_146 != 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            298 as libc::c_int,
            b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
            b"!= 0\0" as *const u8 as *const libc::c_char,
            got_146,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    n = zsave(a.as_mut_ptr(), buf.as_mut_ptr() as *mut libc::c_void);
    let mut got_147: libc::c_int = n as libc::c_int;
    if !(got_147 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            300 as libc::c_int,
            b"n\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_147,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_148: size_t = zload(
        b.as_mut_ptr(),
        buf.as_mut_ptr() as *const libc::c_void,
    );
    if got_148 != n {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            301 as libc::c_int,
            b"zload(b, buf)\0" as *const u8 as *const libc::c_char,
            n,
            got_148,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_149: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_149 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            302 as libc::c_int,
            b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_149,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(b.as_mut_ptr(), _1.as_mut_ptr());
    zneg(c.as_mut_ptr(), _2.as_mut_ptr());
    zadd_unsigned(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_150: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_150 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            307 as libc::c_int,
            b"(zadd_unsigned(a, b, c), zcmp(a, _3))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_150,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_151: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_151 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            308 as libc::c_int,
            b"(zadd_unsigned(a, b, c), zcmp(a, _3))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_151,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), b.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_152: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_152 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            309 as libc::c_int,
            b"(zadd_unsigned(a, b, _2), zcmp(a, _3))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_152,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), _1.as_mut_ptr(), c.as_mut_ptr());
    let mut got_153: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_153 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            310 as libc::c_int,
            b"(zadd_unsigned(a, _1, c), zcmp(a, _3))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_153,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_154: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_154 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            312 as libc::c_int,
            b"(zadd_unsigned(a, _0, _0), zcmp(a, _0))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_154,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_155: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_155 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            313 as libc::c_int,
            b"(zadd_unsigned(a, _0, _1), zcmp(a, _1))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_155,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_156: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_156 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            314 as libc::c_int,
            b"(zadd_unsigned(a, _1, _1), zcmp(a, _2))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_156,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_157: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_157 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            315 as libc::c_int,
            b"(zadd_unsigned(a, _1, _0), zcmp(a, _1))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_157,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zadd_unsigned(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_158: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_158 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            317 as libc::c_int,
            b"(zadd_unsigned(a, _0, _0), zcmp(a, _0))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_158,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_159: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_159 != 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            318 as libc::c_int,
            b"(zadd_unsigned(a, _0, _1), zcmp(a, _1))\0" as *const u8
                as *const libc::c_char,
            b"!= 0\0" as *const u8 as *const libc::c_char,
            got_159,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_160: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_160 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            319 as libc::c_int,
            b"(zadd_unsigned(a, _0, _1), zcmpmag(a, _1))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_160,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_161: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_161 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            320 as libc::c_int,
            b"(zadd_unsigned(a, _1, _1), zcmp(a, _2))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_161,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_162: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_162 != 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            321 as libc::c_int,
            b"(zadd_unsigned(a, _1, _0), zcmp(a, _1))\0" as *const u8
                as *const libc::c_char,
            b"!= 0\0" as *const u8 as *const libc::c_char,
            got_162,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd_unsigned(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_163: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_163 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            322 as libc::c_int,
            b"(zadd_unsigned(a, _1, _0), zcmpmag(a, _1))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_163,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zsub_unsigned(a.as_mut_ptr(), _2.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_164: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_164 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            325 as libc::c_int,
            b"(zsub_unsigned(a, _2, _1), zcmp(a, _1))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_164,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub_unsigned(a.as_mut_ptr(), _2.as_mut_ptr(), b.as_mut_ptr());
    let mut got_165: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_165 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            326 as libc::c_int,
            b"(zsub_unsigned(a, _2, b), zcmp(a, _1))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_165,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub_unsigned(a.as_mut_ptr(), c.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_166: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_166 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            327 as libc::c_int,
            b"(zsub_unsigned(a, c, _1), zcmp(a, _1))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_166,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub_unsigned(a.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr());
    let mut got_167: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_167 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            328 as libc::c_int,
            b"(zsub_unsigned(a, c, b), zcmp(a, _1))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_167,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub_unsigned(a.as_mut_ptr(), _1.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_168: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_168 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            330 as libc::c_int,
            b"(zsub_unsigned(a, _1, _2), zcmp(a, b))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_168,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub_unsigned(a.as_mut_ptr(), b.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_169: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_169 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            331 as libc::c_int,
            b"(zsub_unsigned(a, b, _2), zcmp(a, b))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_169,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub_unsigned(a.as_mut_ptr(), _1.as_mut_ptr(), c.as_mut_ptr());
    let mut got_170: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_170 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            332 as libc::c_int,
            b"(zsub_unsigned(a, _1, c), zcmp(a, b))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_170,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub_unsigned(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_171: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_171 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            333 as libc::c_int,
            b"(zsub_unsigned(a, b, c), zcmp(a, b))\0" as *const u8
                as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_171,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_172: size_t = zbits(_0.as_mut_ptr());
    if got_172 != 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            335 as libc::c_int,
            b"zbits(_0)\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
            got_172,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_173: size_t = zbits(_1.as_mut_ptr());
    if got_173 != 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            336 as libc::c_int,
            b"zbits(_1)\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
            got_173,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_174: size_t = zbits(_2.as_mut_ptr());
    if got_174 != 2 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            337 as libc::c_int,
            b"zbits(_2)\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
            got_174,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_175: size_t = zbits(_3.as_mut_ptr());
    if got_175 != 2 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            338 as libc::c_int,
            b"zbits(_3)\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
            got_175,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_176: size_t = zlsb(_0.as_mut_ptr());
    if got_176 != 18446744073709551615 as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            340 as libc::c_int,
            b"zlsb(_0)\0" as *const u8 as *const libc::c_char,
            18446744073709551615 as libc::c_ulong,
            got_176,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_177: size_t = zlsb(_1.as_mut_ptr());
    if got_177 != 0 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            341 as libc::c_int,
            b"zlsb(_1)\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
            got_177,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_178: size_t = zlsb(_2.as_mut_ptr());
    if got_178 != 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            342 as libc::c_int,
            b"zlsb(_2)\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
            got_178,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_179: size_t = zlsb(_3.as_mut_ptr());
    if got_179 != 0 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            343 as libc::c_int,
            b"zlsb(_3)\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
            got_179,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_180: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_180 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            345 as libc::c_int,
            b"(zand(a, _0, _0), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_180,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_181: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_181 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            346 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_181,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_182: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_182 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            347 as libc::c_int,
            b"(zand(a, _0, _1), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_182,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_183: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_183 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            348 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_183,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _0.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_184: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_184 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            349 as libc::c_int,
            b"(zand(a, _0, _2), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_184,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_185: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_185 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            350 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_185,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _0.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_186: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_186 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            351 as libc::c_int,
            b"(zand(a, _0, _3), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_186,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_187: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_187 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            352 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_187,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_188: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_188 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            353 as libc::c_int,
            b"(zand(a, _1, _1), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_188,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _1.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_189: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_189 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            354 as libc::c_int,
            b"(zand(a, _1, _2), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_189,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_190: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_190 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            355 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_190,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _1.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_191: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_191 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            356 as libc::c_int,
            b"(zand(a, _1, _3), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_191,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _2.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_192: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_192 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            357 as libc::c_int,
            b"(zand(a, _2, _2), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_192,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _2.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_193: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_193 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            358 as libc::c_int,
            b"(zand(a, _2, _3), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_193,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _3.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_194: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_194 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            359 as libc::c_int,
            b"(zand(a, _3, _3), zcmp(a, _3))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_194,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_195: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_195 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            361 as libc::c_int,
            b"(zor(a, _0, _0), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_195,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_196: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_196 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            362 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_196,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_197: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_197 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            363 as libc::c_int,
            b"(zor(a, _0, _1), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_197,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _0.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_198: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_198 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            364 as libc::c_int,
            b"(zor(a, _0, _2), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_198,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _0.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_199: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_199 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            365 as libc::c_int,
            b"(zor(a, _0, _3), zcmp(a, _3))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_199,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_200: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_200 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            366 as libc::c_int,
            b"(zor(a, _1, _1), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_200,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _1.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_201: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_201 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            367 as libc::c_int,
            b"(zor(a, _1, _2), zcmp(a, _3))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_201,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _1.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_202: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_202 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            368 as libc::c_int,
            b"(zor(a, _1, _3), zcmp(a, _3))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_202,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _2.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_203: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_203 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            369 as libc::c_int,
            b"(zor(a, _2, _2), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_203,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _2.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_204: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_204 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            370 as libc::c_int,
            b"(zor(a, _2, _3), zcmp(a, _3))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_204,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _3.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_205: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_205 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            371 as libc::c_int,
            b"(zor(a, _3, _3), zcmp(a, _3))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_205,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_206: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_206 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            373 as libc::c_int,
            b"(zxor(a, _0, _0), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_206,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_207: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_207 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            374 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_207,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_208: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_208 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            375 as libc::c_int,
            b"(zxor(a, _0, _1), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_208,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _0.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_209: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_209 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            376 as libc::c_int,
            b"(zxor(a, _0, _2), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_209,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _0.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_210: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_210 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            377 as libc::c_int,
            b"(zxor(a, _0, _3), zcmp(a, _3))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_210,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_211: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_211 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            378 as libc::c_int,
            b"(zxor(a, _1, _1), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_211,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_212: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_212 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            379 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_212,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _1.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_213: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_213 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            380 as libc::c_int,
            b"(zxor(a, _1, _2), zcmp(a, _3))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_213,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _1.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_214: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_214 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            381 as libc::c_int,
            b"(zxor(a, _1, _3), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_214,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _2.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_215: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_215 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            382 as libc::c_int,
            b"(zxor(a, _2, _2), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_215,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_216: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_216 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            383 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_216,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _2.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_217: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_217 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            384 as libc::c_int,
            b"(zxor(a, _2, _3), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_217,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _3.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_218: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_218 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            385 as libc::c_int,
            b"(zxor(a, _3, _3), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_218,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_219: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_219 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            386 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_219,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(b.as_mut_ptr(), _1.as_mut_ptr());
    zneg(c.as_mut_ptr(), _3.as_mut_ptr());
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zand(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_220: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_220 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            392 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_220,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_221: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_221 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            393 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_221,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zand(a.as_mut_ptr(), b.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_222: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_222 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            395 as libc::c_int,
            b"(zand(a, b, _3), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_222,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _1.as_mut_ptr(), c.as_mut_ptr());
    let mut got_223: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_223 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            396 as libc::c_int,
            b"(zand(a, _1, c), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_223,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), _0.as_mut_ptr(), c.as_mut_ptr());
    let mut got_224: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_224 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            397 as libc::c_int,
            b"(zand(a, _0, c), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_224,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zand(a.as_mut_ptr(), b.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_225: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_225 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            398 as libc::c_int,
            b"(zand(a, b, _0), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_225,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(b.as_mut_ptr(), _1.as_mut_ptr());
    zneg(c.as_mut_ptr(), _2.as_mut_ptr());
    zneg(_3.as_mut_ptr(), _3.as_mut_ptr());
    zor(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_226: libc::c_int = zcmpmag(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_226 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            404 as libc::c_int,
            b"zcmpmag(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_226,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_227: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_227 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            405 as libc::c_int,
            b"zcmp(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_227,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), b.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_228: libc::c_int = zcmpmag(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_228 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            407 as libc::c_int,
            b"zcmpmag(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_228,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_229: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_229 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            408 as libc::c_int,
            b"zcmp(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_229,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _1.as_mut_ptr(), c.as_mut_ptr());
    let mut got_230: libc::c_int = zcmpmag(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_230 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            410 as libc::c_int,
            b"(zcmpmag(a, _3))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_230,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_231: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_231 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            411 as libc::c_int,
            b"(zcmp(a, _3))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_231,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), _0.as_mut_ptr(), c.as_mut_ptr());
    let mut got_232: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_232 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            412 as libc::c_int,
            b"(zor(a, _0, c), zcmp(a, c))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_232,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zor(a.as_mut_ptr(), b.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_233: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_233 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            413 as libc::c_int,
            b"(zor(a, b, _0), zcmp(a, b))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_233,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_3.as_mut_ptr(), _3.as_mut_ptr());
    zneg(b.as_mut_ptr(), _1.as_mut_ptr());
    zneg(c.as_mut_ptr(), _2.as_mut_ptr());
    zxor(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_234: libc::c_int = zcmpmag(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_234 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            419 as libc::c_int,
            b"zcmpmag(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_234,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_235: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_235 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            420 as libc::c_int,
            b"zcmp(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_235,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_3.as_mut_ptr(), _3.as_mut_ptr());
    zxor(a.as_mut_ptr(), b.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_236: libc::c_int = zcmpmag(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_236 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            423 as libc::c_int,
            b"zcmpmag(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_236,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_237: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_237 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            424 as libc::c_int,
            b"zcmp(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_237,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _1.as_mut_ptr(), c.as_mut_ptr());
    let mut got_238: libc::c_int = zcmpmag(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_238 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            426 as libc::c_int,
            b"zcmpmag(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_238,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_239: libc::c_int = zcmp(a.as_mut_ptr(), _3.as_mut_ptr());
    if !(got_239 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            427 as libc::c_int,
            b"zcmp(a, _3)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_239,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), b.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_240: libc::c_int = zcmpmag(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_240 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            429 as libc::c_int,
            b"zcmpmag(a, b)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_240,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_241: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_241 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            430 as libc::c_int,
            b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_241,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zxor(a.as_mut_ptr(), _0.as_mut_ptr(), c.as_mut_ptr());
    let mut got_242: libc::c_int = zcmpmag(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_242 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            432 as libc::c_int,
            b"zcmpmag(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_242,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_243: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_243 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            433 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_243,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_3.as_mut_ptr(), _3.as_mut_ptr());
    zlsh(a.as_mut_ptr(), _0.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_244: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_244 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            436 as libc::c_int,
            b"(zlsh(a, _0, 0), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_244,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_245: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_245 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            437 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_245,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zlsh(a.as_mut_ptr(), _0.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_246: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_246 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            438 as libc::c_int,
            b"(zlsh(a, _0, 1), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_246,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_247: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_247 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            439 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_247,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zlsh(a.as_mut_ptr(), _1.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_248: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_248 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            440 as libc::c_int,
            b"(zlsh(a, _1, 0), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_248,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zlsh(a.as_mut_ptr(), _1.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_249: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_249 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            441 as libc::c_int,
            b"(zlsh(a, _1, 1), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_249,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zlsh(a.as_mut_ptr(), _1.as_mut_ptr(), 2 as libc::c_int as size_t);
    let mut got_250: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_250 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            442 as libc::c_int,
            b"(zlsh(a, _1, 2), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_250,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zlsh(a.as_mut_ptr(), _2.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_251: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_251 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            443 as libc::c_int,
            b"(zlsh(a, _2, 0), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_251,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zlsh(a.as_mut_ptr(), _2.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_252: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_252 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            444 as libc::c_int,
            b"(zlsh(a, _2, 1), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_252,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zset(a.as_mut_ptr(), _0.as_mut_ptr());
    zlsh(a.as_mut_ptr(), a.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_253: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_253 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            447 as libc::c_int,
            b"(zlsh(a, a, 0), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_253,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_254: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_254 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            448 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_254,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zlsh(a.as_mut_ptr(), a.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_255: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_255 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            449 as libc::c_int,
            b"(zlsh(a, a, 1), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_255,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_256: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_256 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            450 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_256,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zset(a.as_mut_ptr(), _1.as_mut_ptr());
    zlsh(a.as_mut_ptr(), a.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_257: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_257 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            452 as libc::c_int,
            b"(zlsh(a, a, 0), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_257,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zlsh(a.as_mut_ptr(), a.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_258: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_258 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            453 as libc::c_int,
            b"(zlsh(a, a, 1), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_258,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zlsh(a.as_mut_ptr(), a.as_mut_ptr(), 2 as libc::c_int as size_t);
    let mut got_259: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_259 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            454 as libc::c_int,
            b"(zlsh(a, a, 2), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_259,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zset(a.as_mut_ptr(), _2.as_mut_ptr());
    zlsh(a.as_mut_ptr(), a.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_260: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_260 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            456 as libc::c_int,
            b"(zlsh(a, a, 0), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_260,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zlsh(a.as_mut_ptr(), a.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_261: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_261 > 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            457 as libc::c_int,
            b"(zlsh(a, a, 1), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"> 0\0" as *const u8 as *const libc::c_char,
            got_261,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), _0.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_262: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_262 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            459 as libc::c_int,
            b"(zrsh(a, _0, 0), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_262,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_263: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_263 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            460 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_263,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), _0.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_264: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_264 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            461 as libc::c_int,
            b"(zrsh(a, _0, 1), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_264,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_265: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_265 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            462 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_265,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), _1.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_266: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_266 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            463 as libc::c_int,
            b"(zrsh(a, _1, 0), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_266,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), _1.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_267: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_267 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            464 as libc::c_int,
            b"(zrsh(a, _1, 1), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_267,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_268: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_268 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            465 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_268,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), _1.as_mut_ptr(), 2 as libc::c_int as size_t);
    let mut got_269: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_269 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            466 as libc::c_int,
            b"(zrsh(a, _1, 2), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_269,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_270: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_270 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            467 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_270,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), _2.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_271: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_271 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            468 as libc::c_int,
            b"(zrsh(a, _2, 0), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_271,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), _2.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_272: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_272 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            469 as libc::c_int,
            b"(zrsh(a, _2, 1), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_272,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), _2.as_mut_ptr(), 2 as libc::c_int as size_t);
    let mut got_273: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_273 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            470 as libc::c_int,
            b"(zrsh(a, _2, 2), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_273,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_274: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_274 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            471 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_274,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zset(a.as_mut_ptr(), _0.as_mut_ptr());
    zrsh(a.as_mut_ptr(), a.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_275: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_275 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            474 as libc::c_int,
            b"(zrsh(a, a, 0), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_275,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_276: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_276 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            475 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_276,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), a.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_277: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_277 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            476 as libc::c_int,
            b"(zrsh(a, a, 1), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_277,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_278: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_278 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            477 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_278,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zset(a.as_mut_ptr(), _1.as_mut_ptr());
    zrsh(a.as_mut_ptr(), a.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_279: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_279 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            479 as libc::c_int,
            b"(zrsh(a, a, 0), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_279,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), a.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_280: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_280 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            480 as libc::c_int,
            b"(zrsh(a, a, 1), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_280,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_281: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_281 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            481 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_281,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), a.as_mut_ptr(), 2 as libc::c_int as size_t);
    let mut got_282: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_282 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            482 as libc::c_int,
            b"(zrsh(a, a, 2), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_282,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_283: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_283 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            483 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_283,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zset(a.as_mut_ptr(), _2.as_mut_ptr());
    zrsh(a.as_mut_ptr(), a.as_mut_ptr(), 0 as libc::c_int as size_t);
    let mut got_284: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_284 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            485 as libc::c_int,
            b"(zrsh(a, a, 0), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_284,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), a.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_285: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_285 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            486 as libc::c_int,
            b"(zrsh(a, a, 1), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_285,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrsh(a.as_mut_ptr(), a.as_mut_ptr(), 2 as libc::c_int as size_t);
    let mut got_286: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_286 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            487 as libc::c_int,
            b"(zrsh(a, a, 2), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_286,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_287: libc::c_int = zzero(a.as_mut_ptr());
    if !(got_287 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            488 as libc::c_int,
            b"zzero(a)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_287,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_288: libc::c_int = zbtest(_0.as_mut_ptr(), 0 as libc::c_int as size_t);
    if !(got_288 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            490 as libc::c_int,
            b"zbtest(_0, 0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_288,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_289: libc::c_int = zbtest(_1.as_mut_ptr(), 0 as libc::c_int as size_t);
    if !(got_289 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            491 as libc::c_int,
            b"zbtest(_1, 0)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_289,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_290: libc::c_int = zbtest(_2.as_mut_ptr(), 0 as libc::c_int as size_t);
    if !(got_290 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            492 as libc::c_int,
            b"zbtest(_2, 0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_290,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_291: libc::c_int = zbtest(_3.as_mut_ptr(), 0 as libc::c_int as size_t);
    if !(got_291 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            493 as libc::c_int,
            b"zbtest(_3, 0)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_291,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_292: libc::c_int = zbtest(_0.as_mut_ptr(), 1 as libc::c_int as size_t);
    if !(got_292 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            494 as libc::c_int,
            b"zbtest(_0, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_292,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_293: libc::c_int = zbtest(_1.as_mut_ptr(), 1 as libc::c_int as size_t);
    if !(got_293 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            495 as libc::c_int,
            b"zbtest(_1, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_293,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_294: libc::c_int = zbtest(_2.as_mut_ptr(), 1 as libc::c_int as size_t);
    if !(got_294 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            496 as libc::c_int,
            b"zbtest(_2, 1)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_294,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_295: libc::c_int = zbtest(_3.as_mut_ptr(), 1 as libc::c_int as size_t);
    if !(got_295 == 1 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            497 as libc::c_int,
            b"zbtest(_3, 1)\0" as *const u8 as *const libc::c_char,
            b"== 1\0" as *const u8 as *const libc::c_char,
            got_295,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_296: libc::c_int = zbtest(_0.as_mut_ptr(), 2 as libc::c_int as size_t);
    if !(got_296 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            498 as libc::c_int,
            b"zbtest(_0, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_296,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_297: libc::c_int = zbtest(_1.as_mut_ptr(), 2 as libc::c_int as size_t);
    if !(got_297 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            499 as libc::c_int,
            b"zbtest(_1, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_297,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_298: libc::c_int = zbtest(_2.as_mut_ptr(), 2 as libc::c_int as size_t);
    if !(got_298 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            500 as libc::c_int,
            b"zbtest(_2, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_298,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_299: libc::c_int = zbtest(_3.as_mut_ptr(), 2 as libc::c_int as size_t);
    if !(got_299 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            501 as libc::c_int,
            b"zbtest(_3, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_299,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    znot(a.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_300: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_300 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            504 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_300,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_301: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_301 != 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            505 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"!= 0\0" as *const u8 as *const libc::c_char,
            got_301,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    znot(a.as_mut_ptr(), a.as_mut_ptr());
    let mut got_302: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_302 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            507 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_302,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 0x1234 as libc::c_int as libc::c_ulonglong);
    zsetu(c.as_mut_ptr(), 0x234 as libc::c_int as libc::c_ulonglong);
    ztrunc(a.as_mut_ptr(), a.as_mut_ptr(), 12 as libc::c_int as size_t);
    let mut got_303: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_303 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            512 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_303,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 0xeeff as libc::c_int as libc::c_ulonglong);
    zsetu(c.as_mut_ptr(), 0xee as libc::c_int as libc::c_ulonglong);
    zsetu(d.as_mut_ptr(), 0xff as libc::c_int as libc::c_ulonglong);
    zsplit(a.as_mut_ptr(), b.as_mut_ptr(), a.as_mut_ptr(), 8 as libc::c_int as size_t);
    let mut got_304: libc::c_int = zcmpmag(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_304 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            518 as libc::c_int,
            b"zcmpmag(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_304,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_305: libc::c_int = zcmpmag(b.as_mut_ptr(), d.as_mut_ptr());
    if !(got_305 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            519 as libc::c_int,
            b"zcmpmag(b, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_305,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 0xeeff as libc::c_int as libc::c_ulonglong);
    zsplit(b.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr(), 8 as libc::c_int as size_t);
    let mut got_306: libc::c_int = zcmpmag(b.as_mut_ptr(), c.as_mut_ptr());
    if !(got_306 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            522 as libc::c_int,
            b"zcmpmag(b, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_306,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_307: libc::c_int = zcmpmag(a.as_mut_ptr(), d.as_mut_ptr());
    if !(got_307 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            523 as libc::c_int,
            b"zcmpmag(a, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_307,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zmul(a.as_mut_ptr(), _2.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_308: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        6 as libc::c_int as libc::c_longlong,
    );
    if !(got_308 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            526 as libc::c_int,
            b"zcmpi(a, 6)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_308,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_3.as_mut_ptr(), _3.as_mut_ptr());
    zmul(a.as_mut_ptr(), _2.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_309: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        -(6 as libc::c_int) as libc::c_longlong,
    );
    if !(got_309 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            529 as libc::c_int,
            b"zcmpi(a, -6)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_309,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_3.as_mut_ptr(), _3.as_mut_ptr());
    zneg(_2.as_mut_ptr(), _2.as_mut_ptr());
    zmul(a.as_mut_ptr(), _2.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_310: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        -(6 as libc::c_int) as libc::c_longlong,
    );
    if !(got_310 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            533 as libc::c_int,
            b"zcmpi(a, -6)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_310,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_3.as_mut_ptr(), _3.as_mut_ptr());
    zmul(a.as_mut_ptr(), _2.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_311: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        6 as libc::c_int as libc::c_longlong,
    );
    if !(got_311 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            536 as libc::c_int,
            b"zcmpi(a, 6)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_311,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_3.as_mut_ptr(), _3.as_mut_ptr());
    zneg(_2.as_mut_ptr(), _2.as_mut_ptr());
    zmul(a.as_mut_ptr(), _3.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_312: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        9 as libc::c_int as libc::c_longlong,
    );
    if !(got_312 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            541 as libc::c_int,
            b"zcmpi(a, 9)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_312,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsqr(a.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_313: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        9 as libc::c_int as libc::c_longlong,
    );
    if !(got_313 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            543 as libc::c_int,
            b"zcmpi(a, 9)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_313,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_3.as_mut_ptr(), _3.as_mut_ptr());
    zmul(a.as_mut_ptr(), _3.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_314: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        9 as libc::c_int as libc::c_longlong,
    );
    if !(got_314 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            546 as libc::c_int,
            b"zcmpi(a, 9)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_314,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsqr(a.as_mut_ptr(), _3.as_mut_ptr());
    let mut got_315: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        9 as libc::c_int as libc::c_longlong,
    );
    if !(got_315 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            548 as libc::c_int,
            b"zcmpi(a, 9)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_315,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_3.as_mut_ptr(), _3.as_mut_ptr());
    zseti(a.as_mut_ptr(), 8 as libc::c_int as libc::c_longlong);
    zseti(b.as_mut_ptr(), 2 as libc::c_int as libc::c_longlong);
    zdiv(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_316: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        4 as libc::c_int as libc::c_longlong,
    );
    if !(got_316 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            554 as libc::c_int,
            b"zcmpi(c, 4)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_316,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(b.as_mut_ptr(), -(2 as libc::c_int) as libc::c_longlong);
    zdiv(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_317: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        -(4 as libc::c_int) as libc::c_longlong,
    );
    if !(got_317 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            557 as libc::c_int,
            b"zcmpi(c, -4)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_317,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(8 as libc::c_int) as libc::c_longlong);
    zseti(b.as_mut_ptr(), 2 as libc::c_int as libc::c_longlong);
    zdiv(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_318: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        -(4 as libc::c_int) as libc::c_longlong,
    );
    if !(got_318 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            561 as libc::c_int,
            b"zcmpi(c, -4)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_318,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(b.as_mut_ptr(), -(2 as libc::c_int) as libc::c_longlong);
    zdiv(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_319: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        4 as libc::c_int as libc::c_longlong,
    );
    if !(got_319 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            564 as libc::c_int,
            b"zcmpi(c, 4)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_319,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 1000 as libc::c_int as libc::c_longlong);
    zseti(b.as_mut_ptr(), 10 as libc::c_int as libc::c_longlong);
    zdiv(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_320: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        100 as libc::c_int as libc::c_longlong,
    );
    if !(got_320 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            569 as libc::c_int,
            b"zcmpi(c, 100)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_320,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(b.as_mut_ptr(), -(10 as libc::c_int) as libc::c_longlong);
    zdiv(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_321: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        -(100 as libc::c_int) as libc::c_longlong,
    );
    if !(got_321 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            572 as libc::c_int,
            b"zcmpi(c, -100)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_321,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(1000 as libc::c_int) as libc::c_longlong);
    zseti(b.as_mut_ptr(), 10 as libc::c_int as libc::c_longlong);
    zdiv(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_322: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        -(100 as libc::c_int) as libc::c_longlong,
    );
    if !(got_322 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            576 as libc::c_int,
            b"zcmpi(c, -100)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_322,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(b.as_mut_ptr(), -(10 as libc::c_int) as libc::c_longlong);
    zdiv(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_323: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        100 as libc::c_int as libc::c_longlong,
    );
    if !(got_323 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            579 as libc::c_int,
            b"zcmpi(c, 100)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_323,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 7 as libc::c_int as libc::c_longlong);
    zseti(b.as_mut_ptr(), 3 as libc::c_int as libc::c_longlong);
    zmod(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_324: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        1 as libc::c_int as libc::c_longlong,
    );
    if !(got_324 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            584 as libc::c_int,
            b"zcmpi(c, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_324,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(b.as_mut_ptr(), -(3 as libc::c_int) as libc::c_longlong);
    zmod(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_325: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        1 as libc::c_int as libc::c_longlong,
    );
    if !(got_325 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            587 as libc::c_int,
            b"zcmpi(c, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_325,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(7 as libc::c_int) as libc::c_longlong);
    zseti(b.as_mut_ptr(), 3 as libc::c_int as libc::c_longlong);
    zmod(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_326: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        1 as libc::c_int as libc::c_longlong,
    );
    if !(got_326 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            591 as libc::c_int,
            b"zcmpi(c, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_326,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(b.as_mut_ptr(), -(3 as libc::c_int) as libc::c_longlong);
    zmod(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_327: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        1 as libc::c_int as libc::c_longlong,
    );
    if !(got_327 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            594 as libc::c_int,
            b"zcmpi(c, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_327,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 7 as libc::c_int as libc::c_longlong);
    zseti(b.as_mut_ptr(), 3 as libc::c_int as libc::c_longlong);
    zdivmod(d.as_mut_ptr(), c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_328: libc::c_int = zcmpi(
        d.as_mut_ptr(),
        2 as libc::c_int as libc::c_longlong,
    );
    if !(got_328 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            599 as libc::c_int,
            b"zcmpi(d, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_328,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_329: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        1 as libc::c_int as libc::c_longlong,
    );
    if !(got_329 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            600 as libc::c_int,
            b"zcmpi(c, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_329,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(b.as_mut_ptr(), -(3 as libc::c_int) as libc::c_longlong);
    zdivmod(d.as_mut_ptr(), c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_330: libc::c_int = zcmpi(
        d.as_mut_ptr(),
        -(2 as libc::c_int) as libc::c_longlong,
    );
    if !(got_330 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            603 as libc::c_int,
            b"zcmpi(d, -2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_330,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_331: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        1 as libc::c_int as libc::c_longlong,
    );
    if !(got_331 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            604 as libc::c_int,
            b"zcmpi(c, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_331,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(7 as libc::c_int) as libc::c_longlong);
    zseti(b.as_mut_ptr(), 3 as libc::c_int as libc::c_longlong);
    zdivmod(d.as_mut_ptr(), c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_332: libc::c_int = zcmpi(
        d.as_mut_ptr(),
        -(2 as libc::c_int) as libc::c_longlong,
    );
    if !(got_332 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            608 as libc::c_int,
            b"zcmpi(d, -2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_332,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_333: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        1 as libc::c_int as libc::c_longlong,
    );
    if !(got_333 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            609 as libc::c_int,
            b"zcmpi(c, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_333,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(b.as_mut_ptr(), -(3 as libc::c_int) as libc::c_longlong);
    zdivmod(d.as_mut_ptr(), c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_334: libc::c_int = zcmpi(
        d.as_mut_ptr(),
        2 as libc::c_int as libc::c_longlong,
    );
    if !(got_334 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            612 as libc::c_int,
            b"zcmpi(d, 2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_334,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_335: libc::c_int = zcmpi(
        c.as_mut_ptr(),
        1 as libc::c_int as libc::c_longlong,
    );
    if !(got_335 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            613 as libc::c_int,
            b"zcmpi(c, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_335,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 10 as libc::c_int as libc::c_longlong);
    zseti(b.as_mut_ptr(), -(1 as libc::c_int) as libc::c_longlong);
    zpow(a.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_336: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_336 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            618 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_336,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 10 as libc::c_int as libc::c_longlong);
    zseti(b.as_mut_ptr(), -(1 as libc::c_int) as libc::c_longlong);
    zseti(a.as_mut_ptr(), 20 as libc::c_int as libc::c_longlong);
    zmodpow(a.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_337: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_337 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            624 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_337,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 10 as libc::c_int as libc::c_longlong);
    zseti(c.as_mut_ptr(), 100000 as libc::c_long as libc::c_longlong);
    zpowu(a.as_mut_ptr(), a.as_mut_ptr(), 5 as libc::c_int as libc::c_ulonglong);
    let mut got_338: libc::c_int = zcmpmag(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_338 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            629 as libc::c_int,
            b"zcmpmag(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_338,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_339: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_339 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            630 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_339,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(10 as libc::c_int) as libc::c_longlong);
    zseti(c.as_mut_ptr(), -(100000 as libc::c_long) as libc::c_longlong);
    zpowu(a.as_mut_ptr(), a.as_mut_ptr(), 5 as libc::c_int as libc::c_ulonglong);
    let mut got_340: libc::c_int = zcmpmag(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_340 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            635 as libc::c_int,
            b"zcmpmag(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_340,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_341: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_341 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            636 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_341,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(10 as libc::c_int) as libc::c_longlong);
    zseti(c.as_mut_ptr(), 10000 as libc::c_long as libc::c_longlong);
    zpowu(a.as_mut_ptr(), a.as_mut_ptr(), 4 as libc::c_int as libc::c_ulonglong);
    let mut got_342: libc::c_int = zcmpmag(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_342 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            641 as libc::c_int,
            b"zcmpmag(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_342,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_343: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_343 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            642 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_343,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 10 as libc::c_int as libc::c_longlong);
    zseti(c.as_mut_ptr(), 3 as libc::c_int as libc::c_longlong);
    zmodpowu(
        a.as_mut_ptr(),
        a.as_mut_ptr(),
        5 as libc::c_int as libc::c_ulonglong,
        c.as_mut_ptr(),
    );
    let mut got_344: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_344 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            647 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_344,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_345: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_345 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            648 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_345,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 10 as libc::c_int as libc::c_longlong);
    zseti(b.as_mut_ptr(), 5 as libc::c_int as libc::c_longlong);
    zseti(c.as_mut_ptr(), 100000 as libc::c_long as libc::c_longlong);
    zpow(a.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_346: libc::c_int = zcmpmag(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_346 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            654 as libc::c_int,
            b"zcmpmag(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_346,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_347: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_347 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            655 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_347,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(10 as libc::c_int) as libc::c_longlong);
    zseti(b.as_mut_ptr(), 5 as libc::c_int as libc::c_longlong);
    zseti(c.as_mut_ptr(), -(100000 as libc::c_long) as libc::c_longlong);
    zpow(a.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_348: libc::c_int = zcmpmag(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_348 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            661 as libc::c_int,
            b"zcmpmag(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_348,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_349: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_349 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            662 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_349,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(10 as libc::c_int) as libc::c_longlong);
    zseti(b.as_mut_ptr(), 4 as libc::c_int as libc::c_longlong);
    zseti(c.as_mut_ptr(), 10000 as libc::c_long as libc::c_longlong);
    zpow(a.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_350: libc::c_int = zcmpmag(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_350 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            668 as libc::c_int,
            b"zcmpmag(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_350,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_351: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_351 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            669 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_351,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 10 as libc::c_int as libc::c_longlong);
    zseti(b.as_mut_ptr(), 5 as libc::c_int as libc::c_longlong);
    zseti(c.as_mut_ptr(), 3 as libc::c_int as libc::c_longlong);
    zmodpow(a.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_352: libc::c_int = zcmpmag(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_352 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            675 as libc::c_int,
            b"zcmpmag(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_352,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_353: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_353 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            676 as libc::c_int,
            b"zcmp(a, _1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_353,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 102 as libc::c_int as libc::c_longlong);
    zseti(b.as_mut_ptr(), 501 as libc::c_int as libc::c_longlong);
    zseti(c.as_mut_ptr(), 5 as libc::c_int as libc::c_longlong);
    zmodmul(a.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_354: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_354 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            682 as libc::c_int,
            b"zcmp(a, _2)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_354,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(
        b.as_mut_ptr(),
        (2 as libc::c_int * 3 as libc::c_int * 3 as libc::c_int * 7 as libc::c_int)
            as libc::c_longlong,
    );
    zseti(
        c.as_mut_ptr(),
        (3 as libc::c_int * 7 as libc::c_int * 11 as libc::c_int) as libc::c_longlong,
    );
    zseti(d.as_mut_ptr(), (3 as libc::c_int * 7 as libc::c_int) as libc::c_longlong);
    zgcd(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_355: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_355 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            687 as libc::c_int,
            b"(zgcd(a, _0, _0), zcmp(a, _0))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_355,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), b.as_mut_ptr(), _0.as_mut_ptr());
    let mut got_356: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_356 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            688 as libc::c_int,
            b"(zgcd(a, b, _0), zcmp(a, b))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_356,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), _0.as_mut_ptr(), c.as_mut_ptr());
    let mut got_357: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_357 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            689 as libc::c_int,
            b"(zgcd(a, _0, c), zcmp(a, c))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_357,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), b.as_mut_ptr(), b.as_mut_ptr());
    let mut got_358: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_358 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            690 as libc::c_int,
            b"(zgcd(a, b, b), zcmp(a, b))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_358,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), b.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_359: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_359 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            691 as libc::c_int,
            b"(zgcd(a, b, _2), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_359,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), _2.as_mut_ptr(), b.as_mut_ptr());
    let mut got_360: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_360 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            692 as libc::c_int,
            b"(zgcd(a, _2, b), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_360,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), _2.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_361: libc::c_int = zcmp(a.as_mut_ptr(), _2.as_mut_ptr());
    if !(got_361 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            693 as libc::c_int,
            b"(zgcd(a, _2, _2), zcmp(a, _2))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_361,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), c.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_362: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_362 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            694 as libc::c_int,
            b"(zgcd(a, c, _2), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_362,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), _2.as_mut_ptr(), c.as_mut_ptr());
    let mut got_363: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_363 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            695 as libc::c_int,
            b"(zgcd(a, _2, c), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_363,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), b.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_364: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_364 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            696 as libc::c_int,
            b"(zgcd(a, b, _1), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_364,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), _1.as_mut_ptr(), c.as_mut_ptr());
    let mut got_365: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_365 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            697 as libc::c_int,
            b"(zgcd(a, _1, c), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_365,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_366: libc::c_int = zcmp(a.as_mut_ptr(), _1.as_mut_ptr());
    if !(got_366 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            698 as libc::c_int,
            b"(zgcd(a, _1, _1), zcmp(a, _1))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_366,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), b.as_mut_ptr(), c.as_mut_ptr());
    let mut got_367: libc::c_int = zcmp(a.as_mut_ptr(), d.as_mut_ptr());
    if !(got_367 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            699 as libc::c_int,
            b"(zgcd(a, b, c), zcmp(a, d))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_367,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zgcd(a.as_mut_ptr(), c.as_mut_ptr(), b.as_mut_ptr());
    let mut got_368: libc::c_int = zcmp(a.as_mut_ptr(), d.as_mut_ptr());
    if !(got_368 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            700 as libc::c_int,
            b"(zgcd(a, c, b), zcmp(a, d))\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_368,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(a.as_mut_ptr(), b"1234\0" as *const u8 as *const libc::c_char);
    let mut got_369: libc::c_int = zcmpi(
        a.as_mut_ptr(),
        1234 as libc::c_int as libc::c_longlong,
    );
    if !(got_369 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            703 as libc::c_int,
            b"zcmpi(a, 1234)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_369,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(b.as_mut_ptr(), b"+1234\0" as *const u8 as *const libc::c_char);
    let mut got_370: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_370 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            705 as libc::c_int,
            b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_370,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_371: size_t = zstr_length(
        _0.as_mut_ptr(),
        10 as libc::c_int as libc::c_ulonglong,
    );
    if got_371 != 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            706 as libc::c_int,
            b"zstr_length(_0, 10)\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
            got_371,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_372: size_t = zstr_length(
        _1.as_mut_ptr(),
        10 as libc::c_int as libc::c_ulonglong,
    );
    if got_372 != 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            707 as libc::c_int,
            b"zstr_length(_1, 10)\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
            got_372,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_373: size_t = zstr_length(
        _2.as_mut_ptr(),
        10 as libc::c_int as libc::c_ulonglong,
    );
    if got_373 != 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            708 as libc::c_int,
            b"zstr_length(_2, 10)\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
            got_373,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_374: size_t = zstr_length(
        _3.as_mut_ptr(),
        10 as libc::c_int as libc::c_ulonglong,
    );
    if got_374 != 1 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            709 as libc::c_int,
            b"zstr_length(_3, 10)\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
            got_374,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_2.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_375: size_t = zstr_length(
        _2.as_mut_ptr(),
        10 as libc::c_int as libc::c_ulonglong,
    );
    if got_375 != 2 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            711 as libc::c_int,
            b"zstr_length(_2, 10)\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
            got_375,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_2.as_mut_ptr(), _2.as_mut_ptr());
    let mut got_376: size_t = zstr_length(
        a.as_mut_ptr(),
        10 as libc::c_int as libc::c_ulonglong,
    );
    if got_376 != 4 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            713 as libc::c_int,
            b"zstr_length(a, 10)\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
            got_376,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    let mut got_377: *const libc::c_char = buf.as_mut_ptr();
    if strcmp(got_377, b"1234\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            715 as libc::c_int,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"1234\0" as *const u8 as *const libc::c_char,
            got_377,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(a.as_mut_ptr(), b"-1234\0" as *const u8 as *const libc::c_char);
    zseti(b.as_mut_ptr(), -(1234 as libc::c_int) as libc::c_longlong);
    zseti(c.as_mut_ptr(), 1234 as libc::c_int as libc::c_longlong);
    let mut got_378: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_378 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            719 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_378,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_379: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_379 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            720 as libc::c_int,
            b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_379,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_380: libc::c_int = zcmpmag(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_380 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            721 as libc::c_int,
            b"zcmpmag(a, c)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_380,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_381: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_381 < 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            722 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"< 0\0" as *const u8 as *const libc::c_char,
            got_381,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    let mut got_382: *const libc::c_char = buf.as_mut_ptr();
    if strcmp(got_382, b"-1234\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            724 as libc::c_int,
            b"buf\0" as *const u8 as *const libc::c_char,
            b"-1234\0" as *const u8 as *const libc::c_char,
            got_382,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_383: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_383, b"-1234\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            725 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"-1234\0" as *const u8 as *const libc::c_char,
            got_383,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(d.as_mut_ptr(), 100000 as libc::c_ulong as libc::c_ulonglong);
    zrand(a.as_mut_ptr(), FAST_RANDOM, UNIFORM, d.as_mut_ptr());
    let mut got_384: libc::c_int = zcmp(a.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_384 >= 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            729 as libc::c_int,
            b"zcmp(a, _0)\0" as *const u8 as *const libc::c_char,
            b">= 0\0" as *const u8 as *const libc::c_char,
            got_384,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_385: libc::c_int = zcmp(a.as_mut_ptr(), d.as_mut_ptr());
    if !(got_385 <= 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            730 as libc::c_int,
            b"zcmp(a, d)\0" as *const u8 as *const libc::c_char,
            b"<= 0\0" as *const u8 as *const libc::c_char,
            got_385,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrand(b.as_mut_ptr(), SECURE_RANDOM, UNIFORM, d.as_mut_ptr());
    let mut got_386: libc::c_int = zcmp(b.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_386 >= 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            732 as libc::c_int,
            b"zcmp(b, _0)\0" as *const u8 as *const libc::c_char,
            b">= 0\0" as *const u8 as *const libc::c_char,
            got_386,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_387: libc::c_int = zcmp(b.as_mut_ptr(), d.as_mut_ptr());
    if !(got_387 <= 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            733 as libc::c_int,
            b"zcmp(b, d)\0" as *const u8 as *const libc::c_char,
            b"<= 0\0" as *const u8 as *const libc::c_char,
            got_387,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zrand(c.as_mut_ptr(), FAST_RANDOM, UNIFORM, d.as_mut_ptr());
    let mut got_388: libc::c_int = zcmp(c.as_mut_ptr(), _0.as_mut_ptr());
    if !(got_388 >= 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            735 as libc::c_int,
            b"zcmp(c, _0)\0" as *const u8 as *const libc::c_char,
            b">= 0\0" as *const u8 as *const libc::c_char,
            got_388,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_389: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_389 <= 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            736 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"<= 0\0" as *const u8 as *const libc::c_char,
            got_389,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_390: libc::c_int = zcmp(a.as_mut_ptr(), b.as_mut_ptr());
    if !(got_390 != 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            737 as libc::c_int,
            b"zcmp(a, b)\0" as *const u8 as *const libc::c_char,
            b"!= 0\0" as *const u8 as *const libc::c_char,
            got_390,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_391: libc::c_int = zcmp(a.as_mut_ptr(), c.as_mut_ptr());
    if !(got_391 != 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            738 as libc::c_int,
            b"zcmp(a, c)\0" as *const u8 as *const libc::c_char,
            b"!= 0\0" as *const u8 as *const libc::c_char,
            got_391,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_392: libc::c_int = zcmp(b.as_mut_ptr(), c.as_mut_ptr());
    if !(got_392 != 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            739 as libc::c_int,
            b"zcmp(b, c)\0" as *const u8 as *const libc::c_char,
            b"!= 0\0" as *const u8 as *const libc::c_char,
            got_392,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(5 as libc::c_int) as libc::c_longlong);
    let mut got_393: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_393 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            741 as libc::c_int,
            b"(zseti(a, -5), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_393,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(4 as libc::c_int) as libc::c_longlong);
    let mut got_394: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_394 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            742 as libc::c_int,
            b"(zseti(a, -4), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_394,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(3 as libc::c_int) as libc::c_longlong);
    let mut got_395: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_395 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            743 as libc::c_int,
            b"(zseti(a, -3), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_395,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(2 as libc::c_int) as libc::c_longlong);
    let mut got_396: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_396 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            744 as libc::c_int,
            b"(zseti(a, -2), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_396,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(1 as libc::c_int) as libc::c_longlong);
    let mut got_397: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_397 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            745 as libc::c_int,
            b"(zseti(a, -1), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_397,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 0 as libc::c_int as libc::c_longlong);
    let mut got_398: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_398 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            746 as libc::c_int,
            b"(zseti(a, 0), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_398,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 1 as libc::c_int as libc::c_longlong);
    let mut got_399: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_399 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            747 as libc::c_int,
            b"(zseti(a, 1), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_399,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 2 as libc::c_int as libc::c_longlong);
    let mut got_400: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_400 == PRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            748 as libc::c_int,
            b"(zseti(a, 2), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== PRIME\0" as *const u8 as *const libc::c_char,
            got_400,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 3 as libc::c_int as libc::c_longlong);
    let mut got_401: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_401 == PRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            749 as libc::c_int,
            b"(zseti(a, 3), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== PRIME\0" as *const u8 as *const libc::c_char,
            got_401,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 4 as libc::c_int as libc::c_longlong);
    let mut got_402: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_402 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            750 as libc::c_int,
            b"(zseti(a, 4), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_402,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 5 as libc::c_int as libc::c_longlong);
    let mut got_403: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_403 != NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            751 as libc::c_int,
            b"(zseti(a, 5), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"!= NONPRIME\0" as *const u8 as *const libc::c_char,
            got_403,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 6 as libc::c_int as libc::c_longlong);
    let mut got_404: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_404 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            752 as libc::c_int,
            b"(zseti(a, 6), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_404,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 7 as libc::c_int as libc::c_longlong);
    let mut got_405: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_405 != NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            753 as libc::c_int,
            b"(zseti(a, 7), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"!= NONPRIME\0" as *const u8 as *const libc::c_char,
            got_405,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 8 as libc::c_int as libc::c_longlong);
    let mut got_406: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_406 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            754 as libc::c_int,
            b"(zseti(a, 8), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_406,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 9 as libc::c_int as libc::c_longlong);
    let mut got_407: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_407 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            755 as libc::c_int,
            b"(zseti(a, 9), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_407,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 10 as libc::c_int as libc::c_longlong);
    let mut got_408: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_408 == NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            756 as libc::c_int,
            b"(zseti(a, 10), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"== NONPRIME\0" as *const u8 as *const libc::c_char,
            got_408,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 11 as libc::c_int as libc::c_longlong);
    let mut got_409: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_409 != NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            757 as libc::c_int,
            b"(zseti(a, 11), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"!= NONPRIME\0" as *const u8 as *const libc::c_char,
            got_409,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 101 as libc::c_int as libc::c_longlong);
    let mut got_410: libc::c_int = zptest(
        0 as *mut C2RustUnnamed,
        a.as_mut_ptr(),
        100 as libc::c_int,
    ) as libc::c_int;
    if !(got_410 != NONPRIME as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            758 as libc::c_int,
            b"(zseti(a, 101), zptest(0, a, 100))\0" as *const u8 as *const libc::c_char,
            b"!= NONPRIME\0" as *const u8 as *const libc::c_char,
            got_410,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zdivmod(a.as_mut_ptr(), b.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            760 as libc::c_int,
            b"zdivmod(a, b, _0, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zdivmod(a.as_mut_ptr(), b.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            761 as libc::c_int,
            b"zdivmod(a, b, _1, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zdivmod(a.as_mut_ptr(), b.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    zdivmod(a.as_mut_ptr(), b.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zdiv(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            764 as libc::c_int,
            b"zdiv(a, _0, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zdiv(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            765 as libc::c_int,
            b"zdiv(a, _1, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zdiv(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    zdiv(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmod(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            768 as libc::c_int,
            b"zmod(a, _0, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmod(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            769 as libc::c_int,
            b"zmod(a, _1, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zmod(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    zmod(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zpow(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            772 as libc::c_int,
            b"zpow(a, _0, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
        zpow(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            773 as libc::c_int,
            b"(zneg(_1, _1), zpow(a, _0, _1))\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zpow(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    zpow(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zpow(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmodmul(a.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            777 as libc::c_int,
            b"zmodmul(a, _1, _1, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmodpow(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            778 as libc::c_int,
            b"zmodpow(a, _0, _0, _1)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
        zmodpow(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            779 as libc::c_int,
            b"(zneg(_1, _1), zmodpow(a, _0, _1, _1))\0" as *const u8
                as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zmodpow(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr(), _1.as_mut_ptr());
    zmodpow(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zmodpow(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr());
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmodpow(a.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            783 as libc::c_int,
            b"zmodpow(a, _0, _0, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
        zmodpow(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            784 as libc::c_int,
            b"(zneg(_1, _1), zmodpow(a, _0, _1, _0))\0" as *const u8
                as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmodpow(a.as_mut_ptr(), _0.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            785 as libc::c_int,
            b"zmodpow(a, _0, _1, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmodpow(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            786 as libc::c_int,
            b"zmodpow(a, _1, _0, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
        zmodpow(a.as_mut_ptr(), _1.as_mut_ptr(), _0.as_mut_ptr(), _0.as_mut_ptr());
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            787 as libc::c_int,
            b"(zneg(_1, _1), zmodpow(a, _1, _0, _0))\0" as *const u8
                as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zpowu(a.as_mut_ptr(), _0.as_mut_ptr(), 0 as libc::c_int as libc::c_ulonglong);
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            788 as libc::c_int,
            b"zpowu(a, _0, 0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zpowu(a.as_mut_ptr(), _0.as_mut_ptr(), 1 as libc::c_int as libc::c_ulonglong);
    zpowu(a.as_mut_ptr(), _1.as_mut_ptr(), 0 as libc::c_int as libc::c_ulonglong);
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zpowu(a.as_mut_ptr(), _1.as_mut_ptr(), 0 as libc::c_int as libc::c_ulonglong);
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmodpowu(
            a.as_mut_ptr(),
            _0.as_mut_ptr(),
            0 as libc::c_int as libc::c_ulonglong,
            _1.as_mut_ptr(),
        );
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            792 as libc::c_int,
            b"zmodpowu(a, _0, 0, _1)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zmodpowu(
        a.as_mut_ptr(),
        _0.as_mut_ptr(),
        1 as libc::c_int as libc::c_ulonglong,
        _1.as_mut_ptr(),
    );
    zmodpowu(
        a.as_mut_ptr(),
        _1.as_mut_ptr(),
        0 as libc::c_int as libc::c_ulonglong,
        _1.as_mut_ptr(),
    );
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zmodpowu(
        a.as_mut_ptr(),
        _1.as_mut_ptr(),
        0 as libc::c_int as libc::c_ulonglong,
        _1.as_mut_ptr(),
    );
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmodpowu(
            a.as_mut_ptr(),
            _0.as_mut_ptr(),
            0 as libc::c_int as libc::c_ulonglong,
            _0.as_mut_ptr(),
        );
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            796 as libc::c_int,
            b"zmodpowu(a, _0, 0, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
        zmodpowu(
            a.as_mut_ptr(),
            _0.as_mut_ptr(),
            1 as libc::c_int as libc::c_ulonglong,
            _0.as_mut_ptr(),
        );
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            797 as libc::c_int,
            b"(zneg(_1, _1), zmodpowu(a, _0, 1, _0))\0" as *const u8
                as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmodpowu(
            a.as_mut_ptr(),
            _0.as_mut_ptr(),
            1 as libc::c_int as libc::c_ulonglong,
            _0.as_mut_ptr(),
        );
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            798 as libc::c_int,
            b"zmodpowu(a, _0, 1, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zmodpowu(
            a.as_mut_ptr(),
            _1.as_mut_ptr(),
            0 as libc::c_int as libc::c_ulonglong,
            _0.as_mut_ptr(),
        );
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            799 as libc::c_int,
            b"zmodpowu(a, _1, 0, _0)\0" as *const u8 as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    if _setjmp(env2.as_mut_ptr()) != 0 {
        ret = 0 as libc::c_int;
        zsetup(env.as_mut_ptr());
    } else {
        zsetup(env2.as_mut_ptr());
        zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
        zmodpowu(
            a.as_mut_ptr(),
            _1.as_mut_ptr(),
            0 as libc::c_int as libc::c_ulonglong,
            _0.as_mut_ptr(),
        );
        fprintf(
            stderr,
            b"Failure at line %i: %s, should not have returned.\n\0" as *const u8
                as *const libc::c_char,
            800 as libc::c_int,
            b"(zneg(_1, _1), zmodpowu(a, _1, 0, _0))\0" as *const u8
                as *const libc::c_char,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zneg(_1.as_mut_ptr(), _1.as_mut_ptr());
    zsetu(a.as_mut_ptr(), 1 as libc::c_longlong as libc::c_ulonglong);
    let mut got_411: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_411, b"1\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            803 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char,
            got_411,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 10 as libc::c_longlong as libc::c_ulonglong);
    let mut got_412: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_412, b"10\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            805 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"10\0" as *const u8 as *const libc::c_char,
            got_412,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 100 as libc::c_longlong as libc::c_ulonglong);
    let mut got_413: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_413, b"100\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            807 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"100\0" as *const u8 as *const libc::c_char,
            got_413,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_414: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_414, b"1000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            809 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"1000\0" as *const u8 as *const libc::c_char,
            got_414,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 10000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_415: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_415, b"10000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            811 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"10000\0" as *const u8 as *const libc::c_char,
            got_415,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 100000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_416: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_416, b"100000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            813 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"100000\0" as *const u8 as *const libc::c_char,
            got_416,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_417: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_417, b"1000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            815 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"1000000\0" as *const u8 as *const libc::c_char,
            got_417,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 10000000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_418: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_418, b"10000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            817 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"10000000\0" as *const u8 as *const libc::c_char,
            got_418,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 100000000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_419: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_419, b"100000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            819 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"100000000\0" as *const u8 as *const libc::c_char,
            got_419,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 999999999 as libc::c_longlong as libc::c_ulonglong);
    let mut got_420: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_420, b"999999999\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            821 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"999999999\0" as *const u8 as *const libc::c_char,
            got_420,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_421: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_421, b"1000000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            823 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"1000000000\0" as *const u8 as *const libc::c_char,
            got_421,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000001 as libc::c_longlong as libc::c_ulonglong);
    let mut got_422: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_422, b"1000000001\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            825 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"1000000001\0" as *const u8 as *const libc::c_char,
            got_422,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2000000000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_423: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_423, b"2000000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            827 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2000000000\0" as *const u8 as *const libc::c_char,
            got_423,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2050000000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_424: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_424, b"2050000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            829 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2050000000\0" as *const u8 as *const libc::c_char,
            got_424,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2100000000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_425: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_425, b"2100000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            831 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2100000000\0" as *const u8 as *const libc::c_char,
            got_425,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2140000000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_426: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_426, b"2140000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            833 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2140000000\0" as *const u8 as *const libc::c_char,
            got_426,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2147000000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_427: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_427, b"2147000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            835 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2147000000\0" as *const u8 as *const libc::c_char,
            got_427,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2147483000 as libc::c_longlong as libc::c_ulonglong);
    let mut got_428: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_428, b"2147483000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            837 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2147483000\0" as *const u8 as *const libc::c_char,
            got_428,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2147483640 as libc::c_longlong as libc::c_ulonglong);
    let mut got_429: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_429, b"2147483640\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            839 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2147483640\0" as *const u8 as *const libc::c_char,
            got_429,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2147483646 as libc::c_longlong as libc::c_ulonglong);
    let mut got_430: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_430, b"2147483646\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            841 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2147483646\0" as *const u8 as *const libc::c_char,
            got_430,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 2147483647 as libc::c_longlong);
    let mut got_431: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_431, b"2147483647\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            844 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2147483647\0" as *const u8 as *const libc::c_char,
            got_431,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(2147483647 as libc::c_longlong));
    let mut got_432: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_432, b"-2147483647\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            846 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"-2147483647\0" as *const u8 as *const libc::c_char,
            got_432,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(2147483647 as libc::c_longlong) - 1 as libc::c_longlong);
    let mut got_433: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_433, b"-2147483648\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            848 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"-2147483648\0" as *const u8 as *const libc::c_char,
            got_433,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2147483647 as libc::c_ulonglong);
    let mut got_434: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_434, b"2147483647\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            851 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2147483647\0" as *const u8 as *const libc::c_char,
            got_434,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2147483648 as libc::c_ulonglong);
    let mut got_435: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_435, b"2147483648\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            853 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2147483648\0" as *const u8 as *const libc::c_char,
            got_435,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 2147483649 as libc::c_ulonglong);
    let mut got_436: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_436, b"2147483649\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            855 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"2147483649\0" as *const u8 as *const libc::c_char,
            got_436,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3000000000 as libc::c_ulonglong);
    let mut got_437: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_437, b"3000000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            858 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3000000000\0" as *const u8 as *const libc::c_char,
            got_437,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3100000000 as libc::c_ulonglong);
    let mut got_438: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_438, b"3100000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            860 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3100000000\0" as *const u8 as *const libc::c_char,
            got_438,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3200000000 as libc::c_ulonglong);
    let mut got_439: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_439, b"3200000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            862 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3200000000\0" as *const u8 as *const libc::c_char,
            got_439,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3300000000 as libc::c_ulonglong);
    let mut got_440: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_440, b"3300000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            864 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3300000000\0" as *const u8 as *const libc::c_char,
            got_440,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3400000000 as libc::c_ulonglong);
    let mut got_441: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_441, b"3400000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            866 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3400000000\0" as *const u8 as *const libc::c_char,
            got_441,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3500000000 as libc::c_ulonglong);
    let mut got_442: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_442, b"3500000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            868 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3500000000\0" as *const u8 as *const libc::c_char,
            got_442,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3600000000 as libc::c_ulonglong);
    let mut got_443: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_443, b"3600000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            870 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3600000000\0" as *const u8 as *const libc::c_char,
            got_443,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3700000000 as libc::c_ulonglong);
    let mut got_444: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_444, b"3700000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            872 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3700000000\0" as *const u8 as *const libc::c_char,
            got_444,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3800000000 as libc::c_ulonglong);
    let mut got_445: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_445, b"3800000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            874 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3800000000\0" as *const u8 as *const libc::c_char,
            got_445,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3900000000 as libc::c_ulonglong);
    let mut got_446: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_446, b"3900000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            876 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3900000000\0" as *const u8 as *const libc::c_char,
            got_446,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 3999999999 as libc::c_ulonglong);
    let mut got_447: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_447, b"3999999999\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            878 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"3999999999\0" as *const u8 as *const libc::c_char,
            got_447,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 4000000000 as libc::c_ulonglong);
    let mut got_448: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_448, b"4000000000\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            880 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"4000000000\0" as *const u8 as *const libc::c_char,
            got_448,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 4000000001 as libc::c_ulonglong);
    let mut got_449: size_t = zstr_length(
        a.as_mut_ptr(),
        10 as libc::c_int as libc::c_ulonglong,
    );
    if got_449 != 10 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %zu, but got %zu.\n\0" as *const u8
                as *const libc::c_char,
            882 as libc::c_int,
            b"zstr_length(a, 10)\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as size_t,
            got_449,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    let mut got_450: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_450, b"4000000001\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            883 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"4000000001\0" as *const u8 as *const libc::c_char,
            got_450,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 4000000000 as libc::c_ulonglong);
    zsetu(b.as_mut_ptr(), 4000000000 as libc::c_ulonglong);
    zadd(c.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr());
    zsets(d.as_mut_ptr(), b"8000000000\0" as *const u8 as *const libc::c_char);
    let mut got_451: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_451 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            889 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_451,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_452: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_452 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            891 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_452,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(c.as_mut_ptr(), c.as_mut_ptr(), a.as_mut_ptr());
    zsets(d.as_mut_ptr(), b"12000000000\0" as *const u8 as *const libc::c_char);
    let mut got_453: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_453 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            894 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_453,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(c.as_mut_ptr(), c.as_mut_ptr(), a.as_mut_ptr());
    zsets(d.as_mut_ptr(), b"8000000000\0" as *const u8 as *const libc::c_char);
    let mut got_454: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_454 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            897 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_454,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsub(c.as_mut_ptr(), c.as_mut_ptr(), a.as_mut_ptr());
    zsets(d.as_mut_ptr(), b"4000000000\0" as *const u8 as *const libc::c_char);
    let mut got_455: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_455 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            900 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_455,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(d.as_mut_ptr(), b"8000000000\0" as *const u8 as *const libc::c_char);
    zrsh(d.as_mut_ptr(), d.as_mut_ptr(), 1 as libc::c_int as size_t);
    let mut got_456: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_456 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            903 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_456,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(a.as_mut_ptr(), b"6234216714\0" as *const u8 as *const libc::c_char);
    zsets(b.as_mut_ptr(), b"9424614147\0" as *const u8 as *const libc::c_char);
    zsets(d.as_mut_ptr(), b"830476546\0" as *const u8 as *const libc::c_char);
    zand(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_457: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_457 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            908 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_457,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(a.as_mut_ptr(), b"234216714\0" as *const u8 as *const libc::c_char);
    zsets(b.as_mut_ptr(), b"9424614147\0" as *const u8 as *const libc::c_char);
    zsets(d.as_mut_ptr(), b"9629466379\0" as *const u8 as *const libc::c_char);
    zor(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_458: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_458 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            913 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_458,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(a.as_mut_ptr(), b"6234216714\0" as *const u8 as *const libc::c_char);
    zsets(b.as_mut_ptr(), b"9424614147\0" as *const u8 as *const libc::c_char);
    zsets(d.as_mut_ptr(), b"13997877769\0" as *const u8 as *const libc::c_char);
    zxor(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_459: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_459 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            918 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_459,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(a.as_mut_ptr(), b"34216714\0" as *const u8 as *const libc::c_char);
    zsets(b.as_mut_ptr(), b"9424614147\0" as *const u8 as *const libc::c_char);
    zsets(d.as_mut_ptr(), b"9458821129\0" as *const u8 as *const libc::c_char);
    zxor(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_460: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_460 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            923 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_460,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zsets(d.as_mut_ptr(), b"1000000000000000000\0" as *const u8 as *const libc::c_char);
    zmul(c.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr());
    let mut got_461: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_461 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            927 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_461,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zdiv(c.as_mut_ptr(), c.as_mut_ptr(), a.as_mut_ptr());
    let mut got_462: libc::c_int = zcmp(c.as_mut_ptr(), a.as_mut_ptr());
    if !(got_462 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            929 as libc::c_int,
            b"zcmp(c, a)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_462,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zsets(d.as_mut_ptr(), b"1000000000000000000\0" as *const u8 as *const libc::c_char);
    zsqr(c.as_mut_ptr(), a.as_mut_ptr());
    let mut got_463: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_463 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            933 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_463,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zmodpowu(
        c.as_mut_ptr(),
        a.as_mut_ptr(),
        5 as libc::c_int as libc::c_ulonglong,
        _3.as_mut_ptr(),
    );
    let mut got_464: libc::c_int = zcmpu(
        c.as_mut_ptr(),
        1 as libc::c_int as libc::c_ulonglong,
    );
    if !(got_464 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            936 as libc::c_int,
            b"zcmpu(c, 1)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_464,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zsets(d.as_mut_ptr(), b"1\0" as *const u8 as *const libc::c_char);
    zpowu(c.as_mut_ptr(), a.as_mut_ptr(), 0 as libc::c_int as libc::c_ulonglong);
    let mut got_465: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_465 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            940 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_465,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zsets(d.as_mut_ptr(), b"1000000000\0" as *const u8 as *const libc::c_char);
    zpowu(c.as_mut_ptr(), a.as_mut_ptr(), 1 as libc::c_int as libc::c_ulonglong);
    let mut got_466: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_466 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            944 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_466,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zsets(d.as_mut_ptr(), b"1000000000000000000\0" as *const u8 as *const libc::c_char);
    zpowu(c.as_mut_ptr(), a.as_mut_ptr(), 2 as libc::c_int as libc::c_ulonglong);
    let mut got_467: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_467 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            948 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_467,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zsets(b.as_mut_ptr(), b"1000000000000000000\0" as *const u8 as *const libc::c_char);
    zsets(
        d.as_mut_ptr(),
        b"1000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    );
    zmul(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    let mut got_468: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_468 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            953 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_468,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zsets(
        d.as_mut_ptr(),
        b"1000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    );
    zmul(b.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr());
    zmul(b.as_mut_ptr(), b.as_mut_ptr(), a.as_mut_ptr());
    let mut got_469: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_469 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            958 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_469,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zsets(
        d.as_mut_ptr(),
        b"1000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    );
    zpowu(c.as_mut_ptr(), a.as_mut_ptr(), 3 as libc::c_int as libc::c_ulonglong);
    let mut got_470: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_470 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            962 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_470,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zsets(
        d.as_mut_ptr(),
        b"1000000000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    );
    zpowu(c.as_mut_ptr(), a.as_mut_ptr(), 4 as libc::c_int as libc::c_ulonglong);
    let mut got_471: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_471 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            966 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_471,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000 as libc::c_ulonglong);
    zsets(
        d.as_mut_ptr(),
        b"1000000000000000000000000000000000000000000000\0" as *const u8
            as *const libc::c_char,
    );
    zpowu(c.as_mut_ptr(), a.as_mut_ptr(), 5 as libc::c_int as libc::c_ulonglong);
    let mut got_472: libc::c_int = zcmp(c.as_mut_ptr(), d.as_mut_ptr());
    if !(got_472 == 0 as libc::c_int) {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %i.\n\0" as *const u8
                as *const libc::c_char,
            970 as libc::c_int,
            b"zcmp(c, d)\0" as *const u8 as *const libc::c_char,
            b"== 0\0" as *const u8 as *const libc::c_char,
            got_472,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 4294967294 as libc::c_ulonglong);
    let mut got_473: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_473, b"4294967294\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            973 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"4294967294\0" as *const u8 as *const libc::c_char,
            got_473,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 4294967295 as libc::c_ulonglong);
    let mut got_474: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_474, b"4294967295\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            975 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"4294967295\0" as *const u8 as *const libc::c_char,
            got_474,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 4294967296 as libc::c_ulonglong);
    let mut got_475: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_475, b"4294967296\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            977 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"4294967296\0" as *const u8 as *const libc::c_char,
            got_475,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 4294967297 as libc::c_ulonglong);
    let mut got_476: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_476, b"4294967297\0" as *const u8 as *const libc::c_char) != 0 {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            979 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"4294967297\0" as *const u8 as *const libc::c_char,
            got_476,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), 9223372036854775807 as libc::c_longlong);
    let mut got_477: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_477, b"9223372036854775807\0" as *const u8 as *const libc::c_char) != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            982 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"9223372036854775807\0" as *const u8 as *const libc::c_char,
            got_477,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(a.as_mut_ptr(), -(9223372036854775807 as libc::c_longlong));
    let mut got_478: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_478, b"-9223372036854775807\0" as *const u8 as *const libc::c_char)
        != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            984 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"-9223372036854775807\0" as *const u8 as *const libc::c_char,
            got_478,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zseti(
        a.as_mut_ptr(),
        -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong,
    );
    let mut got_479: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_479, b"-9223372036854775808\0" as *const u8 as *const libc::c_char)
        != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            986 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"-9223372036854775808\0" as *const u8 as *const libc::c_char,
            got_479,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 18446744073709551614 as libc::c_ulonglong);
    let mut got_480: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_480, b"18446744073709551614\0" as *const u8 as *const libc::c_char)
        != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            989 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"18446744073709551614\0" as *const u8 as *const libc::c_char,
            got_480,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 18446744073709551615 as libc::c_ulonglong);
    let mut got_481: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_481, b"18446744073709551615\0" as *const u8 as *const libc::c_char)
        != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            991 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"18446744073709551615\0" as *const u8 as *const libc::c_char,
            got_481,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(a.as_mut_ptr(), a.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_482: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_482, b"18446744073709551616\0" as *const u8 as *const libc::c_char)
        != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            993 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"18446744073709551616\0" as *const u8 as *const libc::c_char,
            got_482,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zadd(a.as_mut_ptr(), a.as_mut_ptr(), _1.as_mut_ptr());
    let mut got_483: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(got_483, b"18446744073709551617\0" as *const u8 as *const libc::c_char)
        != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            995 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"18446744073709551617\0" as *const u8 as *const libc::c_char,
            got_483,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(
        a.as_mut_ptr(),
        b"1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    );
    let mut got_484: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(
        got_484,
        b"1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            998 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
            got_484,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(
        a.as_mut_ptr(),
        b"+1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    );
    let mut got_485: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(
        got_485,
        b"1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            1000 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
            got_485,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsets(
        a.as_mut_ptr(),
        b"-1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    );
    let mut got_486: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(
        got_486,
        b"-1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            1002 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"-1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
            got_486,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zsetu(a.as_mut_ptr(), 1000000000000000 as libc::c_ulonglong);
    zsqr(a.as_mut_ptr(), a.as_mut_ptr());
    let mut got_487: *const libc::c_char = zstr(a.as_mut_ptr(), buf.as_mut_ptr());
    if strcmp(
        got_487,
        b"1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        fprintf(
            stderr,
            b"Failure at line %i: %s, expected %s, but got %s.\n\0" as *const u8
                as *const libc::c_char,
            1006 as libc::c_int,
            b"zstr(a, buf)\0" as *const u8 as *const libc::c_char,
            b"1000000000000000000000000000000\0" as *const u8 as *const libc::c_char,
            got_487,
        );
        ret = 1 as libc::c_int;
        zfree(a.as_mut_ptr());
        zfree(b.as_mut_ptr());
        zfree(c.as_mut_ptr());
        zfree(d.as_mut_ptr());
        zfree(_0.as_mut_ptr());
        zfree(_1.as_mut_ptr());
        zfree(_2.as_mut_ptr());
        zfree(_3.as_mut_ptr());
        zunsetup();
        return ret;
    }
    zfree(a.as_mut_ptr());
    zfree(b.as_mut_ptr());
    zfree(c.as_mut_ptr());
    zfree(d.as_mut_ptr());
    zfree(_0.as_mut_ptr());
    zfree(_1.as_mut_ptr());
    zfree(_2.as_mut_ptr());
    zfree(_3.as_mut_ptr());
    zunsetup();
    return ret;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

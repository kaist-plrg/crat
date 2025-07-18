use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    fn clock() -> clock_t;
    static mut krec: [V; 1000000];
    static mut kreci: I;
    fn cd(a: K) -> K;
    fn show(a: K) -> K;
    fn matchI(a: K, b: K) -> I;
    fn X(s: S) -> K;
    fn Kd() -> K;
    static mut KTREE: K;
    fn SC(a: S, b: S) -> I;
    static mut lineB: S;
    static mut fom: I;
    static mut fbr: I;
    static mut fll: I;
    static mut fdc: I;
    static mut cls: K;
    static mut feci: I;
    static mut cdp: [C; 0];
    static mut fer: I;
    static mut fer1: I;
    fn kerr(s: cS) -> K;
    fn rc(x: K) -> I;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type clock_t = __clock_t;
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
pub static mut passed: I = 0 as libc::c_int as I;
pub static mut skipped: I = 0 as libc::c_int as I;
pub static mut failed: I = 0 as libc::c_int as I;
pub static mut tests: I = 0 as libc::c_int as I;
pub static mut test_print: I = 0 as libc::c_int as I;
pub static mut testtime: F = 0.;
pub unsafe extern "C" fn ts(mut x: I) -> S {
    match x {
        0 => return b"fail\0" as *const u8 as *const libc::c_char as S,
        1 => return b"OK\0" as *const u8 as *const libc::c_char as S,
        2 => return b"skipped\0" as *const u8 as *const libc::c_char as S,
        _ => {}
    }
    return 0 as S;
}
pub unsafe extern "C" fn tp(mut x: I) -> I {
    match x {
        0 => {
            failed += 1;
            failed;
        }
        1 => {
            passed += 1;
            passed;
        }
        2 => {
            skipped += 1;
            skipped;
        }
        _ => {}
    }
    tests += 1;
    tests;
    return x;
}
pub unsafe extern "C" fn tc(mut a: S, mut b: S) -> I {
    if tests % 50 as libc::c_int as libc::c_longlong == 0 {
        printf(b"t:%lld\n\0" as *const u8 as *const libc::c_char, tests);
    }
    if SC(b"skip\0" as *const u8 as *const libc::c_char as S, a) == 0 {
        return 2 as libc::c_int as I;
    }
    kreci = 0 as libc::c_int as I;
    KTREE = Kd();
    let mut x: K = X(a);
    feci = 0 as libc::c_int as I;
    fdc = feci;
    fll = fdc;
    fbr = fll;
    fom = fbr;
    fer1 = fom;
    fer = fer1;
    if !cls.is_null() {
        cd(cls);
        cls = 0 as K;
    }
    let mut i: I = 0 as libc::c_int as I;
    i = 0 as libc::c_int as I;
    while i < 10 as libc::c_int as libc::c_longlong {
        *cdp.as_mut_ptr().offset(i as isize) = 'a' as i32 as C;
        i += 1;
        i;
    }
    let mut y: K = X(b);
    feci = 0 as libc::c_int as I;
    fdc = feci;
    fll = fdc;
    fbr = fll;
    fom = fbr;
    fer1 = fom;
    fer = fer1;
    if !cls.is_null() {
        cd(cls);
        cls = 0 as K;
    }
    i = 0 as libc::c_int as I;
    while i < 10 as libc::c_int as libc::c_longlong {
        *cdp.as_mut_ptr().offset(i as isize) = 'a' as i32 as C;
        i += 1;
        i;
    }
    let mut m: I = matchI(x, y);
    if m == 0 {
        fprintf(
            stderr,
            b"\nFailed. These are not equal:\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(stderr, b"%s , %s\n\0" as *const u8 as *const libc::c_char, a, b);
        fprintf(
            stderr,
            b"********************************\n\0" as *const u8 as *const libc::c_char,
        );
        show(x);
        fflush(stdout);
        fprintf(
            stderr,
            b"--------------------------------\n\0" as *const u8 as *const libc::c_char,
        );
        show(y);
        fflush(stdout);
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
    cd(x);
    cd(y);
    cd(KTREE);
    let mut c: I = 0 as libc::c_int as I;
    let mut i_0: I = 0 as libc::c_int as I;
    let mut _i: I = kreci;
    while i_0 < _i {
        if !(krec[i_0 as usize]).is_null() {
            c += 1;
            c;
        }
        i_0 += 1;
        i_0;
    }
    if c == 0 {
        return m;
    }
    fprintf(
        stderr,
        b"Failed: Memory Leak - %s, %s \nAllocated K: %lld\nUnfreed K  : %lld\nLeak %%     : %f\n\0"
            as *const u8 as *const libc::c_char,
        a,
        b,
        kreci,
        c,
        c as libc::c_double / kreci as F,
    );
    let mut j: I = -(1 as libc::c_int) as I;
    let mut i_1: I = 0 as libc::c_int as I;
    let mut _i_0: I = c;
    while i_1 < _i_0 {
        loop {
            j += 1;
            j;
            if !((krec[j as usize]).is_null() && j < kreci) {
                break;
            }
        }
        if j >= kreci {
            break;
        }
        let mut k: K = krec[j as usize] as K;
        if !k.is_null() {
            printf(
                b"c:%lld t:%lld n:%lld | k:%p\n\0" as *const u8 as *const libc::c_char,
                rc(k),
                (*k).t,
                (*k).n,
                k,
            );
            show(k);
        }
        i_1 += 1;
        i_1;
    }
    return 0 as libc::c_int as I;
}
pub unsafe extern "C" fn test() -> I {
    testtime = clock() as F;
    testsBook();
    tests01();
    tests02();
    testsIO();
    let mut x: K = 0 as *mut k0;
    x = X(b"567\0" as *const u8 as *const libc::c_char as S);
    if tp(
        (!x.is_null()
            && *(((*x).k).as_mut_ptr() as *mut I)
                == 567 as libc::c_int as libc::c_longlong) as libc::c_int as I,
    ) == 0
    {
        fprintf(
            stderr,
            b"\n\nK string execution broken\n\n\0" as *const u8 as *const libc::c_char,
        );
    }
    cd(x);
    testtime = (clock() as libc::c_double - testtime)
        / 1000000 as libc::c_int as __clock_t as libc::c_double;
    let mut rate: F = passed as libc::c_double
        / (tests as F - skipped as libc::c_double);
    printf(
        b"Test pass rate: %.4f, Total: %lld, Passed: %lld, Skipped: %lld, Failed: %lld, Time: %fs\n\0"
            as *const u8 as *const libc::c_char,
        rate,
        tests,
        passed,
        skipped,
        failed,
        testtime,
    );
    let mut r: I = (1 as libc::c_int as libc::c_double == rate) as libc::c_int as I;
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, ts(r));
    testtime = 0 as libc::c_int as F;
    lineB = 0 as S;
    kerr(b"(nil)\0" as *const u8 as *const libc::c_char);
    return r;
}
unsafe extern "C" fn testsIO() -> I {
    return 0 as libc::c_int as I;
}
unsafe extern "C" fn tests02() -> I {
    let mut s: S = ts(
        tp(
            tc(
                b"`b\0" as *const u8 as *const libc::c_char as S,
                b"(`a`b)[1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
            b"`b\0" as *const u8 as *const libc::c_char,
            b"(`a`b)[1]\0" as *const u8 as *const libc::c_char,
            s,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_0: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"{1+1} 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"{1+1} 0\0" as *const u8 as *const libc::c_char,
            s_0,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_1: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"{a:1;a+a} _n\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"{a:1;a+a} _n\0" as *const u8 as *const libc::c_char,
            s_1,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_2: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"{a:1;a+a}0;a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"{a:1;a+a}0;a\0" as *const u8 as *const libc::c_char,
            s_2,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_3: S = ts(
        tp(
            tc(
                b"99#1\0" as *const u8 as *const libc::c_char as S,
                b"{1}'!99\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            b"99#1\0" as *const u8 as *const libc::c_char,
            b"{1}'!99\0" as *const u8 as *const libc::c_char,
            s_3,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_4: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"2#(1;2;3.0)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"2#(1;2;3.0)\0" as *const u8 as *const libc::c_char,
            s_4,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_5: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"-1_(1;2;3.0)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"-1_(1;2;3.0)\0" as *const u8 as *const libc::c_char,
            s_5,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_6: S = ts(
        tp(
            tc(
                b"{x^2}\0" as *const u8 as *const libc::c_char as S,
                b"f:{x^2};.`f\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int,
            b"{x^2}\0" as *const u8 as *const libc::c_char,
            b"f:{x^2};.`f\0" as *const u8 as *const libc::c_char,
            s_6,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_7: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"(1+)~(1+)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"(1+)~(1+)\0" as *const u8 as *const libc::c_char,
            s_7,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_8: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"(1+)~(2+)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"(1+)~(2+)\0" as *const u8 as *const libc::c_char,
            s_8,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_9: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"{x} ~ {x}\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"{x} ~ {x}\0" as *const u8 as *const libc::c_char,
            s_9,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_10: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"{x} ~ {x }\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"{x} ~ {x }\0" as *const u8 as *const libc::c_char,
            s_10,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_11: S = ts(
        tp(
            tc(
                b"1+2+3\0" as *const u8 as *const libc::c_char as S,
                b"{b:1;c:2;d:3;b+c+d} 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            b"1+2+3\0" as *const u8 as *const libc::c_char,
            b"{b:1;c:2;d:3;b+c+d} 0\0" as *const u8 as *const libc::c_char,
            s_11,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_12: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"{1;;;} 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"{1;;;} 0\0" as *const u8 as *const libc::c_char,
            s_12,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_13: S = ts(
        tp(
            tc(
                b"{_f}\0" as *const u8 as *const libc::c_char as S,
                b"{_f} 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
            b"{_f}\0" as *const u8 as *const libc::c_char,
            b"{_f} 0\0" as *const u8 as *const libc::c_char,
            s_13,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_14: S = ts(
        tp(
            tc(
                b"2 6 24\0" as *const u8 as *const libc::c_char as S,
                b"{:[x<2;1;_f[x-1]*x]}' 2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            b"2 6 24\0" as *const u8 as *const libc::c_char,
            b"{:[x<2;1;_f[x-1]*x]}' 2 3 4\0" as *const u8 as *const libc::c_char,
            s_14,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_15: S = ts(
        tp(
            tc(
                b"2 6 24\0" as *const u8 as *const libc::c_char as S,
                b"{:[x<2;1;x* _f[x-1]]}' 2 3 4\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int,
            b"2 6 24\0" as *const u8 as *const libc::c_char,
            b"{:[x<2;1;x* _f[x-1]]}' 2 3 4\0" as *const u8 as *const libc::c_char,
            s_15,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_16: S = ts(
        tp(
            tc(
                b"60.0\0" as *const u8 as *const libc::c_char as S,
                b"{:[(x<2)&(y<2);2;(x^2)+(y^2)+_f[x-1;y-1]]}[4;4]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int,
            b"60.0\0" as *const u8 as *const libc::c_char,
            b"{:[(x<2)&(y<2);2;(x^2)+(y^2)+_f[x-1;y-1]]}[4;4]\0" as *const u8
                as *const libc::c_char,
            s_16,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_17: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"stack\") , .[{_f _f};0;:]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"(1;\"stack\") , .[{_f _f};0;:]\0" as *const u8 as *const libc::c_char,
            s_17,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_18: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"_f\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"_f\0" as *const u8 as *const libc::c_char,
            s_18,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_19: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"{[a;b;cc]q:0;z:1; a+b*cc}[1;2;3]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"{[a;b;cc]q:0;z:1; a+b*cc}[1;2;3]\0" as *const u8 as *const libc::c_char,
            s_19,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_20: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"{x} 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"{x} 1\0" as *const u8 as *const libc::c_char,
            s_20,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_21: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}[1;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"{x+y}[1;1]\0" as *const u8 as *const libc::c_char,
            s_21,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_22: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{z+y+x}[1;1;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{z+y+x}[1;1;1]\0" as *const u8 as *const libc::c_char,
            s_22,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_23: S = ts(
        tp(
            tc(
                b"12\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+y}; f[1;2];f[8;4]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            b"12\0" as *const u8 as *const libc::c_char,
            b"f:{x+y}; f[1;2];f[8;4]\0" as *const u8 as *const libc::c_char,
            s_23,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_24: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"{4} 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"{4} 0\0" as *const u8 as *const libc::c_char,
            s_24,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_25: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"{[] 2+3} 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"{[] 2+3} 0\0" as *const u8 as *const libc::c_char,
            s_25,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_26: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"a: 1 2;{a} 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"a: 1 2;{a} 0\0" as *const u8 as *const libc::c_char,
            s_26,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_27: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"a: 1 2;{a:3} 0;a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"a: 1 2;{a:3} 0;a\0" as *const u8 as *const libc::c_char,
            s_27,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_28: S = ts(
        tp(
            tc(
                b"3 4\0" as *const u8 as *const libc::c_char as S,
                b"a: 1 2;{a+:2} 0; a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            b"3 4\0" as *const u8 as *const libc::c_char,
            b"a: 1 2;{a+:2} 0; a\0" as *const u8 as *const libc::c_char,
            s_28,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_29: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"b:9;{b:b} 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"b:9;{b:b} 0\0" as *const u8 as *const libc::c_char,
            s_29,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_30: S = ts(
        tp(
            tc(
                b"(2 4)\0" as *const u8 as *const libc::c_char as S,
                b"((1+)[1]; (1+|+)[1;2])\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int,
            b"(2 4)\0" as *const u8 as *const libc::c_char,
            b"((1+)[1]; (1+|+)[1;2])\0" as *const u8 as *const libc::c_char,
            s_30,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_31: S = ts(
        tp(
            tc(
                b"()\0" as *const u8 as *const libc::c_char as S,
                b"=!0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            b"()\0" as *const u8 as *const libc::c_char,
            b"=!0\0" as *const u8 as *const libc::c_char,
            s_31,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_32: S = ts(
        tp(
            tc(
                b"(,,0)\0" as *const u8 as *const libc::c_char as S,
                b"=,1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            b"(,,0)\0" as *const u8 as *const libc::c_char,
            b"=,1\0" as *const u8 as *const libc::c_char,
            s_32,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_33: S = ts(
        tp(
            tc(
                b"(,0;,1)\0" as *const u8 as *const libc::c_char as S,
                b"=1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
            b"(,0;,1)\0" as *const u8 as *const libc::c_char,
            b"=1 2\0" as *const u8 as *const libc::c_char,
            s_33,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_34: S = ts(
        tp(
            tc(
                b"(,0;,1)\0" as *const u8 as *const libc::c_char as S,
                b"=2 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int,
            b"(,0;,1)\0" as *const u8 as *const libc::c_char,
            b"=2 1\0" as *const u8 as *const libc::c_char,
            s_34,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_35: S = ts(
        tp(
            tc(
                b"(1;\"valence\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"{x}[1;]\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int,
            b"(1;\"valence\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"{x}[1;]\";:]\0" as *const u8 as *const libc::c_char,
            s_35,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_36: S = ts(
        tp(
            tc(
                b"(1;\"valence\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"+[1;2;;;]\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            b"(1;\"valence\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"+[1;2;;;]\";:]\0" as *const u8 as *const libc::c_char,
            s_36,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_37: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"valence\"), .[+[1;2;;;]; 5;:]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"(1;\"valence\"), .[+[1;2;;;]; 5;:]\0" as *const u8 as *const libc::c_char,
            s_37,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_38: S = ts(
        tp(
            tc(
                b"(1;\"type\")\0" as *const u8 as *const libc::c_char as S,
                b".[+;(1;`b);:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int,
            b"(1;\"type\")\0" as *const u8 as *const libc::c_char,
            b".[+;(1;`b);:]\0" as *const u8 as *const libc::c_char,
            s_38,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_39: S = ts(
        tp(
            tc(
                b"0 43\0" as *const u8 as *const libc::c_char as S,
                b"b:.[+;(1;42);:]; b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            b"0 43\0" as *const u8 as *const libc::c_char,
            b"b:.[+;(1;42);:]; b\0" as *const u8 as *const libc::c_char,
            s_39,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_40: S = ts(
        tp(
            tc(
                b"{x+y}[1;]\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}[1;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            b"{x+y}[1;]\0" as *const u8 as *const libc::c_char,
            b"{x+y}[1;]\0" as *const u8 as *const libc::c_char,
            s_40,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_41: S = ts(
        tp(
            tc(
                b"{x+y}[;1]\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}[;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            b"{x+y}[;1]\0" as *const u8 as *const libc::c_char,
            b"{x+y}[;1]\0" as *const u8 as *const libc::c_char,
            s_41,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_42: S = ts(
        tp(
            tc(
                b"!3\0" as *const u8 as *const libc::c_char as S,
                b"{x,y,z}[;;2][;1][0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            b"!3\0" as *const u8 as *const libc::c_char,
            b"{x,y,z}[;;2][;1][0]\0" as *const u8 as *const libc::c_char,
            s_42,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_43: S = ts(
        tp(
            tc(
                b"1 2 3 4\0" as *const u8 as *const libc::c_char as S,
                b"{x[z;y]}[,][3 4;1 2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            b"1 2 3 4\0" as *const u8 as *const libc::c_char,
            b"{x[z;y]}[,][3 4;1 2]\0" as *const u8 as *const libc::c_char,
            s_43,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_44: S = ts(
        tp(
            tc(
                b"!3\0" as *const u8 as *const libc::c_char as S,
                b"f:{[a;b;c]a,b,c}; g:f 0; h:g 1; h 2\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            143 as libc::c_int,
            b"!3\0" as *const u8 as *const libc::c_char,
            b"f:{[a;b;c]a,b,c}; g:f 0; h:g 1; h 2\0" as *const u8 as *const libc::c_char,
            s_44,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_45: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"{[a;b;c;d;e]a+b+c+d+e}[;;1;1;1]'[;1] , {[a;b;c;d;e]a+b+c+d+e}[;;1;1;1]'[;1]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"{[a;b;c;d;e]a+b+c+d+e}[;;1;1;1]'[;1] , {[a;b;c;d;e]a+b+c+d+e}[;;1;1;1]'[;1]\0"
                as *const u8 as *const libc::c_char,
            s_45,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_46: S = ts(
        tp(
            tc(
                b"(1 0;1 3)\0" as *const u8 as *const libc::c_char as S,
                b"a:(1 0;3 2); f:{a[x;y]}; (a[]0;f[]0)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            b"(1 0;1 3)\0" as *const u8 as *const libc::c_char,
            b"a:(1 0;3 2); f:{a[x;y]}; (a[]0;f[]0)\0" as *const u8
                as *const libc::c_char,
            s_46,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_47: S = ts(
        tp(
            tc(
                b"2 1\0" as *const u8 as *const libc::c_char as S,
                b"f:![1;]; f 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            b"2 1\0" as *const u8 as *const libc::c_char,
            b"f:![1;]; f 1 2\0" as *const u8 as *const libc::c_char,
            s_47,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_48: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"f:+[;]; g:f 1; g 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"f:+[;]; g:f 1; g 2\0" as *const u8 as *const libc::c_char,
            s_48,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_49: S = ts(
        tp(
            tc(
                b"8.0\0" as *const u8 as *const libc::c_char as S,
                b"f:^[;]; g:f[;3]; g 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            b"8.0\0" as *const u8 as *const libc::c_char,
            b"f:^[;]; g:f[;3]; g 2\0" as *const u8 as *const libc::c_char,
            s_49,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_50: S = ts(
        tp(
            tc(
                b"4 5\0" as *const u8 as *const libc::c_char as S,
                b"+[3;]'1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
            b"4 5\0" as *const u8 as *const libc::c_char,
            b"+[3;]'1 2\0" as *const u8 as *const libc::c_char,
            s_50,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_51: S = ts(
        tp(
            tc(
                b"7 8\0" as *const u8 as *const libc::c_char as S,
                b"+[3;]'[4 5]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            b"7 8\0" as *const u8 as *const libc::c_char,
            b"+[3;]'[4 5]\0" as *const u8 as *const libc::c_char,
            s_51,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_52: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"{a::7} 0; a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"{a::7} 0; a\0" as *const u8 as *const libc::c_char,
            s_52,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_53: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"a::[1;7];a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"a::[1;7];a\0" as *const u8 as *const libc::c_char,
            s_53,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_54: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"{a:::[1;7]}0;a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"{a:::[1;7]}0;a\0" as *const u8 as *const libc::c_char,
            s_54,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_55: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"{a::[1;7];a} 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            156 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"{a::[1;7];a} 0\0" as *const u8 as *const libc::c_char,
            s_55,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_56: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{b:3; g:{b}; b:4; g[]}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{b:3; g:{b}; b:4; g[]}0\0" as *const u8 as *const libc::c_char,
            s_56,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_57: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"{b:3; g:{b}; b:4; b}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"{b:3; g:{b}; b:4; b}0\0" as *const u8 as *const libc::c_char,
            s_57,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_58: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"{b:3; g:{a}; b:4; g[]}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"{b:3; g:{a}; b:4; g[]}0\0" as *const u8 as *const libc::c_char,
            s_58,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_59: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"{b:3; g:{a}; b:4; b}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"{b:3; g:{a}; b:4; b}0\0" as *const u8 as *const libc::c_char,
            s_59,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_60: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"{g:{b:1}; g[]}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"{g:{b:1}; g[]}0\0" as *const u8 as *const libc::c_char,
            s_60,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_61: S = ts(
        tp(
            tc(
                b"12\0" as *const u8 as *const libc::c_char as S,
                b"{g:{b:1}; b::12; g[]}0;b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            b"12\0" as *const u8 as *const libc::c_char,
            b"{g:{b:1}; b::12; g[]}0;b\0" as *const u8 as *const libc::c_char,
            s_61,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_62: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"{g:{b:1}; b::12; g[]}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"{g:{b:1}; b::12; g[]}0\0" as *const u8 as *const libc::c_char,
            s_62,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_63: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"{g:{b:1}; c::12; g[]}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"{g:{b:1}; c::12; g[]}0\0" as *const u8 as *const libc::c_char,
            s_63,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_64: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"{g:{b:1}; b::12; b:4; b}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"{g:{b:1}; b::12; b:4; b}0\0" as *const u8 as *const libc::c_char,
            s_64,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_65: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"{g:{b:1}; b::12; b:4; b}0;b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"{g:{b:1}; b::12; b:4; b}0;b\0" as *const u8 as *const libc::c_char,
            s_65,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_66: S = ts(
        tp(
            tc(
                b"12\0" as *const u8 as *const libc::c_char as S,
                b"{g:{b:1}; b::12; b}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            b"12\0" as *const u8 as *const libc::c_char,
            b"{g:{b:1}; b::12; b}0\0" as *const u8 as *const libc::c_char,
            s_66,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_67: S = ts(
        tp(
            tc(
                b"12\0" as *const u8 as *const libc::c_char as S,
                b"{g:{b:1}; b:4; b::12; b}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            b"12\0" as *const u8 as *const libc::c_char,
            b"{g:{b:1}; b:4; b::12; b}0\0" as *const u8 as *const libc::c_char,
            s_67,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_68: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"{g:{b:1}; b:4; b::12; b}0;b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            171 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"{g:{b:1}; b:4; b::12; b}0;b\0" as *const u8 as *const libc::c_char,
            s_68,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_69: S = ts(
        tp(
            tc(
                b"12\0" as *const u8 as *const libc::c_char as S,
                b"b:3;{g:{b:1};b::12;g[]} 0;b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int,
            b"12\0" as *const u8 as *const libc::c_char,
            b"b:3;{g:{b:1};b::12;g[]} 0;b\0" as *const u8 as *const libc::c_char,
            s_69,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_70: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"{g:{b:1;b}; b:12; g[]}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"{g:{b:1;b}; b:12; g[]}0\0" as *const u8 as *const libc::c_char,
            s_70,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_71: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{b:3; {b:4; b}0; b}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{b:3; {b:4; b}0; b}0\0" as *const u8 as *const libc::c_char,
            s_71,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_72: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{[a;b]{[c;d]c+d}[a;b]}[1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{[a;b]{[c;d]c+d}[a;b]}[1;2]\0" as *const u8 as *const libc::c_char,
            s_72,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_73: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"{[a;a]a+a}[1;9]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"{[a;a]a+a}[1;9]\0" as *const u8 as *const libc::c_char,
            s_73,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_74: S = ts(
        tp(
            tc(
                b"1 10\0" as *const u8 as *const libc::c_char as S,
                b"{a:10; {x,a}x}1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
            b"1 10\0" as *const u8 as *const libc::c_char,
            b"{a:10; {x,a}x}1\0" as *const u8 as *const libc::c_char,
            s_74,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_75: S = ts(
        tp(
            tc(
                b"1 2 10\0" as *const u8 as *const libc::c_char as S,
                b"{a:10;{x,y,a}[x;y]}[1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            b"1 2 10\0" as *const u8 as *const libc::c_char,
            b"{a:10;{x,y,a}[x;y]}[1;2]\0" as *const u8 as *const libc::c_char,
            s_75,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_76: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"{{x+y+z}[x;y;z]+1}[1;2;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"{{x+y+z}[x;y;z]+1}[1;2;3]\0" as *const u8 as *const libc::c_char,
            s_76,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_77: S = ts(
        tp(
            tc(
                b"8\0" as *const u8 as *const libc::c_char as S,
                b"{x+{b:3;g:{b};b:4;g[]}0}5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            b"8\0" as *const u8 as *const libc::c_char,
            b"{x+{b:3;g:{b};b:4;g[]}0}5\0" as *const u8 as *const libc::c_char,
            s_77,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_78: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"{x-{x+{b:3;g:{b};b:4;g[]}0}5}12\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"{x-{x+{b:3;g:{b};b:4;g[]}0}5}12\0" as *const u8 as *const libc::c_char,
            s_78,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_79: S = ts(
        tp(
            tc(
                b"3 5\0" as *const u8 as *const libc::c_char as S,
                b"{x+y,{x-{x+{b:3;g:{b};b:4;g[]}0}5}12}[1;2]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            b"3 5\0" as *const u8 as *const libc::c_char,
            b"{x+y,{x-{x+{b:3;g:{b};b:4;g[]}0}5}12}[1;2]\0" as *const u8
                as *const libc::c_char,
            s_79,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_80: S = ts(
        tp(
            tc(
                b"9\0" as *const u8 as *const libc::c_char as S,
                b"{+/x,{x+y,4}[1;2]}1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            b"9\0" as *const u8 as *const libc::c_char,
            b"{+/x,{x+y,4}[1;2]}1\0" as *const u8 as *const libc::c_char,
            s_80,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_81: S = ts(
        tp(
            tc(
                b"9\0" as *const u8 as *const libc::c_char as S,
                b"{+/x,{x+y,{x-{x+{b:3; g:{b}; b:4; g[]}0}5}12}[1;2]}1\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int,
            b"9\0" as *const u8 as *const libc::c_char,
            b"{+/x,{x+y,{x-{x+{b:3; g:{b}; b:4; g[]}0}5}12}[1;2]}1\0" as *const u8
                as *const libc::c_char,
            s_81,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_82: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"{+/x,{x+y,{x-{x+{b:3; g:{b}; b:4; :7; g[]}0}5}12}[1;2]}1\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"{+/x,{x+y,{x-{x+{b:3; g:{b}; b:4; :7; g[]}0}5}12}[1;2]}1\0" as *const u8
                as *const libc::c_char,
            s_82,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_83: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"{+/x,{x+y,{x-{x+{b:3; g:{b}; b:4; g[]}0}5}12}[1;2]; :7}1\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"{+/x,{x+y,{x-{x+{b:3; g:{b}; b:4; g[]}0}5}12}[1;2]; :7}1\0" as *const u8
                as *const libc::c_char,
            s_83,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_84: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{{b:3; g:{b}; b:4; g[]}0}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{{b:3; g:{b}; b:4; g[]}0}0\0" as *const u8 as *const libc::c_char,
            s_84,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_85: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"{1+{b:3; g:{b}; b:4; g[]}0}0\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"{1+{b:3; g:{b}; b:4; g[]}0}0\0" as *const u8 as *const libc::c_char,
            s_85,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_86: S = ts(
        tp(
            tc(
                b"8\0" as *const u8 as *const libc::c_char as S,
                b"{({b:3;g:{b};b:4;g[]}0) + {b:5;g:{b};b:6;g[]}0}0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            b"8\0" as *const u8 as *const libc::c_char,
            b"{({b:3;g:{b};b:4;g[]}0) + {b:5;g:{b};b:6;g[]}0}0\0" as *const u8
                as *const libc::c_char,
            s_86,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_87: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"b:9; {b:3;g:{b};b:4;g[]}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"b:9; {b:3;g:{b};b:4;g[]}0\0" as *const u8 as *const libc::c_char,
            s_87,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_88: S = ts(
        tp(
            tc(
                b"25\0" as *const u8 as *const libc::c_char as S,
                b"{{+/x,{x+y,{x-{x+{b:3;g:{b};b:4;g[]}0}5}12}[1;2]}1 + {x+{b:3;g:{b};b:4;g[]}0}5 + {x-{x+{b:3;g:{b};b:4;g[]}0}5}12 + x}4\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            b"25\0" as *const u8 as *const libc::c_char,
            b"{{+/x,{x+y,{x-{x+{b:3;g:{b};b:4;g[]}0}5}12}[1;2]}1 + {x+{b:3;g:{b};b:4;g[]}0}5 + {x-{x+{b:3;g:{b};b:4;g[]}0}5}12 + x}4\0"
                as *const u8 as *const libc::c_char,
            s_88,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_89: S = ts(
        tp(
            tc(
                b"1 3 10 3 10\0" as *const u8 as *const libc::c_char as S,
                b"g:1; do[2;{a:10; g::g,{x,a}x}3]; g\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int,
            b"1 3 10 3 10\0" as *const u8 as *const libc::c_char,
            b"g:1; do[2;{a:10; g::g,{x,a}x}3]; g\0" as *const u8 as *const libc::c_char,
            s_89,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_90: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+y+z}; g:f[1;;3]; {b:3; h:{b}; b:4; g[h[ ] ] }0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"f:{x+y+z}; g:f[1;;3]; {b:3; h:{b}; b:4; g[h[ ] ] }0\0" as *const u8
                as *const libc::c_char,
            s_90,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_91: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{:[x<2;1;_f[x-1]*x]}'2 3 4; {b:3; g:{b}; b:4; g[]}0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{:[x<2;1;_f[x-1]*x]}'2 3 4; {b:3; g:{b}; b:4; g[]}0\0" as *const u8
                as *const libc::c_char,
            s_91,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_92: S = ts(
        tp(
            tc(
                b"120\0" as *const u8 as *const libc::c_char as S,
                b"{b:3; g:{b}; a::{:[x<2; 1; _f[x-1]*x]}5; g[ ]}0; a\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            b"120\0" as *const u8 as *const libc::c_char,
            b"{b:3; g:{b}; a::{:[x<2; 1; _f[x-1]*x]}5; g[ ]}0; a\0" as *const u8
                as *const libc::c_char,
            s_92,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_93: S = ts(
        tp(
            tc(
                b"2 6 24\0" as *const u8 as *const libc::c_char as S,
                b"{b:3; g:{b}; a::{:[x<2; 1; _f[x-1]*x]}'2 3 4; g[ ]}0; a\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            197 as libc::c_int,
            b"2 6 24\0" as *const u8 as *const libc::c_char,
            b"{b:3; g:{b}; a::{:[x<2; 1; _f[x-1]*x]}'2 3 4; g[ ]}0; a\0" as *const u8
                as *const libc::c_char,
            s_93,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_94: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"{x + {[a] a+a} y}[1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"{x + {[a] a+a} y}[1;2]\0" as *const u8 as *const libc::c_char,
            s_94,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_95: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"{[a;b] a + {x+x} b}[1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"{[a;b] a + {x+x} b}[1;2]\0" as *const u8 as *const libc::c_char,
            s_95,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_96: S = ts(
        tp(
            tc(
                b"(7 0;7 1;7 2)\0" as *const u8 as *const libc::c_char as S,
                b"f:{(7;x)};{[n]a:n;f'[!a]}[3]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int,
            b"(7 0;7 1;7 2)\0" as *const u8 as *const libc::c_char,
            b"f:{(7;x)};{[n]a:n;f'[!a]}[3]\0" as *const u8 as *const libc::c_char,
            s_96,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_97: S = ts(
        tp(
            tc(
                b"(7 0;7 1;7 2)\0" as *const u8 as *const libc::c_char as S,
                b"{a:5;{(7;x)}'[!3]}[]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int,
            b"(7 0;7 1;7 2)\0" as *const u8 as *const libc::c_char,
            b"{a:5;{(7;x)}'[!3]}[]\0" as *const u8 as *const libc::c_char,
            s_97,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_98: S = ts(
        tp(
            tc(
                b"1 3\0" as *const u8 as *const libc::c_char as S,
                b"a:{x/y}; b:{a[{(x;#y)};x]}; b[(1;1 2 3)]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            b"1 3\0" as *const u8 as *const libc::c_char,
            b"a:{x/y}; b:{a[{(x;#y)};x]}; b[(1;1 2 3)]\0" as *const u8
                as *const libc::c_char,
            s_98,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_99: S = ts(
        tp(
            tc(
                b"2 2#!4\0" as *const u8 as *const libc::c_char as S,
                b"a:{[f;c]f/'c}; c:2 2#!4; {d:a[{(x;y)};x];d}c\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int,
            b"2 2#!4\0" as *const u8 as *const libc::c_char,
            b"a:{[f;c]f/'c}; c:2 2#!4; {d:a[{(x;y)};x];d}c\0" as *const u8
                as *const libc::c_char,
            s_99,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_100: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"{a:x;{a,x}y}[1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"{a:x;{a,x}y}[1;2]\0" as *const u8 as *const libc::c_char,
            s_100,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_101: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"{a:x;{[b]a,b}y}[1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"{a:x;{[b]a,b}y}[1;2]\0" as *const u8 as *const libc::c_char,
            s_101,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_102: S = ts(
        tp(
            tc(
                b"(1 0;3 2)\0" as *const u8 as *const libc::c_char as S,
                b"g:{[x;f]l:x[;0];r:x[;1];l f' r}; d:2 2#!4; g[d;{y,x}]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            b"(1 0;3 2)\0" as *const u8 as *const libc::c_char,
            b"g:{[x;f]l:x[;0];r:x[;1];l f' r}; d:2 2#!4; g[d;{y,x}]\0" as *const u8
                as *const libc::c_char,
            s_102,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_103: S = ts(
        tp(
            tc(
                b"(1 0;3 2)\0" as *const u8 as *const libc::c_char as S,
                b"g:{[x;f]l:x[;0];r:x[;1];l f' r}; a:{y,x}; d:2 2#!4; g[d;a]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int,
            b"(1 0;3 2)\0" as *const u8 as *const libc::c_char,
            b"g:{[x;f]l:x[;0];r:x[;1];l f' r}; a:{y,x}; d:2 2#!4; g[d;a]\0" as *const u8
                as *const libc::c_char,
            s_103,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_104: S = ts(
        tp(
            tc(
                b"(0 3;1 3;2 3;3 3;4 3;5 3)\0" as *const u8 as *const libc::c_char as S,
                b"{{[x;t](t,x)}[x]'!y+x}[3;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            b"(0 3;1 3;2 3;3 3;4 3;5 3)\0" as *const u8 as *const libc::c_char,
            b"{{[x;t](t,x)}[x]'!y+x}[3;3]\0" as *const u8 as *const libc::c_char,
            s_104,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_105: S = ts(
        tp(
            tc(
                b"(0 3;1 3;2 3;3 3;4 3;5 3)\0" as *const u8 as *const libc::c_char as S,
                b"{{[t](t,x)}'!y+x}[3;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int,
            b"(0 3;1 3;2 3;3 3;4 3;5 3)\0" as *const u8 as *const libc::c_char,
            b"{{[t](t,x)}'!y+x}[3;3]\0" as *const u8 as *const libc::c_char,
            s_105,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_106: S = ts(
        tp(
            tc(
                b"(0 3;1 3;2 3;3 3;4 3;5 3)\0" as *const u8 as *const libc::c_char as S,
                b"{a:x;{[t](t,x)}'!y+x}[3;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int,
            b"(0 3;1 3;2 3;3 3;4 3;5 3)\0" as *const u8 as *const libc::c_char,
            b"{a:x;{[t](t,x)}'!y+x}[3;3]\0" as *const u8 as *const libc::c_char,
            s_106,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_107: S = ts(
        tp(
            tc(
                b"0 0 0 1 0 0 0 0 0 0\0" as *const u8 as *const libc::c_char as S,
                b"f:{a:!x;{3=x!10}'a};f 10\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            b"0 0 0 1 0 0 0 0 0 0\0" as *const u8 as *const libc::c_char,
            b"f:{a:!x;{3=x!10}'a};f 10\0" as *const u8 as *const libc::c_char,
            s_107,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_108: S = ts(
        tp(
            tc(
                b"15 20 25\0" as *const u8 as *const libc::c_char as S,
                b"f:{a:x; 1 2 3{a+x*y}\\:5} ;f 10\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            b"15 20 25\0" as *const u8 as *const libc::c_char,
            b"f:{a:x; 1 2 3{a+x*y}\\:5} ;f 10\0" as *const u8 as *const libc::c_char,
            s_108,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_109: S = ts(
        tp(
            tc(
                b"99\0" as *const u8 as *const libc::c_char as S,
                b"f:{a:x; 2{a+x+y}/42}; f 55\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            b"99\0" as *const u8 as *const libc::c_char,
            b"f:{a:x; 2{a+x+y}/42}; f 55\0" as *const u8 as *const libc::c_char,
            s_109,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_110: S = ts(
        tp(
            tc(
                b"29\0" as *const u8 as *const libc::c_char as S,
                b"g:{a:x; 2{a+x+y}/!6}; g 1; g 2\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            b"29\0" as *const u8 as *const libc::c_char,
            b"g:{a:x; 2{a+x+y}/!6}; g 1; g 2\0" as *const u8 as *const libc::c_char,
            s_110,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_111: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"{a:x;{a}()}'1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"{a:x;{a}()}'1 2\0" as *const u8 as *const libc::c_char,
            s_111,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_112: S = ts(
        tp(
            tc(
                b"123\0" as *const u8 as *const libc::c_char as S,
                b"f:{a:x; {a+x}}; g:f 123; h:f 456; g 0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            b"123\0" as *const u8 as *const libc::c_char,
            b"f:{a:x; {a+x}}; g:f 123; h:f 456; g 0\0" as *const u8
                as *const libc::c_char,
            s_112,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_113: S = ts(
        tp(
            tc(
                b"456\0" as *const u8 as *const libc::c_char as S,
                b"f:{a:x; {a+x}}; g:f 123; h:f 456; h 0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int,
            b"456\0" as *const u8 as *const libc::c_char,
            b"f:{a:x; {a+x}}; g:f 123; h:f 456; h 0\0" as *const u8
                as *const libc::c_char,
            s_113,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_114: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"f:{a:x; g::{a+x}; a+:a; h::{a+x}}; f 10; g 0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"f:{a:x; g::{a+x}; a+:a; h::{a+x}}; f 10; g 0\0" as *const u8
                as *const libc::c_char,
            s_114,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_115: S = ts(
        tp(
            tc(
                b"20\0" as *const u8 as *const libc::c_char as S,
                b"f:{a:x; g::{a+x}; a+:a; h::{a+x}}; f 10; g 0; h 0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int,
            b"20\0" as *const u8 as *const libc::c_char,
            b"f:{a:x; g::{a+x}; a+:a; h::{a+x}}; f 10; g 0; h 0\0" as *const u8
                as *const libc::c_char,
            s_115,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_116: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"f:{a}; {a:x; f()}1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"f:{a}; {a:x; f()}1\0" as *const u8 as *const libc::c_char,
            s_116,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_117: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"f: {x {x}/ 1}; f 1; f 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"f: {x {x}/ 1}; f 1; f 1\0" as *const u8 as *const libc::c_char,
            s_117,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_118: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"f:{x}; g:{x f/1}; g 1; g 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"f:{x}; g:{x f/1}; g 1; g 1\0" as *const u8 as *const libc::c_char,
            s_118,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_119: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"f:{x}; g:{x f/1}; g 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"f:{x}; g:{x f/1}; g 2\0" as *const u8 as *const libc::c_char,
            s_119,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_120: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"f:{x}; g:{x f/1}; g 1; g 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"f:{x}; g:{x f/1}; g 1; g 2\0" as *const u8 as *const libc::c_char,
            s_120,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_121: S = ts(
        tp(
            tc(
                b"22\0" as *const u8 as *const libc::c_char as S,
                b"{[e]{[x]e}1}@22\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int,
            b"22\0" as *const u8 as *const libc::c_char,
            b"{[e]{[x]e}1}@22\0" as *const u8 as *const libc::c_char,
            s_121,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_122: S = ts(
        tp(
            tc(
                b"(7 0;7 1;7 2)\0" as *const u8 as *const libc::c_char as S,
                b"f:{(7;x)}; {[n]a:n;f'[!a]}[3]; {[n]a:n;f'[!a]}[3]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int,
            b"(7 0;7 1;7 2)\0" as *const u8 as *const libc::c_char,
            b"f:{(7;x)}; {[n]a:n;f'[!a]}[3]; {[n]a:n;f'[!a]}[3]\0" as *const u8
                as *const libc::c_char,
            s_122,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_123: S = ts(
        tp(
            tc(
                b"456 123\0" as *const u8 as *const libc::c_char as S,
                b"f:{{[a]a,x}}123; f 456\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int,
            b"456 123\0" as *const u8 as *const libc::c_char,
            b"f:{{[a]a,x}}123; f 456\0" as *const u8 as *const libc::c_char,
            s_123,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_124: S = ts(
        tp(
            tc(
                b"456 123\0" as *const u8 as *const libc::c_char as S,
                b"x:999; f:{{[a]a,x}}123; f 456\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int,
            b"456 123\0" as *const u8 as *const libc::c_char,
            b"x:999; f:{{[a]a,x}}123; f 456\0" as *const u8 as *const libc::c_char,
            s_124,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_125: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{{[a]a+x}}[1][2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            229 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{{[a]a+x}}[1][2]\0" as *const u8 as *const libc::c_char,
            s_125,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_126: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{{[a]a+x}}[1]2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{{[a]a+x}}[1]2\0" as *const u8 as *const libc::c_char,
            s_126,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_127: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"{a:x;{[b](a;b)}y}[1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"{a:x;{[b](a;b)}y}[1;2]\0" as *const u8 as *const libc::c_char,
            s_127,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_128: S = ts(
        tp(
            tc(
                b"2 1\0" as *const u8 as *const libc::c_char as S,
                b"a:1;{a:2;{a}[]}[],a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            236 as libc::c_int,
            b"2 1\0" as *const u8 as *const libc::c_char,
            b"a:1;{a:2;{a}[]}[],a\0" as *const u8 as *const libc::c_char,
            s_128,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_129: S = ts(
        tp(
            tc(
                b"2 1\0" as *const u8 as *const libc::c_char as S,
                b"a:1;{a:2;{a+x}[0]}[],a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            237 as libc::c_int,
            b"2 1\0" as *const u8 as *const libc::c_char,
            b"a:1;{a:2;{a+x}[0]}[],a\0" as *const u8 as *const libc::c_char,
            s_129,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_130: S = ts(
        tp(
            tc(
                b"(0;\"s\")\0" as *const u8 as *const libc::c_char as S,
                b"{@[b;\"s\";:]}[]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
            b"(0;\"s\")\0" as *const u8 as *const libc::c_char,
            b"{@[b;\"s\";:]}[]\0" as *const u8 as *const libc::c_char,
            s_130,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_131: S = ts(
        tp(
            tc(
                b"5 3\0" as *const u8 as *const libc::c_char as S,
                b"{m:{x,y}[5]; {m[x]}[3]}[]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            239 as libc::c_int,
            b"5 3\0" as *const u8 as *const libc::c_char,
            b"{m:{x,y}[5]; {m[x]}[3]}[]\0" as *const u8 as *const libc::c_char,
            s_131,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_132: S = ts(
        tp(
            tc(
                b"5 5\0" as *const u8 as *const libc::c_char as S,
                b"{[m] {}[]; {x;m}'1 2}5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int,
            b"5 5\0" as *const u8 as *const libc::c_char,
            b"{[m] {}[]; {x;m}'1 2}5\0" as *const u8 as *const libc::c_char,
            s_132,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_133: S = ts(
        tp(
            tc(
                b".[ :[1;>:;<:];(1;2);:]\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"valence\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int,
            b".[ :[1;>:;<:];(1;2);:]\0" as *const u8 as *const libc::c_char,
            b"(1;\"valence\")\0" as *const u8 as *const libc::c_char,
            s_133,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_134: S = ts(
        tp(
            tc(
                b"15 19\0" as *const u8 as *const libc::c_char as S,
                b"10+/'(2 3;4 5)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int,
            b"15 19\0" as *const u8 as *const libc::c_char,
            b"10+/'(2 3;4 5)\0" as *const u8 as *const libc::c_char,
            s_134,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_135: S = ts(
        tp(
            tc(
                b"(10 12 15;10 14 19)\0" as *const u8 as *const libc::c_char as S,
                b"10+\\'(2 3;4 5)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            b"(10 12 15;10 14 19)\0" as *const u8 as *const libc::c_char,
            b"10+\\'(2 3;4 5)\0" as *const u8 as *const libc::c_char,
            s_135,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_136: S = ts(
        tp(
            tc(
                b"60 200\0" as *const u8 as *const libc::c_char as S,
                b"10*/'(2 3;4 5)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int,
            b"60 200\0" as *const u8 as *const libc::c_char,
            b"10*/'(2 3;4 5)\0" as *const u8 as *const libc::c_char,
            s_136,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_137: S = ts(
        tp(
            tc(
                b"(10 20 60;10 40 200)\0" as *const u8 as *const libc::c_char as S,
                b"10*\\'(2 3;4 5)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            245 as libc::c_int,
            b"(10 20 60;10 40 200)\0" as *const u8 as *const libc::c_char,
            b"10*\\'(2 3;4 5)\0" as *const u8 as *const libc::c_char,
            s_137,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_138: S = ts(
        tp(
            tc(
                b"(\"ab\";\"ba\")\0" as *const u8 as *const libc::c_char as S,
                b"p:{[n]:[1=#n;,n;{x,',/p[n _dv x]}'n]}; perm:{,/p[x]}; perm \"ab\"\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int,
            b"(\"ab\";\"ba\")\0" as *const u8 as *const libc::c_char,
            b"p:{[n]:[1=#n;,n;{x,',/p[n _dv x]}'n]}; perm:{,/p[x]}; perm \"ab\"\0"
                as *const u8 as *const libc::c_char,
            s_138,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_139: S = ts(
        tp(
            tc(
                b".[3 3#0;(1 2;0 1);{x;y};1]\0" as *const u8 as *const libc::c_char as S,
                b"3 3#0 0 0 1 1 0 1 1 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            247 as libc::c_int,
            b".[3 3#0;(1 2;0 1);{x;y};1]\0" as *const u8 as *const libc::c_char,
            b"3 3#0 0 0 1 1 0 1 1 0\0" as *const u8 as *const libc::c_char,
            s_139,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_140: S = ts(
        tp(
            tc(
                b"(2 0;2 1)\0" as *const u8 as *const libc::c_char as S,
                b"for:{[n;f]f'!n}; {[i]for[i]{[j]i,j}}2\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            248 as libc::c_int,
            b"(2 0;2 1)\0" as *const u8 as *const libc::c_char,
            b"for:{[n;f]f'!n}; {[i]for[i]{[j]i,j}}2\0" as *const u8
                as *const libc::c_char,
            s_140,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_141: S = ts(
        tp(
            tc(
                b"(();,1 0;(2 0;2 1))\0" as *const u8 as *const libc::c_char as S,
                b"for:{[n;f]f'!n}; for[3]{[i]for[i]{[j]i,j}}\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            249 as libc::c_int,
            b"(();,1 0;(2 0;2 1))\0" as *const u8 as *const libc::c_char,
            b"for:{[n;f]f'!n}; for[3]{[i]for[i]{[j]i,j}}\0" as *const u8
                as *const libc::c_char,
            s_141,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_142: S = ts(
        tp(
            tc(
                b"21.8\0" as *const u8 as *const libc::c_char as S,
                b"c: 1 4 3 0.5 -2; mdev:{x-(+/x)%#x}; fo:{+/ _sqr mdev x}; fo c\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            250 as libc::c_int,
            b"21.8\0" as *const u8 as *const libc::c_char,
            b"c: 1 4 3 0.5 -2; mdev:{x-(+/x)%#x}; fo:{+/ _sqr mdev x}; fo c\0"
                as *const u8 as *const libc::c_char,
            s_142,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_143: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"mi:{:[x!2;(x-1)%2; _ ((x-1),x)%2.0]}; {mi x}5; {mi x}5\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"mi:{:[x!2;(x-1)%2; _ ((x-1),x)%2.0]}; {mi x}5; {mi x}5\0" as *const u8
                as *const libc::c_char,
            s_143,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_144: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"m:{:[x!2; 1; 2]}; {m x}5; {m x}5\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            252 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"m:{:[x!2; 1; 2]}; {m x}5; {m x}5\0" as *const u8 as *const libc::c_char,
            s_144,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_145: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"m:{:[x; 1; 2]}; {m x}5; {m x}5\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"m:{:[x; 1; 2]}; {m x}5; {m x}5\0" as *const u8 as *const libc::c_char,
            s_145,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_146: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"m:{:[x; 1; 2]}; {m 3}5; {m 3}5\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            254 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"m:{:[x; 1; 2]}; {m 3}5; {m 3}5\0" as *const u8 as *const libc::c_char,
            s_146,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_147: S = ts(
        tp(
            tc(
                b"2 2#16.434 9.6192 7.2 12.114\0" as *const u8 as *const libc::c_char
                    as S,
                b"e:2 2 # 2.1 4.008 3 0.3; e _mul e\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int,
            b"2 2#16.434 9.6192 7.2 12.114\0" as *const u8 as *const libc::c_char,
            b"e:2 2 # 2.1 4.008 3 0.3; e _mul e\0" as *const u8 as *const libc::c_char,
            s_147,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_148: S = ts(
        tp(
            tc(
                b"2 2#0 1 1 0\0" as *const u8 as *const libc::c_char as S,
                b"p:{:[1<x;,/(>:'(x,x)#1,x#0)[;0,'1+_f x-1];,!x]}; p 2; p 2\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int,
            b"2 2#0 1 1 0\0" as *const u8 as *const libc::c_char,
            b"p:{:[1<x;,/(>:'(x,x)#1,x#0)[;0,'1+_f x-1];,!x]}; p 2; p 2\0" as *const u8
                as *const libc::c_char,
            s_148,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_149: S = ts(
        tp(
            tc(
                b"1.5\0" as *const u8 as *const libc::c_char as S,
                b"i:{(2#x)#1.,x#0.};f:{(+%':+x)[;!-1+#x]};*{+/(f(+\\x))[!-1+#x;]*|i -1+#x}2 2#2. 3 1 2\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            258 as libc::c_int,
            b"1.5\0" as *const u8 as *const libc::c_char,
            b"i:{(2#x)#1.,x#0.};f:{(+%':+x)[;!-1+#x]};*{+/(f(+\\x))[!-1+#x;]*|i -1+#x}2 2#2. 3 1 2\0"
                as *const u8 as *const libc::c_char,
            s_149,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_150: S = ts(
        tp(
            tc(
                b"9\0" as *const u8 as *const libc::c_char as S,
                b"{c:{. x};c \"9\"}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            259 as libc::c_int,
            b"9\0" as *const u8 as *const libc::c_char,
            b"{c:{. x};c \"9\"}0\0" as *const u8 as *const libc::c_char,
            s_150,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_151: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b":(3;4)@0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b":(3;4)@0\0" as *const u8 as *const libc::c_char,
            s_151,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_152: S = ts(
        tp(
            tc(
                b"()\0" as *const u8 as *const libc::c_char as S,
                b",/()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            261 as libc::c_int,
            b"()\0" as *const u8 as *const libc::c_char,
            b",/()\0" as *const u8 as *const libc::c_char,
            s_152,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_153: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"{a:x; {a,x}@2}1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"{a:x; {a,x}@2}1\0" as *const u8 as *const libc::c_char,
            s_153,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_154: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"{a:x; {a,x}. 2}1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"{a:x; {a,x}. 2}1\0" as *const u8 as *const libc::c_char,
            s_154,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_155: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"{a:x; {a,x}.,2}1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            264 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"{a:x; {a,x}.,2}1\0" as *const u8 as *const libc::c_char,
            s_155,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_156: S = ts(
        tp(
            tc(
                b"1 1\0" as *const u8 as *const libc::c_char as S,
                b"a:@[!2;0;:;1]; a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            265 as libc::c_int,
            b"1 1\0" as *const u8 as *const libc::c_char,
            b"a:@[!2;0;:;1]; a\0" as *const u8 as *const libc::c_char,
            s_156,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_157: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"if[1;\\\\]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"if[1;\\\\]\0" as *const u8 as *const libc::c_char,
            s_157,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_158: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"do[5;\\\\]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"do[5;\\\\]\0" as *const u8 as *const libc::c_char,
            s_158,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_159: S = ts(
        tp(
            tc(
                b"(,2)\0" as *const u8 as *const libc::c_char as S,
                b"l:,0; (.[`l;0;:;])2; l\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            270 as libc::c_int,
            b"(,2)\0" as *const u8 as *const libc::c_char,
            b"l:,0; (.[`l;0;:;])2; l\0" as *const u8 as *const libc::c_char,
            s_159,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_160: S = ts(
        tp(
            tc(
                b"(.,(`a;2;))\0" as *const u8 as *const libc::c_char as S,
                b"d:.(); (.[`d;`a;:;])2; d\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int,
            b"(.,(`a;2;))\0" as *const u8 as *const libc::c_char,
            b"d:.(); (.[`d;`a;:;])2; d\0" as *const u8 as *const libc::c_char,
            s_160,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_161: S = ts(
        tp(
            tc(
                b"(1 2;3 4)\0" as *const u8 as *const libc::c_char as S,
                b"({x,y}.)'(1 2;3 4)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            272 as libc::c_int,
            b"(1 2;3 4)\0" as *const u8 as *const libc::c_char,
            b"({x,y}.)'(1 2;3 4)\0" as *const u8 as *const libc::c_char,
            s_161,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_162: S = ts(
        tp(
            tc(
                b",0 1 2\0" as *const u8 as *const libc::c_char as S,
                b"= 0 0 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            273 as libc::c_int,
            b",0 1 2\0" as *const u8 as *const libc::c_char,
            b"= 0 0 0\0" as *const u8 as *const libc::c_char,
            s_162,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_163: S = ts(
        tp(
            tc(
                b",1\0" as *const u8 as *const libc::c_char as S,
                b"#:',0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int,
            b",1\0" as *const u8 as *const libc::c_char,
            b"#:',0\0" as *const u8 as *const libc::c_char,
            s_163,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_164: S = ts(
        tp(
            tc(
                b",1\0" as *const u8 as *const libc::c_char as S,
                b"#:',,0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int,
            b",1\0" as *const u8 as *const libc::c_char,
            b"#:',,0\0" as *const u8 as *const libc::c_char,
            s_164,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_165: S = ts(
        tp(
            tc(
                b"(1 3;2 4)\0" as *const u8 as *const libc::c_char as S,
                b"f:{x,y};(f').(1 2;3 4)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            276 as libc::c_int,
            b"(1 3;2 4)\0" as *const u8 as *const libc::c_char,
            b"f:{x,y};(f').(1 2;3 4)\0" as *const u8 as *const libc::c_char,
            s_165,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_166: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:1; .k~...k\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            277 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:1; .k~...k\0" as *const u8 as *const libc::c_char,
            s_166,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_167: S = ts(
        tp(
            tc(
                b"0 1 0 1\0" as *const u8 as *const libc::c_char as S,
                b"?[\"+-<>[].\";]'\"+-+-\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            278 as libc::c_int,
            b"0 1 0 1\0" as *const u8 as *const libc::c_char,
            b"?[\"+-<>[].\";]'\"+-+-\"\0" as *const u8 as *const libc::c_char,
            s_167,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_168: S = ts(
        tp(
            tc(
                b"0 1 0 1\0" as *const u8 as *const libc::c_char as S,
                b"(\"+-<>[].\"?)'\"+-+-\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            279 as libc::c_int,
            b"0 1 0 1\0" as *const u8 as *const libc::c_char,
            b"(\"+-<>[].\"?)'\"+-+-\"\0" as *const u8 as *const libc::c_char,
            s_168,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_169: S = ts(
        tp(
            tc(
                b"9\0" as *const u8 as *const libc::c_char as S,
                b"f:{x*x}; g:{x+1}; (f;g)[0;3]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            280 as libc::c_int,
            b"9\0" as *const u8 as *const libc::c_char,
            b"f:{x*x}; g:{x+1}; (f;g)[0;3]\0" as *const u8 as *const libc::c_char,
            s_169,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_170: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"g:{x+1}; (0 1 2 3;g)[0;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            281 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"g:{x+1}; (0 1 2 3;g)[0;3]\0" as *const u8 as *const libc::c_char,
            s_170,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_171: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"{c:y}[];\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            282 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"{c:y}[];\0" as *const u8 as *const libc::c_char,
            s_171,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_172: S = ts(
        tp(
            tc(
                b"\"foo \"\0" as *const u8 as *const libc::c_char as S,
                b"foo:{[] \"foo \"}; foo[]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            283 as libc::c_int,
            b"\"foo \"\0" as *const u8 as *const libc::c_char,
            b"foo:{[] \"foo \"}; foo[]\0" as *const u8 as *const libc::c_char,
            s_172,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_173: S = ts(
        tp(
            tc(
                b"{(y)+x}\0" as *const u8 as *const libc::c_char as S,
                b"{(y)+x}\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            284 as libc::c_int,
            b"{(y)+x}\0" as *const u8 as *const libc::c_char,
            b"{(y)+x}\0" as *const u8 as *const libc::c_char,
            s_173,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_174: S = ts(
        tp(
            tc(
                b"(0 1;2 3;4 5)\0" as *const u8 as *const libc::c_char as S,
                b"t:.+(`f`g`h;3 2#!6); t`f`g`h\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            285 as libc::c_int,
            b"(0 1;2 3;4 5)\0" as *const u8 as *const libc::c_char,
            b"t:.+(`f`g`h;3 2#!6); t`f`g`h\0" as *const u8 as *const libc::c_char,
            s_174,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_175: S = ts(
        tp(
            tc(
                b"(0 1;2 3)\0" as *const u8 as *const libc::c_char as S,
                b"t:.+(`f`g`h;3 2#!6); t`f`g\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int,
            b"(0 1;2 3)\0" as *const u8 as *const libc::c_char,
            b"t:.+(`f`g`h;3 2#!6); t`f`g\0" as *const u8 as *const libc::c_char,
            s_175,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_176: S = ts(
        tp(
            tc(
                b"0=/!0\0" as *const u8 as *const libc::c_char as S,
                b"0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            287 as libc::c_int,
            b"0=/!0\0" as *const u8 as *const libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char,
            s_176,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_177: S = ts(
        tp(
            tc(
                b"=/!0\0" as *const u8 as *const libc::c_char as S,
                b"1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            288 as libc::c_int,
            b"=/!0\0" as *const u8 as *const libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char,
            s_177,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_178: S = ts(
        tp(
            tc(
                b"=/1 2 3\0" as *const u8 as *const libc::c_char as S,
                b"0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            289 as libc::c_int,
            b"=/1 2 3\0" as *const u8 as *const libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char,
            s_178,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_179: S = ts(
        tp(
            tc(
                b"120\0" as *const u8 as *const libc::c_char as S,
                b"fac:{[n] :[n=0;1;n*fac n-1]}; fac 5\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int,
            b"120\0" as *const u8 as *const libc::c_char,
            b"fac:{[n] :[n=0;1;n*fac n-1]}; fac 5\0" as *const u8 as *const libc::c_char,
            s_179,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_180: S = ts(
        tp(
            tc(
                b"120\0" as *const u8 as *const libc::c_char as S,
                b"fac:{[n] :[n=0;1;n*fac[n-1]]}; fac 5\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            291 as libc::c_int,
            b"120\0" as *const u8 as *const libc::c_char,
            b"fac:{[n] :[n=0;1;n*fac[n-1]]}; fac 5\0" as *const u8
                as *const libc::c_char,
            s_180,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_181: S = ts(
        tp(
            tc(
                b"120\0" as *const u8 as *const libc::c_char as S,
                b"fac:{[n] :[n=0;1;n*_f n-1]}; fac 5\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int,
            b"120\0" as *const u8 as *const libc::c_char,
            b"fac:{[n] :[n=0;1;n*_f n-1]}; fac 5\0" as *const u8 as *const libc::c_char,
            s_181,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_182: S = ts(
        tp(
            tc(
                b"120\0" as *const u8 as *const libc::c_char as S,
                b"fac:{[n] :[n=0;1;n*_f[n-1]]}; fac 5\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int,
            b"120\0" as *const u8 as *const libc::c_char,
            b"fac:{[n] :[n=0;1;n*_f[n-1]]}; fac 5\0" as *const u8 as *const libc::c_char,
            s_182,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_183: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"a:3;a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"a:3;a\0" as *const u8 as *const libc::c_char,
            s_183,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_184: S = ts(
        tp(
            tc(
                b"\"abcdefghij\"2 3\0" as *const u8 as *const libc::c_char as S,
                b"\"cd\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int,
            b"\"abcdefghij\"2 3\0" as *const u8 as *const libc::c_char,
            b"\"cd\"\0" as *const u8 as *const libc::c_char,
            s_184,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_185: S = ts(
        tp(
            tc(
                b",1 1\0" as *const u8 as *const libc::c_char as S,
                b"0(+\\)\\1 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int,
            b",1 1\0" as *const u8 as *const libc::c_char,
            b"0(+\\)\\1 1\0" as *const u8 as *const libc::c_char,
            s_185,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_186: S = ts(
        tp(
            tc(
                b"+[a+2;a:3]\0" as *const u8 as *const libc::c_char as S,
                b"8\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            297 as libc::c_int,
            b"+[a+2;a:3]\0" as *const u8 as *const libc::c_char,
            b"8\0" as *const u8 as *const libc::c_char,
            s_186,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_187: S = ts(
        tp(
            tc(
                b"(a;a:2)\0" as *const u8 as *const libc::c_char as S,
                b"2 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            298 as libc::c_int,
            b"(a;a:2)\0" as *const u8 as *const libc::c_char,
            b"2 2\0" as *const u8 as *const libc::c_char,
            s_187,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_188: S = ts(
        tp(
            tc(
                b"\"F\"\0" as *const u8 as *const libc::c_char as S,
                b"tk:(\"F\";\"G\");call:{nm:*tk; tk::1 _ tk;if[nm~\"G\"; :0];call();nm};call()\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            299 as libc::c_int,
            b"\"F\"\0" as *const u8 as *const libc::c_char,
            b"tk:(\"F\";\"G\");call:{nm:*tk; tk::1 _ tk;if[nm~\"G\"; :0];call();nm};call()\0"
                as *const u8 as *const libc::c_char,
            s_188,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_189: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"(a)=a:12\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"(a)=a:12\0" as *const u8 as *const libc::c_char,
            s_189,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_190: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"{a}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"{a}0\0" as *const u8 as *const libc::c_char,
            s_190,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_191: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"{a x}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"{a x}0\0" as *const u8 as *const libc::c_char,
            s_191,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_192: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"f:{:[x>0;2*f[x-1];1]}; g:f; f:{0}; g 4\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"f:{:[x>0;2*f[x-1];1]}; g:f; f:{0}; g 4\0" as *const u8
                as *const libc::c_char,
            s_192,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_193: S = ts(
        tp(
            tc(
                b"{p[`a]:1;do[1;p[`b]:2];p}[]\0" as *const u8 as *const libc::c_char
                    as S,
                b".((`a;1;);(`b;2;))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            304 as libc::c_int,
            b"{p[`a]:1;do[1;p[`b]:2];p}[]\0" as *const u8 as *const libc::c_char,
            b".((`a;1;);(`b;2;))\0" as *const u8 as *const libc::c_char,
            s_193,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_194: S = ts(
        tp(
            tc(
                b"1 1\0" as *const u8 as *const libc::c_char as S,
                b"ds:{-':x}; a: 1 2 3; ds a; ds a\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            305 as libc::c_int,
            b"1 1\0" as *const u8 as *const libc::c_char,
            b"ds:{-':x}; a: 1 2 3; ds a; ds a\0" as *const u8 as *const libc::c_char,
            s_194,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_195: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"g:{a:x; {:[x=0; a:x; g(0)]}a; a}; g 2\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            306 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"g:{a:x; {:[x=0; a:x; g(0)]}a; a}; g 2\0" as *const u8
                as *const libc::c_char,
            s_195,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_196: S = ts(
        tp(
            tc(
                b"9\0" as *const u8 as *const libc::c_char as S,
                b"c:0; f:{c+:1;:[x;:[y;f[x-1;f[x;y-1]];f[x-1;1]];y+1]}; f[2;3]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            307 as libc::c_int,
            b"9\0" as *const u8 as *const libc::c_char,
            b"c:0; f:{c+:1;:[x;:[y;f[x-1;f[x;y-1]];f[x-1;1]];y+1]}; f[2;3]\0"
                as *const u8 as *const libc::c_char,
            s_196,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_197: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"(.((`a;1);(`b;2)))(`a)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"(.((`a;1);(`b;2)))(`a)\0" as *const u8 as *const libc::c_char,
            s_197,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_198: S = ts(
        tp(
            tc(
                b"(,1)\0" as *const u8 as *const libc::c_char as S,
                b"(.((`a;1);(`b;2)))(,`a)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            309 as libc::c_int,
            b"(,1)\0" as *const u8 as *const libc::c_char,
            b"(.((`a;1);(`b;2)))(,`a)\0" as *const u8 as *const libc::c_char,
            s_198,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_199: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b":+[2]3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b":+[2]3\0" as *const u8 as *const libc::c_char,
            s_199,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_200: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b":+2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            311 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b":+2\0" as *const u8 as *const libc::c_char,
            s_200,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_201: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"f:{ if[x;:+[x]y]; y }; f[2;3]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"f:{ if[x;:+[x]y]; y }; f[2;3]\0" as *const u8 as *const libc::c_char,
            s_201,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_202: S = ts(
        tp(
            tc(
                b"-1\0" as *const u8 as *const libc::c_char as S,
                b"4: (.((`a;1);(`b;2)))(,`a)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            313 as libc::c_int,
            b"-1\0" as *const u8 as *const libc::c_char,
            b"4: (.((`a;1);(`b;2)))(,`a)\0" as *const u8 as *const libc::c_char,
            s_202,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_203: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"(1 2 1)\\1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            314 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"(1 2 1)\\1\0" as *const u8 as *const libc::c_char,
            s_203,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_204: S = ts(
        tp(
            tc(
                b"12 6 3\0" as *const u8 as *const libc::c_char as S,
                b"0 1 1 3 2 5 3 7 4 9 5 11 6\\ 12\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            315 as libc::c_int,
            b"12 6 3\0" as *const u8 as *const libc::c_char,
            b"0 1 1 3 2 5 3 7 4 9 5 11 6\\ 12\0" as *const u8 as *const libc::c_char,
            s_204,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_205: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"d:.((`a;0);(`b;1)); `d .`b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"d:.((`a;0);(`b;1)); `d .`b\0" as *const u8 as *const libc::c_char,
            s_205,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_206: S = ts(
        tp(
            tc(
                b".[;;;;]\0" as *const u8 as *const libc::c_char as S,
                b".[;;;;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int,
            b".[;;;;]\0" as *const u8 as *const libc::c_char,
            b".[;;;;]\0" as *const u8 as *const libc::c_char,
            s_206,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_207: S = ts(
        tp(
            tc(
                b".[]\0" as *const u8 as *const libc::c_char as S,
                b".[]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            318 as libc::c_int,
            b".[]\0" as *const u8 as *const libc::c_char,
            b".[]\0" as *const u8 as *const libc::c_char,
            s_207,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_208: S = ts(
        tp(
            tc(
                b"(1 3;2 4)\0" as *const u8 as *const libc::c_char as S,
                b"1 2 ,'' 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            319 as libc::c_int,
            b"(1 3;2 4)\0" as *const u8 as *const libc::c_char,
            b"1 2 ,'' 3 4\0" as *const u8 as *const libc::c_char,
            s_208,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_209: S = ts(
        tp(
            tc(
                b"(1 3;2 4)\0" as *const u8 as *const libc::c_char as S,
                b"{x,''y} . (1 2;3 4)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            320 as libc::c_int,
            b"(1 3;2 4)\0" as *const u8 as *const libc::c_char,
            b"{x,''y} . (1 2;3 4)\0" as *const u8 as *const libc::c_char,
            s_209,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_210: S = ts(
        tp(
            tc(
                b"((1 3;1 4);(2 3;2 4))\0" as *const u8 as *const libc::c_char as S,
                b"(,/:\\:) . (1 2;3 4)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int,
            b"((1 3;1 4);(2 3;2 4))\0" as *const u8 as *const libc::c_char,
            b"(,/:\\:) . (1 2;3 4)\0" as *const u8 as *const libc::c_char,
            s_210,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_211: S = ts(
        tp(
            tc(
                b"(1 2;4 3;5 4)\0" as *const u8 as *const libc::c_char as S,
                b".\"1 2 ,': 3 4 5\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int,
            b"(1 2;4 3;5 4)\0" as *const u8 as *const libc::c_char,
            b".\"1 2 ,': 3 4 5\"\0" as *const u8 as *const libc::c_char,
            s_211,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_212: S = ts(
        tp(
            tc(
                b"(1 2;4 3;5 4)\0" as *const u8 as *const libc::c_char as S,
                b".\"{x,':y} . (1 2;3 4 5)\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            323 as libc::c_int,
            b"(1 2;4 3;5 4)\0" as *const u8 as *const libc::c_char,
            b".\"{x,':y} . (1 2;3 4 5)\"\0" as *const u8 as *const libc::c_char,
            s_212,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_213: S = ts(
        tp(
            tc(
                b"(1 2;4 3;5 4)\0" as *const u8 as *const libc::c_char as S,
                b".\"(,':) . (1 2;3 4 5)\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int,
            b"(1 2;4 3;5 4)\0" as *const u8 as *const libc::c_char,
            b".\"(,':) . (1 2;3 4 5)\"\0" as *const u8 as *const libc::c_char,
            s_213,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_214: S = ts(
        tp(
            tc(
                b".[*; (3;4); :]\0" as *const u8 as *const libc::c_char as S,
                b"(0;12)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            327 as libc::c_int,
            b".[*; (3;4); :]\0" as *const u8 as *const libc::c_char,
            b"(0;12)\0" as *const u8 as *const libc::c_char,
            s_214,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_215: S = ts(
        tp(
            tc(
                b".[*;3 4 5;:]\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"valence\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            328 as libc::c_int,
            b".[*;3 4 5;:]\0" as *const u8 as *const libc::c_char,
            b"(1;\"valence\")\0" as *const u8 as *const libc::c_char,
            s_215,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_216: S = ts(
        tp(
            tc(
                b"(.[-1 -2 _; ,!9; :])\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"domain\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int,
            b"(.[-1 -2 _; ,!9; :])\0" as *const u8 as *const libc::c_char,
            b"(1;\"domain\")\0" as *const u8 as *const libc::c_char,
            s_216,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_217: S = ts(
        tp(
            tc(
                b"(.[0 10 100 _; ,!9; :])\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"length\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_int,
            b"(.[0 10 100 _; ,!9; :])\0" as *const u8 as *const libc::c_char,
            b"(1;\"length\")\0" as *const u8 as *const libc::c_char,
            s_217,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_218: S = ts(
        tp(
            tc(
                b"(.[.:;,\"{_foo[x]}\";:])\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"reserved\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            332 as libc::c_int,
            b"(.[.:;,\"{_foo[x]}\";:])\0" as *const u8 as *const libc::c_char,
            b"(1;\"reserved\")\0" as *const u8 as *const libc::c_char,
            s_218,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_219: S = ts(
        tp(
            tc(
                b"(.[.:;,\"{_vs.a[x]}\";:])\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            333 as libc::c_int,
            b"(.[.:;,\"{_vs.a[x]}\";:])\0" as *const u8 as *const libc::c_char,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            s_219,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_220: S = ts(
        tp(
            tc(
                b"(.[.:;,\"{_vs.abc[x]}\";:])\0" as *const u8 as *const libc::c_char
                    as S,
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            334 as libc::c_int,
            b"(.[.:;,\"{_vs.abc[x]}\";:])\0" as *const u8 as *const libc::c_char,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            s_220,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_221: S = ts(
        tp(
            tc(
                b"(.[.:;,\"{_vs_a[x]}\";:])\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_int,
            b"(.[.:;,\"{_vs_a[x]}\";:])\0" as *const u8 as *const libc::c_char,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            s_221,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_222: S = ts(
        tp(
            tc(
                b"(@[.:;\"{_foo[x]} 0\";:])\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"reserved\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int,
            b"(@[.:;\"{_foo[x]} 0\";:])\0" as *const u8 as *const libc::c_char,
            b"(1;\"reserved\")\0" as *const u8 as *const libc::c_char,
            s_222,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_223: S = ts(
        tp(
            tc(
                b"a:1+`d\0" as *const u8 as *const libc::c_char as S,
                b"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            337 as libc::c_int,
            b"a:1+`d\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            s_223,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_224: S = ts(
        tp(
            tc(
                b"-9131\0" as *const u8 as *const libc::c_char as S,
                b"_jd 20100101\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int,
            b"-9131\0" as *const u8 as *const libc::c_char,
            b"_jd 20100101\0" as *const u8 as *const libc::c_char,
            s_224,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_225: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"(::)[1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            342 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"(::)[1]\0" as *const u8 as *const libc::c_char,
            s_225,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_226: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"(:)[0;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            343 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"(:)[0;2]\0" as *const u8 as *const libc::c_char,
            s_226,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_227: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b":[1;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b":[1;]\0" as *const u8 as *const libc::c_char,
            s_227,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_228: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b":[0;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b":[0;]\0" as *const u8 as *const libc::c_char,
            s_228,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_229: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b":[0;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b":[0;1]\0" as *const u8 as *const libc::c_char,
            s_229,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_230: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b":[1;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            349 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b":[1;1]\0" as *const u8 as *const libc::c_char,
            s_230,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_231: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b":[0;1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            350 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b":[0;1;2]\0" as *const u8 as *const libc::c_char,
            s_231,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_232: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b":[1;2;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            351 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b":[1;2;3]\0" as *const u8 as *const libc::c_char,
            s_232,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_233: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b":[1+1;2;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            352 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b":[1+1;2;3]\0" as *const u8 as *const libc::c_char,
            s_233,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_234: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b":[0;2;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            353 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b":[0;2;3]\0" as *const u8 as *const libc::c_char,
            s_234,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_235: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b":[1;2;3;4;5;6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            354 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b":[1;2;3;4;5;6]\0" as *const u8 as *const libc::c_char,
            s_235,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_236: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b":[0;1;0;3;0;5;1;7;0;9;10]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            355 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b":[0;1;0;3;0;5;1;7;0;9;10]\0" as *const u8 as *const libc::c_char,
            s_236,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_237: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b":[0+0;1;1-1;2;1;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            356 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b":[0+0;1;1-1;2;1;3]\0" as *const u8 as *const libc::c_char,
            s_237,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_238: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b":[0+0;1;1-1;2;-1+1;3;4]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            357 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b":[0+0;1;1-1;2;-1+1;3;4]\0" as *const u8 as *const libc::c_char,
            s_238,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_239: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b":[0;;0;;0;;1;2;0;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            358 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b":[0;;0;;0;;1;2;0;]\0" as *const u8 as *const libc::c_char,
            s_239,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_240: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b":[0;;0;;0;;0;2;0;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b":[0;;0;;0;;0;2;0;]\0" as *const u8 as *const libc::c_char,
            s_240,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_241: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b":[0;;0;;0;;0;2;0;;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b":[0;;0;;0;;0;2;0;;]\0" as *const u8 as *const libc::c_char,
            s_241,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_242: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b":[0;;0;;0;;0;2;0;; 1 ]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            361 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b":[0;;0;;0;;0;2;0;; 1 ]\0" as *const u8 as *const libc::c_char,
            s_242,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_243: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b":[,0;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            362 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b":[,0;1]\0" as *const u8 as *const libc::c_char,
            s_243,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_244: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b":[,1;1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            363 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b":[,1;1;2]\0" as *const u8 as *const libc::c_char,
            s_244,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_245: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b":[,,0;1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            364 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b":[,,0;1;2]\0" as *const u8 as *const libc::c_char,
            s_245,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_246: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b":[,,1;1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            365 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b":[,,1;1;2]\0" as *const u8 as *const libc::c_char,
            s_246,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_247: S = ts(
        tp(
            tc(
                b"(1;\"type\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\":[,!0;1;2]\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            366 as libc::c_int,
            b"(1;\"type\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\":[,!0;1;2]\";:]\0" as *const u8 as *const libc::c_char,
            s_247,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_248: S = ts(
        tp(
            tc(
                b"2 3\0" as *const u8 as *const libc::c_char as S,
                b"f:{[a] :[a;2;3]}; (f 1;f 0)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            367 as libc::c_int,
            b"2 3\0" as *const u8 as *const libc::c_char,
            b"f:{[a] :[a;2;3]}; (f 1;f 0)\0" as *const u8 as *const libc::c_char,
            s_248,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_249: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"parse\"),:[1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            368 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"(1;\"parse\"),:[1]\0" as *const u8 as *const libc::c_char,
            s_249,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_250: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"if[3<4;a:20]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            369 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"if[3<4;a:20]\0" as *const u8 as *const libc::c_char,
            s_250,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_251: S = ts(
        tp(
            tc(
                b"40\0" as *const u8 as *const libc::c_char as S,
                b"a:10;if[5<6;a:40];a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            370 as libc::c_int,
            b"40\0" as *const u8 as *const libc::c_char,
            b"a:10;if[5<6;a:40];a\0" as *const u8 as *const libc::c_char,
            s_251,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_252: S = ts(
        tp(
            tc(
                b"30\0" as *const u8 as *const libc::c_char as S,
                b"a:30;if[5>6;a:20];a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            371 as libc::c_int,
            b"30\0" as *const u8 as *const libc::c_char,
            b"a:30;if[5>6;a:20];a\0" as *const u8 as *const libc::c_char,
            s_252,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_253: S = ts(
        tp(
            tc(
                b"30\0" as *const u8 as *const libc::c_char as S,
                b"a:10;if[1;a:20;a:30;];a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            372 as libc::c_int,
            b"30\0" as *const u8 as *const libc::c_char,
            b"a:10;if[1;a:20;a:30;];a\0" as *const u8 as *const libc::c_char,
            s_253,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_254: S = ts(
        tp(
            tc(
                b"20\0" as *const u8 as *const libc::c_char as S,
                b"a:10;if[2>,1;a:20];a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            373 as libc::c_int,
            b"20\0" as *const u8 as *const libc::c_char,
            b"a:10;if[2>,1;a:20];a\0" as *const u8 as *const libc::c_char,
            s_254,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_255: S = ts(
        tp(
            tc(
                b"20\0" as *const u8 as *const libc::c_char as S,
                b"a:10;if[2>,,,1;a:20];a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            374 as libc::c_int,
            b"20\0" as *const u8 as *const libc::c_char,
            b"a:10;if[2>,,,1;a:20];a\0" as *const u8 as *const libc::c_char,
            s_255,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_256: S = ts(
        tp(
            tc(
                b"(1;\"type\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"a:10;if[,!0;a:20];a\";:]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            375 as libc::c_int,
            b"(1;\"type\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"a:10;if[,!0;a:20];a\";:]\0" as *const u8 as *const libc::c_char,
            s_256,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_257: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"i:0;while[7>i;0+0;i+:1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            376 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"i:0;while[7>i;0+0;i+:1]\0" as *const u8 as *const libc::c_char,
            s_257,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_258: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"i:0;while[7>i;0+0;i+:1];i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            377 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"i:0;while[7>i;0+0;i+:1];i\0" as *const u8 as *const libc::c_char,
            s_258,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_259: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"i:0;while[5>i+:1];i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            378 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"i:0;while[5>i+:1];i\0" as *const u8 as *const libc::c_char,
            s_259,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_260: S = ts(
        tp(
            tc(
                b",5\0" as *const u8 as *const libc::c_char as S,
                b"i:,0;while[5>i+:1];i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            379 as libc::c_int,
            b",5\0" as *const u8 as *const libc::c_char,
            b"i:,0;while[5>i+:1];i\0" as *const u8 as *const libc::c_char,
            s_260,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_261: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"do[5;0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            382 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"do[5;0]\0" as *const u8 as *const libc::c_char,
            s_261,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_262: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"i:0;do[7;0+0;i+:1;];i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            383 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"i:0;do[7;0+0;i+:1;];i\0" as *const u8 as *const libc::c_char,
            s_262,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_263: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"i:0;do[,5;i+:1];i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"i:0;do[,5;i+:1];i\0" as *const u8 as *const libc::c_char,
            s_263,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_264: S = ts(
        tp(
            tc(
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"\\b\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"\\b\";:]\0" as *const u8 as *const libc::c_char,
            s_264,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_265: S = ts(
        tp(
            tc(
                b"51\0" as *const u8 as *const libc::c_char as S,
                b"a:101#\"1+\"; . a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            b"51\0" as *const u8 as *const libc::c_char,
            b"a:101#\"1+\"; . a\0" as *const u8 as *const libc::c_char,
            s_265,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_266: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"(+/)[1 2 3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            391 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"(+/)[1 2 3]\0" as *const u8 as *const libc::c_char,
            s_266,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_267: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"(+/)[1; 1 2 3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            392 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"(+/)[1; 1 2 3]\0" as *const u8 as *const libc::c_char,
            s_267,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_268: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"~0.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            394 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"~0.0\0" as *const u8 as *const libc::c_char,
            s_268,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_269: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"~ -0.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            395 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"~ -0.0\0" as *const u8 as *const libc::c_char,
            s_269,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_270: S = ts(
        tp(
            tc(
                b"(1;\"valence\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\",\\\\: 1 2 3\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            397 as libc::c_int,
            b"(1;\"valence\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\",\\\\: 1 2 3\";:]\0" as *const u8 as *const libc::c_char,
            s_270,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_271: S = ts(
        tp(
            tc(
                b"((0 2;0 3);(1 2;1 3))\0" as *const u8 as *const libc::c_char as S,
                b"0 1 ,/:\\:2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            399 as libc::c_int,
            b"((0 2;0 3);(1 2;1 3))\0" as *const u8 as *const libc::c_char,
            b"0 1 ,/:\\:2 3\0" as *const u8 as *const libc::c_char,
            s_271,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_272: S = ts(
        tp(
            tc(
                b"((0 2;0 3);(1 2;1 3))\0" as *const u8 as *const libc::c_char as S,
                b"0 1 ,/:\\:\\:2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            400 as libc::c_int,
            b"((0 2;0 3);(1 2;1 3))\0" as *const u8 as *const libc::c_char,
            b"0 1 ,/:\\:\\:2 3\0" as *const u8 as *const libc::c_char,
            s_272,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_273: S = ts(
        tp(
            tc(
                b"((0 2;1 2);(0 3;1 3))\0" as *const u8 as *const libc::c_char as S,
                b"0 1 ,\\:/: 2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            402 as libc::c_int,
            b"((0 2;1 2);(0 3;1 3))\0" as *const u8 as *const libc::c_char,
            b"0 1 ,\\:/: 2 3\0" as *const u8 as *const libc::c_char,
            s_273,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_274: S = ts(
        tp(
            tc(
                b"((0;1;2 3;4 5);(0;1;6 7;8 9))\0" as *const u8 as *const libc::c_char
                    as S,
                b"0 1 ,/: ((2 3;4 5);(6 7;8 9))\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            403 as libc::c_int,
            b"((0;1;2 3;4 5);(0;1;6 7;8 9))\0" as *const u8 as *const libc::c_char,
            b"0 1 ,/: ((2 3;4 5);(6 7;8 9))\0" as *const u8 as *const libc::c_char,
            s_274,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_275: S = ts(
        tp(
            tc(
                b"((0 1 2 3;0 1 4 5);(0 1 6 7; 0 1 8 9))\0" as *const u8
                    as *const libc::c_char as S,
                b"0 1 ,/:/: ((2 3;4 5);(6 7;8 9))\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            404 as libc::c_int,
            b"((0 1 2 3;0 1 4 5);(0 1 6 7; 0 1 8 9))\0" as *const u8
                as *const libc::c_char,
            b"0 1 ,/:/: ((2 3;4 5);(6 7;8 9))\0" as *const u8 as *const libc::c_char,
            s_275,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_276: S = ts(
        tp(
            tc(
                b"(((0 1 2;0 1 3);(0 1 4; 0 1 5));((0 1 6; 0 1 7);(0 1 8; 0 1 9)))\0"
                    as *const u8 as *const libc::c_char as S,
                b"0 1 ,/:/:/: ((2 3;4 5);(6 7;8 9))\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            405 as libc::c_int,
            b"(((0 1 2;0 1 3);(0 1 4; 0 1 5));((0 1 6; 0 1 7);(0 1 8; 0 1 9)))\0"
                as *const u8 as *const libc::c_char,
            b"0 1 ,/:/:/: ((2 3;4 5);(6 7;8 9))\0" as *const u8 as *const libc::c_char,
            s_276,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_277: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:(1;1.0;\"c\";`d;1 2;3.0 4.0;\"ef\";`g`h;();(1;`z)); &/{x~_db _bd x}'a,,a\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:(1;1.0;\"c\";`d;1 2;3.0 4.0;\"ef\";`g`h;();(1;`z)); &/{x~_db _bd x}'a,,a\0"
                as *const u8 as *const libc::c_char,
            s_277,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_278: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:(!11)#\\:`a`b; &/{x~_db _bd x}'a\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            409 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:(!11)#\\:`a`b; &/{x~_db _bd x}'a\0" as *const u8 as *const libc::c_char,
            s_278,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_279: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:(!11)#\\:\"cd\"; &/{x~_db _bd x}'a\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            410 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:(!11)#\\:\"cd\"; &/{x~_db _bd x}'a\0" as *const u8
                as *const libc::c_char,
            s_279,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_280: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:(!11)#\\:(`a;\"c\";1); &/{x~_db _bd x}'a\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            411 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:(!11)#\\:(`a;\"c\";1); &/{x~_db _bd x}'a\0" as *const u8
                as *const libc::c_char,
            s_280,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_281: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:(!11)#\\:,(`a;\"c\";1); &/{x~_db _bd x}'a\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            412 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:(!11)#\\:,(`a;\"c\";1); &/{x~_db _bd x}'a\0" as *const u8
                as *const libc::c_char,
            s_281,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_282: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"_db_bd_db_bd 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            414 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"_db_bd_db_bd 2\0" as *const u8 as *const libc::c_char,
            s_282,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_283: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"1,&/{x~_db _bd x}/: (+;+:;-;-:)\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            416 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"1,&/{x~_db _bd x}/: (+;+:;-;-:)\0" as *const u8 as *const libc::c_char,
            s_283,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_284: S = ts(
        tp(
            tc(
                b"0 2\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"1+1\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            418 as libc::c_int,
            b"0 2\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"1+1\";:]\0" as *const u8 as *const libc::c_char,
            s_284,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_285: S = ts(
        tp(
            tc(
                b"32\0" as *const u8 as *const libc::c_char as S,
                b"(+/ *)[1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            420 as libc::c_int,
            b"32\0" as *const u8 as *const libc::c_char,
            b"(+/ *)[1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char,
            s_285,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_286: S = ts(
        tp(
            tc(
                b"32\0" as *const u8 as *const libc::c_char as S,
                b"1 2 3 _dot 4 5 6\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            b"32\0" as *const u8 as *const libc::c_char,
            b"1 2 3 _dot 4 5 6\0" as *const u8 as *const libc::c_char,
            s_286,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_287: S = ts(
        tp(
            tc(
                b"(0.5 0 0;0 0.5 0;0 0 0.5)\0" as *const u8 as *const libc::c_char as S,
                b"_inv (2 0 0;0 2 0; 0 0 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            423 as libc::c_int,
            b"(0.5 0 0;0 0.5 0;0 0 0.5)\0" as *const u8 as *const libc::c_char,
            b"_inv (2 0 0;0 2 0; 0 0 2)\0" as *const u8 as *const libc::c_char,
            s_287,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_288: S = ts(
        tp(
            tc(
                b"(0.1 0 0;0 0.1 0;0 0 0.1)\0" as *const u8 as *const libc::c_char as S,
                b"_inv (10 0 0;0 10 0; 0 0 10)\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            424 as libc::c_int,
            b"(0.1 0 0;0 0.1 0;0 0 0.1)\0" as *const u8 as *const libc::c_char,
            b"_inv (10 0 0;0 10 0; 0 0 10)\0" as *const u8 as *const libc::c_char,
            s_288,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_289: S = ts(
        tp(
            tc(
                b"400 400\0" as *const u8 as *const libc::c_char as S,
                b"10 10 _mul 20 20\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            425 as libc::c_int,
            b"400 400\0" as *const u8 as *const libc::c_char,
            b"10 10 _mul 20 20\0" as *const u8 as *const libc::c_char,
            s_289,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_290: S = ts(
        tp(
            tc(
                b"13\0" as *const u8 as *const libc::c_char as S,
                b"11(_+)\\2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            428 as libc::c_int,
            b"13\0" as *const u8 as *const libc::c_char,
            b"11(_+)\\2\0" as *const u8 as *const libc::c_char,
            s_290,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_291: S = ts(
        tp(
            tc(
                b"11 13 16\0" as *const u8 as *const libc::c_char as S,
                b"11(_+)\\2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            429 as libc::c_int,
            b"11 13 16\0" as *const u8 as *const libc::c_char,
            b"11(_+)\\2 3\0" as *const u8 as *const libc::c_char,
            s_291,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_292: S = ts(
        tp(
            tc(
                b"2010\0" as *const u8 as *const libc::c_char as S,
                b"10 _sv 2 0 1 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            b"2010\0" as *const u8 as *const libc::c_char,
            b"10 _sv 2 0 1 0\0" as *const u8 as *const libc::c_char,
            s_292,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_293: S = ts(
        tp(
            tc(
                b"11\0" as *const u8 as *const libc::c_char as S,
                b"2 _sv 1 0 1 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            431 as libc::c_int,
            b"11\0" as *const u8 as *const libc::c_char,
            b"2 _sv 1 0 1 1\0" as *const u8 as *const libc::c_char,
            s_293,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_294: S = ts(
        tp(
            tc(
                b"1 2 3 4 _bin/: 1 2 3 4 5 0 0.5 1.5 2.5 3.5 4.5\0" as *const u8
                    as *const libc::c_char as S,
                b"0 1 2 3 4 0 0 1 2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            433 as libc::c_int,
            b"1 2 3 4 _bin/: 1 2 3 4 5 0 0.5 1.5 2.5 3.5 4.5\0" as *const u8
                as *const libc::c_char,
            b"0 1 2 3 4 0 0 1 2 3 4\0" as *const u8 as *const libc::c_char,
            s_294,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_295: S = ts(
        tp(
            tc(
                b"4:.\"\\\\p\"\0" as *const u8 as *const libc::c_char as S,
                b"1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            437 as libc::c_int,
            b"4:.\"\\\\p\"\0" as *const u8 as *const libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char,
            s_295,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_296: S = ts(
        tp(
            tc(
                b"3999\0" as *const u8 as *const libc::c_char as S,
                b"#5:2000#1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int,
            b"3999\0" as *const u8 as *const libc::c_char,
            b"#5:2000#1\0" as *const u8 as *const libc::c_char,
            s_296,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_297: S = ts(
        tp(
            tc(
                b"5:(+)\0" as *const u8 as *const libc::c_char as S,
                b"(,\"+\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            442 as libc::c_int,
            b"5:(+)\0" as *const u8 as *const libc::c_char,
            b"(,\"+\")\0" as *const u8 as *const libc::c_char,
            s_297,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_298: S = ts(
        tp(
            tc(
                b"5:(|/)\0" as *const u8 as *const libc::c_char as S,
                b"\"|/\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            443 as libc::c_int,
            b"5:(|/)\0" as *const u8 as *const libc::c_char,
            b"\"|/\"\0" as *const u8 as *const libc::c_char,
            s_298,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_299: S = ts(
        tp(
            tc(
                b"5:(_acos;_tanh;_abs;_size;_bin;_vsx;_ssr;_vs)\0" as *const u8
                    as *const libc::c_char as S,
                b"\"(_acos;_tanh;_abs;_size;_bin;_vsx;_ssr;_vs)\"\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            444 as libc::c_int,
            b"5:(_acos;_tanh;_abs;_size;_bin;_vsx;_ssr;_vs)\0" as *const u8
                as *const libc::c_char,
            b"\"(_acos;_tanh;_abs;_size;_bin;_vsx;_ssr;_vs)\"\0" as *const u8
                as *const libc::c_char,
            s_299,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_300: S = ts(
        tp(
            tc(
                b"3 3#!0\0" as *const u8 as *const libc::c_char as S,
                b"(0 0 0;0 0 0; 0 0 0)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            447 as libc::c_int,
            b"3 3#!0\0" as *const u8 as *const libc::c_char,
            b"(0 0 0;0 0 0; 0 0 0)\0" as *const u8 as *const libc::c_char,
            s_300,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_301: S = ts(
        tp(
            tc(
                b"3 3#0#0.0\0" as *const u8 as *const libc::c_char as S,
                b"(0 0 0.0;0 0 0.0; 0 0 0.0)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            448 as libc::c_int,
            b"3 3#0#0.0\0" as *const u8 as *const libc::c_char,
            b"(0 0 0.0;0 0 0.0; 0 0 0.0)\0" as *const u8 as *const libc::c_char,
            s_301,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_302: S = ts(
        tp(
            tc(
                b"2 2#0#\"\"\0" as *const u8 as *const libc::c_char as S,
                b"(\"  \";\"  \")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            449 as libc::c_int,
            b"2 2#0#\"\"\0" as *const u8 as *const libc::c_char,
            b"(\"  \";\"  \")\0" as *const u8 as *const libc::c_char,
            s_302,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_303: S = ts(
        tp(
            tc(
                b"1 1#0#`\0" as *const u8 as *const libc::c_char as S,
                b",,`\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            450 as libc::c_int,
            b"1 1#0#`\0" as *const u8 as *const libc::c_char,
            b",,`\0" as *const u8 as *const libc::c_char,
            s_303,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_304: S = ts(
        tp(
            tc(
                b"2 2#()\0" as *const u8 as *const libc::c_char as S,
                b"((;);(;))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            451 as libc::c_int,
            b"2 2#()\0" as *const u8 as *const libc::c_char,
            b"((;);(;))\0" as *const u8 as *const libc::c_char,
            s_304,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_305: S = ts(
        tp(
            tc(
                b"{x+y}/1 2 3 4\0" as *const u8 as *const libc::c_char as S,
                b"10\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            453 as libc::c_int,
            b"{x+y}/1 2 3 4\0" as *const u8 as *const libc::c_char,
            b"10\0" as *const u8 as *const libc::c_char,
            s_305,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_306: S = ts(
        tp(
            tc(
                b"!0\0" as *const u8 as *const libc::c_char as S,
                b"2 _vs 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            456 as libc::c_int,
            b"!0\0" as *const u8 as *const libc::c_char,
            b"2 _vs 0\0" as *const u8 as *const libc::c_char,
            s_306,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_307: S = ts(
        tp(
            tc(
                b"(,0)\0" as *const u8 as *const libc::c_char as S,
                b"2 _vsx 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            458 as libc::c_int,
            b"(,0)\0" as *const u8 as *const libc::c_char,
            b"2 _vsx 0\0" as *const u8 as *const libc::c_char,
            s_307,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_308: S = ts(
        tp(
            tc(
                b"(,1)\0" as *const u8 as *const libc::c_char as S,
                b"2 _vs 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            459 as libc::c_int,
            b"(,1)\0" as *const u8 as *const libc::c_char,
            b"2 _vs 1\0" as *const u8 as *const libc::c_char,
            s_308,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_309: S = ts(
        tp(
            tc(
                b"(,1)\0" as *const u8 as *const libc::c_char as S,
                b"2 _vsx 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            460 as libc::c_int,
            b"(,1)\0" as *const u8 as *const libc::c_char,
            b"2 _vsx 1\0" as *const u8 as *const libc::c_char,
            s_309,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_310: S = ts(
        tp(
            tc(
                b"1 0\0" as *const u8 as *const libc::c_char as S,
                b"2 _vs 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_int,
            b"1 0\0" as *const u8 as *const libc::c_char,
            b"2 _vs 2\0" as *const u8 as *const libc::c_char,
            s_310,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_311: S = ts(
        tp(
            tc(
                b"1 0\0" as *const u8 as *const libc::c_char as S,
                b"2 _vsx 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            462 as libc::c_int,
            b"1 0\0" as *const u8 as *const libc::c_char,
            b"2 _vsx 2\0" as *const u8 as *const libc::c_char,
            s_311,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_312: S = ts(
        tp(
            tc(
                b"1 0 1 1\0" as *const u8 as *const libc::c_char as S,
                b"2 _vs 11\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            463 as libc::c_int,
            b"1 0 1 1\0" as *const u8 as *const libc::c_char,
            b"2 _vs 11\0" as *const u8 as *const libc::c_char,
            s_312,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_313: S = ts(
        tp(
            tc(
                b"1 0 1 1\0" as *const u8 as *const libc::c_char as S,
                b"2 _vsx 11\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            464 as libc::c_int,
            b"1 0 1 1\0" as *const u8 as *const libc::c_char,
            b"2 _vsx 11\0" as *const u8 as *const libc::c_char,
            s_313,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_314: S = ts(
        tp(
            tc(
                b"1 1 0 0\0" as *const u8 as *const libc::c_char as S,
                b"2 _vs 12\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            465 as libc::c_int,
            b"1 1 0 0\0" as *const u8 as *const libc::c_char,
            b"2 _vs 12\0" as *const u8 as *const libc::c_char,
            s_314,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_315: S = ts(
        tp(
            tc(
                b"1 1 0 0\0" as *const u8 as *const libc::c_char as S,
                b"2 _vsx 12\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            466 as libc::c_int,
            b"1 1 0 0\0" as *const u8 as *const libc::c_char,
            b"2 _vsx 12\0" as *const u8 as *const libc::c_char,
            s_315,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_316: S = ts(
        tp(
            tc(
                b"2 0 1 0\0" as *const u8 as *const libc::c_char as S,
                b"10 _vs 2010\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            467 as libc::c_int,
            b"2 0 1 0\0" as *const u8 as *const libc::c_char,
            b"10 _vs 2010\0" as *const u8 as *const libc::c_char,
            s_316,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_317: S = ts(
        tp(
            tc(
                b"2 0 1 0\0" as *const u8 as *const libc::c_char as S,
                b"10 _vsx 2010\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            468 as libc::c_int,
            b"2 0 1 0\0" as *const u8 as *const libc::c_char,
            b"10 _vsx 2010\0" as *const u8 as *const libc::c_char,
            s_317,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_318: S = ts(
        tp(
            tc(
                b"1 2 3 4 5\0" as *const u8 as *const libc::c_char as S,
                b"10 _vs 12345\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            469 as libc::c_int,
            b"1 2 3 4 5\0" as *const u8 as *const libc::c_char,
            b"10 _vs 12345\0" as *const u8 as *const libc::c_char,
            s_318,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_319: S = ts(
        tp(
            tc(
                b"1 2 3 4 5\0" as *const u8 as *const libc::c_char as S,
                b"10 _vsx 12345\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            470 as libc::c_int,
            b"1 2 3 4 5\0" as *const u8 as *const libc::c_char,
            b"10 _vsx 12345\0" as *const u8 as *const libc::c_char,
            s_319,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_320: S = ts(
        tp(
            tc(
                b"1 6 40\0" as *const u8 as *const libc::c_char as S,
                b"24 60 60 _vs 4000\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            474 as libc::c_int,
            b"1 6 40\0" as *const u8 as *const libc::c_char,
            b"24 60 60 _vs 4000\0" as *const u8 as *const libc::c_char,
            s_320,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_321: S = ts(
        tp(
            tc(
                b"1 6 40\0" as *const u8 as *const libc::c_char as S,
                b"24 60 60 _vsx 4000\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            475 as libc::c_int,
            b"1 6 40\0" as *const u8 as *const libc::c_char,
            b"24 60 60 _vsx 4000\0" as *const u8 as *const libc::c_char,
            s_321,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_322: S = ts(
        tp(
            tc(
                b"0 0 0\0" as *const u8 as *const libc::c_char as S,
                b"1 2 3 _vs 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            476 as libc::c_int,
            b"0 0 0\0" as *const u8 as *const libc::c_char,
            b"1 2 3 _vs 0\0" as *const u8 as *const libc::c_char,
            s_322,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_323: S = ts(
        tp(
            tc(
                b"0 0 0\0" as *const u8 as *const libc::c_char as S,
                b"1 2 3 _vsx 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            477 as libc::c_int,
            b"0 0 0\0" as *const u8 as *const libc::c_char,
            b"1 2 3 _vsx 0\0" as *const u8 as *const libc::c_char,
            s_323,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_324: S = ts(
        tp(
            tc(
                b"23 59 0\0" as *const u8 as *const libc::c_char as S,
                b"24 60 60 _vs -60\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            478 as libc::c_int,
            b"23 59 0\0" as *const u8 as *const libc::c_char,
            b"24 60 60 _vs -60\0" as *const u8 as *const libc::c_char,
            s_324,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_325: S = ts(
        tp(
            tc(
                b"23 59 0\0" as *const u8 as *const libc::c_char as S,
                b"24 60 60 _vsx -60\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            479 as libc::c_int,
            b"23 59 0\0" as *const u8 as *const libc::c_char,
            b"24 60 60 _vsx -60\0" as *const u8 as *const libc::c_char,
            s_325,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_326: S = ts(
        tp(
            tc(
                b"13 20 0\0" as *const u8 as *const libc::c_char as S,
                b"24 60 60 _vs -6000000\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            480 as libc::c_int,
            b"13 20 0\0" as *const u8 as *const libc::c_char,
            b"24 60 60 _vs -6000000\0" as *const u8 as *const libc::c_char,
            s_326,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_327: S = ts(
        tp(
            tc(
                b"13 20 0\0" as *const u8 as *const libc::c_char as S,
                b"24 60 60 _vsx -6000000\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            481 as libc::c_int,
            b"13 20 0\0" as *const u8 as *const libc::c_char,
            b"24 60 60 _vsx -6000000\0" as *const u8 as *const libc::c_char,
            s_327,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_328: S = ts(
        tp(
            tc(
                b"(0 0 10;0 16 40;13 46 40)\0" as *const u8 as *const libc::c_char as S,
                b"24 60 60 _vs/: 10 1000 1000000\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            482 as libc::c_int,
            b"(0 0 10;0 16 40;13 46 40)\0" as *const u8 as *const libc::c_char,
            b"24 60 60 _vs/: 10 1000 1000000\0" as *const u8 as *const libc::c_char,
            s_328,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_329: S = ts(
        tp(
            tc(
                b"(0 0 10;0 16 40;13 46 40)\0" as *const u8 as *const libc::c_char as S,
                b"24 60 60 _vsx/: 10 1000 1000000\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            483 as libc::c_int,
            b"(0 0 10;0 16 40;13 46 40)\0" as *const u8 as *const libc::c_char,
            b"24 60 60 _vsx/: 10 1000 1000000\0" as *const u8 as *const libc::c_char,
            s_329,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_330: S = ts(
        tp(
            tc(
                b"0 0\0" as *const u8 as *const libc::c_char as S,
                b"1 1 _vs 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            484 as libc::c_int,
            b"0 0\0" as *const u8 as *const libc::c_char,
            b"1 1 _vs 0\0" as *const u8 as *const libc::c_char,
            s_330,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_331: S = ts(
        tp(
            tc(
                b"0 0\0" as *const u8 as *const libc::c_char as S,
                b"1 1 _vsx 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            485 as libc::c_int,
            b"0 0\0" as *const u8 as *const libc::c_char,
            b"1 1 _vsx 0\0" as *const u8 as *const libc::c_char,
            s_331,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_332: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"0, (_+[2;]) 11\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            487 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"0, (_+[2;]) 11\0" as *const u8 as *const libc::c_char,
            s_332,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_333: S = ts(
        tp(
            tc(
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"if[1; `0:\xE2\x80\x9Cbad unicode quotes\xE2\x80\x9D]\";:]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            488 as libc::c_int,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"if[1; `0:\xE2\x80\x9Cbad unicode quotes\xE2\x80\x9D]\";:]\0"
                as *const u8 as *const libc::c_char,
            s_333,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_334: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"25<#0:\"README.md\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"25<#0:\"README.md\"\0" as *const u8 as *const libc::c_char,
            s_334,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_335: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+y+z}; f'[1;2;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            492 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"f:{x+y+z}; f'[1;2;3]\0" as *const u8 as *const libc::c_char,
            s_335,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_336: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+y+z}; f''[1;2;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            493 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"f:{x+y+z}; f''[1;2;3]\0" as *const u8 as *const libc::c_char,
            s_336,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_337: S = ts(
        tp(
            tc(
                b"()\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+y+z}; f'[();2;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            494 as libc::c_int,
            b"()\0" as *const u8 as *const libc::c_char,
            b"f:{x+y+z}; f'[();2;3]\0" as *const u8 as *const libc::c_char,
            s_337,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_338: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"{x}'[]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            495 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"{x}'[]\0" as *const u8 as *const libc::c_char,
            s_338,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_339: S = ts(
        tp(
            tc(
                b"12 15 18\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+y+z}; f'[1 2 3;4 5 6;7 8 9]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            496 as libc::c_int,
            b"12 15 18\0" as *const u8 as *const libc::c_char,
            b"f:{x+y+z}; f'[1 2 3;4 5 6;7 8 9]\0" as *const u8 as *const libc::c_char,
            s_339,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_340: S = ts(
        tp(
            tc(
                b"22 25 28\0" as *const u8 as *const libc::c_char as S,
                b"f:{[a;b;c;d] a+b+c+d}; f'[1 2 3;4 5 6;7 8 9;10]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            497 as libc::c_int,
            b"22 25 28\0" as *const u8 as *const libc::c_char,
            b"f:{[a;b;c;d] a+b+c+d}; f'[1 2 3;4 5 6;7 8 9;10]\0" as *const u8
                as *const libc::c_char,
            s_340,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_341: S = ts(
        tp(
            tc(
                b"22 26 30\0" as *const u8 as *const libc::c_char as S,
                b"f:{[a;b;c;d] a+b+c+d}; f'[1 2 3;4 5 6;7 8 9;10 11 12]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            498 as libc::c_int,
            b"22 26 30\0" as *const u8 as *const libc::c_char,
            b"f:{[a;b;c;d] a+b+c+d}; f'[1 2 3;4 5 6;7 8 9;10 11 12]\0" as *const u8
                as *const libc::c_char,
            s_341,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_342: S = ts(
        tp(
            tc(
                b"12 14 16\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+y+z}; f'[1;4 5 6;7 8 9]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            499 as libc::c_int,
            b"12 14 16\0" as *const u8 as *const libc::c_char,
            b"f:{x+y+z}; f'[1;4 5 6;7 8 9]\0" as *const u8 as *const libc::c_char,
            s_342,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_343: S = ts(
        tp(
            tc(
                b"12 14 16\0" as *const u8 as *const libc::c_char as S,
                b"{x+y+z}'[1;4 5 6;7 8 9]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            500 as libc::c_int,
            b"12 14 16\0" as *const u8 as *const libc::c_char,
            b"{x+y+z}'[1;4 5 6;7 8 9]\0" as *const u8 as *const libc::c_char,
            s_343,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_344: S = ts(
        tp(
            tc(
                b"(9 12;9 12)\0" as *const u8 as *const libc::c_char as S,
                b"{x+y+z}'[(1 2;1 2);(3 4;3 4);(5 6;5 6)]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            501 as libc::c_int,
            b"(9 12;9 12)\0" as *const u8 as *const libc::c_char,
            b"{x+y+z}'[(1 2;1 2);(3 4;3 4);(5 6;5 6)]\0" as *const u8
                as *const libc::c_char,
            s_344,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_345: S = ts(
        tp(
            tc(
                b"9 12\0" as *const u8 as *const libc::c_char as S,
                b"{x+y+z}'[1 2;3 4;5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            502 as libc::c_int,
            b"9 12\0" as *const u8 as *const libc::c_char,
            b"{x+y+z}'[1 2;3 4;5 6]\0" as *const u8 as *const libc::c_char,
            s_345,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_346: S = ts(
        tp(
            tc(
                b"9 12\0" as *const u8 as *const libc::c_char as S,
                b"{x+y+z}''''''[1 2;3 4;5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            503 as libc::c_int,
            b"9 12\0" as *const u8 as *const libc::c_char,
            b"{x+y+z}''''''[1 2;3 4;5 6]\0" as *const u8 as *const libc::c_char,
            s_346,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_347: S = ts(
        tp(
            tc(
                b"12 15 18\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+y+z}; f''[1 2 3;4 5 6;7 8 9]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            504 as libc::c_int,
            b"12 15 18\0" as *const u8 as *const libc::c_char,
            b"f:{x+y+z}; f''[1 2 3;4 5 6;7 8 9]\0" as *const u8 as *const libc::c_char,
            s_347,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_348: S = ts(
        tp(
            tc(
                b"16 17 18\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}/[1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            507 as libc::c_int,
            b"16 17 18\0" as *const u8 as *const libc::c_char,
            b"{x+y}/[1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char,
            s_348,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_349: S = ts(
        tp(
            tc(
                b"16 17 18\0" as *const u8 as *const libc::c_char as S,
                b"(+)/[1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            508 as libc::c_int,
            b"16 17 18\0" as *const u8 as *const libc::c_char,
            b"(+)/[1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char,
            s_349,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_350: S = ts(
        tp(
            tc(
                b"()\0" as *const u8 as *const libc::c_char as S,
                b"{x+y+z}/[();1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            509 as libc::c_int,
            b"()\0" as *const u8 as *const libc::c_char,
            b"{x+y+z}/[();1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char,
            s_350,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_351: S = ts(
        tp(
            tc(
                b"1 4 2 5 3 6\0" as *const u8 as *const libc::c_char as S,
                b"{x,y,z}/[();1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            510 as libc::c_int,
            b"1 4 2 5 3 6\0" as *const u8 as *const libc::c_char,
            b"{x,y,z}/[();1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char,
            s_351,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_352: S = ts(
        tp(
            tc(
                b"(,7)\0" as *const u8 as *const libc::c_char as S,
                b"@/[,1;0 0 0;+;1 2 3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            511 as libc::c_int,
            b"(,7)\0" as *const u8 as *const libc::c_char,
            b"@/[,1;0 0 0;+;1 2 3]\0" as *const u8 as *const libc::c_char,
            s_352,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_353: S = ts(
        tp(
            tc(
                b"(,5)\0" as *const u8 as *const libc::c_char as S,
                b"@/[,1;4#0;+;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            512 as libc::c_int,
            b"(,5)\0" as *const u8 as *const libc::c_char,
            b"@/[,1;4#0;+;1]\0" as *const u8 as *const libc::c_char,
            s_353,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_354: S = ts(
        tp(
            tc(
                b"3 1\0" as *const u8 as *const libc::c_char as S,
                b"./[1 1;0 0;+; 1 1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            513 as libc::c_int,
            b"3 1\0" as *const u8 as *const libc::c_char,
            b"./[1 1;0 0;+; 1 1]\0" as *const u8 as *const libc::c_char,
            s_354,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_355: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"+/[1 1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            514 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"+/[1 1]\0" as *const u8 as *const libc::c_char,
            s_355,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_356: S = ts(
        tp(
            tc(
                b"3 3\0" as *const u8 as *const libc::c_char as S,
                b"+/[1 1;1 1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            515 as libc::c_int,
            b"3 3\0" as *const u8 as *const libc::c_char,
            b"+/[1 1;1 1]\0" as *const u8 as *const libc::c_char,
            s_356,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_357: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"+/[]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            516 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"+/[]\0" as *const u8 as *const libc::c_char,
            s_357,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_358: S = ts(
        tp(
            tc(
                b"(1 2 3;5 6 7;10 11 12;16 17 18)\0" as *const u8 as *const libc::c_char
                    as S,
                b"(+)\\[1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            519 as libc::c_int,
            b"(1 2 3;5 6 7;10 11 12;16 17 18)\0" as *const u8 as *const libc::c_char,
            b"(+)\\[1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char,
            s_358,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_359: S = ts(
        tp(
            tc(
                b"(1 2 3;5 6 7;10 11 12;16 17 18)\0" as *const u8 as *const libc::c_char
                    as S,
                b"+\\[1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            520 as libc::c_int,
            b"(1 2 3;5 6 7;10 11 12;16 17 18)\0" as *const u8 as *const libc::c_char,
            b"+\\[1 2 3;4 5 6]\0" as *const u8 as *const libc::c_char,
            s_359,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_360: S = ts(
        tp(
            tc(
                b"(1 2;1 2 3 5;1 2 3 5 4 6)\0" as *const u8 as *const libc::c_char as S,
                b"{x,y,z}\\[1 2;3 4;5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            521 as libc::c_int,
            b"(1 2;1 2 3 5;1 2 3 5 4 6)\0" as *const u8 as *const libc::c_char,
            b"{x,y,z}\\[1 2;3 4;5 6]\0" as *const u8 as *const libc::c_char,
            s_360,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_361: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{x+y+z}\\[1;1;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            522 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{x+y+z}\\[1;1;1]\0" as *const u8 as *const libc::c_char,
            s_361,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_362: S = ts(
        tp(
            tc(
                b"(1 1;3 3;5 5)\0" as *const u8 as *const libc::c_char as S,
                b"{x+y+z}\\[1 1;1 1;1 1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            523 as libc::c_int,
            b"(1 1;3 3;5 5)\0" as *const u8 as *const libc::c_char,
            b"{x+y+z}\\[1 1;1 1;1 1]\0" as *const u8 as *const libc::c_char,
            s_362,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_363: S = ts(
        tp(
            tc(
                b"(,1)\0" as *const u8 as *const libc::c_char as S,
                b"{x,y,z}[1;();()]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            524 as libc::c_int,
            b"(,1)\0" as *const u8 as *const libc::c_char,
            b"{x,y,z}[1;();()]\0" as *const u8 as *const libc::c_char,
            s_363,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_364: S = ts(
        tp(
            tc(
                b"1 3 6\0" as *const u8 as *const libc::c_char as S,
                b"+\\[1 2 3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            525 as libc::c_int,
            b"1 3 6\0" as *const u8 as *const libc::c_char,
            b"+\\[1 2 3]\0" as *const u8 as *const libc::c_char,
            s_364,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_365: S = ts(
        tp(
            tc(
                b"1 3 6\0" as *const u8 as *const libc::c_char as S,
                b"(+\\)[1 2 3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            526 as libc::c_int,
            b"1 3 6\0" as *const u8 as *const libc::c_char,
            b"(+\\)[1 2 3]\0" as *const u8 as *const libc::c_char,
            s_365,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_366: S = ts(
        tp(
            tc(
                b"(1 1;2 2;3 3)\0" as *const u8 as *const libc::c_char as S,
                b"+\\[1 1;1 1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            527 as libc::c_int,
            b"(1 1;2 2;3 3)\0" as *const u8 as *const libc::c_char,
            b"+\\[1 1;1 1]\0" as *const u8 as *const libc::c_char,
            s_366,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_367: S = ts(
        tp(
            tc(
                b"(1 1;2 2;3 3)\0" as *const u8 as *const libc::c_char as S,
                b"(+\\)[1 1;1 1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            528 as libc::c_int,
            b"(1 1;2 2;3 3)\0" as *const u8 as *const libc::c_char,
            b"(+\\)[1 1;1 1]\0" as *const u8 as *const libc::c_char,
            s_367,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_368: S = ts(
        tp(
            tc(
                b"(1 1;10 10;19 19)\0" as *const u8 as *const libc::c_char as S,
                b"{[a;b;c;d]a+b+c+d}\\[1 1;2 2;3 3;4 4]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            529 as libc::c_int,
            b"(1 1;10 10;19 19)\0" as *const u8 as *const libc::c_char,
            b"{[a;b;c;d]a+b+c+d}\\[1 1;2 2;3 3;4 4]\0" as *const u8
                as *const libc::c_char,
            s_368,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_369: S = ts(
        tp(
            tc(
                b"(33 34;39 40)\0" as *const u8 as *const libc::c_char as S,
                b"{x+y+z}'/[(1 2;3 4);(5 6;7 8);(9 10;11 12)]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            532 as libc::c_int,
            b"(33 34;39 40)\0" as *const u8 as *const libc::c_char,
            b"{x+y+z}'/[(1 2;3 4);(5 6;7 8);(9 10;11 12)]\0" as *const u8
                as *const libc::c_char,
            s_369,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_370: S = ts(
        tp(
            tc(
                b"(31 32;41 42)\0" as *const u8 as *const libc::c_char as S,
                b"{x+y+z}/'[(1 2;3 4);(5 6;7 8);(9 10;11 12)]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            533 as libc::c_int,
            b"(31 32;41 42)\0" as *const u8 as *const libc::c_char,
            b"{x+y+z}/'[(1 2;3 4);(5 6;7 8);(9 10;11 12)]\0" as *const u8
                as *const libc::c_char,
            s_370,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_371: S = ts(
        tp(
            tc(
                b"(@[0$;0 1;:])\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"type\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            535 as libc::c_int,
            b"(@[0$;0 1;:])\0" as *const u8 as *const libc::c_char,
            b"(1;\"type\")\0" as *const u8 as *const libc::c_char,
            s_371,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_372: S = ts(
        tp(
            tc(
                b"(@[0$;0 1.0;:])\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"type\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            536 as libc::c_int,
            b"(@[0$;0 1.0;:])\0" as *const u8 as *const libc::c_char,
            b"(1;\"type\")\0" as *const u8 as *const libc::c_char,
            s_372,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_373: S = ts(
        tp(
            tc(
                b"(@[0$;`a`a;:])\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"type\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            537 as libc::c_int,
            b"(@[0$;`a`a;:])\0" as *const u8 as *const libc::c_char,
            b"(1;\"type\")\0" as *const u8 as *const libc::c_char,
            s_373,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_374: S = ts(
        tp(
            tc(
                b"(@[.:;\"a:(1 2)[0]:3\";:] )\0" as *const u8 as *const libc::c_char
                    as S,
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            539 as libc::c_int,
            b"(@[.:;\"a:(1 2)[0]:3\";:] )\0" as *const u8 as *const libc::c_char,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            s_374,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_375: S = ts(
        tp(
            tc(
                b"(@[.:;\"a:(0 1;2 3); a[0][1]:9\";:] )\0" as *const u8
                    as *const libc::c_char as S,
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            540 as libc::c_int,
            b"(@[.:;\"a:(0 1;2 3); a[0][1]:9\";:] )\0" as *const u8
                as *const libc::c_char,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            s_375,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_376: S = ts(
        tp(
            tc(
                b"(@[.:;\"(1):2\";:] )\0" as *const u8 as *const libc::c_char as S,
                b"(0;2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            541 as libc::c_int,
            b"(@[.:;\"(1):2\";:] )\0" as *const u8 as *const libc::c_char,
            b"(0;2)\0" as *const u8 as *const libc::c_char,
            s_376,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_377: S = ts(
        tp(
            tc(
                b"(@[.:;\"a: 1 1 1; a/[0] +: 10\";:] )\0" as *const u8
                    as *const libc::c_char as S,
                b"(1;\"valence\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            542 as libc::c_int,
            b"(@[.:;\"a: 1 1 1; a/[0] +: 10\";:] )\0" as *const u8
                as *const libc::c_char,
            b"(1;\"valence\")\0" as *const u8 as *const libc::c_char,
            s_377,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_378: S = ts(
        tp(
            tc(
                b"(@[.:;\"5 (a:5)/1\";:] )\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"rank\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            543 as libc::c_int,
            b"(@[.:;\"5 (a:5)/1\";:] )\0" as *const u8 as *const libc::c_char,
            b"(1;\"rank\")\0" as *const u8 as *const libc::c_char,
            s_378,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_379: S = ts(
        tp(
            tc(
                b"13\0" as *const u8 as *const libc::c_char as S,
                b"({x(|+\\)\\1 1} 5)[5;0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            545 as libc::c_int,
            b"13\0" as *const u8 as *const libc::c_char,
            b"({x(|+\\)\\1 1} 5)[5;0]\0" as *const u8 as *const libc::c_char,
            s_379,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_380: S = ts(
        tp(
            tc(
                b"(.[.:;,\"@[a-b]\";:])\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            546 as libc::c_int,
            b"(.[.:;,\"@[a-b]\";:])\0" as *const u8 as *const libc::c_char,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            s_380,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_381: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"_ceiling 4.6\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            548 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"_ceiling 4.6\0" as *const u8 as *const libc::c_char,
            s_381,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_382: S = ts(
        tp(
            tc(
                b"-4\0" as *const u8 as *const libc::c_char as S,
                b"_ceiling -4.6\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            549 as libc::c_int,
            b"-4\0" as *const u8 as *const libc::c_char,
            b"_ceiling -4.6\0" as *const u8 as *const libc::c_char,
            s_382,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_383: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"_ceiling 5.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"_ceiling 5.0\0" as *const u8 as *const libc::c_char,
            s_383,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_384: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"_ceiling 1.001\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            551 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"_ceiling 1.001\0" as *const u8 as *const libc::c_char,
            s_384,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_385: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"_ceiling 1.0000001\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            552 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"_ceiling 1.0000001\0" as *const u8 as *const libc::c_char,
            s_385,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_386: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"_ceiling 1.00000000000000000001\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            553 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"_ceiling 1.00000000000000000001\0" as *const u8 as *const libc::c_char,
            s_386,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_387: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"_ceiling 0.00000000000000000001\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            554 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"_ceiling 0.00000000000000000001\0" as *const u8 as *const libc::c_char,
            s_387,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_388: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"@[1 2;0;0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            556 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"@[1 2;0;0]\0" as *const u8 as *const libc::c_char,
            s_388,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_389: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b".[1 2;0;0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            557 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b".[1 2;0;0]\0" as *const u8 as *const libc::c_char,
            s_389,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_390: S = ts(
        tp(
            tc(
                b"6 2\0" as *const u8 as *const libc::c_char as S,
                b"@[1 2;0;5 6 7]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            558 as libc::c_int,
            b"6 2\0" as *const u8 as *const libc::c_char,
            b"@[1 2;0;5 6 7]\0" as *const u8 as *const libc::c_char,
            s_390,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_391: S = ts(
        tp(
            tc(
                b"6 2\0" as *const u8 as *const libc::c_char as S,
                b".[1 2;0;5 6 7]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            559 as libc::c_int,
            b"6 2\0" as *const u8 as *const libc::c_char,
            b".[1 2;0;5 6 7]\0" as *const u8 as *const libc::c_char,
            s_391,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_392: S = ts(
        tp(
            tc(
                b"(1;\"index\")\0" as *const u8 as *const libc::c_char as S,
                b"f:4 4#!16;i:(0;1 2;3 4 5;6 7 8 9); @[f;i;:]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            561 as libc::c_int,
            b"(1;\"index\")\0" as *const u8 as *const libc::c_char,
            b"f:4 4#!16;i:(0;1 2;3 4 5;6 7 8 9); @[f;i;:]\0" as *const u8
                as *const libc::c_char,
            s_392,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_393: S = ts(
        tp(
            tc(
                b"(1;\"index\")\0" as *const u8 as *const libc::c_char as S,
                b"f:4 4#!16;i:(0;1 2;3 4 5;6 7 8 9); @[.:;\"f@'i\";:]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            562 as libc::c_int,
            b"(1;\"index\")\0" as *const u8 as *const libc::c_char,
            b"f:4 4#!16;i:(0;1 2;3 4 5;6 7 8 9); @[.:;\"f@'i\";:]\0" as *const u8
                as *const libc::c_char,
            s_393,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_394: S = ts(
        tp(
            tc(
                b"(1;\"valence\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"1 2(+\\:)'1 2\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            564 as libc::c_int,
            b"(1;\"valence\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"1 2(+\\:)'1 2\";:]\0" as *const u8 as *const libc::c_char,
            s_394,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_395: S = ts(
        tp(
            tc(
                b"(1;\"valence\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"1 2(+\\\\:)'1 2\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            565 as libc::c_int,
            b"(1;\"valence\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"1 2(+\\\\:)'1 2\";:]\0" as *const u8 as *const libc::c_char,
            s_395,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_396: S = ts(
        tp(
            tc(
                b"(1;\"type\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"(2=\\\"2\\\") 2\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            566 as libc::c_int,
            b"(1;\"type\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"(2=\\\"2\\\") 2\";:]\0" as *const u8 as *const libc::c_char,
            s_396,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_397: S = ts(
        tp(
            tc(
                b"(0;6 7 8)\0" as *const u8 as *const libc::c_char as S,
                b"a:{x+y};@[.:;\"a'[5;1 2 3]\";:]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            569 as libc::c_int,
            b"(0;6 7 8)\0" as *const u8 as *const libc::c_char,
            b"a:{x+y};@[.:;\"a'[5;1 2 3]\";:]\0" as *const u8 as *const libc::c_char,
            s_397,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_398: S = ts(
        tp(
            tc(
                b"(0;6 7 8)\0" as *const u8 as *const libc::c_char as S,
                b"a:{x+y};@[.:;\"a\\\\:[5 6 7;1]\";:]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            570 as libc::c_int,
            b"(0;6 7 8)\0" as *const u8 as *const libc::c_char,
            b"a:{x+y};@[.:;\"a\\\\:[5 6 7;1]\";:]\0" as *const u8 as *const libc::c_char,
            s_398,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_399: S = ts(
        tp(
            tc(
                b"(\"ab\"\n \"ac\")\0" as *const u8 as *const libc::c_char as S,
                b"{x,y}'[\"a\";\"bc\"]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            571 as libc::c_int,
            b"(\"ab\"\n \"ac\")\0" as *const u8 as *const libc::c_char,
            b"{x,y}'[\"a\";\"bc\"]\0" as *const u8 as *const libc::c_char,
            s_399,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_400: S = ts(
        tp(
            tc(
                b"-1 0 1\0" as *const u8 as *const libc::c_char as S,
                b"flip:{[f;x;y]f[y;x]};flip[-\\:][2;1 2 3]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            572 as libc::c_int,
            b"-1 0 1\0" as *const u8 as *const libc::c_char,
            b"flip:{[f;x;y]f[y;x]};flip[-\\:][2;1 2 3]\0" as *const u8
                as *const libc::c_char,
            s_400,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_401: S = ts(
        tp(
            tc(
                b"10#2\0" as *const u8 as *const libc::c_char as S,
                b"{x+1}'10#1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            574 as libc::c_int,
            b"10#2\0" as *const u8 as *const libc::c_char,
            b"{x+1}'10#1\0" as *const u8 as *const libc::c_char,
            s_401,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_402: S = ts(
        tp(
            tc(
                b"c:{&:'y(?,/(1!)\\'1,')/,&x-y}; {x@<x}@c[4;2]\0" as *const u8
                    as *const libc::c_char as S,
                b"(0 1;0 2;0 3;1 2;1 3;2 3)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            575 as libc::c_int,
            b"c:{&:'y(?,/(1!)\\'1,')/,&x-y}; {x@<x}@c[4;2]\0" as *const u8
                as *const libc::c_char,
            b"(0 1;0 2;0 3;1 2;1 3;2 3)\0" as *const u8 as *const libc::c_char,
            s_402,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_403: S = ts(
        tp(
            tc(
                b":\0" as *const u8 as *const libc::c_char as S,
                b":\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            577 as libc::c_int,
            b":\0" as *const u8 as *const libc::c_char,
            b":\0" as *const u8 as *const libc::c_char,
            s_403,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_404: S = ts(
        tp(
            tc(
                b"::\0" as *const u8 as *const libc::c_char as S,
                b"::\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            578 as libc::c_int,
            b"::\0" as *const u8 as *const libc::c_char,
            b"::\0" as *const u8 as *const libc::c_char,
            s_404,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_405: S = ts(
        tp(
            tc(
                b"::::\0" as *const u8 as *const libc::c_char as S,
                b"::::\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            579 as libc::c_int,
            b"::::\0" as *const u8 as *const libc::c_char,
            b"::::\0" as *const u8 as *const libc::c_char,
            s_405,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_406: S = ts(
        tp(
            tc(
                b"5:(:)\0" as *const u8 as *const libc::c_char as S,
                b",\":\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            580 as libc::c_int,
            b"5:(:)\0" as *const u8 as *const libc::c_char,
            b",\":\"\0" as *const u8 as *const libc::c_char,
            s_406,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_407: S = ts(
        tp(
            tc(
                b"5:(::)\0" as *const u8 as *const libc::c_char as S,
                b"\"::\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            581 as libc::c_int,
            b"5:(::)\0" as *const u8 as *const libc::c_char,
            b"\"::\"\0" as *const u8 as *const libc::c_char,
            s_407,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_408: S = ts(
        tp(
            tc(
                b"5:(::::)\0" as *const u8 as *const libc::c_char as S,
                b"\"::::\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            582 as libc::c_int,
            b"5:(::::)\0" as *const u8 as *const libc::c_char,
            b"\"::::\"\0" as *const u8 as *const libc::c_char,
            s_408,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_409: S = ts(
        tp(
            tc(
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\":::\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            583 as libc::c_int,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\":::\";:]\0" as *const u8 as *const libc::c_char,
            s_409,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_410: S = ts(
        tp(
            tc(
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\":::::\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            584 as libc::c_int,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\":::::\";:]\0" as *const u8 as *const libc::c_char,
            s_410,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_411: S = ts(
        tp(
            tc(
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\":::5::::\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            585 as libc::c_int,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\":::5::::\";:]\0" as *const u8 as *const libc::c_char,
            s_411,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_412: S = ts(
        tp(
            tc(
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"a _abs: \";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            586 as libc::c_int,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"a _abs: \";:]\0" as *const u8 as *const libc::c_char,
            s_412,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_413: S = ts(
        tp(
            tc(
                b"(1;\"parse\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"4:::\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            587 as libc::c_int,
            b"(1;\"parse\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"4:::\";:]\0" as *const u8 as *const libc::c_char,
            s_413,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_414: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"a:2 3;a@*:0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            589 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"a:2 3;a@*:0\0" as *const u8 as *const libc::c_char,
            s_414,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_415: S = ts(
        tp(
            tc(
                b"0=#:\0" as *const u8 as *const libc::c_char as S,
                b"a:0;a=#:\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            590 as libc::c_int,
            b"0=#:\0" as *const u8 as *const libc::c_char,
            b"a:0;a=#:\0" as *const u8 as *const libc::c_char,
            s_415,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_416: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"_n, do[3;.\"\\\" \\\"_sv 1\"]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            592 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"_n, do[3;.\"\\\" \\\"_sv 1\"]\0" as *const u8 as *const libc::c_char,
            s_416,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_417: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"_n, do[3;.\"1_sv \\\" \\\"\"]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            593 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"_n, do[3;.\"1_sv \\\" \\\"\"]\0" as *const u8 as *const libc::c_char,
            s_417,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_418: S = ts(
        tp(
            tc(
                b"6 7 8\0" as *const u8 as *const libc::c_char as S,
                b"(21>+/)(2+)/!3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            595 as libc::c_int,
            b"6 7 8\0" as *const u8 as *const libc::c_char,
            b"(21>+/)(2+)/!3\0" as *const u8 as *const libc::c_char,
            s_418,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_419: S = ts(
        tp(
            tc(
                b"(1;\"wsfull\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"0I#0\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            596 as libc::c_int,
            b"(1;\"wsfull\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"0I#0\";:]\0" as *const u8 as *const libc::c_char,
            s_419,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_420: S = ts(
        tp(
            tc(
                b"(1;\"int\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"&0I\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            597 as libc::c_int,
            b"(1;\"int\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"&0I\";:]\0" as *const u8 as *const libc::c_char,
            s_420,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_421: S = ts(
        tp(
            tc(
                b"(1;\"wsfull\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"!0I\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            601 as libc::c_int,
            b"(1;\"wsfull\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"!0I\";:]\0" as *const u8 as *const libc::c_char,
            s_421,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_422: S = ts(
        tp(
            tc(
                b"1 3\0" as *const u8 as *const libc::c_char as S,
                b"+\\ 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            604 as libc::c_int,
            b"1 3\0" as *const u8 as *const libc::c_char,
            b"+\\ 1 2\0" as *const u8 as *const libc::c_char,
            s_422,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_423: S = ts(
        tp(
            tc(
                b"1 3.0\0" as *const u8 as *const libc::c_char as S,
                b"+\\ 1.0 2.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            605 as libc::c_int,
            b"1 3.0\0" as *const u8 as *const libc::c_char,
            b"+\\ 1.0 2.0\0" as *const u8 as *const libc::c_char,
            s_423,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_424: S = ts(
        tp(
            tc(
                b"1 2 4.0\0" as *const u8 as *const libc::c_char as S,
                b"1.0 +\\ 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            606 as libc::c_int,
            b"1 2 4.0\0" as *const u8 as *const libc::c_char,
            b"1.0 +\\ 1 2\0" as *const u8 as *const libc::c_char,
            s_424,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_425: S = ts(
        tp(
            tc(
                b"1 2 4.0\0" as *const u8 as *const libc::c_char as S,
                b"1.0 +\\ 1.0 2.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            607 as libc::c_int,
            b"1 2 4.0\0" as *const u8 as *const libc::c_char,
            b"1.0 +\\ 1.0 2.0\0" as *const u8 as *const libc::c_char,
            s_425,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_426: S = ts(
        tp(
            tc(
                b"1 2 4\0" as *const u8 as *const libc::c_char as S,
                b"1 +\\ 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            608 as libc::c_int,
            b"1 2 4\0" as *const u8 as *const libc::c_char,
            b"1 +\\ 1 2\0" as *const u8 as *const libc::c_char,
            s_426,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_427: S = ts(
        tp(
            tc(
                b"1 2 4.0\0" as *const u8 as *const libc::c_char as S,
                b"1 +\\ 1.0 2.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            609 as libc::c_int,
            b"1 2 4.0\0" as *const u8 as *const libc::c_char,
            b"1 +\\ 1.0 2.0\0" as *const u8 as *const libc::c_char,
            s_427,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_428: S = ts(
        tp(
            tc(
                b"10 1\0" as *const u8 as *const libc::c_char as S,
                b".\"10 -'': 2 3\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            611 as libc::c_int,
            b"10 1\0" as *const u8 as *const libc::c_char,
            b".\"10 -'': 2 3\"\0" as *const u8 as *const libc::c_char,
            s_428,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_429: S = ts(
        tp(
            tc(
                b"10 1\0" as *const u8 as *const libc::c_char as S,
                b".\"10 -': 2 3\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            612 as libc::c_int,
            b"10 1\0" as *const u8 as *const libc::c_char,
            b".\"10 -': 2 3\"\0" as *const u8 as *const libc::c_char,
            s_429,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_430: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b".\"10 -': 2\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            613 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b".\"10 -': 2\"\0" as *const u8 as *const libc::c_char,
            s_430,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_431: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b".\"10 +': 2\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            614 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b".\"10 +': 2\"\0" as *const u8 as *const libc::c_char,
            s_431,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_432: S = ts(
        tp(
            tc(
                b"10 5\0" as *const u8 as *const libc::c_char as S,
                b".\"10 +': 2 3\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            615 as libc::c_int,
            b"10 5\0" as *const u8 as *const libc::c_char,
            b".\"10 +': 2 3\"\0" as *const u8 as *const libc::c_char,
            s_432,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_433: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b".\"+':2\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            616 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b".\"+':2\"\0" as *const u8 as *const libc::c_char,
            s_433,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_434: S = ts(
        tp(
            tc(
                b"5.0\0" as *const u8 as *const libc::c_char as S,
                b".\"+':2.5\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            617 as libc::c_int,
            b"5.0\0" as *const u8 as *const libc::c_char,
            b".\"+':2.5\"\0" as *const u8 as *const libc::c_char,
            s_434,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_435: S = ts(
        tp(
            tc(
                b",5\0" as *const u8 as *const libc::c_char as S,
                b"+':2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            618 as libc::c_int,
            b",5\0" as *const u8 as *const libc::c_char,
            b"+':2 3\0" as *const u8 as *const libc::c_char,
            s_435,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_436: S = ts(
        tp(
            tc(
                b"0 12345 1406932606 654583808 1358247936\0" as *const u8
                    as *const libc::c_char as S,
                b"4{((1103515245*x)+12345)!(_2^31)}\\0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            620 as libc::c_int,
            b"0 12345 1406932606 654583808 1358247936\0" as *const u8
                as *const libc::c_char,
            b"4{((1103515245*x)+12345)!(_2^31)}\\0\0" as *const u8
                as *const libc::c_char,
            s_436,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_437: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"(_2^31)-2147483648\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            621 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"(_2^31)-2147483648\0" as *const u8 as *const libc::c_char,
            s_437,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_438: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"-8!7\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            622 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"-8!7\0" as *const u8 as *const libc::c_char,
            s_438,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_439: S = ts(
        tp(
            tc(
                b"+/[1;]\0" as *const u8 as *const libc::c_char as S,
                b"+/[1;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            627 as libc::c_int,
            b"+/[1;]\0" as *const u8 as *const libc::c_char,
            b"+/[1;]\0" as *const u8 as *const libc::c_char,
            s_439,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_440: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"+/[1;]2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            628 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"+/[1;]2 3 4\0" as *const u8 as *const libc::c_char,
            s_440,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_441: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"(+/)[1;]2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            629 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"(+/)[1;]2 3 4\0" as *const u8 as *const libc::c_char,
            s_441,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_442: S = ts(
        tp(
            tc(
                b"1 3 6 10\0" as *const u8 as *const libc::c_char as S,
                b"+\\[1;]2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            630 as libc::c_int,
            b"1 3 6 10\0" as *const u8 as *const libc::c_char,
            b"+\\[1;]2 3 4\0" as *const u8 as *const libc::c_char,
            s_442,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_443: S = ts(
        tp(
            tc(
                b"+\\[1;]\0" as *const u8 as *const libc::c_char as S,
                b"+\\[1;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            631 as libc::c_int,
            b"+\\[1;]\0" as *const u8 as *const libc::c_char,
            b"+\\[1;]\0" as *const u8 as *const libc::c_char,
            s_443,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_444: S = ts(
        tp(
            tc(
                b"+/[;1]\0" as *const u8 as *const libc::c_char as S,
                b"+/[;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            632 as libc::c_int,
            b"+/[;1]\0" as *const u8 as *const libc::c_char,
            b"+/[;1]\0" as *const u8 as *const libc::c_char,
            s_444,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_445: S = ts(
        tp(
            tc(
                b"+'[;1]\0" as *const u8 as *const libc::c_char as S,
                b"+'[;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            633 as libc::c_int,
            b"+'[;1]\0" as *const u8 as *const libc::c_char,
            b"+'[;1]\0" as *const u8 as *const libc::c_char,
            s_445,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_446: S = ts(
        tp(
            tc(
                b"3 4 5\0" as *const u8 as *const libc::c_char as S,
                b"+'[1;]2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            634 as libc::c_int,
            b"3 4 5\0" as *const u8 as *const libc::c_char,
            b"+'[1;]2 3 4\0" as *const u8 as *const libc::c_char,
            s_446,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_447: S = ts(
        tp(
            tc(
                b"{x+y}/[1;]\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}/[1;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            635 as libc::c_int,
            b"{x+y}/[1;]\0" as *const u8 as *const libc::c_char,
            b"{x+y}/[1;]\0" as *const u8 as *const libc::c_char,
            s_447,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_448: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}/[1;]2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            636 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"{x+y}/[1;]2 3 4\0" as *const u8 as *const libc::c_char,
            s_448,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_449: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"({x+y}/)[1;]2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            637 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"({x+y}/)[1;]2 3 4\0" as *const u8 as *const libc::c_char,
            s_449,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_450: S = ts(
        tp(
            tc(
                b"1 3 6 10\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}\\[1;]2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            638 as libc::c_int,
            b"1 3 6 10\0" as *const u8 as *const libc::c_char,
            b"{x+y}\\[1;]2 3 4\0" as *const u8 as *const libc::c_char,
            s_450,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_451: S = ts(
        tp(
            tc(
                b"{x+y}\\[1;]\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}\\[1;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            639 as libc::c_int,
            b"{x+y}\\[1;]\0" as *const u8 as *const libc::c_char,
            b"{x+y}\\[1;]\0" as *const u8 as *const libc::c_char,
            s_451,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_452: S = ts(
        tp(
            tc(
                b"{x+y}/[;1]\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}/[;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            640 as libc::c_int,
            b"{x+y}/[;1]\0" as *const u8 as *const libc::c_char,
            b"{x+y}/[;1]\0" as *const u8 as *const libc::c_char,
            s_452,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_453: S = ts(
        tp(
            tc(
                b"{x+y}'[1;]\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}'[1;]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            641 as libc::c_int,
            b"{x+y}'[1;]\0" as *const u8 as *const libc::c_char,
            b"{x+y}'[1;]\0" as *const u8 as *const libc::c_char,
            s_453,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_454: S = ts(
        tp(
            tc(
                b"3 4 5\0" as *const u8 as *const libc::c_char as S,
                b"{x+y}'[1;]2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            642 as libc::c_int,
            b"3 4 5\0" as *const u8 as *const libc::c_char,
            b"{x+y}'[1;]2 3 4\0" as *const u8 as *const libc::c_char,
            s_454,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_455: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"(,':)[1 2;3 4] ~ (1 2;4 3)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            643 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"(,':)[1 2;3 4] ~ (1 2;4 3)\0" as *const u8 as *const libc::c_char,
            s_455,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_456: S = ts(
        tp(
            tc(
                b"(\"ab\";,\"ab\";(\"ab\";\"cd\"))\0" as *const u8 as *const libc::c_char
                    as S,
                b"(2 2 # \"abcd\")[(0;,0;0 1)]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            645 as libc::c_int,
            b"(\"ab\";,\"ab\";(\"ab\";\"cd\"))\0" as *const u8 as *const libc::c_char,
            b"(2 2 # \"abcd\")[(0;,0;0 1)]\0" as *const u8 as *const libc::c_char,
            s_456,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_457: S = ts(
        tp(
            tc(
                b"(\"a\";)\0" as *const u8 as *const libc::c_char as S,
                b"\"a\",:[1;;\"b\"]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            646 as libc::c_int,
            b"(\"a\";)\0" as *const u8 as *const libc::c_char,
            b"\"a\",:[1;;\"b\"]\0" as *const u8 as *const libc::c_char,
            s_457,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_458: S = ts(
        tp(
            tc(
                b"0 1 2 3\0" as *const u8 as *const libc::c_char as S,
                b"a:b:!4; a[1]:9; b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            649 as libc::c_int,
            b"0 1 2 3\0" as *const u8 as *const libc::c_char,
            b"a:b:!4; a[1]:9; b\0" as *const u8 as *const libc::c_char,
            s_458,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_459: S = ts(
        tp(
            tc(
                b"0 1 2 3\0" as *const u8 as *const libc::c_char as S,
                b"a:b:!4; a[1 2]:8 9; b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            650 as libc::c_int,
            b"0 1 2 3\0" as *const u8 as *const libc::c_char,
            b"a:b:!4; a[1 2]:8 9; b\0" as *const u8 as *const libc::c_char,
            s_459,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_460: S = ts(
        tp(
            tc(
                b"(.1;.2;.3;.4)\0" as *const u8 as *const libc::c_char as S,
                b"a:b:(.1;.2;.3;.4); a[1]:.9; b\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            651 as libc::c_int,
            b"(.1;.2;.3;.4)\0" as *const u8 as *const libc::c_char,
            b"a:b:(.1;.2;.3;.4); a[1]:.9; b\0" as *const u8 as *const libc::c_char,
            s_460,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_461: S = ts(
        tp(
            tc(
                b"(.1;.2;.3;.4)\0" as *const u8 as *const libc::c_char as S,
                b"a:b:(.1;.2;.3;.4); a[1 2]:(.8;.9); b\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            652 as libc::c_int,
            b"(.1;.2;.3;.4)\0" as *const u8 as *const libc::c_char,
            b"a:b:(.1;.2;.3;.4); a[1 2]:(.8;.9); b\0" as *const u8
                as *const libc::c_char,
            s_461,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_462: S = ts(
        tp(
            tc(
                b"d:.((`a;1;);(`b;2;));d[!d]:d\0" as *const u8 as *const libc::c_char
                    as S,
                b"(.((`a;1;);(`b;2;));.((`a;1;);(`b;2;)))\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            653 as libc::c_int,
            b"d:.((`a;1;);(`b;2;));d[!d]:d\0" as *const u8 as *const libc::c_char,
            b"(.((`a;1;);(`b;2;));.((`a;1;);(`b;2;)))\0" as *const u8
                as *const libc::c_char,
            s_462,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_463: S = ts(
        tp(
            tc(
                b"x:2 2#!4;x[0]:x;x\0" as *const u8 as *const libc::c_char as S,
                b"((0 1;2 3);2 3)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            654 as libc::c_int,
            b"x:2 2#!4;x[0]:x;x\0" as *const u8 as *const libc::c_char,
            b"((0 1;2 3);2 3)\0" as *const u8 as *const libc::c_char,
            s_463,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_464: S = ts(
        tp(
            tc(
                b"x:!10;y:x;y[1]:100;x\0" as *const u8 as *const libc::c_char as S,
                b"!10\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            655 as libc::c_int,
            b"x:!10;y:x;y[1]:100;x\0" as *const u8 as *const libc::c_char,
            b"!10\0" as *const u8 as *const libc::c_char,
            s_464,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_465: S = ts(
        tp(
            tc(
                b"x:(1;1.0;\"1\");y:x;z:x;z[0]:2;y\0" as *const u8 as *const libc::c_char
                    as S,
                b"(1;1.0;\"1\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            656 as libc::c_int,
            b"x:(1;1.0;\"1\");y:x;z:x;z[0]:2;y\0" as *const u8 as *const libc::c_char,
            b"(1;1.0;\"1\")\0" as *const u8 as *const libc::c_char,
            s_465,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_466: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"x:.+(`a`b;1 2); y:x; .\"y.a:11; x.a\"\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            657 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"x:.+(`a`b;1 2); y:x; .\"y.a:11; x.a\"\0" as *const u8
                as *const libc::c_char,
            s_466,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_467: S = ts(
        tp(
            tc(
                b"x:.+(`a`b;1 2); y:x; y.a:11;x\0" as *const u8 as *const libc::c_char
                    as S,
                b".+(`a`b;1 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            658 as libc::c_int,
            b"x:.+(`a`b;1 2); y:x; y.a:11;x\0" as *const u8 as *const libc::c_char,
            b".+(`a`b;1 2)\0" as *const u8 as *const libc::c_char,
            s_467,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_468: S = ts(
        tp(
            tc(
                b"d:.+(`a`b;1 2);d[`a]:d\0" as *const u8 as *const libc::c_char as S,
                b".+(`a`b;1 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            659 as libc::c_int,
            b"d:.+(`a`b;1 2);d[`a]:d\0" as *const u8 as *const libc::c_char,
            b".+(`a`b;1 2)\0" as *const u8 as *const libc::c_char,
            s_468,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_469: S = ts(
        tp(
            tc(
                b"d:.+(`a`b;1 2);d.a:d\0" as *const u8 as *const libc::c_char as S,
                b".+(`a`b;1 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            660 as libc::c_int,
            b"d:.+(`a`b;1 2);d.a:d\0" as *const u8 as *const libc::c_char,
            b".+(`a`b;1 2)\0" as *const u8 as *const libc::c_char,
            s_469,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_470: S = ts(
        tp(
            tc(
                b"d:.+(`a`b;1 2); d[!d]:d\0" as *const u8 as *const libc::c_char as S,
                b"(.((`a;1;);(`b;2;));.((`a;1;);(`b;2;)))\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            661 as libc::c_int,
            b"d:.+(`a`b;1 2); d[!d]:d\0" as *const u8 as *const libc::c_char,
            b"(.((`a;1;);(`b;2;));.((`a;1;);(`b;2;)))\0" as *const u8
                as *const libc::c_char,
            s_470,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_471: S = ts(
        tp(
            tc(
                b".((`a;(9 9 9;9 9 9;9 9 9););(`b;2;))\0" as *const u8
                    as *const libc::c_char as S,
                b"d:.+(`a`b;(3 3#9;2));e:d;d.a[1;2]:e\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            662 as libc::c_int,
            b".((`a;(9 9 9;9 9 9;9 9 9););(`b;2;))\0" as *const u8
                as *const libc::c_char,
            b"d:.+(`a`b;(3 3#9;2));e:d;d.a[1;2]:e\0" as *const u8 as *const libc::c_char,
            s_471,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_472: S = ts(
        tp(
            tc(
                b"(.((`a;(9 9;9 9););(`b;2;));.((`a;(9 9;9 9););(`b;2;)))\0" as *const u8
                    as *const libc::c_char as S,
                b"d:.+(`a`b;(2 2#9;2));d[]:d\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            663 as libc::c_int,
            b"(.((`a;(9 9;9 9););(`b;2;));.((`a;(9 9;9 9););(`b;2;)))\0" as *const u8
                as *const libc::c_char,
            b"d:.+(`a`b;(2 2#9;2));d[]:d\0" as *const u8 as *const libc::c_char,
            s_472,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_473: S = ts(
        tp(
            tc(
                b"(2;(9 9 9;9 9 9;9 9 9))\0" as *const u8 as *const libc::c_char as S,
                b"d:.+(`a`b;(3 3#9;2));d[]:|d[]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            664 as libc::c_int,
            b"(2;(9 9 9;9 9 9;9 9 9))\0" as *const u8 as *const libc::c_char,
            b"d:.+(`a`b;(3 3#9;2));d[]:|d[]\0" as *const u8 as *const libc::c_char,
            s_473,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_474: S = ts(
        tp(
            tc(
                b"x:.((`a;(0 1 2;3 4 5;6 7 8););(`b;10;));(x;x;x)\0" as *const u8
                    as *const libc::c_char as S,
                b"d:.+(`a`b;(3 3#!9; 10));d.a[2]:(d;d;d)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            665 as libc::c_int,
            b"x:.((`a;(0 1 2;3 4 5;6 7 8););(`b;10;));(x;x;x)\0" as *const u8
                as *const libc::c_char,
            b"d:.+(`a`b;(3 3#!9; 10));d.a[2]:(d;d;d)\0" as *const u8
                as *const libc::c_char,
            s_474,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_475: S = ts(
        tp(
            tc(
                b".,(`d;;)\0" as *const u8 as *const libc::c_char as S,
                b"d:.k\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            666 as libc::c_int,
            b".,(`d;;)\0" as *const u8 as *const libc::c_char,
            b"d:.k\0" as *const u8 as *const libc::c_char,
            s_475,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_476: S = ts(
        tp(
            tc(
                b".,(`d;.,(`d;;);)\0" as *const u8 as *const libc::c_char as S,
                b"d:.k;d:.k;d\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            667 as libc::c_int,
            b".,(`d;.,(`d;;);)\0" as *const u8 as *const libc::c_char,
            b"d:.k;d:.k;d\0" as *const u8 as *const libc::c_char,
            s_476,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_477: S = ts(
        tp(
            tc(
                b".((`a;;);(`b;;))\0" as *const u8 as *const libc::c_char as S,
                b"a:b:.k;a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            668 as libc::c_int,
            b".((`a;;);(`b;;))\0" as *const u8 as *const libc::c_char,
            b"a:b:.k;a\0" as *const u8 as *const libc::c_char,
            s_477,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_478: S = ts(
        tp(
            tc(
                b".((`a;;);(`b;;))\0" as *const u8 as *const libc::c_char as S,
                b"a:b:.k;b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            669 as libc::c_int,
            b".((`a;;);(`b;;))\0" as *const u8 as *const libc::c_char,
            b"a:b:.k;b\0" as *const u8 as *const libc::c_char,
            s_478,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_479: S = ts(
        tp(
            tc(
                b".((`a;.,(`c;;););(`b;.,(`c;;);))\0" as *const u8 as *const libc::c_char
                    as S,
                b"c:.+(`a`b;(.k;.k));c\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            670 as libc::c_int,
            b".((`a;.,(`c;;););(`b;.,(`c;;);))\0" as *const u8 as *const libc::c_char,
            b"c:.+(`a`b;(.k;.k));c\0" as *const u8 as *const libc::c_char,
            s_479,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_480: S = ts(
        tp(
            tc(
                b"(.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;)))\0"
                    as *const u8 as *const libc::c_char as S,
                b"a:b:c:(.k;.k;.k);a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            671 as libc::c_int,
            b"(.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;)))\0"
                as *const u8 as *const libc::c_char,
            b"a:b:c:(.k;.k;.k);a\0" as *const u8 as *const libc::c_char,
            s_480,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_481: S = ts(
        tp(
            tc(
                b"(.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;)))\0"
                    as *const u8 as *const libc::c_char as S,
                b"a:b:c:(.k;.k;.k);b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            672 as libc::c_int,
            b"(.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;)))\0"
                as *const u8 as *const libc::c_char,
            b"a:b:c:(.k;.k;.k);b\0" as *const u8 as *const libc::c_char,
            s_481,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_482: S = ts(
        tp(
            tc(
                b"(.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;)))\0"
                    as *const u8 as *const libc::c_char as S,
                b"a:b:c:(.k;.k;.k);c\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            673 as libc::c_int,
            b"(.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;));.((`a;;);(`b;;);(`c;;)))\0"
                as *const u8 as *const libc::c_char,
            b"a:b:c:(.k;.k;.k);c\0" as *const u8 as *const libc::c_char,
            s_482,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_483: S = ts(
        tp(
            tc(
                b"(.((`x;;);(`y;;));.((`x;;);(`y;;)))\0" as *const u8
                    as *const libc::c_char as S,
                b"x:y:(.k;.k);x\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            674 as libc::c_int,
            b"(.((`x;;);(`y;;));.((`x;;);(`y;;)))\0" as *const u8 as *const libc::c_char,
            b"x:y:(.k;.k);x\0" as *const u8 as *const libc::c_char,
            s_483,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_484: S = ts(
        tp(
            tc(
                b"(.((`x;;);(`y;;));.((`x;;);(`y;;)))\0" as *const u8
                    as *const libc::c_char as S,
                b"x:y:(.k;.k);y\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            675 as libc::c_int,
            b"(.((`x;;);(`y;;));.((`x;;);(`y;;)))\0" as *const u8 as *const libc::c_char,
            b"x:y:(.k;.k);y\0" as *const u8 as *const libc::c_char,
            s_484,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_485: S = ts(
        tp(
            tc(
                b"3 4 5\0" as *const u8 as *const libc::c_char as S,
                b"x:2 3#!6; y:x; z:y[1]; y[1;1]:99; z\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            676 as libc::c_int,
            b"3 4 5\0" as *const u8 as *const libc::c_char,
            b"x:2 3#!6; y:x; z:y[1]; y[1;1]:99; z\0" as *const u8 as *const libc::c_char,
            s_485,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_486: S = ts(
        tp(
            tc(
                b"3.1 4.1 5.1\0" as *const u8 as *const libc::c_char as S,
                b"x:.1+2 3#!6; y:x; z:y[1]; y[1;1]:99.1; z\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            677 as libc::c_int,
            b"3.1 4.1 5.1\0" as *const u8 as *const libc::c_char,
            b"x:.1+2 3#!6; y:x; z:y[1]; y[1;1]:99.1; z\0" as *const u8
                as *const libc::c_char,
            s_486,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_487: S = ts(
        tp(
            tc(
                b"1 0\0" as *const u8 as *const libc::c_char as S,
                b"(\"a.c\";\"bc\") _sm \"*.[ch]\"\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            679 as libc::c_int,
            b"1 0\0" as *const u8 as *const libc::c_char,
            b"(\"a.c\";\"bc\") _sm \"*.[ch]\"\0" as *const u8 as *const libc::c_char,
            s_487,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_488: S = ts(
        tp(
            tc(
                b"0 1\0" as *const u8 as *const libc::c_char as S,
                b"($`one;$`two) _sm $`two\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            680 as libc::c_int,
            b"0 1\0" as *const u8 as *const libc::c_char,
            b"($`one;$`two) _sm $`two\0" as *const u8 as *const libc::c_char,
            s_488,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_489: S = ts(
        tp(
            tc(
                b"0 1\0" as *const u8 as *const libc::c_char as S,
                b"`one `two  _sm $`two\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            681 as libc::c_int,
            b"0 1\0" as *const u8 as *const libc::c_char,
            b"`one `two  _sm $`two\0" as *const u8 as *const libc::c_char,
            s_489,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_490: S = ts(
        tp(
            tc(
                b"$`xxx\0" as *const u8 as *const libc::c_char as S,
                b"`hostname _setenv $`xxx; _getenv `hostname\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            683 as libc::c_int,
            b"$`xxx\0" as *const u8 as *const libc::c_char,
            b"`hostname _setenv $`xxx; _getenv `hostname\0" as *const u8
                as *const libc::c_char,
            s_490,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_491: S = ts(
        tp(
            tc(
                b"\".k\"\0" as *const u8 as *const libc::c_char as S,
                b".\"\\\\d a\"; .\"\\\\d .k\"; $_d\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            685 as libc::c_int,
            b"\".k\"\0" as *const u8 as *const libc::c_char,
            b".\"\\\\d a\"; .\"\\\\d .k\"; $_d\0" as *const u8 as *const libc::c_char,
            s_491,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_492: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"a:.((`b;1);(`c;2)); `a[`b`c]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            686 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"a:.((`b;1);(`c;2)); `a[`b`c]\0" as *const u8 as *const libc::c_char,
            s_492,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_493: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"a:1; \\b:2; c:3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            687 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"a:1; \\b:2; c:3\0" as *const u8 as *const libc::c_char,
            s_493,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_494: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"5, a[5]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            688 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"5, a[5]\0" as *const u8 as *const libc::c_char,
            s_494,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_495: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"5 6, a[5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            689 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"5 6, a[5 6]\0" as *const u8 as *const libc::c_char,
            s_495,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_496: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"5, a 5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            690 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"5, a 5\0" as *const u8 as *const libc::c_char,
            s_496,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_497: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"5 6, a 5 6\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            691 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"5 6, a 5 6\0" as *const u8 as *const libc::c_char,
            s_497,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_498: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"a:{a[x]}; a 5,\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            692 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"a:{a[x]}; a 5,\0" as *const u8 as *const libc::c_char,
            s_498,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_499: S = ts(
        tp(
            tc(
                b"4 3 2 1 0\0" as *const u8 as *const libc::c_char as S,
                b"r:{:[x;x,_f[x-1];0]}; r[4]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            693 as libc::c_int,
            b"4 3 2 1 0\0" as *const u8 as *const libc::c_char,
            b"r:{:[x;x,_f[x-1];0]}; r[4]\0" as *const u8 as *const libc::c_char,
            s_499,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_500: S = ts(
        tp(
            tc(
                b"4 3 2 1 0\0" as *const u8 as *const libc::c_char as S,
                b"r:{:[x;x,r[x-1];0]}; r[4]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            694 as libc::c_int,
            b"4 3 2 1 0\0" as *const u8 as *const libc::c_char,
            b"r:{:[x;x,r[x-1];0]}; r[4]\0" as *const u8 as *const libc::c_char,
            s_500,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_501: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"`a.1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            695 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"`a.1\0" as *const u8 as *const libc::c_char,
            s_501,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_502: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"`a@1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            696 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"`a@1\0" as *const u8 as *const libc::c_char,
            s_502,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_503: S = ts(
        tp(
            tc(
                b"1 2 3\0" as *const u8 as *const libc::c_char as S,
                b"`a@1 2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            697 as libc::c_int,
            b"1 2 3\0" as *const u8 as *const libc::c_char,
            b"`a@1 2 3\0" as *const u8 as *const libc::c_char,
            s_503,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_504: S = ts(
        tp(
            tc(
                b"9\0" as *const u8 as *const libc::c_char as S,
                b"a:8 9 4; `a . 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            698 as libc::c_int,
            b"9\0" as *const u8 as *const libc::c_char,
            b"a:8 9 4; `a . 1\0" as *const u8 as *const libc::c_char,
            s_504,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_505: S = ts(
        tp(
            tc(
                b"9\0" as *const u8 as *const libc::c_char as S,
                b"a:8 9 4; `a @ 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            699 as libc::c_int,
            b"9\0" as *const u8 as *const libc::c_char,
            b"a:8 9 4; `a @ 1\0" as *const u8 as *const libc::c_char,
            s_505,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_506: S = ts(
        tp(
            tc(
                b"9 4\0" as *const u8 as *const libc::c_char as S,
                b"a:8 9 4; `a @ 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            700 as libc::c_int,
            b"9 4\0" as *const u8 as *const libc::c_char,
            b"a:8 9 4; `a @ 1 2\0" as *const u8 as *const libc::c_char,
            s_506,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_507: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"m:3 4#!12; m[1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            701 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"m:3 4#!12; m[1;2]\0" as *const u8 as *const libc::c_char,
            s_507,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_508: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"m:3 4#!12; m\\2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            702 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"m:3 4#!12; m\\2\0" as *const u8 as *const libc::c_char,
            s_508,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_509: S = ts(
        tp(
            tc(
                b"2 2#0\0" as *const u8 as *const libc::c_char as S,
                b"2<2 2#1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            703 as libc::c_int,
            b"2 2#0\0" as *const u8 as *const libc::c_char,
            b"2<2 2#1\0" as *const u8 as *const libc::c_char,
            s_509,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_510: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"m:3 4#!12; 1 m\\2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            704 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"m:3 4#!12; 1 m\\2\0" as *const u8 as *const libc::c_char,
            s_510,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_511: S = ts(
        tp(
            tc(
                b"2 3 7 43 1807 3263443\0" as *const u8 as *const libc::c_char as S,
                b"{x, 1+*/x}/[5;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            705 as libc::c_int,
            b"2 3 7 43 1807 3263443\0" as *const u8 as *const libc::c_char,
            b"{x, 1+*/x}/[5;2]\0" as *const u8 as *const libc::c_char,
            s_511,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_512: S = ts(
        tp(
            tc(
                b"\0" as *const u8 as *const libc::c_char as S,
                b"/$!2 2, (,\"0\";,\"0\";,\"0\";,\"1\";,\"1\";,\"0\";,\"1\";,\"1\")\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            706 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char,
            b"/$!2 2, (,\"0\";,\"0\";,\"0\";,\"1\";,\"1\";,\"0\";,\"1\";,\"1\")\0"
                as *const u8 as *const libc::c_char,
            s_512,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_513: S = ts(
        tp(
            tc(
                b"13\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+1}; 2 f/11\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            707 as libc::c_int,
            b"13\0" as *const u8 as *const libc::c_char,
            b"f:{x+1}; 2 f/11\0" as *const u8 as *const libc::c_char,
            s_513,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_514: S = ts(
        tp(
            tc(
                b"13\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+1}; f/[2;11]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            708 as libc::c_int,
            b"13\0" as *const u8 as *const libc::c_char,
            b"f:{x+1}; f/[2;11]\0" as *const u8 as *const libc::c_char,
            s_514,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_515: S = ts(
        tp(
            tc(
                b"13\0" as *const u8 as *const libc::c_char as S,
                b"f:{x+1}; f/[;11] 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            709 as libc::c_int,
            b"13\0" as *const u8 as *const libc::c_char,
            b"f:{x+1}; f/[;11] 2\0" as *const u8 as *const libc::c_char,
            s_515,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_516: S = ts(
        tp(
            tc(
                b"1968 880\0" as *const u8 as *const libc::c_char as S,
                b"_mul[(3 5;1 3)]/[;1 0] 5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            710 as libc::c_int,
            b"1968 880\0" as *const u8 as *const libc::c_char,
            b"_mul[(3 5;1 3)]/[;1 0] 5\0" as *const u8 as *const libc::c_char,
            s_516,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_517: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"#\"   \"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            711 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"#\"   \"\0" as *const u8 as *const libc::c_char,
            s_517,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_518: S = ts(
        tp(
            tc(
                b".[{x};1;:]\0" as *const u8 as *const libc::c_char as S,
                b"0 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            712 as libc::c_int,
            b".[{x};1;:]\0" as *const u8 as *const libc::c_char,
            b"0 1\0" as *const u8 as *const libc::c_char,
            s_518,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_519: S = ts(
        tp(
            tc(
                b".[{x};1 2;:]\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"valence\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            713 as libc::c_int,
            b".[{x};1 2;:]\0" as *const u8 as *const libc::c_char,
            b"(1;\"valence\")\0" as *const u8 as *const libc::c_char,
            s_519,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_520: S = ts(
        tp(
            tc(
                b".[{x};\"2\";:]\0" as *const u8 as *const libc::c_char as S,
                b"(0;\"2\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            714 as libc::c_int,
            b".[{x};\"2\";:]\0" as *const u8 as *const libc::c_char,
            b"(0;\"2\")\0" as *const u8 as *const libc::c_char,
            s_520,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_521: S = ts(
        tp(
            tc(
                b".[{x};\"2+2\";:]\0" as *const u8 as *const libc::c_char as S,
                b"(0;\"2+2\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            715 as libc::c_int,
            b".[{x};\"2+2\";:]\0" as *const u8 as *const libc::c_char,
            b"(0;\"2+2\")\0" as *const u8 as *const libc::c_char,
            s_521,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_522: S = ts(
        tp(
            tc(
                b".[{. x};\"2+2\";:]\0" as *const u8 as *const libc::c_char as S,
                b"0 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            716 as libc::c_int,
            b".[{. x};\"2+2\";:]\0" as *const u8 as *const libc::c_char,
            b"0 4\0" as *const u8 as *const libc::c_char,
            s_522,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_523: S = ts(
        tp(
            tc(
                b"4 5 6>5\0" as *const u8 as *const libc::c_char as S,
                b"0 0 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            717 as libc::c_int,
            b"4 5 6>5\0" as *const u8 as *const libc::c_char,
            b"0 0 1\0" as *const u8 as *const libc::c_char,
            s_523,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_524: S = ts(
        tp(
            tc(
                b"4 5 6<5\0" as *const u8 as *const libc::c_char as S,
                b"1 0 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            718 as libc::c_int,
            b"4 5 6<5\0" as *const u8 as *const libc::c_char,
            b"1 0 0\0" as *const u8 as *const libc::c_char,
            s_524,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_525: S = ts(
        tp(
            tc(
                b"5<4 5 6\0" as *const u8 as *const libc::c_char as S,
                b"0 0 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            719 as libc::c_int,
            b"5<4 5 6\0" as *const u8 as *const libc::c_char,
            b"0 0 1\0" as *const u8 as *const libc::c_char,
            s_525,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_526: S = ts(
        tp(
            tc(
                b"5>4 5 6\0" as *const u8 as *const libc::c_char as S,
                b"1 0 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            720 as libc::c_int,
            b"5>4 5 6\0" as *const u8 as *const libc::c_char,
            b"1 0 0\0" as *const u8 as *const libc::c_char,
            s_526,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_527: S = ts(
        tp(
            tc(
                b"b:.[+;(1;\"b\");:]; b[1]\0" as *const u8 as *const libc::c_char as S,
                b"\"type\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            721 as libc::c_int,
            b"b:.[+;(1;\"b\");:]; b[1]\0" as *const u8 as *const libc::c_char,
            b"\"type\"\0" as *const u8 as *const libc::c_char,
            s_527,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_528: S = ts(
        tp(
            tc(
                b"'\"abc\"\0" as *const u8 as *const libc::c_char as S,
                b"\"abc\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            722 as libc::c_int,
            b"'\"abc\"\0" as *const u8 as *const libc::c_char,
            b"\"abc\"\0" as *const u8 as *const libc::c_char,
            s_528,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_529: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"fac:{:[x>1;x*fac[x-1];1]}; fac 3\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            723 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"fac:{:[x>1;x*fac[x-1];1]}; fac 3\0" as *const u8 as *const libc::c_char,
            s_529,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_530: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"fac:{:[x>1;x*fac x-1;1]}; fac 3\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            724 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"fac:{:[x>1;x*fac x-1;1]}; fac 3\0" as *const u8 as *const libc::c_char,
            s_530,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_531: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"fac:{:[x>1;x*_f[x-1];1]}; fac 3\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            725 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"fac:{:[x>1;x*_f[x-1];1]}; fac 3\0" as *const u8 as *const libc::c_char,
            s_531,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_532: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"fac:{:[x>1;x*_f x-1;1]}; fac 3\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            726 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"fac:{:[x>1;x*_f x-1;1]}; fac 3\0" as *const u8 as *const libc::c_char,
            s_532,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_533: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"1 2 3 . (,1)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            727 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"1 2 3 . (,1)\0" as *const u8 as *const libc::c_char,
            s_533,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_534: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"(-0i) = -0I\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            728 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"(-0i) = -0I\0" as *const u8 as *const libc::c_char,
            s_534,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_535: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0n = 0N\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            729 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0n = 0N\0" as *const u8 as *const libc::c_char,
            s_535,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_536: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0n < 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            730 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0n < 0\0" as *const u8 as *const libc::c_char,
            s_536,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_537: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"0n > 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            731 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"0n > 0\0" as *const u8 as *const libc::c_char,
            s_537,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_538: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0n < 0i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            732 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0n < 0i\0" as *const u8 as *const libc::c_char,
            s_538,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_539: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0n < -0i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            733 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0n < -0i\0" as *const u8 as *const libc::c_char,
            s_539,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_540: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0n < 0I\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            734 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0n < 0I\0" as *const u8 as *const libc::c_char,
            s_540,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_541: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0n < -0I\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            735 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0n < -0I\0" as *const u8 as *const libc::c_char,
            s_541,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_542: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0N < 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            736 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0N < 0\0" as *const u8 as *const libc::c_char,
            s_542,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_543: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"0N > 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            737 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"0N > 0\0" as *const u8 as *const libc::c_char,
            s_543,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_544: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0N < 0i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            738 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0N < 0i\0" as *const u8 as *const libc::c_char,
            s_544,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_545: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0N < -0i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            739 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0N < -0i\0" as *const u8 as *const libc::c_char,
            s_545,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_546: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0N < 0I\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            740 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0N < 0I\0" as *const u8 as *const libc::c_char,
            s_546,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_547: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0N < -0I\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            741 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0N < -0I\0" as *const u8 as *const libc::c_char,
            s_547,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_548: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"2=1.999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            742 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"2=1.999999999999\0" as *const u8 as *const libc::c_char,
            s_548,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_549: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"2=1.9999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            743 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"2=1.9999999999999\0" as *const u8 as *const libc::c_char,
            s_549,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_550: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"_1.999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            744 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"_1.999999999999\0" as *const u8 as *const libc::c_char,
            s_550,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_551: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"_1.9999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            745 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"_1.9999999999999\0" as *const u8 as *const libc::c_char,
            s_551,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_552: S = ts(
        tp(
            tc(
                b"1.0\0" as *const u8 as *const libc::c_char as S,
                b"_floor 1.999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            746 as libc::c_int,
            b"1.0\0" as *const u8 as *const libc::c_char,
            b"_floor 1.999999999999\0" as *const u8 as *const libc::c_char,
            s_552,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_553: S = ts(
        tp(
            tc(
                b"1.0\0" as *const u8 as *const libc::c_char as S,
                b"_floor 1.9999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            747 as libc::c_int,
            b"1.0\0" as *const u8 as *const libc::c_char,
            b"_floor 1.9999999999999\0" as *const u8 as *const libc::c_char,
            s_553,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_554: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"4.9406564584124654e-324=-4.9406564584124654e-324\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            748 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"4.9406564584124654e-324=-4.9406564584124654e-324\0" as *const u8
                as *const libc::c_char,
            s_554,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_555: S = ts(
        tp(
            tc(
                b"0N\0" as *const u8 as *const libc::c_char as S,
                b"_ 0n\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            749 as libc::c_int,
            b"0N\0" as *const u8 as *const libc::c_char,
            b"_ 0n\0" as *const u8 as *const libc::c_char,
            s_555,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_556: S = ts(
        tp(
            tc(
                b"0I\0" as *const u8 as *const libc::c_char as S,
                b"_ 0i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            750 as libc::c_int,
            b"0I\0" as *const u8 as *const libc::c_char,
            b"_ 0i\0" as *const u8 as *const libc::c_char,
            s_556,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_557: S = ts(
        tp(
            tc(
                b"-0I\0" as *const u8 as *const libc::c_char as S,
                b"_ -0i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            751 as libc::c_int,
            b"-0I\0" as *const u8 as *const libc::c_char,
            b"_ -0i\0" as *const u8 as *const libc::c_char,
            s_557,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_558: S = ts(
        tp(
            tc(
                b"-0I\0" as *const u8 as *const libc::c_char as S,
                b"_ -5+1.0*1+-0I\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            752 as libc::c_int,
            b"-0I\0" as *const u8 as *const libc::c_char,
            b"_ -5+1.0*1+-0I\0" as *const u8 as *const libc::c_char,
            s_558,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_559: S = ts(
        tp(
            tc(
                b"1.618033988749908447\0" as *const u8 as *const libc::c_char as S,
                b"?[{(x^2)+(-x)+(-1)};0;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            753 as libc::c_int,
            b"1.618033988749908447\0" as *const u8 as *const libc::c_char,
            b"?[{(x^2)+(-x)+(-1)};0;1]\0" as *const u8 as *const libc::c_char,
            s_559,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_560: S = ts(
        tp(
            tc(
                b"-0.6180339887194686854\0" as *const u8 as *const libc::c_char as S,
                b"?[{(x^2)+(-x)+(-1)};0;.25]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            754 as libc::c_int,
            b"-0.6180339887194686854\0" as *const u8 as *const libc::c_char,
            b"?[{(x^2)+(-x)+(-1)};0;.25]\0" as *const u8 as *const libc::c_char,
            s_560,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_561: S = ts(
        tp(
            tc(
                b"0n -0i 0 0i\0" as *const u8 as *const libc::c_char as S,
                b"v@<v:0. 0i -0i 0n\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            755 as libc::c_int,
            b"0n -0i 0 0i\0" as *const u8 as *const libc::c_char,
            b"v@<v:0. 0i -0i 0n\0" as *const u8 as *const libc::c_char,
            s_561,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_562: S = ts(
        tp(
            tc(
                b"0 1 2 3\0" as *const u8 as *const libc::c_char as S,
                b"<(\"aaa\";\"bb\";,\"c\";\"d\")\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            756 as libc::c_int,
            b"0 1 2 3\0" as *const u8 as *const libc::c_char,
            b"<(\"aaa\";\"bb\";,\"c\";\"d\")\0" as *const u8 as *const libc::c_char,
            s_562,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_563: S = ts(
        tp(
            tc(
                b"0 1 3 2\0" as *const u8 as *const libc::c_char as S,
                b"<(\"aaa\";\"bb\";\"c\";,\"d\")\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            757 as libc::c_int,
            b"0 1 3 2\0" as *const u8 as *const libc::c_char,
            b"<(\"aaa\";\"bb\";\"c\";,\"d\")\0" as *const u8 as *const libc::c_char,
            s_563,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_564: S = ts(
        tp(
            tc(
                b"0 1\0" as *const u8 as *const libc::c_char as S,
                b"<(\"abd\";\"ad\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            758 as libc::c_int,
            b"0 1\0" as *const u8 as *const libc::c_char,
            b"<(\"abd\";\"ad\")\0" as *const u8 as *const libc::c_char,
            s_564,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_565: S = ts(
        tp(
            tc(
                b"1 0\0" as *const u8 as *const libc::c_char as S,
                b"<(\"ad\";\"abd\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            759 as libc::c_int,
            b"1 0\0" as *const u8 as *const libc::c_char,
            b"<(\"ad\";\"abd\")\0" as *const u8 as *const libc::c_char,
            s_565,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_566: S = ts(
        tp(
            tc(
                b"3 2 0 1\0" as *const u8 as *const libc::c_char as S,
                b">(\"ab\";,\"a\";\"aba\";\"a\")\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            760 as libc::c_int,
            b"3 2 0 1\0" as *const u8 as *const libc::c_char,
            b">(\"ab\";,\"a\";\"aba\";\"a\")\0" as *const u8 as *const libc::c_char,
            s_566,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_567: S = ts(
        tp(
            tc(
                b"0 1\0" as *const u8 as *const libc::c_char as S,
                b"<(\"\";,\"\\000\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            761 as libc::c_int,
            b"0 1\0" as *const u8 as *const libc::c_char,
            b"<(\"\";,\"\\000\")\0" as *const u8 as *const libc::c_char,
            s_567,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_568: S = ts(
        tp(
            tc(
                b"1 0\0" as *const u8 as *const libc::c_char as S,
                b">(\"\";,\"\\000\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            762 as libc::c_int,
            b"1 0\0" as *const u8 as *const libc::c_char,
            b">(\"\";,\"\\000\")\0" as *const u8 as *const libc::c_char,
            s_568,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_569: S = ts(
        tp(
            tc(
                b"0 1 2\0" as *const u8 as *const libc::c_char as S,
                b"<(\"\";,\"\\000\";\" \")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            763 as libc::c_int,
            b"0 1 2\0" as *const u8 as *const libc::c_char,
            b"<(\"\";,\"\\000\";\" \")\0" as *const u8 as *const libc::c_char,
            s_569,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_570: S = ts(
        tp(
            tc(
                b"2 1 0\0" as *const u8 as *const libc::c_char as S,
                b">(\"\";,\"\\000\";\" \")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            764 as libc::c_int,
            b"2 1 0\0" as *const u8 as *const libc::c_char,
            b">(\"\";,\"\\000\";\" \")\0" as *const u8 as *const libc::c_char,
            s_570,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_571: S = ts(
        tp(
            tc(
                b"1 2 0\0" as *const u8 as *const libc::c_char as S,
                b"<(\" \";\"\";,\"\\000\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            765 as libc::c_int,
            b"1 2 0\0" as *const u8 as *const libc::c_char,
            b"<(\" \";\"\";,\"\\000\")\0" as *const u8 as *const libc::c_char,
            s_571,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_572: S = ts(
        tp(
            tc(
                b"0 2 1\0" as *const u8 as *const libc::c_char as S,
                b">(\" \";\"\";,\"\\000\")\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            766 as libc::c_int,
            b"0 2 1\0" as *const u8 as *const libc::c_char,
            b">(\" \";\"\";,\"\\000\")\0" as *const u8 as *const libc::c_char,
            s_572,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_573: S = ts(
        tp(
            tc(
                b"0 1 2\0" as *const u8 as *const libc::c_char as S,
                b"<(,\"\\000\";\"\\000\\000\";\"\\000\\000\\000\")\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            767 as libc::c_int,
            b"0 1 2\0" as *const u8 as *const libc::c_char,
            b"<(,\"\\000\";\"\\000\\000\";\"\\000\\000\\000\")\0" as *const u8
                as *const libc::c_char,
            s_573,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_574: S = ts(
        tp(
            tc(
                b"2 1 0\0" as *const u8 as *const libc::c_char as S,
                b">(,\"\\000\";\"\\000\\000\";\"\\000\\000\\000\")\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            768 as libc::c_int,
            b"2 1 0\0" as *const u8 as *const libc::c_char,
            b">(,\"\\000\";\"\\000\\000\";\"\\000\\000\\000\")\0" as *const u8
                as *const libc::c_char,
            s_574,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_575: S = ts(
        tp(
            tc(
                b"(,0;1 5;,2;,3;,4)\0" as *const u8 as *const libc::c_char as S,
                b"=\"arthur\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            769 as libc::c_int,
            b"(,0;1 5;,2;,3;,4)\0" as *const u8 as *const libc::c_char,
            b"=\"arthur\"\0" as *const u8 as *const libc::c_char,
            s_575,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_576: S = ts(
        tp(
            tc(
                b"(0 4 5;1 6;,2;,3)\0" as *const u8 as *const libc::c_char as S,
                b"=``a`b`c```a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            770 as libc::c_int,
            b"(0 4 5;1 6;,2;,3)\0" as *const u8 as *const libc::c_char,
            b"=``a`b`c```a\0" as *const u8 as *const libc::c_char,
            s_576,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_577: S = ts(
        tp(
            tc(
                b"(,0)\0" as *const u8 as *const libc::c_char as S,
                b"(?,0)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            771 as libc::c_int,
            b"(,0)\0" as *const u8 as *const libc::c_char,
            b"(?,0)\0" as *const u8 as *const libc::c_char,
            s_577,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_578: S = ts(
        tp(
            tc(
                b"(,\"\\000\";\"\\000\\000\";\"\\000\\000\\000\")\0" as *const u8
                    as *const libc::c_char as S,
                b"?(,\"\\000\";\"\\000\\000\";\"\\000\\000\\000\")\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            772 as libc::c_int,
            b"(,\"\\000\";\"\\000\\000\";\"\\000\\000\\000\")\0" as *const u8
                as *const libc::c_char,
            b"?(,\"\\000\";\"\\000\\000\";\"\\000\\000\\000\")\0" as *const u8
                as *const libc::c_char,
            s_578,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_579: S = ts(
        tp(
            tc(
                b"(\"\\000a\";\"\\000b\";\"\\000c\")\0" as *const u8
                    as *const libc::c_char as S,
                b"?(\"\\000a\";\"\\000b\";\"\\000c\")\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            773 as libc::c_int,
            b"(\"\\000a\";\"\\000b\";\"\\000c\")\0" as *const u8 as *const libc::c_char,
            b"?(\"\\000a\";\"\\000b\";\"\\000c\")\0" as *const u8 as *const libc::c_char,
            s_579,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_580: S = ts(
        tp(
            tc(
                b"1 0 1 0\0" as *const u8 as *const libc::c_char as S,
                b"x:1 0 1 0; +\\x; x\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            774 as libc::c_int,
            b"1 0 1 0\0" as *const u8 as *const libc::c_char,
            b"x:1 0 1 0; +\\x; x\0" as *const u8 as *const libc::c_char,
            s_580,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_581: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"c:2; .k[\"c+1\"]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            775 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"c:2; .k[\"c+1\"]\0" as *const u8 as *const libc::c_char,
            s_581,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_582: S = ts(
        tp(
            tc(
                b"43\0" as *const u8 as *const libc::c_char as S,
                b"c:2; d.c:42; d[\"c+1\"]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            776 as libc::c_int,
            b"43\0" as *const u8 as *const libc::c_char,
            b"c:2; d.c:42; d[\"c+1\"]\0" as *const u8 as *const libc::c_char,
            s_582,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_583: S = ts(
        tp(
            tc(
                b"`a`b`c\0" as *const u8 as *const libc::c_char as S,
                b"a:1;b:2;c:3; .\"\\\\v\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            777 as libc::c_int,
            b"`a`b`c\0" as *const u8 as *const libc::c_char,
            b"a:1;b:2;c:3; .\"\\\\v\"\0" as *const u8 as *const libc::c_char,
            s_583,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_584: S = ts(
        tp(
            tc(
                b"2 2.0\0" as *const u8 as *const libc::c_char as S,
                b"?2. 1.9999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            778 as libc::c_int,
            b"2 2.0\0" as *const u8 as *const libc::c_char,
            b"?2. 1.9999999999999\0" as *const u8 as *const libc::c_char,
            s_584,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_585: S = ts(
        tp(
            tc(
                b"(,0;,1)\0" as *const u8 as *const libc::c_char as S,
                b"=2. 1.9999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            779 as libc::c_int,
            b"(,0;,1)\0" as *const u8 as *const libc::c_char,
            b"=2. 1.9999999999999\0" as *const u8 as *const libc::c_char,
            s_585,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_586: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"v:1.9999999999999 2.;v?2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            780 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"v:1.9999999999999 2.;v?2\0" as *const u8 as *const libc::c_char,
            s_586,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_587: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"v:1.9999999999999 2.;v?2.\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            781 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"v:1.9999999999999 2.;v?2.\0" as *const u8 as *const libc::c_char,
            s_587,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_588: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"v:1.9999999999999 2.;(v;_hash v)?2\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            782 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"v:1.9999999999999 2.;(v;_hash v)?2\0" as *const u8 as *const libc::c_char,
            s_588,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_589: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"v:1.9999999999999 2.;(v;_hash v)?2.\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            783 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"v:1.9999999999999 2.;(v;_hash v)?2.\0" as *const u8 as *const libc::c_char,
            s_589,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_590: S = ts(
        tp(
            tc(
                b"()\0" as *const u8 as *const libc::c_char as S,
                b"\"\",()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            784 as libc::c_int,
            b"()\0" as *const u8 as *const libc::c_char,
            b"\"\",()\0" as *const u8 as *const libc::c_char,
            s_590,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_591: S = ts(
        tp(
            tc(
                b"\"\"\0" as *const u8 as *const libc::c_char as S,
                b"\"\",/()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            785 as libc::c_int,
            b"\"\"\0" as *const u8 as *const libc::c_char,
            b"\"\",/()\0" as *const u8 as *const libc::c_char,
            s_591,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_592: S = ts(
        tp(
            tc(
                b"(,0)\0" as *const u8 as *const libc::c_char as S,
                b"0,/()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            786 as libc::c_int,
            b"(,0)\0" as *const u8 as *const libc::c_char,
            b"0,/()\0" as *const u8 as *const libc::c_char,
            s_592,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_593: S = ts(
        tp(
            tc(
                b"201512 12\0" as *const u8 as *const libc::c_char as S,
                b"0 100_vs 20151212\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            787 as libc::c_int,
            b"201512 12\0" as *const u8 as *const libc::c_char,
            b"0 100_vs 20151212\0" as *const u8 as *const libc::c_char,
            s_593,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_594: S = ts(
        tp(
            tc(
                b"-1 0\0" as *const u8 as *const libc::c_char as S,
                b"0 100_vs-100\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            788 as libc::c_int,
            b"-1 0\0" as *const u8 as *const libc::c_char,
            b"0 100_vs-100\0" as *const u8 as *const libc::c_char,
            s_594,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_595: S = ts(
        tp(
            tc(
                b"\"\"\0" as *const u8 as *const libc::c_char as S,
                b"\" \"/\"\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            789 as libc::c_int,
            b"\"\"\0" as *const u8 as *const libc::c_char,
            b"\" \"/\"\"\0" as *const u8 as *const libc::c_char,
            s_595,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_596: S = ts(
        tp(
            tc(
                b"\" \"\0" as *const u8 as *const libc::c_char as S,
                b"\" \"/\" \"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            790 as libc::c_int,
            b"\" \"\0" as *const u8 as *const libc::c_char,
            b"\" \"/\" \"\0" as *const u8 as *const libc::c_char,
            s_596,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_597: S = ts(
        tp(
            tc(
                b"\" \"\0" as *const u8 as *const libc::c_char as S,
                b"\";\"/\" \"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            791 as libc::c_int,
            b"\" \"\0" as *const u8 as *const libc::c_char,
            b"\";\"/\" \"\0" as *const u8 as *const libc::c_char,
            s_597,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_598: S = ts(
        tp(
            tc(
                b"\" ; ; \"\0" as *const u8 as *const libc::c_char as S,
                b"\";\"/(,\" \";,\" \";,\" \")\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            792 as libc::c_int,
            b"\" ; ; \"\0" as *const u8 as *const libc::c_char,
            b"\";\"/(,\" \";,\" \";,\" \")\0" as *const u8 as *const libc::c_char,
            s_598,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_599: S = ts(
        tp(
            tc(
                b"(,\"alpha\")\0" as *const u8 as *const libc::c_char as S,
                b"\" \"\\\"alpha\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            793 as libc::c_int,
            b"(,\"alpha\")\0" as *const u8 as *const libc::c_char,
            b"\" \"\\\"alpha\"\0" as *const u8 as *const libc::c_char,
            s_599,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_600: S = ts(
        tp(
            tc(
                b"(\"\";\"lph\";\"\")\0" as *const u8 as *const libc::c_char as S,
                b"\"a\"\\\"alpha\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            794 as libc::c_int,
            b"(\"\";\"lph\";\"\")\0" as *const u8 as *const libc::c_char,
            b"\"a\"\\\"alpha\"\0" as *const u8 as *const libc::c_char,
            s_600,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_601: S = ts(
        tp(
            tc(
                b"(,\"a\";\"pha\")\0" as *const u8 as *const libc::c_char as S,
                b"\"l\"\\\"alpha\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            795 as libc::c_int,
            b"(,\"a\";\"pha\")\0" as *const u8 as *const libc::c_char,
            b"\"l\"\\\"alpha\"\0" as *const u8 as *const libc::c_char,
            s_601,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_602: S = ts(
        tp(
            tc(
                b"(\"\";\"\";\"\";\"\")\0" as *const u8 as *const libc::c_char as S,
                b"\"a\"\\\"aaa\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            796 as libc::c_int,
            b"(\"\";\"\";\"\";\"\")\0" as *const u8 as *const libc::c_char,
            b"\"a\"\\\"aaa\"\0" as *const u8 as *const libc::c_char,
            s_602,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_603: S = ts(
        tp(
            tc(
                b"x:.+(`a`b;1 2); y:x; f:{.[x;,`a;:;11]}; f`y; x\0" as *const u8
                    as *const libc::c_char as S,
                b".+(`a`b;1 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            797 as libc::c_int,
            b"x:.+(`a`b;1 2); y:x; f:{.[x;,`a;:;11]}; f`y; x\0" as *const u8
                as *const libc::c_char,
            b".+(`a`b;1 2)\0" as *const u8 as *const libc::c_char,
            s_603,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_604: S = ts(
        tp(
            tc(
                b"x:.+(`a`b;1 2); y:x; f:{.[x;,`a;:;11]}; f`y; y\0" as *const u8
                    as *const libc::c_char as S,
                b".+(`a`b;11 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            798 as libc::c_int,
            b"x:.+(`a`b;1 2); y:x; f:{.[x;,`a;:;11]}; f`y; y\0" as *const u8
                as *const libc::c_char,
            b".+(`a`b;11 2)\0" as *const u8 as *const libc::c_char,
            s_604,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_605: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"-20<10\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            799 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"-20<10\0" as *const u8 as *const libc::c_char,
            s_605,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_606: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"-20.0<10.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            800 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"-20.0<10.0\0" as *const u8 as *const libc::c_char,
            s_606,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_607: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"-10.0<20\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            801 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"-10.0<20\0" as *const u8 as *const libc::c_char,
            s_607,
        );
    }
    test_print = 0 as libc::c_int as I;
    return 0 as libc::c_int as I;
}
unsafe extern "C" fn tests01() -> I {
    let mut s: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            882 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char,
            s,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_0: S = ts(
        tp(
            tc(
                b"1 1\0" as *const u8 as *const libc::c_char as S,
                b"1 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            883 as libc::c_int,
            b"1 1\0" as *const u8 as *const libc::c_char,
            b"1 1\0" as *const u8 as *const libc::c_char,
            s_0,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_1: S = ts(
        tp(
            tc(
                b"1 1\0" as *const u8 as *const libc::c_char as S,
                b"1,1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            884 as libc::c_int,
            b"1 1\0" as *const u8 as *const libc::c_char,
            b"1,1\0" as *const u8 as *const libc::c_char,
            s_1,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_2: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"1+1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            885 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"1+1\0" as *const u8 as *const libc::c_char,
            s_2,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_3: S = ts(
        tp(
            tc(
                b"()\0" as *const u8 as *const libc::c_char as S,
                b"()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            886 as libc::c_int,
            b"()\0" as *const u8 as *const libc::c_char,
            b"()\0" as *const u8 as *const libc::c_char,
            s_3,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_4: S = ts(
        tp(
            tc(
                b"()\0" as *const u8 as *const libc::c_char as S,
                b"1 2 3@()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            887 as libc::c_int,
            b"()\0" as *const u8 as *const libc::c_char,
            b"1 2 3@()\0" as *const u8 as *const libc::c_char,
            s_4,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_5: S = ts(
        tp(
            tc(
                b"()\0" as *const u8 as *const libc::c_char as S,
                b"(1 2 3)()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            888 as libc::c_int,
            b"()\0" as *const u8 as *const libc::c_char,
            b"(1 2 3)()\0" as *const u8 as *const libc::c_char,
            s_5,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_6: S = ts(
        tp(
            tc(
                b"1 2 3\0" as *const u8 as *const libc::c_char as S,
                b"(1 2 3).()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            889 as libc::c_int,
            b"1 2 3\0" as *const u8 as *const libc::c_char,
            b"(1 2 3).()\0" as *const u8 as *const libc::c_char,
            s_6,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_7: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"3 4[0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            890 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"3 4[0]\0" as *const u8 as *const libc::c_char,
            s_7,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_8: S = ts(
        tp(
            tc(
                b"10+1+\0" as *const u8 as *const libc::c_char as S,
                b"10 + 1 2[0] +\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            891 as libc::c_int,
            b"10+1+\0" as *const u8 as *const libc::c_char,
            b"10 + 1 2[0] +\0" as *const u8 as *const libc::c_char,
            s_8,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_9: S = ts(
        tp(
            tc(
                b"7 6\0" as *const u8 as *const libc::c_char as S,
                b"1+1+1+1+1+|||1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            892 as libc::c_int,
            b"7 6\0" as *const u8 as *const libc::c_char,
            b"1+1+1+1+1+|||1 2\0" as *const u8 as *const libc::c_char,
            s_9,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_10: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b";\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            893 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b";\0" as *const u8 as *const libc::c_char,
            s_10,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_11: S = ts(
        tp(
            tc(
                b"(;)\0" as *const u8 as *const libc::c_char as S,
                b"(;)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            894 as libc::c_int,
            b"(;)\0" as *const u8 as *const libc::c_char,
            b"(;)\0" as *const u8 as *const libc::c_char,
            s_11,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_12: S = ts(
        tp(
            tc(
                b"(;;)\0" as *const u8 as *const libc::c_char as S,
                b"(;;)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            895 as libc::c_int,
            b"(;;)\0" as *const u8 as *const libc::c_char,
            b"(;;)\0" as *const u8 as *const libc::c_char,
            s_12,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_13: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"(((((1)))))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            896 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"(((((1)))))\0" as *const u8 as *const libc::c_char,
            s_13,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_14: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"1+(1+(1+(1+1)))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            897 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"1+(1+(1+(1+1)))\0" as *const u8 as *const libc::c_char,
            s_14,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_15: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"1+(((((1)))))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            898 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"1+(((((1)))))\0" as *const u8 as *const libc::c_char,
            s_15,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_16: S = ts(
        tp(
            tc(
                b"2 2\0" as *const u8 as *const libc::c_char as S,
                b"(2 2;3 4)[0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            899 as libc::c_int,
            b"2 2\0" as *const u8 as *const libc::c_char,
            b"(2 2;3 4)[0]\0" as *const u8 as *const libc::c_char,
            s_16,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_17: S = ts(
        tp(
            tc(
                b"(1 2;3 4)\0" as *const u8 as *const libc::c_char as S,
                b"(1 2;3 4)[0 1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            900 as libc::c_int,
            b"(1 2;3 4)\0" as *const u8 as *const libc::c_char,
            b"(1 2;3 4)[0 1]\0" as *const u8 as *const libc::c_char,
            s_17,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_18: S = ts(
        tp(
            tc(
                b"1 3\0" as *const u8 as *const libc::c_char as S,
                b"(1 2;3 4)[0 1;0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            901 as libc::c_int,
            b"1 3\0" as *const u8 as *const libc::c_char,
            b"(1 2;3 4)[0 1;0]\0" as *const u8 as *const libc::c_char,
            s_18,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_19: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"(1 2;3 4)[0;0 1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            902 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"(1 2;3 4)[0;0 1]\0" as *const u8 as *const libc::c_char,
            s_19,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_20: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"(10 20;30 40)[0;0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            903 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"(10 20;30 40)[0;0]\0" as *const u8 as *const libc::c_char,
            s_20,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_21: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"(10 20;30 40)[0][0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            904 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"(10 20;30 40)[0][0]\0" as *const u8 as *const libc::c_char,
            s_21,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_22: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"(1 2;3 4)[0;1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            905 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"(1 2;3 4)[0;1]\0" as *const u8 as *const libc::c_char,
            s_22,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_23: S = ts(
        tp(
            tc(
                b"20\0" as *const u8 as *const libc::c_char as S,
                b"(10 20;30 40)[0][1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            906 as libc::c_int,
            b"20\0" as *const u8 as *const libc::c_char,
            b"(10 20;30 40)[0][1]\0" as *const u8 as *const libc::c_char,
            s_23,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_24: S = ts(
        tp(
            tc(
                b"30\0" as *const u8 as *const libc::c_char as S,
                b"((10 20;30 40);50)[0;1;0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            907 as libc::c_int,
            b"30\0" as *const u8 as *const libc::c_char,
            b"((10 20;30 40);50)[0;1;0]\0" as *const u8 as *const libc::c_char,
            s_24,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_25: S = ts(
        tp(
            tc(
                b"30\0" as *const u8 as *const libc::c_char as S,
                b"((10 20;30 40);50)[0][1][0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            908 as libc::c_int,
            b"30\0" as *const u8 as *const libc::c_char,
            b"((10 20;30 40);50)[0][1][0]\0" as *const u8 as *const libc::c_char,
            s_25,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_26: S = ts(
        tp(
            tc(
                b"10 20 30\0" as *const u8 as *const libc::c_char as S,
                b"m:(1 2 3;10 20 30;100 200 300); m[1;]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            909 as libc::c_int,
            b"10 20 30\0" as *const u8 as *const libc::c_char,
            b"m:(1 2 3;10 20 30;100 200 300); m[1;]\0" as *const u8
                as *const libc::c_char,
            s_26,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_27: S = ts(
        tp(
            tc(
                b"2 20 200\0" as *const u8 as *const libc::c_char as S,
                b"m:(1 2 3;10 20 30;100 200 300); m[;1]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            910 as libc::c_int,
            b"2 20 200\0" as *const u8 as *const libc::c_char,
            b"m:(1 2 3;10 20 30;100 200 300); m[;1]\0" as *const u8
                as *const libc::c_char,
            s_27,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_28: S = ts(
        tp(
            tc(
                b"a:2 2 2#1+!10\0" as *const u8 as *const libc::c_char as S,
                b"a[0 1;;0 1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            912 as libc::c_int,
            b"a:2 2 2#1+!10\0" as *const u8 as *const libc::c_char,
            b"a[0 1;;0 1]\0" as *const u8 as *const libc::c_char,
            s_28,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_29: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"# (+;+)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            913 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"# (+;+)\0" as *const u8 as *const libc::c_char,
            s_29,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_30: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"4:(`a 1 \\ )\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            914 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"4:(`a 1 \\ )\0" as *const u8 as *const libc::c_char,
            s_30,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_31: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"7=4:(`.)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            915 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"7=4:(`.)\0" as *const u8 as *const libc::c_char,
            s_31,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_32: S = ts(
        tp(
            tc(
                b"+\0" as *const u8 as *const libc::c_char as S,
                b"+\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            916 as libc::c_int,
            b"+\0" as *const u8 as *const libc::c_char,
            b"+\0" as *const u8 as *const libc::c_char,
            s_32,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_33: S = ts(
        tp(
            tc(
                b"+-|\0" as *const u8 as *const libc::c_char as S,
                b"||;+-|\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            917 as libc::c_int,
            b"+-|\0" as *const u8 as *const libc::c_char,
            b"||;+-|\0" as *const u8 as *const libc::c_char,
            s_33,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_34: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"1 2 . 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            918 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"1 2 . 0\0" as *const u8 as *const libc::c_char,
            s_34,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_35: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"11, .(`a;();:;11);a;a;a;a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            919 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"11, .(`a;();:;11);a;a;a;a\0" as *const u8 as *const libc::c_char,
            s_35,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_36: S = ts(
        tp(
            tc(
                b"`a\0" as *const u8 as *const libc::c_char as S,
                b".(`a;();:;22)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            920 as libc::c_int,
            b"`a\0" as *const u8 as *const libc::c_char,
            b".(`a;();:;22)\0" as *const u8 as *const libc::c_char,
            s_36,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_37: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"33, .(`a;();:;33);a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            921 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"33, .(`a;();:;33);a\0" as *const u8 as *const libc::c_char,
            s_37,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_38: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            922 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:1\0" as *const u8 as *const libc::c_char,
            s_38,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_39: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"a:2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            923 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"a:2\0" as *const u8 as *const libc::c_char,
            s_39,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_40: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"+[1;2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            924 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"+[1;2]\0" as *const u8 as *const libc::c_char,
            s_40,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_41: S = ts(
        tp(
            tc(
                b"`b\0" as *const u8 as *const libc::c_char as S,
                b".[`b;();:;33]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            925 as libc::c_int,
            b"`b\0" as *const u8 as *const libc::c_char,
            b".[`b;();:;33]\0" as *const u8 as *const libc::c_char,
            s_41,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_42: S = ts(
        tp(
            tc(
                b"33\0" as *const u8 as *const libc::c_char as S,
                b"..[`b;();:;33]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            926 as libc::c_int,
            b"33\0" as *const u8 as *const libc::c_char,
            b"..[`b;();:;33]\0" as *const u8 as *const libc::c_char,
            s_42,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_43: S = ts(
        tp(
            tc(
                b"1+2 3\0" as *const u8 as *const libc::c_char as S,
                b"..[`b;();:;3 4]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            927 as libc::c_int,
            b"1+2 3\0" as *const u8 as *const libc::c_char,
            b"..[`b;();:;3 4]\0" as *const u8 as *const libc::c_char,
            s_43,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_44: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"a:1;b:4;b:5;c:1;d:2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            928 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"a:1;b:4;b:5;c:1;d:2\0" as *const u8 as *const libc::c_char,
            s_44,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_45: S = ts(
        tp(
            tc(
                b"30\0" as *const u8 as *const libc::c_char as S,
                b"((10 20;30 40);50)[0;1;0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            929 as libc::c_int,
            b"30\0" as *const u8 as *const libc::c_char,
            b"((10 20;30 40);50)[0;1;0]\0" as *const u8 as *const libc::c_char,
            s_45,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_46: S = ts(
        tp(
            tc(
                b"`b\0" as *const u8 as *const libc::c_char as S,
                b"b:1;.[`b;();-:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            930 as libc::c_int,
            b"`b\0" as *const u8 as *const libc::c_char,
            b"b:1;.[`b;();-:]\0" as *const u8 as *const libc::c_char,
            s_46,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_47: S = ts(
        tp(
            tc(
                b"33\0" as *const u8 as *const libc::c_char as S,
                b"b:33;..[`b;()]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            931 as libc::c_int,
            b"33\0" as *const u8 as *const libc::c_char,
            b"b:33;..[`b;()]\0" as *const u8 as *const libc::c_char,
            s_47,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_48: S = ts(
        tp(
            tc(
                b"-33\0" as *const u8 as *const libc::c_char as S,
                b"b:33; ..[`b;();-:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            932 as libc::c_int,
            b"-33\0" as *const u8 as *const libc::c_char,
            b"b:33; ..[`b;();-:]\0" as *const u8 as *const libc::c_char,
            s_48,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_49: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"a:10\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            933 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"a:10\0" as *const u8 as *const libc::c_char,
            s_49,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_50: S = ts(
        tp(
            tc(
                b"1 2 3\0" as *const u8 as *const libc::c_char as S,
                b"a:1 2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            934 as libc::c_int,
            b"1 2 3\0" as *const u8 as *const libc::c_char,
            b"a:1 2 3\0" as *const u8 as *const libc::c_char,
            s_50,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_51: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"a:1+a:1+a:1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            935 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"a:1+a:1+a:1\0" as *const u8 as *const libc::c_char,
            s_51,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_52: S = ts(
        tp(
            tc(
                b"1 1 1 1\0" as *const u8 as *const libc::c_char as S,
                b"a:b:c:d:1;(a,b,c,d)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            936 as libc::c_int,
            b"1 1 1 1\0" as *const u8 as *const libc::c_char,
            b"a:b:c:d:1;(a,b,c,d)\0" as *const u8 as *const libc::c_char,
            s_52,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_53: S = ts(
        tp(
            tc(
                b"+\0" as *const u8 as *const libc::c_char as S,
                b"a:+\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            937 as libc::c_int,
            b"+\0" as *const u8 as *const libc::c_char,
            b"a:+\0" as *const u8 as *const libc::c_char,
            s_53,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_54: S = ts(
        tp(
            tc(
                b"11+\0" as *const u8 as *const libc::c_char as S,
                b"11+\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            938 as libc::c_int,
            b"11+\0" as *const u8 as *const libc::c_char,
            b"11+\0" as *const u8 as *const libc::c_char,
            s_54,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_55: S = ts(
        tp(
            tc(
                b"1-1+\0" as *const u8 as *const libc::c_char as S,
                b"1-1+\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            939 as libc::c_int,
            b"1-1+\0" as *const u8 as *const libc::c_char,
            b"1-1+\0" as *const u8 as *const libc::c_char,
            s_55,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_56: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"(10+1+) ~ (11+)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            940 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"(10+1+) ~ (11+)\0" as *const u8 as *const libc::c_char,
            s_56,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_57: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"a:\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            941 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"a:\0" as *const u8 as *const libc::c_char,
            s_57,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_58: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"#(+;+:;0:;:)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            942 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"#(+;+:;0:;:)\0" as *const u8 as *const libc::c_char,
            s_58,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_59: S = ts(
        tp(
            tc(
                b"-10\0" as *const u8 as *const libc::c_char as S,
                b"a:0;a:a-10\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            943 as libc::c_int,
            b"-10\0" as *const u8 as *const libc::c_char,
            b"a:0;a:a-10\0" as *const u8 as *const libc::c_char,
            s_59,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_60: S = ts(
        tp(
            tc(
                b"-12\0" as *const u8 as *const libc::c_char as S,
                b".(10;();:;-12)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            944 as libc::c_int,
            b"-12\0" as *const u8 as *const libc::c_char,
            b".(10;();:;-12)\0" as *const u8 as *const libc::c_char,
            s_60,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_61: S = ts(
        tp(
            tc(
                b"-3\0" as *const u8 as *const libc::c_char as S,
                b"z:1;z+:2;z-:\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            945 as libc::c_int,
            b"-3\0" as *const u8 as *const libc::c_char,
            b"z:1;z+:2;z-:\0" as *const u8 as *const libc::c_char,
            s_61,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_62: S = ts(
        tp(
            tc(
                b"7 8\0" as *const u8 as *const libc::c_char as S,
                b"a:1 2 3;1+a[0 1]:6 7\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            946 as libc::c_int,
            b"7 8\0" as *const u8 as *const libc::c_char,
            b"a:1 2 3;1+a[0 1]:6 7\0" as *const u8 as *const libc::c_char,
            s_62,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_63: S = ts(
        tp(
            tc(
                b"`a`b\0" as *const u8 as *const libc::c_char as S,
                b"a:1 2 3;a[0 1]:`a`b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            947 as libc::c_int,
            b"`a`b\0" as *const u8 as *const libc::c_char,
            b"a:1 2 3;a[0 1]:`a`b\0" as *const u8 as *const libc::c_char,
            s_63,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_64: S = ts(
        tp(
            tc(
                b"`a`b\0" as *const u8 as *const libc::c_char as S,
                b"a:(1 2 3;1.0 2.0 3.0); a[0 1]:`a`b\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            948 as libc::c_int,
            b"`a`b\0" as *const u8 as *const libc::c_char,
            b"a:(1 2 3;1.0 2.0 3.0); a[0 1]:`a`b\0" as *const u8 as *const libc::c_char,
            s_64,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_65: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:(((((1)))))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            949 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:(((((1)))))\0" as *const u8 as *const libc::c_char,
            s_65,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_66: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"a:2;f:(1+a+);a:10; f 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            950 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"a:2;f:(1+a+);a:10; f 3\0" as *const u8 as *const libc::c_char,
            s_66,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_67: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"a:b:10;a:14;b~14\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            951 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"a:b:10;a:14;b~14\0" as *const u8 as *const libc::c_char,
            s_67,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_68: S = ts(
        tp(
            tc(
                b"-10\0" as *const u8 as *const libc::c_char as S,
                b"a:0; a -: 10\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            952 as libc::c_int,
            b"-10\0" as *const u8 as *const libc::c_char,
            b"a:0; a -: 10\0" as *const u8 as *const libc::c_char,
            s_68,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_69: S = ts(
        tp(
            tc(
                b"-1 -2 -3\0" as *const u8 as *const libc::c_char as S,
                b"a:1 2 3; a -:\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            953 as libc::c_int,
            b"-1 -2 -3\0" as *const u8 as *const libc::c_char,
            b"a:1 2 3; a -:\0" as *const u8 as *const libc::c_char,
            s_69,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_70: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"a: 1 2 3; 1+a[0]+:4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            954 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"a: 1 2 3; 1+a[0]+:4\0" as *const u8 as *const libc::c_char,
            s_70,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_71: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"a: 1 2 3; 1+a[0]+:4;a[0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            955 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"a: 1 2 3; 1+a[0]+:4;a[0]\0" as *const u8 as *const libc::c_char,
            s_71,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_72: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"a: 1 2 3; 1-a[0]-:\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            956 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"a: 1 2 3; 1-a[0]-:\0" as *const u8 as *const libc::c_char,
            s_72,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_73: S = ts(
        tp(
            tc(
                b"-1\0" as *const u8 as *const libc::c_char as S,
                b"a: 1 2 3; 1-a[0]-:;a[0]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            957 as libc::c_int,
            b"-1\0" as *const u8 as *const libc::c_char,
            b"a: 1 2 3; 1-a[0]-:;a[0]\0" as *const u8 as *const libc::c_char,
            s_73,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_74: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"a:2; z:1 2 3; z[a]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            958 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"a:2; z:1 2 3; z[a]\0" as *const u8 as *const libc::c_char,
            s_74,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_75: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:0; z:(1 2; 3 4); z[a][a]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            959 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:0; z:(1 2; 3 4); z[a][a]\0" as *const u8 as *const libc::c_char,
            s_75,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_76: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:0;z:((1 2;3 4);(5 6;7 8);(9 10;11 12));z[a][a][a]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            960 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:0;z:((1 2;3 4);(5 6;7 8);(9 10;11 12));z[a][a][a]\0" as *const u8
                as *const libc::c_char,
            s_76,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_77: S = ts(
        tp(
            tc(
                b"8\0" as *const u8 as *const libc::c_char as S,
                b"a:0;z:((1 2;3 4);(5 6;7 8);(9 10;11 12));z[a][a][a:1]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            961 as libc::c_int,
            b"8\0" as *const u8 as *const libc::c_char,
            b"a:0;z:((1 2;3 4);(5 6;7 8);(9 10;11 12));z[a][a][a:1]\0" as *const u8
                as *const libc::c_char,
            s_77,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_78: S = ts(
        tp(
            tc(
                b"1 2\0" as *const u8 as *const libc::c_char as S,
                b"a:_n; 1 2[a][a][a][a][a][a][a][a][][][][][][][a][][][][][][a][][][][]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            962 as libc::c_int,
            b"1 2\0" as *const u8 as *const libc::c_char,
            b"a:_n; 1 2[a][a][a][a][a][a][a][a][][][][][][][a][][][][][][a][][][][]\0"
                as *const u8 as *const libc::c_char,
            s_78,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_79: S = ts(
        tp(
            tc(
                b"(1 2;3 4)\0" as *const u8 as *const libc::c_char as S,
                b"( 1 2 \n 3 4)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            963 as libc::c_int,
            b"(1 2;3 4)\0" as *const u8 as *const libc::c_char,
            b"( 1 2 \n 3 4)\0" as *const u8 as *const libc::c_char,
            s_79,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_80: S = ts(
        tp(
            tc(
                b"0 1 2\0" as *const u8 as *const libc::c_char as S,
                b"! 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            964 as libc::c_int,
            b"0 1 2\0" as *const u8 as *const libc::c_char,
            b"! 3\0" as *const u8 as *const libc::c_char,
            s_80,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_81: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"2*2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            965 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"2*2\0" as *const u8 as *const libc::c_char,
            s_81,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_82: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"+/()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            966 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"+/()\0" as *const u8 as *const libc::c_char,
            s_82,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_83: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"|/()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            967 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"|/()\0" as *const u8 as *const libc::c_char,
            s_83,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_84: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"*/()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            968 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"*/()\0" as *const u8 as *const libc::c_char,
            s_84,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_85: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"&/()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            969 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"&/()\0" as *const u8 as *const libc::c_char,
            s_85,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_86: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"+/1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            970 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"+/1 2\0" as *const u8 as *const libc::c_char,
            s_86,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_87: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"+/1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            971 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"+/1\0" as *const u8 as *const libc::c_char,
            s_87,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_88: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"+/,1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            972 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"+/,1\0" as *const u8 as *const libc::c_char,
            s_88,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_89: S = ts(
        tp(
            tc(
                b"1 3\0" as *const u8 as *const libc::c_char as S,
                b"+\\1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            973 as libc::c_int,
            b"1 3\0" as *const u8 as *const libc::c_char,
            b"+\\1 2\0" as *const u8 as *const libc::c_char,
            s_89,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_90: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"+\\1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            974 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"+\\1\0" as *const u8 as *const libc::c_char,
            s_90,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_91: S = ts(
        tp(
            tc(
                b"14\0" as *const u8 as *const libc::c_char as S,
                b"2+/3 4 5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            975 as libc::c_int,
            b"14\0" as *const u8 as *const libc::c_char,
            b"2+/3 4 5\0" as *const u8 as *const libc::c_char,
            s_91,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_92: S = ts(
        tp(
            tc(
                b"a:(1 2 3;\"CCC\";`x`y`z)\0" as *const u8 as *const libc::c_char as S,
                b"++a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            976 as libc::c_int,
            b"a:(1 2 3;\"CCC\";`x`y`z)\0" as *const u8 as *const libc::c_char,
            b"++a\0" as *const u8 as *const libc::c_char,
            s_92,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_93: S = ts(
        tp(
            tc(
                b"1 2 4 7\0" as *const u8 as *const libc::c_char as S,
                b"1+\\1 2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            977 as libc::c_int,
            b"1 2 4 7\0" as *const u8 as *const libc::c_char,
            b"1+\\1 2 3\0" as *const u8 as *const libc::c_char,
            s_93,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_94: S = ts(
        tp(
            tc(
                b"(.,(`k;))\0" as *const u8 as *const libc::c_char as S,
                b".,(`k;)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            978 as libc::c_int,
            b"(.,(`k;))\0" as *const u8 as *const libc::c_char,
            b".,(`k;)\0" as *const u8 as *const libc::c_char,
            s_94,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_95: S = ts(
        tp(
            tc(
                b"0 8\0" as *const u8 as *const libc::c_char as S,
                b"a:0;z:((1 2;3 4);(5 6;7 8);(9 10;11 12));a,(z+a:0)[a][a][a:1]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            979 as libc::c_int,
            b"0 8\0" as *const u8 as *const libc::c_char,
            b"a:0;z:((1 2;3 4);(5 6;7 8);(9 10;11 12));a,(z+a:0)[a][a][a:1]\0"
                as *const u8 as *const libc::c_char,
            s_95,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_96: S = ts(
        tp(
            tc(
                b"0 0 0 1\0" as *const u8 as *const libc::c_char as S,
                b"1 < -1 0 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            980 as libc::c_int,
            b"0 0 0 1\0" as *const u8 as *const libc::c_char,
            b"1 < -1 0 1 2\0" as *const u8 as *const libc::c_char,
            s_96,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_97: S = ts(
        tp(
            tc(
                b"0 0 0 1\0" as *const u8 as *const libc::c_char as S,
                b"1 < -1 0 1 2.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            981 as libc::c_int,
            b"0 0 0 1\0" as *const u8 as *const libc::c_char,
            b"1 < -1 0 1 2.0\0" as *const u8 as *const libc::c_char,
            s_97,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_98: S = ts(
        tp(
            tc(
                b"0 0 0 1\0" as *const u8 as *const libc::c_char as S,
                b"1.0 < -1 0 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            982 as libc::c_int,
            b"0 0 0 1\0" as *const u8 as *const libc::c_char,
            b"1.0 < -1 0 1 2\0" as *const u8 as *const libc::c_char,
            s_98,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_99: S = ts(
        tp(
            tc(
                b"0 0 0 1\0" as *const u8 as *const libc::c_char as S,
                b"1.0 < -1 0 1 2.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            983 as libc::c_int,
            b"0 0 0 1\0" as *const u8 as *const libc::c_char,
            b"1.0 < -1 0 1 2.0\0" as *const u8 as *const libc::c_char,
            s_99,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_100: S = ts(
        tp(
            tc(
                b"1 1 0 0\0" as *const u8 as *const libc::c_char as S,
                b"1 > -1 0 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            984 as libc::c_int,
            b"1 1 0 0\0" as *const u8 as *const libc::c_char,
            b"1 > -1 0 1 2\0" as *const u8 as *const libc::c_char,
            s_100,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_101: S = ts(
        tp(
            tc(
                b"1 1 0 0\0" as *const u8 as *const libc::c_char as S,
                b"1 > -1 0 1 2.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            985 as libc::c_int,
            b"1 1 0 0\0" as *const u8 as *const libc::c_char,
            b"1 > -1 0 1 2.0\0" as *const u8 as *const libc::c_char,
            s_101,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_102: S = ts(
        tp(
            tc(
                b"1 1 0 0\0" as *const u8 as *const libc::c_char as S,
                b"1.0 > -1 0 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            986 as libc::c_int,
            b"1 1 0 0\0" as *const u8 as *const libc::c_char,
            b"1.0 > -1 0 1 2\0" as *const u8 as *const libc::c_char,
            s_102,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_103: S = ts(
        tp(
            tc(
                b"1 1 0 0\0" as *const u8 as *const libc::c_char as S,
                b"1.0 > -1 0 1 2.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            987 as libc::c_int,
            b"1 1 0 0\0" as *const u8 as *const libc::c_char,
            b"1.0 > -1 0 1 2.0\0" as *const u8 as *const libc::c_char,
            s_103,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_104: S = ts(
        tp(
            tc(
                b"0 0 0 1\0" as *const u8 as *const libc::c_char as S,
                b"\"c\" < \"abcd\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            988 as libc::c_int,
            b"0 0 0 1\0" as *const u8 as *const libc::c_char,
            b"\"c\" < \"abcd\"\0" as *const u8 as *const libc::c_char,
            s_104,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_105: S = ts(
        tp(
            tc(
                b"1 1 0 0\0" as *const u8 as *const libc::c_char as S,
                b"\"c\" > \"abcd\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            989 as libc::c_int,
            b"1 1 0 0\0" as *const u8 as *const libc::c_char,
            b"\"c\" > \"abcd\"\0" as *const u8 as *const libc::c_char,
            s_105,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_106: S = ts(
        tp(
            tc(
                b"0 0 0 1\0" as *const u8 as *const libc::c_char as S,
                b"`c < `a`b`c`d\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            990 as libc::c_int,
            b"0 0 0 1\0" as *const u8 as *const libc::c_char,
            b"`c < `a`b`c`d\0" as *const u8 as *const libc::c_char,
            s_106,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_107: S = ts(
        tp(
            tc(
                b"1 1 0 0\0" as *const u8 as *const libc::c_char as S,
                b"`c > `a`b`c`d\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            991 as libc::c_int,
            b"1 1 0 0\0" as *const u8 as *const libc::c_char,
            b"`c > `a`b`c`d\0" as *const u8 as *const libc::c_char,
            s_107,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_108: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"f:|/0(0|+)\\; f 2 1 -4 2 -1 5\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            992 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"f:|/0(0|+)\\; f 2 1 -4 2 -1 5\0" as *const u8 as *const libc::c_char,
            s_108,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_109: S = ts(
        tp(
            tc(
                b"5#45\0" as *const u8 as *const libc::c_char as S,
                b"f:|/0(0|+)\\; a:!10; (f (!10); f[!10]; f a; f @ a; f .,a)\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            993 as libc::c_int,
            b"5#45\0" as *const u8 as *const libc::c_char,
            b"f:|/0(0|+)\\; a:!10; (f (!10); f[!10]; f a; f @ a; f .,a)\0" as *const u8
                as *const libc::c_char,
            s_109,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_110: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b".\"0+0\\n1+1\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            994 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b".\"0+0\\n1+1\"\0" as *const u8 as *const libc::c_char,
            s_110,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_111: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b".\"1+1\\n2+2\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            995 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b".\"1+1\\n2+2\"\0" as *const u8 as *const libc::c_char,
            s_111,
        );
    }
    test_print = 0 as libc::c_int as I;
    return 0 as libc::c_int as I;
}
unsafe extern "C" fn testsBook() -> I {
    let mut s: S = ts(
        tp(
            tc(
                b"1 0 1 0 0 2 0 1 2 1\0" as *const u8 as *const libc::c_char as S,
                b"A.I:1 2 3;A.F:2 5 7;B.F: 5 2 5 2 2 7 2 5 7 5; A.F ?/: B.F\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1003 as libc::c_int,
            b"1 0 1 0 0 2 0 1 2 1\0" as *const u8 as *const libc::c_char,
            b"A.I:1 2 3;A.F:2 5 7;B.F: 5 2 5 2 2 7 2 5 7 5; A.F ?/: B.F\0" as *const u8
                as *const libc::c_char,
            s,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_0: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"+/1 2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1005 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"+/1 2 3 4\0" as *const u8 as *const libc::c_char,
            s_0,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_1: S = ts(
        tp(
            tc(
                b"26\0" as *const u8 as *const libc::c_char as S,
                b"16 +/ 1 2 3 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1006 as libc::c_int,
            b"26\0" as *const u8 as *const libc::c_char,
            b"16 +/ 1 2 3 4\0" as *const u8 as *const libc::c_char,
            s_1,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_2: S = ts(
        tp(
            tc(
                b"3.5\0" as *const u8 as *const libc::c_char as S,
                b"(1;\"a\";3.5;`xyz) 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1007 as libc::c_int,
            b"3.5\0" as *const u8 as *const libc::c_char,
            b"(1;\"a\";3.5;`xyz) 2\0" as *const u8 as *const libc::c_char,
            s_2,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_3: S = ts(
        tp(
            tc(
                b"25 20 5\0" as *const u8 as *const libc::c_char as S,
                b"x:10; (x+5;x:20;x-5)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1008 as libc::c_int,
            b"25 20 5\0" as *const u8 as *const libc::c_char,
            b"x:10; (x+5;x:20;x-5)\0" as *const u8 as *const libc::c_char,
            s_3,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_4: S = ts(
        tp(
            tc(
                b"(98 54;2.5;3.5 -1; 54; 24)\0" as *const u8 as *const libc::c_char as S,
                b"x:99 55; (x-1;3.5-1;3.5 -1;x[1]-1;(12+13)- 1)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1009 as libc::c_int,
            b"(98 54;2.5;3.5 -1; 54; 24)\0" as *const u8 as *const libc::c_char,
            b"x:99 55; (x-1;3.5-1;3.5 -1;x[1]-1;(12+13)- 1)\0" as *const u8
                as *const libc::c_char,
            s_4,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_5: S = ts(
        tp(
            tc(
                b"-3 -4 -5\0" as *const u8 as *const libc::c_char as S,
                b"- 3 4 5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1010 as libc::c_int,
            b"-3 -4 -5\0" as *const u8 as *const libc::c_char,
            b"- 3 4 5\0" as *const u8 as *const libc::c_char,
            s_5,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_6: S = ts(
        tp(
            tc(
                b"(-5 -2;-3;8 0 -2)\0" as *const u8 as *const libc::c_char as S,
                b"- (5 2; 3; -8 0 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1011 as libc::c_int,
            b"(-5 -2;-3;8 0 -2)\0" as *const u8 as *const libc::c_char,
            b"- (5 2; 3; -8 0 2)\0" as *const u8 as *const libc::c_char,
            s_6,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_7: S = ts(
        tp(
            tc(
                b"5 9\0" as *const u8 as *const libc::c_char as S,
                b"2 6 + 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1012 as libc::c_int,
            b"5 9\0" as *const u8 as *const libc::c_char,
            b"2 6 + 3\0" as *const u8 as *const libc::c_char,
            s_7,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_8: S = ts(
        tp(
            tc(
                b"5 -6\0" as *const u8 as *const libc::c_char as S,
                b"2 + 3 -8\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1013 as libc::c_int,
            b"5 -6\0" as *const u8 as *const libc::c_char,
            b"2 + 3 -8\0" as *const u8 as *const libc::c_char,
            s_8,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_9: S = ts(
        tp(
            tc(
                b"5 -2\0" as *const u8 as *const libc::c_char as S,
                b"2 6 + 3 -8\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1014 as libc::c_int,
            b"5 -2\0" as *const u8 as *const libc::c_char,
            b"2 6 + 3 -8\0" as *const u8 as *const libc::c_char,
            s_9,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_10: S = ts(
        tp(
            tc(
                b"((7 8;9 10 11);(13;15 16))\0" as *const u8 as *const libc::c_char as S,
                b"(2;3 4) + ((5 6;7 8 9);(10;11 12))\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1015 as libc::c_int,
            b"((7 8;9 10 11);(13;15 16))\0" as *const u8 as *const libc::c_char,
            b"(2;3 4) + ((5 6;7 8 9);(10;11 12))\0" as *const u8 as *const libc::c_char,
            s_10,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_11: S = ts(
        tp(
            tc(
                b"(2 7;-23)\0" as *const u8 as *const libc::c_char as S,
                b"2 4 -23 8 7 @ (0 4; 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1016 as libc::c_int,
            b"(2 7;-23)\0" as *const u8 as *const libc::c_char,
            b"2 4 -23 8 7 @ (0 4; 2)\0" as *const u8 as *const libc::c_char,
            s_11,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_12: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"@\"a\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1017 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"@\"a\"\0" as *const u8 as *const libc::c_char,
            s_12,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_13: S = ts(
        tp(
            tc(
                b"(1;(+;102))\0" as *const u8 as *const libc::c_char as S,
                b"f:+; (@f;(f;102))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1018 as libc::c_int,
            b"(1;(+;102))\0" as *const u8 as *const libc::c_char,
            b"f:+; (@f;(f;102))\0" as *const u8 as *const libc::c_char,
            s_13,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_14: S = ts(
        tp(
            tc(
                b"(1;;2)\0" as *const u8 as *const libc::c_char as S,
                b"(1; _n ; 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1019 as libc::c_int,
            b"(1;;2)\0" as *const u8 as *const libc::c_char,
            b"(1; _n ; 2)\0" as *const u8 as *const libc::c_char,
            s_14,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_15: S = ts(
        tp(
            tc(
                b"4 3\0" as *const u8 as *const libc::c_char as S,
                b"^ (1 2 3; \"abc\"; `x `y `z; 5.4 1.2 -3.56)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1020 as libc::c_int,
            b"4 3\0" as *const u8 as *const libc::c_char,
            b"^ (1 2 3; \"abc\"; `x `y `z; 5.4 1.2 -3.56)\0" as *const u8
                as *const libc::c_char,
            s_15,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_16: S = ts(
        tp(
            tc(
                b"2 3 2\0" as *const u8 as *const libc::c_char as S,
                b"^ ((1 2; `a `b; \"AB\"); (\"CD\"; 3 4; `c `d))\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1021 as libc::c_int,
            b"2 3 2\0" as *const u8 as *const libc::c_char,
            b"^ ((1 2; `a `b; \"AB\"); (\"CD\"; 3 4; `c `d))\0" as *const u8
                as *const libc::c_char,
            s_16,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_17: S = ts(
        tp(
            tc(
                b"2 3\0" as *const u8 as *const libc::c_char as S,
                b"^ ((0 1 2; `a; \"AB\"); (\"CD\"; 3 4; `c `d))\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1022 as libc::c_int,
            b"2 3\0" as *const u8 as *const libc::c_char,
            b"^ ((0 1 2; `a; \"AB\"); (\"CD\"; 3 4; `c `d))\0" as *const u8
                as *const libc::c_char,
            s_17,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_18: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"# 1 -2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1023 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"# 1 -2 3\0" as *const u8 as *const libc::c_char,
            s_18,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_19: S = ts(
        tp(
            tc(
                b"-2\0" as *const u8 as *const libc::c_char as S,
                b"1 -2 3[1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1024 as libc::c_int,
            b"-2\0" as *const u8 as *const libc::c_char,
            b"1 -2 3[1]\0" as *const u8 as *const libc::c_char,
            s_19,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_20: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"# 3 4 5.721 1.023e10\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1025 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"# 3 4 5.721 1.023e10\0" as *const u8 as *const libc::c_char,
            s_20,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_21: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"#\"hello\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1026 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"#\"hello\"\0" as *const u8 as *const libc::c_char,
            s_21,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_22: S = ts(
        tp(
            tc(
                b"9 14 19 6 5 4 3 5 7 0\0" as *const u8 as *const libc::c_char as S,
                b"d: 9 8 7 6 5 4 3 2 1 0;i: 2 7 1 8 2 8; y: 5 3 6 2 7 4;@[d;i;+;y]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1028 as libc::c_int,
            b"9 14 19 6 5 4 3 5 7 0\0" as *const u8 as *const libc::c_char,
            b"d: 9 8 7 6 5 4 3 2 1 0;i: 2 7 1 8 2 8; y: 5 3 6 2 7 4;@[d;i;+;y]\0"
                as *const u8 as *const libc::c_char,
            s_22,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_23: S = ts(
        tp(
            tc(
                b"((9;\"a\";\"b\";\"c\");(8;1.5;`xyz);(7;100;3.76;`efgh))\0" as *const u8
                    as *const libc::c_char as S,
                b"d:9 8 7; i:(0;(1;2 2)); y: (\"abc\";((1.5;`xyz);(100;(3.76;`efgh)))); @[d;i;,;y]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1029 as libc::c_int,
            b"((9;\"a\";\"b\";\"c\");(8;1.5;`xyz);(7;100;3.76;`efgh))\0" as *const u8
                as *const libc::c_char,
            b"d:9 8 7; i:(0;(1;2 2)); y: (\"abc\";((1.5;`xyz);(100;(3.76;`efgh)))); @[d;i;,;y]\0"
                as *const u8 as *const libc::c_char,
            s_23,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_24: S = ts(
        tp(
            tc(
                b"9 60 70 6 5 4 3 30 40 0\0" as *const u8 as *const libc::c_char as S,
                b"d:9 8 7 6 5 4 3 2 1 0;i:2 7 1 8 2 8; y: 50 30 60 20 70 40; @[d;i;:;y]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1030 as libc::c_int,
            b"9 60 70 6 5 4 3 30 40 0\0" as *const u8 as *const libc::c_char,
            b"d:9 8 7 6 5 4 3 2 1 0;i:2 7 1 8 2 8; y: 50 30 60 20 70 40; @[d;i;:;y]\0"
                as *const u8 as *const libc::c_char,
            s_24,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_25: S = ts(
        tp(
            tc(
                b"(5 2.14;(\"a\";\"b\";\"cxyz\"))\0" as *const u8 as *const libc::c_char
                    as S,
                b".[(5 2.14; \"abc\");1 2; ,; \"xyz\"]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1031 as libc::c_int,
            b"(5 2.14;(\"a\";\"b\";\"cxyz\"))\0" as *const u8 as *const libc::c_char,
            b".[(5 2.14; \"abc\");1 2; ,; \"xyz\"]\0" as *const u8
                as *const libc::c_char,
            s_25,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_26: S = ts(
        tp(
            tc(
                b"((1 2 3 400 600;4 5 6 7 500);(8 9;10;11 12);(13 14 100 300;15 16 17 18 200;19 20))\0"
                    as *const u8 as *const libc::c_char as S,
                b"d:((1 2 3;4 5 6 7);(8 9;10; 11 12);(13 14;15 16 17 18; 19 20)); i:(2 0;0 1 0); y:(100 200 300; 400 500 600); .[d;i;,;y]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1033 as libc::c_int,
            b"((1 2 3 400 600;4 5 6 7 500);(8 9;10;11 12);(13 14 100 300;15 16 17 18 200;19 20))\0"
                as *const u8 as *const libc::c_char,
            b"d:((1 2 3;4 5 6 7);(8 9;10; 11 12);(13 14;15 16 17 18; 19 20)); i:(2 0;0 1 0); y:(100 200 300; 400 500 600); .[d;i;,;y]\0"
                as *const u8 as *const libc::c_char,
            s_26,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_27: S = ts(
        tp(
            tc(
                b"(600 500;(8 9;10;11 12);(300;200;19 20))\0" as *const u8
                    as *const libc::c_char as S,
                b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20)); i: (2 0; 0 1 0); y:(100 200 300; 400 500 600);.[d;i;:;y]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1034 as libc::c_int,
            b"(600 500;(8 9;10;11 12);(300;200;19 20))\0" as *const u8
                as *const libc::c_char,
            b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20)); i: (2 0; 0 1 0); y:(100 200 300; 400 500 600);.[d;i;:;y]\0"
                as *const u8 as *const libc::c_char,
            s_27,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_28: S = ts(
        tp(
            tc(
                b"((1 2 3;-4 -5 -6 -7);(8 9;10;11 12);(13 14;-15 -16 -17 -18;19 20))\0"
                    as *const u8 as *const libc::c_char as S,
                b"d:((1 2 3; 4 5 6 7);(8 9;10; 11 12);(13 14; 15 16 17 18;19 20)); i: (2 0; 0 1 0); y: (100 200 300; 400 500 600); .[d;i;-:]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1035 as libc::c_int,
            b"((1 2 3;-4 -5 -6 -7);(8 9;10;11 12);(13 14;-15 -16 -17 -18;19 20))\0"
                as *const u8 as *const libc::c_char,
            b"d:((1 2 3; 4 5 6 7);(8 9;10; 11 12);(13 14; 15 16 17 18;19 20)); i: (2 0; 0 1 0); y: (100 200 300; 400 500 600); .[d;i;-:]\0"
                as *const u8 as *const libc::c_char,
            s_28,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_29: S = ts(
        tp(
            tc(
                b"2 3 4 5 6\0" as *const u8 as *const libc::c_char as S,
                b".[2 3;();,;4 5 6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1036 as libc::c_int,
            b"2 3 4 5 6\0" as *const u8 as *const libc::c_char,
            b".[2 3;();,;4 5 6]\0" as *const u8 as *const libc::c_char,
            s_29,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_30: S = ts(
        tp(
            tc(
                b"(2 4;3 5)\0" as *const u8 as *const libc::c_char as S,
                b".[2 3; ,_n;,;4 5]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1037 as libc::c_int,
            b"(2 4;3 5)\0" as *const u8 as *const libc::c_char,
            b".[2 3; ,_n;,;4 5]\0" as *const u8 as *const libc::c_char,
            s_30,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_31: S = ts(
        tp(
            tc(
                b"\"c\"\0" as *const u8 as *const libc::c_char as S,
                b"(5 2.14;\"abc\") . 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1038 as libc::c_int,
            b"\"c\"\0" as *const u8 as *const libc::c_char,
            b"(5 2.14;\"abc\") . 1 2\0" as *const u8 as *const libc::c_char,
            s_31,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_32: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"-2~4:0.0 0i\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1039 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"-2~4:0.0 0i\0" as *const u8 as *const libc::c_char,
            s_32,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_33: S = ts(
        tp(
            tc(
                b"0.0 0i\0" as *const u8 as *const libc::c_char as S,
                b"0.0 0I\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1040 as libc::c_int,
            b"0.0 0i\0" as *const u8 as *const libc::c_char,
            b"0.0 0I\0" as *const u8 as *const libc::c_char,
            s_33,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_34: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0i = 0I\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1041 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0i = 0I\0" as *const u8 as *const libc::c_char,
            s_34,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_35: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"0i ~ 0I\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1042 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"0i ~ 0I\0" as *const u8 as *const libc::c_char,
            s_35,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_36: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"@1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1043 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"@1\0" as *const u8 as *const libc::c_char,
            s_36,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_37: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"@2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1044 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"@2 3\0" as *const u8 as *const libc::c_char,
            s_37,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_38: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"@\"Z\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1045 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"@\"Z\"\0" as *const u8 as *const libc::c_char,
            s_38,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_39: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"@\"\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1046 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"@\"\"\0" as *const u8 as *const libc::c_char,
            s_39,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_40: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"@{x+y}\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1047 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"@{x+y}\0" as *const u8 as *const libc::c_char,
            s_40,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_41: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"@(+;-)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1048 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"@(+;-)\0" as *const u8 as *const libc::c_char,
            s_41,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_42: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"@ `symbol\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1049 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"@ `symbol\0" as *const u8 as *const libc::c_char,
            s_42,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_43: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"@ .((`a;2);(`b;3))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1050 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"@ .((`a;2);(`b;3))\0" as *const u8 as *const libc::c_char,
            s_43,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_44: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"@ _n\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1051 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"@ _n\0" as *const u8 as *const libc::c_char,
            s_44,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_45: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"#3 1 4 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1052 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"#3 1 4 2\0" as *const u8 as *const libc::c_char,
            s_45,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_46: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"#(8 1 6;3 7;4 9 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1053 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"#(8 1 6;3 7;4 9 2)\0" as *const u8 as *const libc::c_char,
            s_46,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_47: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"#\"A\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1054 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"#\"A\"\0" as *const u8 as *const libc::c_char,
            s_47,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_48: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"#,\"A\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1055 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"#,\"A\"\0" as *const u8 as *const libc::c_char,
            s_48,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_49: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"#\"count\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1056 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"#\"count\"\0" as *const u8 as *const libc::c_char,
            s_49,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_50: S = ts(
        tp(
            tc(
                b"\"tares\"\0" as *const u8 as *const libc::c_char as S,
                b"1 _ \"stares\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1057 as libc::c_int,
            b"\"tares\"\0" as *const u8 as *const libc::c_char,
            b"1 _ \"stares\"\0" as *const u8 as *const libc::c_char,
            s_50,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_51: S = ts(
        tp(
            tc(
                b"\"star\"\0" as *const u8 as *const libc::c_char as S,
                b"-2 _ \"stares\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1058 as libc::c_int,
            b"\"star\"\0" as *const u8 as *const libc::c_char,
            b"-2 _ \"stares\"\0" as *const u8 as *const libc::c_char,
            s_51,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_52: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"0 _ 7\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1059 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"0 _ 7\0" as *const u8 as *const libc::c_char,
            s_52,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_53: S = ts(
        tp(
            tc(
                b"\"\"\0" as *const u8 as *const libc::c_char as S,
                b"88 _ \"\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1060 as libc::c_int,
            b"\"\"\0" as *const u8 as *const libc::c_char,
            b"88 _ \"\"\0" as *const u8 as *const libc::c_char,
            s_53,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_54: S = ts(
        tp(
            tc(
                b"!0\0" as *const u8 as *const libc::c_char as S,
                b"9 _ !6\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1061 as libc::c_int,
            b"!0\0" as *const u8 as *const libc::c_char,
            b"9 _ !6\0" as *const u8 as *const libc::c_char,
            s_54,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_55: S = ts(
        tp(
            tc(
                b"(0 1 2;3 4 5)\0" as *const u8 as *const libc::c_char as S,
                b"0 3 _ 0 1 2 3 4 5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1062 as libc::c_int,
            b"(0 1 2;3 4 5)\0" as *const u8 as *const libc::c_char,
            b"0 3 _ 0 1 2 3 4 5\0" as *const u8 as *const libc::c_char,
            s_55,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_56: S = ts(
        tp(
            tc(
                b"(0 1 2 3;4 5)\0" as *const u8 as *const libc::c_char as S,
                b"0 4 _ 0 1 2 3 4 5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1063 as libc::c_int,
            b"(0 1 2 3;4 5)\0" as *const u8 as *const libc::c_char,
            b"0 4 _ 0 1 2 3 4 5\0" as *const u8 as *const libc::c_char,
            s_56,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_57: S = ts(
        tp(
            tc(
                b"(\"seas\";\"hells\")\0" as *const u8 as *const libc::c_char as S,
                b"0 4 _ \"seashells\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1064 as libc::c_int,
            b"(\"seas\";\"hells\")\0" as *const u8 as *const libc::c_char,
            b"0 4 _ \"seashells\"\0" as *const u8 as *const libc::c_char,
            s_57,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_58: S = ts(
        tp(
            tc(
                b"(\"try\";\" to\";\" cut\";\" into\";\" words\")\0" as *const u8
                    as *const libc::c_char as S,
                b"a:\"try to cut into words\"; (0,(&a=\" \")) _ a\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1065 as libc::c_int,
            b"(\"try\";\" to\";\" cut\";\" into\";\" words\")\0" as *const u8
                as *const libc::c_char,
            b"a:\"try to cut into words\"; (0,(&a=\" \")) _ a\0" as *const u8
                as *const libc::c_char,
            s_58,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_59: S = ts(
        tp(
            tc(
                b"(!0;1 2;3 4 5)\0" as *const u8 as *const libc::c_char as S,
                b"1 1 3 _ !6\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1066 as libc::c_int,
            b"(!0;1 2;3 4 5)\0" as *const u8 as *const libc::c_char,
            b"1 1 3 _ !6\0" as *const u8 as *const libc::c_char,
            s_59,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_60: S = ts(
        tp(
            tc(
                b"(,3)\0" as *const u8 as *const libc::c_char as S,
                b"^ 1 2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1067 as libc::c_int,
            b"(,3)\0" as *const u8 as *const libc::c_char,
            b"^ 1 2 3\0" as *const u8 as *const libc::c_char,
            s_60,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_61: S = ts(
        tp(
            tc(
                b"1 3\0" as *const u8 as *const libc::c_char as S,
                b"^,1 2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1068 as libc::c_int,
            b"1 3\0" as *const u8 as *const libc::c_char,
            b"^,1 2 3\0" as *const u8 as *const libc::c_char,
            s_61,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_62: S = ts(
        tp(
            tc(
                b"0 1 2 3 4\0" as *const u8 as *const libc::c_char as S,
                b"!5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1069 as libc::c_int,
            b"0 1 2 3 4\0" as *const u8 as *const libc::c_char,
            b"!5\0" as *const u8 as *const libc::c_char,
            s_62,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_63: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"3=3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1070 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"3=3\0" as *const u8 as *const libc::c_char,
            s_63,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_64: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"3= -3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1071 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"3= -3\0" as *const u8 as *const libc::c_char,
            s_64,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_65: S = ts(
        tp(
            tc(
                b"0 1 1\0" as *const u8 as *const libc::c_char as S,
                b"\"cat\" = \"rat\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1072 as libc::c_int,
            b"0 1 1\0" as *const u8 as *const libc::c_char,
            b"\"cat\" = \"rat\"\0" as *const u8 as *const libc::c_char,
            s_65,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_66: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"`abc = `abcdefg\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1073 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"`abc = `abcdefg\0" as *const u8 as *const libc::c_char,
            s_66,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_67: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"3.0=3.1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1074 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"3.0=3.1\0" as *const u8 as *const libc::c_char,
            s_67,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_68: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"3.0 = 3.0001\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1075 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"3.0 = 3.0001\0" as *const u8 as *const libc::c_char,
            s_68,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_69: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"3.0 = 3.0000000000000001\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1076 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"3.0 = 3.0000000000000001\0" as *const u8 as *const libc::c_char,
            s_69,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_70: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"9 8 7 6 5 4 3 ? 7\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1077 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"9 8 7 6 5 4 3 ? 7\0" as *const u8 as *const libc::c_char,
            s_70,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_71: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"9 8 7 6 5 4 3 ? 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1078 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"9 8 7 6 5 4 3 ? 1\0" as *const u8 as *const libc::c_char,
            s_71,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_72: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"(8 16; \"abcdef\"; 4 9 2; `x`y`z`w; 4 9 2) ? 4 9 2\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1079 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"(8 16; \"abcdef\"; 4 9 2; `x`y`z`w; 4 9 2) ? 4 9 2\0" as *const u8
                as *const libc::c_char,
            s_72,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_73: S = ts(
        tp(
            tc(
                b"2 6\0" as *const u8 as *const libc::c_char as S,
                b"words:(\"canoe\";\"tug\";\"raft\";\"rowboat\";\"ark\";\"liner\"); (words?\"raft\";words?\"submarine\")\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1080 as libc::c_int,
            b"2 6\0" as *const u8 as *const libc::c_char,
            b"words:(\"canoe\";\"tug\";\"raft\";\"rowboat\";\"ark\";\"liner\"); (words?\"raft\";words?\"submarine\")\0"
                as *const u8 as *const libc::c_char,
            s_73,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_74: S = ts(
        tp(
            tc(
                b"\"a\"\0" as *const u8 as *const libc::c_char as S,
                b"*\"abc\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1081 as libc::c_int,
            b"\"a\"\0" as *const u8 as *const libc::c_char,
            b"*\"abc\"\0" as *const u8 as *const libc::c_char,
            s_74,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_75: S = ts(
        tp(
            tc(
                b"\"abc\"\0" as *const u8 as *const libc::c_char as S,
                b"*(\"abc\";\"defg\";\"hijkl\")\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1082 as libc::c_int,
            b"\"abc\"\0" as *const u8 as *const libc::c_char,
            b"*(\"abc\";\"defg\";\"hijkl\")\0" as *const u8 as *const libc::c_char,
            s_75,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_76: S = ts(
        tp(
            tc(
                b"(,`pqr)\0" as *const u8 as *const libc::c_char as S,
                b"*,,`pqr\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1083 as libc::c_int,
            b"(,`pqr)\0" as *const u8 as *const libc::c_char,
            b"*,,`pqr\0" as *const u8 as *const libc::c_char,
            s_76,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_77: S = ts(
        tp(
            tc(
                b"`pqr\0" as *const u8 as *const libc::c_char as S,
                b"*`pqr\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1084 as libc::c_int,
            b"`pqr\0" as *const u8 as *const libc::c_char,
            b"*`pqr\0" as *const u8 as *const libc::c_char,
            s_77,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_78: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"*!0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1085 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"*!0\0" as *const u8 as *const libc::c_char,
            s_78,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_79: S = ts(
        tp(
            tc(
                b"0.0\0" as *const u8 as *const libc::c_char as S,
                b"*0#0.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1086 as libc::c_int,
            b"0.0\0" as *const u8 as *const libc::c_char,
            b"*0#0.0\0" as *const u8 as *const libc::c_char,
            s_79,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_80: S = ts(
        tp(
            tc(
                b"\" \"\0" as *const u8 as *const libc::c_char as S,
                b"*0#\"\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1087 as libc::c_int,
            b"\" \"\0" as *const u8 as *const libc::c_char,
            b"*0#\"\"\0" as *const u8 as *const libc::c_char,
            s_80,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_81: S = ts(
        tp(
            tc(
                b"`\0" as *const u8 as *const libc::c_char as S,
                b"*0#`\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1088 as libc::c_int,
            b"`\0" as *const u8 as *const libc::c_char,
            b"*0#`\0" as *const u8 as *const libc::c_char,
            s_81,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_82: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b"*()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1089 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b"*()\0" as *const u8 as *const libc::c_char,
            s_82,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_83: S = ts(
        tp(
            tc(
                b"(0 4 8;1 5 9;2 6 10;3 7 11)\0" as *const u8 as *const libc::c_char
                    as S,
                b"+3 4#!12\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1091 as libc::c_int,
            b"(0 4 8;1 5 9;2 6 10;3 7 11)\0" as *const u8 as *const libc::c_char,
            b"+3 4#!12\0" as *const u8 as *const libc::c_char,
            s_83,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_84: S = ts(
        tp(
            tc(
                b"((0 1 2;4 5;10 11);(3;6 7 8 9;12))\0" as *const u8
                    as *const libc::c_char as S,
                b"a:((0 1 2;3);(4 5;6 7 8 9);(10 11;12)); +a\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1092 as libc::c_int,
            b"((0 1 2;4 5;10 11);(3;6 7 8 9;12))\0" as *const u8 as *const libc::c_char,
            b"a:((0 1 2;3);(4 5;6 7 8 9);(10 11;12)); +a\0" as *const u8
                as *const libc::c_char,
            s_84,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_85: S = ts(
        tp(
            tc(
                b"((1;\"C\";`x);(2;\"C\";`y);(3;\"C\";`z))\0" as *const u8
                    as *const libc::c_char as S,
                b"+(1 2 3;\"C\";`x`y`z)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1093 as libc::c_int,
            b"((1;\"C\";`x);(2;\"C\";`y);(3;\"C\";`z))\0" as *const u8
                as *const libc::c_char,
            b"+(1 2 3;\"C\";`x`y`z)\0" as *const u8 as *const libc::c_char,
            s_85,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_86: S = ts(
        tp(
            tc(
                b"((1;\"C\";`x);(2;\"C\";`y);(3;\"C\";`z))\0" as *const u8
                    as *const libc::c_char as S,
                b"+(1 2 3;\"CCC\";`x`y`z)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1094 as libc::c_int,
            b"((1;\"C\";`x);(2;\"C\";`y);(3;\"C\";`z))\0" as *const u8
                as *const libc::c_char,
            b"+(1 2 3;\"CCC\";`x`y`z)\0" as *const u8 as *const libc::c_char,
            s_86,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_87: S = ts(
        tp(
            tc(
                b"`abc\0" as *const u8 as *const libc::c_char as S,
                b"+`abc\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1095 as libc::c_int,
            b"`abc\0" as *const u8 as *const libc::c_char,
            b"+`abc\0" as *const u8 as *const libc::c_char,
            s_87,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_88: S = ts(
        tp(
            tc(
                b"67\0" as *const u8 as *const libc::c_char as S,
                b"+67\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1096 as libc::c_int,
            b"67\0" as *const u8 as *const libc::c_char,
            b"+67\0" as *const u8 as *const libc::c_char,
            s_88,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_89: S = ts(
        tp(
            tc(
                b"{x*y}\0" as *const u8 as *const libc::c_char as S,
                b"+{x*y}\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1097 as libc::c_int,
            b"{x*y}\0" as *const u8 as *const libc::c_char,
            b"+{x*y}\0" as *const u8 as *const libc::c_char,
            s_89,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_90: S = ts(
        tp(
            tc(
                b"`a`b`c\0" as *const u8 as *const libc::c_char as S,
                b"+`a`b`c\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1098 as libc::c_int,
            b"`a`b`c\0" as *const u8 as *const libc::c_char,
            b"+`a`b`c\0" as *const u8 as *const libc::c_char,
            s_90,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_91: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"_ 4.6\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1099 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"_ 4.6\0" as *const u8 as *const libc::c_char,
            s_91,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_92: S = ts(
        tp(
            tc(
                b"-5\0" as *const u8 as *const libc::c_char as S,
                b"_ -4.6\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1100 as libc::c_int,
            b"-5\0" as *const u8 as *const libc::c_char,
            b"_ -4.6\0" as *const u8 as *const libc::c_char,
            s_92,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_93: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"2=1.999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1101 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"2=1.999\0" as *const u8 as *const libc::c_char,
            s_93,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_94: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"2=1.9999999999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1102 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"2=1.9999999999999999999\0" as *const u8 as *const libc::c_char,
            s_94,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_95: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"1=1.001\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1103 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"1=1.001\0" as *const u8 as *const libc::c_char,
            s_95,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_96: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"1=1.00000000000000000001\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1104 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"1=1.00000000000000000001\0" as *const u8 as *const libc::c_char,
            s_96,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_97: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"_ 1.999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1105 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"_ 1.999\0" as *const u8 as *const libc::c_char,
            s_97,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_98: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"_ 1.999999999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1106 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"_ 1.999999999999999999\0" as *const u8 as *const libc::c_char,
            s_98,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_99: S = ts(
        tp(
            tc(
                b"(,\"0\";,\"9\")\0" as *const u8 as *const libc::c_char as S,
                b"$ 0 9\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1107 as libc::c_int,
            b"(,\"0\";,\"9\")\0" as *const u8 as *const libc::c_char,
            b"$ 0 9\0" as *const u8 as *const libc::c_char,
            s_99,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_100: S = ts(
        tp(
            tc(
                b"\"123\"\0" as *const u8 as *const libc::c_char as S,
                b"$\"123\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1108 as libc::c_int,
            b"\"123\"\0" as *const u8 as *const libc::c_char,
            b"$\"123\"\0" as *const u8 as *const libc::c_char,
            s_100,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_101: S = ts(
        tp(
            tc(
                b"(,\"+\")\0" as *const u8 as *const libc::c_char as S,
                b"$(+)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1109 as libc::c_int,
            b"(,\"+\")\0" as *const u8 as *const libc::c_char,
            b"$(+)\0" as *const u8 as *const libc::c_char,
            s_101,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_102: S = ts(
        tp(
            tc(
                b"\"+:\"\0" as *const u8 as *const libc::c_char as S,
                b"$(+:)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1110 as libc::c_int,
            b"\"+:\"\0" as *const u8 as *const libc::c_char,
            b"$(+:)\0" as *const u8 as *const libc::c_char,
            s_102,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_103: S = ts(
        tp(
            tc(
                b"\"{x}\"\0" as *const u8 as *const libc::c_char as S,
                b"${x}\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1111 as libc::c_int,
            b"\"{x}\"\0" as *const u8 as *const libc::c_char,
            b"${x}\0" as *const u8 as *const libc::c_char,
            s_103,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_104: S = ts(
        tp(
            tc(
                b"\"_ssr\"\0" as *const u8 as *const libc::c_char as S,
                b"$(_ssr)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1112 as libc::c_int,
            b"\"_ssr\"\0" as *const u8 as *const libc::c_char,
            b"$(_ssr)\0" as *const u8 as *const libc::c_char,
            s_104,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_105: S = ts(
        tp(
            tc(
                b"\"+/\"\0" as *const u8 as *const libc::c_char as S,
                b"$(+/)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1113 as libc::c_int,
            b"\"+/\"\0" as *const u8 as *const libc::c_char,
            b"$(+/)\0" as *const u8 as *const libc::c_char,
            s_105,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_106: S = ts(
        tp(
            tc(
                b"\"+/:\"\0" as *const u8 as *const libc::c_char as S,
                b"$(+/:)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1114 as libc::c_int,
            b"\"+/:\"\0" as *const u8 as *const libc::c_char,
            b"$(+/:)\0" as *const u8 as *const libc::c_char,
            s_106,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_107: S = ts(
        tp(
            tc(
                b"\"_ssr/\"\0" as *const u8 as *const libc::c_char as S,
                b"$(_ssr/)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1115 as libc::c_int,
            b"\"_ssr/\"\0" as *const u8 as *const libc::c_char,
            b"$(_ssr/)\0" as *const u8 as *const libc::c_char,
            s_107,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_108: S = ts(
        tp(
            tc(
                b"\"+*\"\0" as *const u8 as *const libc::c_char as S,
                b"$(+*)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1116 as libc::c_int,
            b"\"+*\"\0" as *const u8 as *const libc::c_char,
            b"$(+*)\0" as *const u8 as *const libc::c_char,
            s_108,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_109: S = ts(
        tp(
            tc(
                b"\"**\"\0" as *const u8 as *const libc::c_char as S,
                b"2$2.345\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1117 as libc::c_int,
            b"\"**\"\0" as *const u8 as *const libc::c_char,
            b"2$2.345\0" as *const u8 as *const libc::c_char,
            s_109,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_110: S = ts(
        tp(
            tc(
                b"\"   abcd\"\0" as *const u8 as *const libc::c_char as S,
                b"7$`abcd\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1118 as libc::c_int,
            b"\"   abcd\"\0" as *const u8 as *const libc::c_char,
            b"7$`abcd\0" as *const u8 as *const libc::c_char,
            s_110,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_111: S = ts(
        tp(
            tc(
                b"\"   2.35\"\0" as *const u8 as *const libc::c_char as S,
                b"7.2 $ 2.345\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1119 as libc::c_int,
            b"\"   2.35\"\0" as *const u8 as *const libc::c_char,
            b"7.2 $ 2.345\0" as *const u8 as *const libc::c_char,
            s_111,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_112: S = ts(
        tp(
            tc(
                b"\" 714.00\"\0" as *const u8 as *const libc::c_char as S,
                b"7.2 $ 714\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1120 as libc::c_int,
            b"\" 714.00\"\0" as *const u8 as *const libc::c_char,
            b"7.2 $ 714\0" as *const u8 as *const libc::c_char,
            s_112,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_113: S = ts(
        tp(
            tc(
                b"\"1.2e-34\"\0" as *const u8 as *const libc::c_char as S,
                b"$ 1.2e-34\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1122 as libc::c_int,
            b"\"1.2e-34\"\0" as *const u8 as *const libc::c_char,
            b"$ 1.2e-34\0" as *const u8 as *const libc::c_char,
            s_113,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_114: S = ts(
        tp(
            tc(
                b"\"2.35e+00 \"\0" as *const u8 as *const libc::c_char as S,
                b"-9.2 $ 2.345\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1123 as libc::c_int,
            b"\"2.35e+00 \"\0" as *const u8 as *const libc::c_char,
            b"-9.2 $ 2.345\0" as *const u8 as *const libc::c_char,
            s_114,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_115: S = ts(
        tp(
            tc(
                b"\"7.14e+02 \"\0" as *const u8 as *const libc::c_char as S,
                b"-9.2 $ 714\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1124 as libc::c_int,
            b"\"7.14e+02 \"\0" as *const u8 as *const libc::c_char,
            b"-9.2 $ 714\0" as *const u8 as *const libc::c_char,
            s_115,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_116: S = ts(
        tp(
            tc(
                b"27\0" as *const u8 as *const libc::c_char as S,
                b"0$\"27\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1130 as libc::c_int,
            b"27\0" as *const u8 as *const libc::c_char,
            b"0$\"27\"\0" as *const u8 as *const libc::c_char,
            s_116,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_117: S = ts(
        tp(
            tc(
                b"3.4\0" as *const u8 as *const libc::c_char as S,
                b"0.0$\"3.4\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1131 as libc::c_int,
            b"3.4\0" as *const u8 as *const libc::c_char,
            b"0.0$\"3.4\"\0" as *const u8 as *const libc::c_char,
            s_117,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_118: S = ts(
        tp(
            tc(
                b"27.0\0" as *const u8 as *const libc::c_char as S,
                b"0.0$\"27\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1132 as libc::c_int,
            b"27.0\0" as *const u8 as *const libc::c_char,
            b"0.0$\"27\"\0" as *const u8 as *const libc::c_char,
            s_118,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_119: S = ts(
        tp(
            tc(
                b"\"ab\"\0" as *const u8 as *const libc::c_char as S,
                b"\" \"$\"ab\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1133 as libc::c_int,
            b"\"ab\"\0" as *const u8 as *const libc::c_char,
            b"\" \"$\"ab\"\0" as *const u8 as *const libc::c_char,
            s_119,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_120: S = ts(
        tp(
            tc(
                b"`abc\0" as *const u8 as *const libc::c_char as S,
                b"`$\"abc\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1134 as libc::c_int,
            b"`abc\0" as *const u8 as *const libc::c_char,
            b"`$\"abc\"\0" as *const u8 as *const libc::c_char,
            s_120,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_121: S = ts(
        tp(
            tc(
                b"(23;(23.4;`abc))\0" as *const u8 as *const libc::c_char as S,
                b"(0;(0.0;`)) $ (\"23\";(\"23.4\";\"abc\"))\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1135 as libc::c_int,
            b"(23;(23.4;`abc))\0" as *const u8 as *const libc::c_char,
            b"(0;(0.0;`)) $ (\"23\";(\"23.4\";\"abc\"))\0" as *const u8
                as *const libc::c_char,
            s_121,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_122: S = ts(
        tp(
            tc(
                b"4#$ _log 2\0" as *const u8 as *const libc::c_char as S,
                b"4#$ (_exp) ? 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1136 as libc::c_int,
            b"4#$ _log 2\0" as *const u8 as *const libc::c_char,
            b"4#$ (_exp) ? 2\0" as *const u8 as *const libc::c_char,
            s_122,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_123: S = ts(
        tp(
            tc(
                b"4#$_sqrt 2\0" as *const u8 as *const libc::c_char as S,
                b"4#$ {x^2} ? 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1137 as libc::c_int,
            b"4#$_sqrt 2\0" as *const u8 as *const libc::c_char,
            b"4#$ {x^2} ? 2\0" as *const u8 as *const libc::c_char,
            s_123,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_124: S = ts(
        tp(
            tc(
                b"2 0 3 1\0" as *const u8 as *const libc::c_char as S,
                b"> 3 1 4 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1138 as libc::c_int,
            b"2 0 3 1\0" as *const u8 as *const libc::c_char,
            b"> 3 1 4 2\0" as *const u8 as *const libc::c_char,
            s_124,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_125: S = ts(
        tp(
            tc(
                b"4 3 2 1\0" as *const u8 as *const libc::c_char as S,
                b"3 1 4 2[2 0 3 1]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1139 as libc::c_int,
            b"4 3 2 1\0" as *const u8 as *const libc::c_char,
            b"3 1 4 2[2 0 3 1]\0" as *const u8 as *const libc::c_char,
            s_125,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_126: S = ts(
        tp(
            tc(
                b"0 2 1\0" as *const u8 as *const libc::c_char as S,
                b"> (8 1 6;3 5 7; 3 9 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1140 as libc::c_int,
            b"0 2 1\0" as *const u8 as *const libc::c_char,
            b"> (8 1 6;3 5 7; 3 9 2)\0" as *const u8 as *const libc::c_char,
            s_126,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_127: S = ts(
        tp(
            tc(
                b"\"zoned\"\0" as *const u8 as *const libc::c_char as S,
                b"a:\"dozen\"; a[>a]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1141 as libc::c_int,
            b"\"zoned\"\0" as *const u8 as *const libc::c_char,
            b"a:\"dozen\"; a[>a]\0" as *const u8 as *const libc::c_char,
            s_127,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_128: S = ts(
        tp(
            tc(
                b"2 1 0\0" as *const u8 as *const libc::c_char as S,
                b"> `aaa `bb `c\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1142 as libc::c_int,
            b"2 1 0\0" as *const u8 as *const libc::c_char,
            b"> `aaa `bb `c\0" as *const u8 as *const libc::c_char,
            s_128,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_129: S = ts(
        tp(
            tc(
                b"1 3 0 2\0" as *const u8 as *const libc::c_char as S,
                b"< 3 1 4 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1144 as libc::c_int,
            b"1 3 0 2\0" as *const u8 as *const libc::c_char,
            b"< 3 1 4 2\0" as *const u8 as *const libc::c_char,
            s_129,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_130: S = ts(
        tp(
            tc(
                b"1 2 3 4\0" as *const u8 as *const libc::c_char as S,
                b"3 1 4 2[1 3 0 2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1145 as libc::c_int,
            b"1 2 3 4\0" as *const u8 as *const libc::c_char,
            b"3 1 4 2[1 3 0 2]\0" as *const u8 as *const libc::c_char,
            s_130,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_131: S = ts(
        tp(
            tc(
                b"\"adept\"\0" as *const u8 as *const libc::c_char as S,
                b"a:\"taped\"; a[<a]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1146 as libc::c_int,
            b"\"adept\"\0" as *const u8 as *const libc::c_char,
            b"a:\"taped\"; a[<a]\0" as *const u8 as *const libc::c_char,
            s_131,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_132: S = ts(
        tp(
            tc(
                b"(1 2 0;(3 5 7;4 9 2;8 1 6))\0" as *const u8 as *const libc::c_char
                    as S,
                b"ms:(8 1 6;3 5 7;4 9 2); (<ms;ms[<ms])\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1147 as libc::c_int,
            b"(1 2 0;(3 5 7;4 9 2;8 1 6))\0" as *const u8 as *const libc::c_char,
            b"ms:(8 1 6;3 5 7;4 9 2); (<ms;ms[<ms])\0" as *const u8
                as *const libc::c_char,
            s_132,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_133: S = ts(
        tp(
            tc(
                b"(0 2 3;1 4 5)\0" as *const u8 as *const libc::c_char as S,
                b"= 2 1 2 2 1 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1148 as libc::c_int,
            b"(0 2 3;1 4 5)\0" as *const u8 as *const libc::c_char,
            b"= 2 1 2 2 1 1\0" as *const u8 as *const libc::c_char,
            s_133,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_134: S = ts(
        tp(
            tc(
                b"(,0;1 2 4;,3;,5;,6)\0" as *const u8 as *const libc::c_char as S,
                b"=\"weekend\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1149 as libc::c_int,
            b"(,0;1 2 4;,3;,5;,6)\0" as *const u8 as *const libc::c_char,
            b"=\"weekend\"\0" as *const u8 as *const libc::c_char,
            s_134,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_135: S = ts(
        tp(
            tc(
                b"(0 2 5;1 4;,3)\0" as *const u8 as *const libc::c_char as S,
                b"=(9 2 3;4 5;9 2 3;6 7 8;4 5;9 2 3)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1150 as libc::c_int,
            b"(0 2 5;1 4;,3)\0" as *const u8 as *const libc::c_char,
            b"=(9 2 3;4 5;9 2 3;6 7 8;4 5;9 2 3)\0" as *const u8 as *const libc::c_char,
            s_135,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_136: S = ts(
        tp(
            tc(
                b"\"e\"\0" as *const u8 as *const libc::c_char as S,
                b"\"abcdefg\"@4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1151 as libc::c_int,
            b"\"e\"\0" as *const u8 as *const libc::c_char,
            b"\"abcdefg\"@4\0" as *const u8 as *const libc::c_char,
            s_136,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_137: S = ts(
        tp(
            tc(
                b"\"faded\"\0" as *const u8 as *const libc::c_char as S,
                b"\"abcdefg\"@ 5 0 3 4 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1152 as libc::c_int,
            b"\"faded\"\0" as *const u8 as *const libc::c_char,
            b"\"abcdefg\"@ 5 0 3 4 3\0" as *const u8 as *const libc::c_char,
            s_137,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_138: S = ts(
        tp(
            tc(
                b"(\"fa\";(\"d\";,\"ed\"))\0" as *const u8 as *const libc::c_char as S,
                b"\"abcdefg\"@(5 0;(3;,4 3))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1153 as libc::c_int,
            b"(\"fa\";(\"d\";,\"ed\"))\0" as *const u8 as *const libc::c_char,
            b"\"abcdefg\"@(5 0;(3;,4 3))\0" as *const u8 as *const libc::c_char,
            s_138,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_139: S = ts(
        tp(
            tc(
                b"((7 4 9 2;3 5);(3 5;7 4 9 2;8 1 6))\0" as *const u8
                    as *const libc::c_char as S,
                b"(8 1 6;3 5;7 4 9 2) @ ( 2 1; 1 2 0)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1154 as libc::c_int,
            b"((7 4 9 2;3 5);(3 5;7 4 9 2;8 1 6))\0" as *const u8 as *const libc::c_char,
            b"(8 1 6;3 5;7 4 9 2) @ ( 2 1; 1 2 0)\0" as *const u8 as *const libc::c_char,
            s_139,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_140: S = ts(
        tp(
            tc(
                b"(2 3 4;(\"abcdefg\";2 3 4))\0" as *const u8 as *const libc::c_char
                    as S,
                b"d:.((`a;2 3 4);(`b;\"abcdefg\")); (d @`a; d @ `b`a)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1155 as libc::c_int,
            b"(2 3 4;(\"abcdefg\";2 3 4))\0" as *const u8 as *const libc::c_char,
            b"d:.((`a;2 3 4);(`b;\"abcdefg\")); (d @`a; d @ `b`a)\0" as *const u8
                as *const libc::c_char,
            s_140,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_141: S = ts(
        tp(
            tc(
                b"((8 9;10;11 12);11 12;11)\0" as *const u8 as *const libc::c_char as S,
                b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20) ); (d . 1; d . 1 2; d . 1 2 0)\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1156 as libc::c_int,
            b"((8 9;10;11 12);11 12;11)\0" as *const u8 as *const libc::c_char,
            b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20) ); (d . 1; d . 1 2; d . 1 2 0)\0"
                as *const u8 as *const libc::c_char,
            s_141,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_142: S = ts(
        tp(
            tc(
                b"11 11\0" as *const u8 as *const libc::c_char as S,
                b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20) ); ((((d @ 1)@2)@0);(d @/1 2 0))\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1157 as libc::c_int,
            b"11 11\0" as *const u8 as *const libc::c_char,
            b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20) ); ((((d @ 1)@2)@0);(d @/1 2 0))\0"
                as *const u8 as *const libc::c_char,
            s_142,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_143: S = ts(
        tp(
            tc(
                b"((13 14;15 16 17 18);(1 2 3;4 5 6 7))\0" as *const u8
                    as *const libc::c_char as S,
                b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20) ); d . (2 0;0 1)\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1158 as libc::c_int,
            b"((13 14;15 16 17 18);(1 2 3;4 5 6 7))\0" as *const u8
                as *const libc::c_char,
            b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20) ); d . (2 0;0 1)\0"
                as *const u8 as *const libc::c_char,
            s_143,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_144: S = ts(
        tp(
            tc(
                b"(1 2 3;8 9;13 14)\0" as *const u8 as *const libc::c_char as S,
                b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20) ); d . (;0)\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1159 as libc::c_int,
            b"(1 2 3;8 9;13 14)\0" as *const u8 as *const libc::c_char,
            b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20) ); d . (;0)\0"
                as *const u8 as *const libc::c_char,
            s_144,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_145: S = ts(
        tp(
            tc(
                b"((2 1;5 4);(14 13;16 15;20 19))\0" as *const u8 as *const libc::c_char
                    as S,
                b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20)); d . (0 2;;1 0)\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1160 as libc::c_int,
            b"((2 1;5 4);(14 13;16 15;20 19))\0" as *const u8 as *const libc::c_char,
            b"d:((1 2 3;4 5 6 7);(8 9;10;11 12);(13 14;15 16 17 18;19 20)); d . (0 2;;1 0)\0"
                as *const u8 as *const libc::c_char,
            s_145,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_146: S = ts(
        tp(
            tc(
                b"30\0" as *const u8 as *const libc::c_char as S,
                b"(1;.((`a; 2 3 4);(`b; 10 20 30 40))) . (1; `b; 2)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1161 as libc::c_int,
            b"30\0" as *const u8 as *const libc::c_char,
            b"(1;.((`a; 2 3 4);(`b; 10 20 30 40))) . (1; `b; 2)\0" as *const u8
                as *const libc::c_char,
            s_146,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_147: S = ts(
        tp(
            tc(
                b"1 2 3\0" as *const u8 as *const libc::c_char as S,
                b"1 2 3 . ()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1162 as libc::c_int,
            b"1 2 3\0" as *const u8 as *const libc::c_char,
            b"1 2 3 . ()\0" as *const u8 as *const libc::c_char,
            s_147,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_148: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"10 . ()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1163 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"10 . ()\0" as *const u8 as *const libc::c_char,
            s_148,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_149: S = ts(
        tp(
            tc(
                b"1 4 5 6 7\0" as *const u8 as *const libc::c_char as S,
                b"1 , 4 5 6 7\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1164 as libc::c_int,
            b"1 4 5 6 7\0" as *const u8 as *const libc::c_char,
            b"1 , 4 5 6 7\0" as *const u8 as *const libc::c_char,
            s_149,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_150: S = ts(
        tp(
            tc(
                b"1 2 3 4\0" as *const u8 as *const libc::c_char as S,
                b"1 2 3 , 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1165 as libc::c_int,
            b"1 2 3 4\0" as *const u8 as *const libc::c_char,
            b"1 2 3 , 4\0" as *const u8 as *const libc::c_char,
            s_150,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_151: S = ts(
        tp(
            tc(
                b"1 2 3 4 5 6 7\0" as *const u8 as *const libc::c_char as S,
                b"1 2 3 , 4 5 6 7\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1166 as libc::c_int,
            b"1 2 3 4 5 6 7\0" as *const u8 as *const libc::c_char,
            b"1 2 3 , 4 5 6 7\0" as *const u8 as *const libc::c_char,
            s_151,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_152: S = ts(
        tp(
            tc(
                b"(1;2;3;8 1 6;3 5 7;4 9 2)\0" as *const u8 as *const libc::c_char as S,
                b"1 2 3, (8 1 6;3 5 7;4 9 2)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1167 as libc::c_int,
            b"(1;2;3;8 1 6;3 5 7;4 9 2)\0" as *const u8 as *const libc::c_char,
            b"1 2 3, (8 1 6;3 5 7;4 9 2)\0" as *const u8 as *const libc::c_char,
            s_152,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_153: S = ts(
        tp(
            tc(
                b"0 0 0 1\0" as *const u8 as *const libc::c_char as S,
                b"1 < -1 0 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1168 as libc::c_int,
            b"0 0 0 1\0" as *const u8 as *const libc::c_char,
            b"1 < -1 0 1 2\0" as *const u8 as *const libc::c_char,
            s_153,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_154: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"\"a\" < \"z\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1169 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"\"a\" < \"z\"\0" as *const u8 as *const libc::c_char,
            s_154,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_155: S = ts(
        tp(
            tc(
                b"0 1\0" as *const u8 as *const libc::c_char as S,
                b"\"aA\" < \"Z\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1170 as libc::c_int,
            b"0 1\0" as *const u8 as *const libc::c_char,
            b"\"aA\" < \"Z\"\0" as *const u8 as *const libc::c_char,
            s_155,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_156: S = ts(
        tp(
            tc(
                b"0 1\0" as *const u8 as *const libc::c_char as S,
                b"`inch`mile < `foot`yard\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1171 as libc::c_int,
            b"0 1\0" as *const u8 as *const libc::c_char,
            b"`inch`mile < `foot`yard\0" as *const u8 as *const libc::c_char,
            s_156,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_157: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"1 < 1.000000000001\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1172 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"1 < 1.000000000001\0" as *const u8 as *const libc::c_char,
            s_157,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_158: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"1 < 1.0000000000000000001\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1173 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"1 < 1.0000000000000000001\0" as *const u8 as *const libc::c_char,
            s_158,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_159: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0.999999999999 < 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1174 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0.999999999999 < 1\0" as *const u8 as *const libc::c_char,
            s_159,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_160: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"0.999999999999999999 < 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1175 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"0.999999999999999999 < 1\0" as *const u8 as *const libc::c_char,
            s_160,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_161: S = ts(
        tp(
            tc(
                b"x:.((`a;4 4);(`b;\"aa\"));x\0" as *const u8 as *const libc::c_char
                    as S,
                b".(. x)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1176 as libc::c_int,
            b"x:.((`a;4 4);(`b;\"aa\"));x\0" as *const u8 as *const libc::c_char,
            b".(. x)\0" as *const u8 as *const libc::c_char,
            s_161,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_162: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"2 3 ~ 2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1177 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"2 3 ~ 2 3\0" as *const u8 as *const libc::c_char,
            s_162,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_163: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"() ~ !0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1178 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"() ~ !0\0" as *const u8 as *const libc::c_char,
            s_163,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_164: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"\"a\" ~ ,\"a\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1179 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"\"a\" ~ ,\"a\"\0" as *const u8 as *const libc::c_char,
            s_164,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_165: S = ts(
        tp(
            tc(
                b"8\0" as *const u8 as *const libc::c_char as S,
                b"3|8\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1180 as libc::c_int,
            b"8\0" as *const u8 as *const libc::c_char,
            b"3|8\0" as *const u8 as *const libc::c_char,
            s_165,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_166: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"3| -8\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1181 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"3| -8\0" as *const u8 as *const libc::c_char,
            s_166,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_167: S = ts(
        tp(
            tc(
                b"987.65\0" as *const u8 as *const libc::c_char as S,
                b"123.45 | 987.65\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1182 as libc::c_int,
            b"987.65\0" as *const u8 as *const libc::c_char,
            b"123.45 | 987.65\0" as *const u8 as *const libc::c_char,
            s_167,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_168: S = ts(
        tp(
            tc(
                b"123.45\0" as *const u8 as *const libc::c_char as S,
                b"123.45 | -987.65\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1183 as libc::c_int,
            b"123.45\0" as *const u8 as *const libc::c_char,
            b"123.45 | -987.65\0" as *const u8 as *const libc::c_char,
            s_168,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_169: S = ts(
        tp(
            tc(
                b"0 1 1 1\0" as *const u8 as *const libc::c_char as S,
                b"0 0 1 1 | 0 1 0 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1184 as libc::c_int,
            b"0 1 1 1\0" as *const u8 as *const libc::c_char,
            b"0 0 1 1 | 0 1 0 1\0" as *const u8 as *const libc::c_char,
            s_169,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_170: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"3 & 8\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1185 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"3 & 8\0" as *const u8 as *const libc::c_char,
            s_170,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_171: S = ts(
        tp(
            tc(
                b"-8\0" as *const u8 as *const libc::c_char as S,
                b"3 & -8\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1186 as libc::c_int,
            b"-8\0" as *const u8 as *const libc::c_char,
            b"3 & -8\0" as *const u8 as *const libc::c_char,
            s_171,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_172: S = ts(
        tp(
            tc(
                b"123.45\0" as *const u8 as *const libc::c_char as S,
                b"123.45 & 987.65\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1187 as libc::c_int,
            b"123.45\0" as *const u8 as *const libc::c_char,
            b"123.45 & 987.65\0" as *const u8 as *const libc::c_char,
            s_172,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_173: S = ts(
        tp(
            tc(
                b"-987.65\0" as *const u8 as *const libc::c_char as S,
                b"123.45 & -987.65\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1188 as libc::c_int,
            b"-987.65\0" as *const u8 as *const libc::c_char,
            b"123.45 & -987.65\0" as *const u8 as *const libc::c_char,
            s_173,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_174: S = ts(
        tp(
            tc(
                b"0 0 0 1\0" as *const u8 as *const libc::c_char as S,
                b"0 0 1 1 & 0 1 0 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1189 as libc::c_int,
            b"0 0 0 1\0" as *const u8 as *const libc::c_char,
            b"0 0 1 1 & 0 1 0 1\0" as *const u8 as *const libc::c_char,
            s_174,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_175: S = ts(
        tp(
            tc(
                b"1 1 0 0\0" as *const u8 as *const libc::c_char as S,
                b"1 > -1 0 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1190 as libc::c_int,
            b"1 1 0 0\0" as *const u8 as *const libc::c_char,
            b"1 > -1 0 1 2\0" as *const u8 as *const libc::c_char,
            s_175,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_176: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"\"a\" > \"z\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1191 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"\"a\" > \"z\"\0" as *const u8 as *const libc::c_char,
            s_176,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_177: S = ts(
        tp(
            tc(
                b"1 0\0" as *const u8 as *const libc::c_char as S,
                b"\"aA\" > \"Z\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1192 as libc::c_int,
            b"1 0\0" as *const u8 as *const libc::c_char,
            b"\"aA\" > \"Z\"\0" as *const u8 as *const libc::c_char,
            s_177,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_178: S = ts(
        tp(
            tc(
                b"1 0\0" as *const u8 as *const libc::c_char as S,
                b"`inch`mile > `foot`yard\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1193 as libc::c_int,
            b"1 0\0" as *const u8 as *const libc::c_char,
            b"`inch`mile > `foot`yard\0" as *const u8 as *const libc::c_char,
            s_178,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_179: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"1.000000000001 > 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1194 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"1.000000000001 > 1\0" as *const u8 as *const libc::c_char,
            s_179,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_180: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"1.0000000000000000001 > 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1195 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"1.0000000000000000001 > 1\0" as *const u8 as *const libc::c_char,
            s_180,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_181: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"1 > 0.999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1196 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"1 > 0.999999999999\0" as *const u8 as *const libc::c_char,
            s_181,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_182: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"1 > 0.999999999999999999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1197 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"1 > 0.999999999999999999\0" as *const u8 as *const libc::c_char,
            s_182,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_183: S = ts(
        tp(
            tc(
                b"0 1\0" as *const u8 as *const libc::c_char as S,
                b"~ 1 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1198 as libc::c_int,
            b"0 1\0" as *const u8 as *const libc::c_char,
            b"~ 1 0\0" as *const u8 as *const libc::c_char,
            s_183,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_184: S = ts(
        tp(
            tc(
                b"0 1 0\0" as *const u8 as *const libc::c_char as S,
                b"~ 4.6 0 -4.6\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1199 as libc::c_int,
            b"0 1 0\0" as *const u8 as *const libc::c_char,
            b"~ 4.6 0 -4.6\0" as *const u8 as *const libc::c_char,
            s_184,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_185: S = ts(
        tp(
            tc(
                b"~`c\0" as *const u8 as *const libc::c_char as S,
                b"`c.\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1200 as libc::c_int,
            b"~`c\0" as *const u8 as *const libc::c_char,
            b"`c.\0" as *const u8 as *const libc::c_char,
            s_185,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_186: S = ts(
        tp(
            tc(
                b"8.0\0" as *const u8 as *const libc::c_char as S,
                b"2^3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1202 as libc::c_int,
            b"8.0\0" as *const u8 as *const libc::c_char,
            b"2^3\0" as *const u8 as *const libc::c_char,
            s_186,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_187: S = ts(
        tp(
            tc(
                b"-8.0\0" as *const u8 as *const libc::c_char as S,
                b"-2^3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1203 as libc::c_int,
            b"-8.0\0" as *const u8 as *const libc::c_char,
            b"-2^3\0" as *const u8 as *const libc::c_char,
            s_187,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_188: S = ts(
        tp(
            tc(
                b"1.0\0" as *const u8 as *const libc::c_char as S,
                b"0^0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1204 as libc::c_int,
            b"1.0\0" as *const u8 as *const libc::c_char,
            b"0^0\0" as *const u8 as *const libc::c_char,
            s_188,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_189: S = ts(
        tp(
            tc(
                b"4.0\0" as *const u8 as *const libc::c_char as S,
                b"-2.0^2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1205 as libc::c_int,
            b"4.0\0" as *const u8 as *const libc::c_char,
            b"-2.0^2\0" as *const u8 as *const libc::c_char,
            s_189,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_190: S = ts(
        tp(
            tc(
                b"3#$1.414214\0" as *const u8 as *const libc::c_char as S,
                b"3#$ 2.0^0.5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1206 as libc::c_int,
            b"3#$1.414214\0" as *const u8 as *const libc::c_char,
            b"3#$ 2.0^0.5\0" as *const u8 as *const libc::c_char,
            s_190,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_191: S = ts(
        tp(
            tc(
                b"0i\0" as *const u8 as *const libc::c_char as S,
                b"10^1000\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1207 as libc::c_int,
            b"0i\0" as *const u8 as *const libc::c_char,
            b"10^1000\0" as *const u8 as *const libc::c_char,
            s_191,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_192: S = ts(
        tp(
            tc(
                b"9 6 8 7\0" as *const u8 as *const libc::c_char as S,
                b"? 9 6 8 6 9 7 8 9 6\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1208 as libc::c_int,
            b"9 6 8 7\0" as *const u8 as *const libc::c_char,
            b"? 9 6 8 6 9 7 8 9 6\0" as *const u8 as *const libc::c_char,
            s_192,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_193: S = ts(
        tp(
            tc(
                b"\"strange\"\0" as *const u8 as *const libc::c_char as S,
                b"?\"strange\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1209 as libc::c_int,
            b"\"strange\"\0" as *const u8 as *const libc::c_char,
            b"?\"strange\"\0" as *const u8 as *const libc::c_char,
            s_193,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_194: S = ts(
        tp(
            tc(
                b"\"racon\"\0" as *const u8 as *const libc::c_char as S,
                b"?\"raccoon\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1210 as libc::c_int,
            b"\"racon\"\0" as *const u8 as *const libc::c_char,
            b"?\"raccoon\"\0" as *const u8 as *const libc::c_char,
            s_194,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_195: S = ts(
        tp(
            tc(
                b"(9 2 3;4 5;6 7 8)\0" as *const u8 as *const libc::c_char as S,
                b"? (9 2 3;4 5;9 2 3;6 7 8;4 5;9 2 3)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1211 as libc::c_int,
            b"(9 2 3;4 5;6 7 8)\0" as *const u8 as *const libc::c_char,
            b"? (9 2 3;4 5;9 2 3;6 7 8;4 5;9 2 3)\0" as *const u8 as *const libc::c_char,
            s_195,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_196: S = ts(
        tp(
            tc(
                b"2.0\0" as *const u8 as *const libc::c_char as S,
                b"%% 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1212 as libc::c_int,
            b"2.0\0" as *const u8 as *const libc::c_char,
            b"%% 2\0" as *const u8 as *const libc::c_char,
            s_196,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_197: S = ts(
        tp(
            tc(
                b"1.0\0" as *const u8 as *const libc::c_char as S,
                b"%%1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1213 as libc::c_int,
            b"1.0\0" as *const u8 as *const libc::c_char,
            b"%%1\0" as *const u8 as *const libc::c_char,
            s_197,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_198: S = ts(
        tp(
            tc(
                b"0.0\0" as *const u8 as *const libc::c_char as S,
                b"%%0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1214 as libc::c_int,
            b"0.0\0" as *const u8 as *const libc::c_char,
            b"%%0\0" as *const u8 as *const libc::c_char,
            s_198,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_199: S = ts(
        tp(
            tc(
                b"0i\0" as *const u8 as *const libc::c_char as S,
                b"%0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1215 as libc::c_int,
            b"0i\0" as *const u8 as *const libc::c_char,
            b"%0\0" as *const u8 as *const libc::c_char,
            s_199,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_200: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"0i>3.13\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1216 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"0i>3.13\0" as *const u8 as *const libc::c_char,
            s_200,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_201: S = ts(
        tp(
            tc(
                b"0N\0" as *const u8 as *const libc::c_char as S,
                b"0%0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1217 as libc::c_int,
            b"0N\0" as *const u8 as *const libc::c_char,
            b"0%0\0" as *const u8 as *const libc::c_char,
            s_201,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_202: S = ts(
        tp(
            tc(
                b"2 4 1 3\0" as *const u8 as *const libc::c_char as S,
                b"|3 1 4 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1218 as libc::c_int,
            b"2 4 1 3\0" as *const u8 as *const libc::c_char,
            b"|3 1 4 2\0" as *const u8 as *const libc::c_char,
            s_202,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_203: S = ts(
        tp(
            tc(
                b"m:(8 1 6;3 5 7;4 9 2); |m\0" as *const u8 as *const libc::c_char as S,
                b"(4 9 2;3 5 7;8 1 6)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1219 as libc::c_int,
            b"m:(8 1 6;3 5 7;4 9 2); |m\0" as *const u8 as *const libc::c_char,
            b"(4 9 2;3 5 7;8 1 6)\0" as *const u8 as *const libc::c_char,
            s_203,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_204: S = ts(
        tp(
            tc(
                b"`three\0" as *const u8 as *const libc::c_char as S,
                b"*|`one`two`three\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1220 as libc::c_int,
            b"`three\0" as *const u8 as *const libc::c_char,
            b"*|`one`two`three\0" as *const u8 as *const libc::c_char,
            s_204,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_205: S = ts(
        tp(
            tc(
                b"\"a\"\0" as *const u8 as *const libc::c_char as S,
                b"|\"a\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1221 as libc::c_int,
            b"\"a\"\0" as *const u8 as *const libc::c_char,
            b"|\"a\"\0" as *const u8 as *const libc::c_char,
            s_205,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_206: S = ts(
        tp(
            tc(
                b"!0\0" as *const u8 as *const libc::c_char as S,
                b"|!0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1222 as libc::c_int,
            b"!0\0" as *const u8 as *const libc::c_char,
            b"|!0\0" as *const u8 as *const libc::c_char,
            s_206,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_207: S = ts(
        tp(
            tc(
                b"(,3 1 4 2)\0" as *const u8 as *const libc::c_char as S,
                b"|,3 1 4 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1223 as libc::c_int,
            b"(,3 1 4 2)\0" as *const u8 as *const libc::c_char,
            b"|,3 1 4 2\0" as *const u8 as *const libc::c_char,
            s_207,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_208: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"5 ! 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1224 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"5 ! 3\0" as *const u8 as *const libc::c_char,
            s_208,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_209: S = ts(
        tp(
            tc(
                b"0.0\0" as *const u8 as *const libc::c_char as S,
                b"2.0 ! 1.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1226 as libc::c_int,
            b"0.0\0" as *const u8 as *const libc::c_char,
            b"2.0 ! 1.0\0" as *const u8 as *const libc::c_char,
            s_209,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_210: S = ts(
        tp(
            tc(
                b"2.0\0" as *const u8 as *const libc::c_char as S,
                b"5.0 ! 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1227 as libc::c_int,
            b"2.0\0" as *const u8 as *const libc::c_char,
            b"5.0 ! 3\0" as *const u8 as *const libc::c_char,
            s_210,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_211: S = ts(
        tp(
            tc(
                b"(\"  0\";\"0.1\")\0" as *const u8 as *const libc::c_char as S,
                b"3 $ 1.8 -2.7 ! 0.2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1228 as libc::c_int,
            b"(\"  0\";\"0.1\")\0" as *const u8 as *const libc::c_char,
            b"3 $ 1.8 -2.7 ! 0.2\0" as *const u8 as *const libc::c_char,
            s_211,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_212: S = ts(
        tp(
            tc(
                b"-1\0" as *const u8 as *const libc::c_char as S,
                b"5 ! -3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1229 as libc::c_int,
            b"-1\0" as *const u8 as *const libc::c_char,
            b"5 ! -3\0" as *const u8 as *const libc::c_char,
            s_212,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_213: S = ts(
        tp(
            tc(
                b"-1.0\0" as *const u8 as *const libc::c_char as S,
                b"5.0 ! -3.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1230 as libc::c_int,
            b"-1.0\0" as *const u8 as *const libc::c_char,
            b"5.0 ! -3.0\0" as *const u8 as *const libc::c_char,
            s_213,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_214: S = ts(
        tp(
            tc(
                b"-3 0 -1\0" as *const u8 as *const libc::c_char as S,
                b"-3 4 -17 ! -4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1231 as libc::c_int,
            b"-3 0 -1\0" as *const u8 as *const libc::c_char,
            b"-3 4 -17 ! -4\0" as *const u8 as *const libc::c_char,
            s_214,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_215: S = ts(
        tp(
            tc(
                b"-3 0 -1.0\0" as *const u8 as *const libc::c_char as S,
                b"-3.0 4 -17 ! -4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1232 as libc::c_int,
            b"-3 0 -1.0\0" as *const u8 as *const libc::c_char,
            b"-3.0 4 -17 ! -4\0" as *const u8 as *const libc::c_char,
            s_215,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_216: S = ts(
        tp(
            tc(
                b"\"fghabcde\"\0" as *const u8 as *const libc::c_char as S,
                b"5 ! \"abcdefgh\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1233 as libc::c_int,
            b"\"fghabcde\"\0" as *const u8 as *const libc::c_char,
            b"5 ! \"abcdefgh\"\0" as *const u8 as *const libc::c_char,
            s_216,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_217: S = ts(
        tp(
            tc(
                b"\"fghabcde\"\0" as *const u8 as *const libc::c_char as S,
                b"21 ! \"abcdefgh\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1234 as libc::c_int,
            b"\"fghabcde\"\0" as *const u8 as *const libc::c_char,
            b"21 ! \"abcdefgh\"\0" as *const u8 as *const libc::c_char,
            s_217,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_218: S = ts(
        tp(
            tc(
                b"\"defghabc\"\0" as *const u8 as *const libc::c_char as S,
                b"-5 ! \"abcdefgh\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1235 as libc::c_int,
            b"\"defghabc\"\0" as *const u8 as *const libc::c_char,
            b"-5 ! \"abcdefgh\"\0" as *const u8 as *const libc::c_char,
            s_218,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_219: S = ts(
        tp(
            tc(
                b"\"defghabc\"\0" as *const u8 as *const libc::c_char as S,
                b"-21 ! \"abcdefgh\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1236 as libc::c_int,
            b"\"defghabc\"\0" as *const u8 as *const libc::c_char,
            b"-21 ! \"abcdefgh\"\0" as *const u8 as *const libc::c_char,
            s_219,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_220: S = ts(
        tp(
            tc(
                b"!0\0" as *const u8 as *const libc::c_char as S,
                b"^ 3.14\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1237 as libc::c_int,
            b"!0\0" as *const u8 as *const libc::c_char,
            b"^ 3.14\0" as *const u8 as *const libc::c_char,
            s_220,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_221: S = ts(
        tp(
            tc(
                b"4 3 2\0" as *const u8 as *const libc::c_char as S,
                b"r:((\"ab\";\"cd\";\"ef\");(\"gh\";\"ij\";\"kl\");(\"mn\";\"op\";\"qr\");(\"st\";\"uv\";\"wx\"));^r\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1238 as libc::c_int,
            b"4 3 2\0" as *const u8 as *const libc::c_char,
            b"r:((\"ab\";\"cd\";\"ef\");(\"gh\";\"ij\";\"kl\");(\"mn\";\"op\";\"qr\");(\"st\";\"uv\";\"wx\"));^r\0"
                as *const u8 as *const libc::c_char,
            s_221,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_222: S = ts(
        tp(
            tc(
                b"4 3\0" as *const u8 as *const libc::c_char as S,
                b"s:((\"aby\";\"cd\";\"ef\");(\"gh\";\"i\";\"kl1\");(\"mn\";\"opz\";\"qr\");(\"st\";\"uv\";\"w\"));^s\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1239 as libc::c_int,
            b"4 3\0" as *const u8 as *const libc::c_char,
            b"s:((\"aby\";\"cd\";\"ef\");(\"gh\";\"i\";\"kl1\");(\"mn\";\"opz\";\"qr\");(\"st\";\"uv\";\"w\"));^s\0"
                as *const u8 as *const libc::c_char,
            s_222,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_223: S = ts(
        tp(
            tc(
                b"(,4)\0" as *const u8 as *const libc::c_char as S,
                b"t:((\"ab\";\"cd\";\"ef\");(\"gh\";\"ij\");(\"kl\";\"mn\";\"op\";\"qr\");(\"st\";\"uv\";\"wx\"));^t\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1240 as libc::c_int,
            b"(,4)\0" as *const u8 as *const libc::c_char,
            b"t:((\"ab\";\"cd\";\"ef\");(\"gh\";\"ij\");(\"kl\";\"mn\";\"op\";\"qr\");(\"st\";\"uv\";\"wx\"));^t\0"
                as *const u8 as *const libc::c_char,
            s_223,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_224: S = ts(
        tp(
            tc(
                b"4 5 6\0" as *const u8 as *const libc::c_char as S,
                b"3 # 4 5 6 7 8 9\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1241 as libc::c_int,
            b"4 5 6\0" as *const u8 as *const libc::c_char,
            b"3 # 4 5 6 7 8 9\0" as *const u8 as *const libc::c_char,
            s_224,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_225: S = ts(
        tp(
            tc(
                b"7 8 9\0" as *const u8 as *const libc::c_char as S,
                b"-3 # 4 5 6 7 8 9\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1242 as libc::c_int,
            b"7 8 9\0" as *const u8 as *const libc::c_char,
            b"-3 # 4 5 6 7 8 9\0" as *const u8 as *const libc::c_char,
            s_225,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_226: S = ts(
        tp(
            tc(
                b"4 5 6 7 8 9 4 5\0" as *const u8 as *const libc::c_char as S,
                b"8 # 4 5 6 7 8 9\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1243 as libc::c_int,
            b"4 5 6 7 8 9 4 5\0" as *const u8 as *const libc::c_char,
            b"8 # 4 5 6 7 8 9\0" as *const u8 as *const libc::c_char,
            s_226,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_227: S = ts(
        tp(
            tc(
                b"9 4 5 6 7 8 9 4 5 6 7 8 9\0" as *const u8 as *const libc::c_char as S,
                b"-13 # 4 5 6 7 8 9\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1244 as libc::c_int,
            b"9 4 5 6 7 8 9 4 5 6 7 8 9\0" as *const u8 as *const libc::c_char,
            b"-13 # 4 5 6 7 8 9\0" as *const u8 as *const libc::c_char,
            s_227,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_228: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"r:((\"ab\";\"cd\";\"ef\");(\"gh\";\"ij\";\"kl\");(\"mn\";\"op\";\"qr\");(\"st\";\"uv\";\"wx\")); r ~ 4 3 2 # \"abcdefghijklmnopqrstuvwxyz\"\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1245 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"r:((\"ab\";\"cd\";\"ef\");(\"gh\";\"ij\";\"kl\");(\"mn\";\"op\";\"qr\");(\"st\";\"uv\";\"wx\")); r ~ 4 3 2 # \"abcdefghijklmnopqrstuvwxyz\"\0"
                as *const u8 as *const libc::c_char,
            s_228,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_229: S = ts(
        tp(
            tc(
                b"2 3 4 0\0" as *const u8 as *const libc::c_char as S,
                b"^ 2 3 4 0 7 8 # \"abc\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1246 as libc::c_int,
            b"2 3 4 0\0" as *const u8 as *const libc::c_char,
            b"^ 2 3 4 0 7 8 # \"abc\"\0" as *const u8 as *const libc::c_char,
            s_229,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_230: S = ts(
        tp(
            tc(
                b"(\"ab\";\"cd\")\0" as *const u8 as *const libc::c_char as S,
                b"2 -1 # \"abcd\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1247 as libc::c_int,
            b"(\"ab\";\"cd\")\0" as *const u8 as *const libc::c_char,
            b"2 -1 # \"abcd\"\0" as *const u8 as *const libc::c_char,
            s_230,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_231: S = ts(
        tp(
            tc(
                b"(\"abcd\";\"efgh\")\0" as *const u8 as *const libc::c_char as S,
                b"2 -1 # \"abcdefgh\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1248 as libc::c_int,
            b"(\"abcd\";\"efgh\")\0" as *const u8 as *const libc::c_char,
            b"2 -1 # \"abcdefgh\"\0" as *const u8 as *const libc::c_char,
            s_231,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_232: S = ts(
        tp(
            tc(
                b"2 2\0" as *const u8 as *const libc::c_char as S,
                b"^ 2 -1 # \"abcd\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1249 as libc::c_int,
            b"2 2\0" as *const u8 as *const libc::c_char,
            b"^ 2 -1 # \"abcd\"\0" as *const u8 as *const libc::c_char,
            s_232,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_233: S = ts(
        tp(
            tc(
                b"2 4\0" as *const u8 as *const libc::c_char as S,
                b"^ 2 -1 # \"abcdefgh\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1250 as libc::c_int,
            b"2 4\0" as *const u8 as *const libc::c_char,
            b"^ 2 -1 # \"abcdefgh\"\0" as *const u8 as *const libc::c_char,
            s_233,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_234: S = ts(
        tp(
            tc(
                b"(,\"abc\";,\"def\")\0" as *const u8 as *const libc::c_char as S,
                b"2 -1 3 # \"abcdef\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1251 as libc::c_int,
            b"(,\"abc\";,\"def\")\0" as *const u8 as *const libc::c_char,
            b"2 -1 3 # \"abcdef\"\0" as *const u8 as *const libc::c_char,
            s_234,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_235: S = ts(
        tp(
            tc(
                b"((\"abc\";\"def\");(\"ghi\";\"jkl\"))\0" as *const u8
                    as *const libc::c_char as S,
                b"2 -1 3 # \"abcdefghijkl\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1252 as libc::c_int,
            b"((\"abc\";\"def\");(\"ghi\";\"jkl\"))\0" as *const u8
                as *const libc::c_char,
            b"2 -1 3 # \"abcdefghijkl\"\0" as *const u8 as *const libc::c_char,
            s_235,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_236: S = ts(
        tp(
            tc(
                b"2 1 3\0" as *const u8 as *const libc::c_char as S,
                b"^ 2 -1 3 # \"abcdef\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1253 as libc::c_int,
            b"2 1 3\0" as *const u8 as *const libc::c_char,
            b"^ 2 -1 3 # \"abcdef\"\0" as *const u8 as *const libc::c_char,
            s_236,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_237: S = ts(
        tp(
            tc(
                b"2 2 3\0" as *const u8 as *const libc::c_char as S,
                b"^ 2 -1 3 # \"abcdefghijkl\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1254 as libc::c_int,
            b"2 2 3\0" as *const u8 as *const libc::c_char,
            b"^ 2 -1 3 # \"abcdefghijkl\"\0" as *const u8 as *const libc::c_char,
            s_237,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_238: S = ts(
        tp(
            tc(
                b"14\0" as *const u8 as *const libc::c_char as S,
                b".\"2+3*4\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1255 as libc::c_int,
            b"14\0" as *const u8 as *const libc::c_char,
            b".\"2+3*4\"\0" as *const u8 as *const libc::c_char,
            s_238,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_239: S = ts(
        tp(
            tc(
                b"2 4 7\0" as *const u8 as *const libc::c_char as S,
                b"& 0 0 1 0 1 0 0 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1256 as libc::c_int,
            b"2 4 7\0" as *const u8 as *const libc::c_char,
            b"& 0 0 1 0 1 0 0 1\0" as *const u8 as *const libc::c_char,
            s_239,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_240: S = ts(
        tp(
            tc(
                b"0 0 0 2 2 2 2\0" as *const u8 as *const libc::c_char as S,
                b"& 3 0 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1257 as libc::c_int,
            b"0 0 0 2 2 2 2\0" as *const u8 as *const libc::c_char,
            b"& 3 0 4\0" as *const u8 as *const libc::c_char,
            s_240,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_241: S = ts(
        tp(
            tc(
                b"0 0 0\0" as *const u8 as *const libc::c_char as S,
                b"&3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1258 as libc::c_int,
            b"0 0 0\0" as *const u8 as *const libc::c_char,
            b"&3\0" as *const u8 as *const libc::c_char,
            s_241,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_242: S = ts(
        tp(
            tc(
                b"(!6;!4;!5)\0" as *const u8 as *const libc::c_char as S,
                b"!:' 6 4 5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1261 as libc::c_int,
            b"(!6;!4;!5)\0" as *const u8 as *const libc::c_char,
            b"!:' 6 4 5\0" as *const u8 as *const libc::c_char,
            s_242,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_243: S = ts(
        tp(
            tc(
                b"6 4 5\0" as *const u8 as *const libc::c_char as S,
                b" #:'!:'6 4 5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1262 as libc::c_int,
            b"6 4 5\0" as *const u8 as *const libc::c_char,
            b" #:'!:'6 4 5\0" as *const u8 as *const libc::c_char,
            s_243,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_244: S = ts(
        tp(
            tc(
                b"(1 0;2 1 0)\0" as *const u8 as *const libc::c_char as S,
                b"|:'!:'2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1263 as libc::c_int,
            b"(1 0;2 1 0)\0" as *const u8 as *const libc::c_char,
            b"|:'!:'2 3\0" as *const u8 as *const libc::c_char,
            s_244,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_245: S = ts(
        tp(
            tc(
                b"15 6 10\0" as *const u8 as *const libc::c_char as S,
                b"+/' !:'6 4 5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1264 as libc::c_int,
            b"15 6 10\0" as *const u8 as *const libc::c_char,
            b"+/' !:'6 4 5\0" as *const u8 as *const libc::c_char,
            s_245,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_246: S = ts(
        tp(
            tc(
                b"\"ab\"\0" as *const u8 as *const libc::c_char as S,
                b"\"a\" ,' \"b\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1265 as libc::c_int,
            b"\"ab\"\0" as *const u8 as *const libc::c_char,
            b"\"a\" ,' \"b\"\0" as *const u8 as *const libc::c_char,
            s_246,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_247: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"(\"a\",'\"b\") ~ \"a\" , \"b\"\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1266 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"(\"a\",'\"b\") ~ \"a\" , \"b\"\0" as *const u8 as *const libc::c_char,
            s_247,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_248: S = ts(
        tp(
            tc(
                b"(\"ab\";\"ac\";\"ad\")\0" as *const u8 as *const libc::c_char as S,
                b" \"a\" ,' \"bcd\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1267 as libc::c_int,
            b"(\"ab\";\"ac\";\"ad\")\0" as *const u8 as *const libc::c_char,
            b" \"a\" ,' \"bcd\"\0" as *const u8 as *const libc::c_char,
            s_248,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_249: S = ts(
        tp(
            tc(
                b"(\"ad\";\"bd\";\"cd\")\0" as *const u8 as *const libc::c_char as S,
                b" \"abc\" ,' \"d\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1268 as libc::c_int,
            b"(\"ad\";\"bd\";\"cd\")\0" as *const u8 as *const libc::c_char,
            b" \"abc\" ,' \"d\"\0" as *const u8 as *const libc::c_char,
            s_249,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_250: S = ts(
        tp(
            tc(
                b"(\"ad\";\"be\";\"cf\")\0" as *const u8 as *const libc::c_char as S,
                b" \"abc\" ,' \"def\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1269 as libc::c_int,
            b"(\"ad\";\"be\";\"cf\")\0" as *const u8 as *const libc::c_char,
            b" \"abc\" ,' \"def\"\0" as *const u8 as *const libc::c_char,
            s_250,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_251: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"((,\"a\") ,' (,\"b\")) ~ ,\"ab\"\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1270 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"((,\"a\") ,' (,\"b\")) ~ ,\"ab\"\0" as *const u8 as *const libc::c_char,
            s_251,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_252: S = ts(
        tp(
            tc(
                b"(1 2;3 4),''(5 6;7 8)\0" as *const u8 as *const libc::c_char as S,
                b"((1 5;2 6);(3 7;4 8))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1271 as libc::c_int,
            b"(1 2;3 4),''(5 6;7 8)\0" as *const u8 as *const libc::c_char,
            b"((1 5;2 6);(3 7;4 8))\0" as *const u8 as *const libc::c_char,
            s_252,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_253: S = ts(
        tp(
            tc(
                b"5+''(1 2;3 4)\0" as *const u8 as *const libc::c_char as S,
                b"(6 7;8 9)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1272 as libc::c_int,
            b"5+''(1 2;3 4)\0" as *const u8 as *const libc::c_char,
            b"(6 7;8 9)\0" as *const u8 as *const libc::c_char,
            s_253,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_254: S = ts(
        tp(
            tc(
                b"(1 2;3 4)+''5\0" as *const u8 as *const libc::c_char as S,
                b"(6 7;8 9)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1273 as libc::c_int,
            b"(1 2;3 4)+''5\0" as *const u8 as *const libc::c_char,
            b"(6 7;8 9)\0" as *const u8 as *const libc::c_char,
            s_254,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_255: S = ts(
        tp(
            tc(
                b"5,''(1 2;3 4)\0" as *const u8 as *const libc::c_char as S,
                b"((5 1;5 2);(5 3;5 4))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1274 as libc::c_int,
            b"5,''(1 2;3 4)\0" as *const u8 as *const libc::c_char,
            b"((5 1;5 2);(5 3;5 4))\0" as *const u8 as *const libc::c_char,
            s_255,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_256: S = ts(
        tp(
            tc(
                b"(1 2;3 4),''5\0" as *const u8 as *const libc::c_char as S,
                b"((1 5;2 5);(3 5;4 5))\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1275 as libc::c_int,
            b"(1 2;3 4),''5\0" as *const u8 as *const libc::c_char,
            b"((1 5;2 5);(3 5;4 5))\0" as *const u8 as *const libc::c_char,
            s_256,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_257: S = ts(
        tp(
            tc(
                b"1,''2\0" as *const u8 as *const libc::c_char as S,
                b"1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1276 as libc::c_int,
            b"1,''2\0" as *const u8 as *const libc::c_char,
            b"1 2\0" as *const u8 as *const libc::c_char,
            s_257,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_258: S = ts(
        tp(
            tc(
                b"1.1,''2.2\0" as *const u8 as *const libc::c_char as S,
                b"1.1 2.2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1277 as libc::c_int,
            b"1.1,''2.2\0" as *const u8 as *const libc::c_char,
            b"1.1 2.2\0" as *const u8 as *const libc::c_char,
            s_258,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_259: S = ts(
        tp(
            tc(
                b"`a,''`b\0" as *const u8 as *const libc::c_char as S,
                b"`a `b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1278 as libc::c_int,
            b"`a,''`b\0" as *const u8 as *const libc::c_char,
            b"`a `b\0" as *const u8 as *const libc::c_char,
            s_259,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_260: S = ts(
        tp(
            tc(
                b"(2 5 6 7;3 5 6 7;4 5 6 7)\0" as *const u8 as *const libc::c_char as S,
                b"2 3 4 ,\\: 5 6 7\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1280 as libc::c_int,
            b"(2 5 6 7;3 5 6 7;4 5 6 7)\0" as *const u8 as *const libc::c_char,
            b"2 3 4 ,\\: 5 6 7\0" as *const u8 as *const libc::c_char,
            s_260,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_261: S = ts(
        tp(
            tc(
                b"3 5 5 11 11\0" as *const u8 as *const libc::c_char as S,
                b"-': 1 4 9 14 25 36\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1281 as libc::c_int,
            b"3 5 5 11 11\0" as *const u8 as *const libc::c_char,
            b"-': 1 4 9 14 25 36\0" as *const u8 as *const libc::c_char,
            s_261,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_262: S = ts(
        tp(
            tc(
                b"(2 3 4 5;2 3 4 6;2 3 4 7)\0" as *const u8 as *const libc::c_char as S,
                b"2 3 4 ,/: 5 6 7\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1282 as libc::c_int,
            b"(2 3 4 5;2 3 4 6;2 3 4 7)\0" as *const u8 as *const libc::c_char,
            b"2 3 4 ,/: 5 6 7\0" as *const u8 as *const libc::c_char,
            s_262,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_263: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"1 7 2 4 6 10 3 ? 4\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1283 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"1 7 2 4 6 10 3 ? 4\0" as *const u8 as *const libc::c_char,
            s_263,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_264: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"1 7 2 4 6 10 3 ? 4 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1284 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"1 7 2 4 6 10 3 ? 4 3\0" as *const u8 as *const libc::c_char,
            s_264,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_265: S = ts(
        tp(
            tc(
                b"3 6\0" as *const u8 as *const libc::c_char as S,
                b"1 7 2 4 6 10 3 ?/: 4 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1285 as libc::c_int,
            b"3 6\0" as *const u8 as *const libc::c_char,
            b"1 7 2 4 6 10 3 ?/: 4 3\0" as *const u8 as *const libc::c_char,
            s_265,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_266: S = ts(
        tp(
            tc(
                b"16\0" as *const u8 as *const libc::c_char as S,
                b"10+/1 2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1286 as libc::c_int,
            b"16\0" as *const u8 as *const libc::c_char,
            b"10+/1 2 3\0" as *const u8 as *const libc::c_char,
            s_266,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_267: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"+/1 2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1287 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"+/1 2 3\0" as *const u8 as *const libc::c_char,
            s_267,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_268: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"+/1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1288 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"+/1\0" as *const u8 as *const libc::c_char,
            s_268,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_269: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"+/,1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1289 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"+/,1\0" as *const u8 as *const libc::c_char,
            s_269,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_270: S = ts(
        tp(
            tc(
                b"9\0" as *const u8 as *const libc::c_char as S,
                b"|/1 4 -6 9 1 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1290 as libc::c_int,
            b"9\0" as *const u8 as *const libc::c_char,
            b"|/1 4 -6 9 1 3\0" as *const u8 as *const libc::c_char,
            s_270,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_271: S = ts(
        tp(
            tc(
                b"-6\0" as *const u8 as *const libc::c_char as S,
                b"&/ 1 4 -6 9 1 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1291 as libc::c_int,
            b"-6\0" as *const u8 as *const libc::c_char,
            b"&/ 1 4 -6 9 1 3\0" as *const u8 as *const libc::c_char,
            s_271,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_272: S = ts(
        tp(
            tc(
                b"(\"a\";1;2;`bc;\"x\";\"y\";\"z\";2.35)\0" as *const u8
                    as *const libc::c_char as S,
                b",//(\"a\";(1 2;`bc;(\"xyz\";2.35)))\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1292 as libc::c_int,
            b"(\"a\";1;2;`bc;\"x\";\"y\";\"z\";2.35)\0" as *const u8
                as *const libc::c_char,
            b",//(\"a\";(1 2;`bc;(\"xyz\";2.35)))\0" as *const u8 as *const libc::c_char,
            s_272,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_273: S = ts(
        tp(
            tc(
                b"25.0\0" as *const u8 as *const libc::c_char as S,
                b"|/,//(1;(2.3 25.0;(6 7 -9;10)))\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1293 as libc::c_int,
            b"25.0\0" as *const u8 as *const libc::c_char,
            b"|/,//(1;(2.3 25.0;(6 7 -9;10)))\0" as *const u8 as *const libc::c_char,
            s_273,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_274: S = ts(
        tp(
            tc(
                b"-9.0\0" as *const u8 as *const libc::c_char as S,
                b"&/,//(1;(2.3 25;(6 7 -9;10)))\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1294 as libc::c_int,
            b"-9.0\0" as *const u8 as *const libc::c_char,
            b"&/,//(1;(2.3 25;(6 7 -9;10)))\0" as *const u8 as *const libc::c_char,
            s_274,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_275: S = ts(
        tp(
            tc(
                b"1 4 9 16\0" as *const u8 as *const libc::c_char as S,
                b"+\\ 1 3 5 7\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1295 as libc::c_int,
            b"1 4 9 16\0" as *const u8 as *const libc::c_char,
            b"+\\ 1 3 5 7\0" as *const u8 as *const libc::c_char,
            s_275,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_276: S = ts(
        tp(
            tc(
                b"\"aeh\"\0" as *const u8 as *const libc::c_char as S,
                b"m:3 3#\"abcdefghi\"; f:m@'; f 0 1 1\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1296 as libc::c_int,
            b"\"aeh\"\0" as *const u8 as *const libc::c_char,
            b"m:3 3#\"abcdefghi\"; f:m@'; f 0 1 1\0" as *const u8 as *const libc::c_char,
            s_276,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_277: S = ts(
        tp(
            tc(
                b"(\"aeh\";\"aei\")\0" as *const u8 as *const libc::c_char as S,
                b"m:3 3#\"abcdefghi\"; f:m@'; f'(0 1 1;0 1 2)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1297 as libc::c_int,
            b"(\"aeh\";\"aei\")\0" as *const u8 as *const libc::c_char,
            b"m:3 3#\"abcdefghi\"; f:m@'; f'(0 1 1;0 1 2)\0" as *const u8
                as *const libc::c_char,
            s_277,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_278: S = ts(
        tp(
            tc(
                b"6 7 8\0" as *const u8 as *const libc::c_char as S,
                b"a:1 2 3;a+:5; a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1300 as libc::c_int,
            b"6 7 8\0" as *const u8 as *const libc::c_char,
            b"a:1 2 3;a+:5; a\0" as *const u8 as *const libc::c_char,
            s_278,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_279: S = ts(
        tp(
            tc(
                b"(0 2 4;1 3 5)\0" as *const u8 as *const libc::c_char as S,
                b"b: 3 2 # ! 6; r: b +:; b\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1301 as libc::c_int,
            b"(0 2 4;1 3 5)\0" as *const u8 as *const libc::c_char,
            b"b: 3 2 # ! 6; r: b +:; b\0" as *const u8 as *const libc::c_char,
            s_279,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_280: S = ts(
        tp(
            tc(
                b"((101 2 103;103 4 105);(103 105;101 103))\0" as *const u8
                    as *const libc::c_char as S,
                b"a:(1 2 3;3 4 5); r: a[1 0;0 2] +: 100; (a;r)\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1302 as libc::c_int,
            b"((101 2 103;103 4 105);(103 105;101 103))\0" as *const u8
                as *const libc::c_char,
            b"a:(1 2 3;3 4 5); r: a[1 0;0 2] +: 100; (a;r)\0" as *const u8
                as *const libc::c_char,
            s_280,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_281: S = ts(
        tp(
            tc(
                b"(10 11 12 13;(`ab;\"cde\");-8 -9 -10 -11)\0" as *const u8
                    as *const libc::c_char as S,
                b"a:3 4#!12; a[1]:(`ab;\"cde\"); a[0]+:10;a[2] -:;a\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1303 as libc::c_int,
            b"(10 11 12 13;(`ab;\"cde\");-8 -9 -10 -11)\0" as *const u8
                as *const libc::c_char,
            b"a:3 4#!12; a[1]:(`ab;\"cde\"); a[0]+:10;a[2] -:;a\0" as *const u8
                as *const libc::c_char,
            s_281,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_282: S = ts(
        tp(
            tc(
                b"(101 2 103 4;101 103 )\0" as *const u8 as *const libc::c_char as S,
                b"a:1+!4;r: a[0 2] +: 100; (a;r)\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1304 as libc::c_int,
            b"(101 2 103 4;101 103 )\0" as *const u8 as *const libc::c_char,
            b"a:1+!4;r: a[0 2] +: 100; (a;r)\0" as *const u8 as *const libc::c_char,
            s_282,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_283: S = ts(
        tp(
            tc(
                b"(.,(`a;(`b;\"c\")))\0" as *const u8 as *const libc::c_char as S,
                b"d:.,(`a;1 2 3); d[`a]: (`b;\"c\"); d\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1305 as libc::c_int,
            b"(.,(`a;(`b;\"c\")))\0" as *const u8 as *const libc::c_char,
            b"d:.,(`a;1 2 3); d[`a]: (`b;\"c\"); d\0" as *const u8
                as *const libc::c_char,
            s_283,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_284: S = ts(
        tp(
            tc(
                b".((`a;1;)\n  (`b;1;)\n  (`c;1;))\0" as *const u8 as *const libc::c_char
                    as S,
                b"a[`a`b`c]:1;a\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1306 as libc::c_int,
            b".((`a;1;)\n  (`b;1;)\n  (`c;1;))\0" as *const u8 as *const libc::c_char,
            b"a[`a`b`c]:1;a\0" as *const u8 as *const libc::c_char,
            s_284,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_285: S = ts(
        tp(
            tc(
                b"10 20 30\0" as *const u8 as *const libc::c_char as S,
                b"a:3 3#!9; a[;1]:10 20 30\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1307 as libc::c_int,
            b"10 20 30\0" as *const u8 as *const libc::c_char,
            b"a:3 3#!9; a[;1]:10 20 30\0" as *const u8 as *const libc::c_char,
            s_285,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_286: S = ts(
        tp(
            tc(
                b"10 20 30\0" as *const u8 as *const libc::c_char as S,
                b"a:3 3#!9; a[1;]:10 20 30\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1308 as libc::c_int,
            b"10 20 30\0" as *const u8 as *const libc::c_char,
            b"a:3 3#!9; a[1;]:10 20 30\0" as *const u8 as *const libc::c_char,
            s_286,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_287: S = ts(
        tp(
            tc(
                b"3 4\0" as *const u8 as *const libc::c_char as S,
                b"d:.((`a;1);(`b;2)); d[!d]:3 4\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1309 as libc::c_int,
            b"3 4\0" as *const u8 as *const libc::c_char,
            b"d:.((`a;1);(`b;2)); d[!d]:3 4\0" as *const u8 as *const libc::c_char,
            s_287,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_288: S = ts(
        tp(
            tc(
                b"3 4\0" as *const u8 as *const libc::c_char as S,
                b"d:.((`a;1);(`b;2)); .[`d;_n;:;3 4]; d[!d]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1310 as libc::c_int,
            b"3 4\0" as *const u8 as *const libc::c_char,
            b"d:.((`a;1);(`b;2)); .[`d;_n;:;3 4]; d[!d]\0" as *const u8
                as *const libc::c_char,
            s_288,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_289: S = ts(
        tp(
            tc(
                b"3 4\0" as *const u8 as *const libc::c_char as S,
                b"d:.((`a;1);(`b;2)); @[`d;_n;:;3 4]; d[!d]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1311 as libc::c_int,
            b"3 4\0" as *const u8 as *const libc::c_char,
            b"d:.((`a;1);(`b;2)); @[`d;_n;:;3 4]; d[!d]\0" as *const u8
                as *const libc::c_char,
            s_289,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_290: S = ts(
        tp(
            tc(
                b"3 4\0" as *const u8 as *const libc::c_char as S,
                b"d:.((`a;1);(`b;2)); d[]:3 4; d[!d]\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1312 as libc::c_int,
            b"3 4\0" as *const u8 as *const libc::c_char,
            b"d:.((`a;1);(`b;2)); d[]:3 4; d[!d]\0" as *const u8 as *const libc::c_char,
            s_290,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_291: S = ts(
        tp(
            tc(
                b"2#1.0\0" as *const u8 as *const libc::c_char as S,
                b"_abs - _cos 0 0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1315 as libc::c_int,
            b"2#1.0\0" as *const u8 as *const libc::c_char,
            b"_abs - _cos 0 0\0" as *const u8 as *const libc::c_char,
            s_291,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_292: S = ts(
        tp(
            tc(
                b"a.b_c.d:1;a.b_c.d\0" as *const u8 as *const libc::c_char as S,
                b"1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1318 as libc::c_int,
            b"a.b_c.d:1;a.b_c.d\0" as *const u8 as *const libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char,
            s_292,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_293: S = ts(
        tp(
            tc(
                b"#:'(1;,1;1 2;1 2 3)\0" as *const u8 as *const libc::c_char as S,
                b"1 1 2 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1320 as libc::c_int,
            b"#:'(1;,1;1 2;1 2 3)\0" as *const u8 as *const libc::c_char,
            b"1 1 2 3\0" as *const u8 as *const libc::c_char,
            s_293,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_294: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"#'(1;1 2)(#[1;];#[1 2;])\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1321 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"#'(1;1 2)(#[1;];#[1 2;])\0" as *const u8 as *const libc::c_char,
            s_294,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_295: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"4 _in 1 7 2 4 6 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1325 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"4 _in 1 7 2 4 6 3\0" as *const u8 as *const libc::c_char,
            s_295,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_296: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"4 3 _in 1 7 2 4 6 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1326 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"4 3 _in 1 7 2 4 6 3\0" as *const u8 as *const libc::c_char,
            s_296,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_297: S = ts(
        tp(
            tc(
                b"1 1\0" as *const u8 as *const libc::c_char as S,
                b"4 3 _in\\: 1 7 2 4 6 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1327 as libc::c_int,
            b"1 1\0" as *const u8 as *const libc::c_char,
            b"4 3 _in\\: 1 7 2 4 6 3\0" as *const u8 as *const libc::c_char,
            s_297,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_298: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"(\"abcdefg\";\"bdf\"), dir:.((`a;2 3 4);(`b;\"abcdefg\")); (`dir . `b; `dir . (`b;1 3 5))\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1328 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"(\"abcdefg\";\"bdf\"), dir:.((`a;2 3 4);(`b;\"abcdefg\")); (`dir . `b; `dir . (`b;1 3 5))\0"
                as *const u8 as *const libc::c_char,
            s_298,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_299: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"(1 2 3;\"2+3\";`button), c:.((`a;1 2 3);(`b;\"2+3\";.,(`c;`button))); (c.a;c.b;c.b..c)\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1329 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"(1 2 3;\"2+3\";`button), c:.((`a;1 2 3);(`b;\"2+3\";.,(`c;`button))); (c.a;c.b;c.b..c)\0"
                as *const u8 as *const libc::c_char,
            s_299,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_300: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"1 2, d:_n;d. 1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1331 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"1 2, d:_n;d. 1 2\0" as *const u8 as *const libc::c_char,
            s_300,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_301: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"_n, .\"r:2+3*4\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1332 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"_n, .\"r:2+3*4\"\0" as *const u8 as *const libc::c_char,
            s_301,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_302: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{x-2} 5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1334 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{x-2} 5\0" as *const u8 as *const libc::c_char,
            s_302,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_303: S = ts(
        tp(
            tc(
                b"9.0\0" as *const u8 as *const libc::c_char as S,
                b"{x^2} @ 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1335 as libc::c_int,
            b"9.0\0" as *const u8 as *const libc::c_char,
            b"{x^2} @ 3\0" as *const u8 as *const libc::c_char,
            s_303,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_304: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"15, f:{a:10; :x+a; a:20}; f[5]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1336 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"15, f:{a:10; :x+a; a:20}; f[5]\0" as *const u8 as *const libc::c_char,
            s_304,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_305: S = ts(
        tp(
            tc(
                b"12 23\0" as *const u8 as *const libc::c_char as S,
                b"a:2 3;b:10 20; {a+b} . ,_n\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1337 as libc::c_int,
            b"12 23\0" as *const u8 as *const libc::c_char,
            b"a:2 3;b:10 20; {a+b} . ,_n\0" as *const u8 as *const libc::c_char,
            s_305,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_306: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"\"{[x;y] x+y}\", ${[x;y] x+y}\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1338 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"\"{[x;y] x+y}\", ${[x;y] x+y}\0" as *const u8 as *const libc::c_char,
            s_306,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_307: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"{x+y+z}[1;2;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1339 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"{x+y+z}[1;2;3]\0" as *const u8 as *const libc::c_char,
            s_307,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_308: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"g:{x+y+z}[1;;3]; g[2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1340 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"g:{x+y+z}[1;;3]; g[2]\0" as *const u8 as *const libc::c_char,
            s_308,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_309: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"h:{x+y+z}[1]; h[2;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1341 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"h:{x+y+z}[1]; h[2;3]\0" as *const u8 as *const libc::c_char,
            s_309,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_310: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"e:{x+y+z}[;2]; e[1;3]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1342 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"e:{x+y+z}[;2]; e[1;3]\0" as *const u8 as *const libc::c_char,
            s_310,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_311: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"(,5; 12 6 3) , f:{:[x ! 2; x; _ x %% 2]} ; (f\\ 5; f\\ 12)\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1343 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"(,5; 12 6 3) , f:{:[x ! 2; x; _ x %% 2]} ; (f\\ 5; f\\ 12)\0" as *const u8
                as *const libc::c_char,
            s_311,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_312: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"640640 320320 160160 80080, b:{x>100000}; b f\\ 640640\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1344 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"640640 320320 160160 80080, b:{x>100000}; b f\\ 640640\0" as *const u8
                as *const libc::c_char,
            s_312,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_313: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"2, {}$\"1+1\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1345 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"2, {}$\"1+1\"\0" as *const u8 as *const libc::c_char,
            s_313,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_314: S = ts(
        tp(
            tc(
                b"(\"canoe\";`dinghy;\"kayak\";66545;{x+y})\0" as *const u8
                    as *const libc::c_char as S,
                b"(\"canoe\";`dinghy),(\"kayak\";66545;{x+y})\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1346 as libc::c_int,
            b"(\"canoe\";`dinghy;\"kayak\";66545;{x+y})\0" as *const u8
                as *const libc::c_char,
            b"(\"canoe\";`dinghy),(\"kayak\";66545;{x+y})\0" as *const u8
                as *const libc::c_char,
            s_314,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_315: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"1++, 1+a:+\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1347 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"1++, 1+a:+\0" as *const u8 as *const libc::c_char,
            s_315,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_316: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"+\\(!2;!3)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1348 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"+\\(!2;!3)\0" as *const u8 as *const libc::c_char,
            s_316,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_317: S = ts(
        tp(
            tc(
                b"(1;\"index\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"1+/() 1 2 3\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1349 as libc::c_int,
            b"(1;\"index\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"1+/() 1 2 3\";:]\0" as *const u8 as *const libc::c_char,
            s_317,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_318: S = ts(
        tp(
            tc(
                b"640640\0" as *const u8 as *const libc::c_char as S,
                b"f:{:[x!2;x;_ x*0.5]}; (640640<) f/640640\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1351 as libc::c_int,
            b"640640\0" as *const u8 as *const libc::c_char,
            b"f:{:[x!2;x;_ x*0.5]}; (640640<) f/640640\0" as *const u8
                as *const libc::c_char,
            s_318,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_319: S = ts(
        tp(
            tc(
                b"320320\0" as *const u8 as *const libc::c_char as S,
                b"f:{:[x!2;x;_ x*0.5]}; (640639<) f/640640\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1352 as libc::c_int,
            b"320320\0" as *const u8 as *const libc::c_char,
            b"f:{:[x!2;x;_ x*0.5]}; (640639<) f/640640\0" as *const u8
                as *const libc::c_char,
            s_319,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_320: S = ts(
        tp(
            tc(
                b"5005\0" as *const u8 as *const libc::c_char as S,
                b"f:{:[x!2;x;_ x*0.5]}; (10000<) f/640640\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1353 as libc::c_int,
            b"5005\0" as *const u8 as *const libc::c_char,
            b"f:{:[x!2;x;_ x*0.5]}; (10000<) f/640640\0" as *const u8
                as *const libc::c_char,
            s_320,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_321: S = ts(
        tp(
            tc(
                b"5005\0" as *const u8 as *const libc::c_char as S,
                b"f:{:[x!2;x;_ x*0.5]}; {x>10000} f/640640\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1354 as libc::c_int,
            b"5005\0" as *const u8 as *const libc::c_char,
            b"f:{:[x!2;x;_ x*0.5]}; {x>10000} f/640640\0" as *const u8
                as *const libc::c_char,
            s_321,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_322: S = ts(
        tp(
            tc(
                b"(,640640)\0" as *const u8 as *const libc::c_char as S,
                b"f:{:[x!2;x;_ x*0.5]}; (640640<) f\\640640\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1356 as libc::c_int,
            b"(,640640)\0" as *const u8 as *const libc::c_char,
            b"f:{:[x!2;x;_ x*0.5]}; (640640<) f\\640640\0" as *const u8
                as *const libc::c_char,
            s_322,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_323: S = ts(
        tp(
            tc(
                b"640640 320320\0" as *const u8 as *const libc::c_char as S,
                b"f:{:[x!2;x;_ x*0.5]}; (640639<) f\\640640\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1357 as libc::c_int,
            b"640640 320320\0" as *const u8 as *const libc::c_char,
            b"f:{:[x!2;x;_ x*0.5]}; (640639<) f\\640640\0" as *const u8
                as *const libc::c_char,
            s_323,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_324: S = ts(
        tp(
            tc(
                b"640640 320320 160160 80080 40040 20020 10010 5005\0" as *const u8
                    as *const libc::c_char as S,
                b"f:{:[x!2;x;_ x*0.5]}; (10000<) f\\640640\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1358 as libc::c_int,
            b"640640 320320 160160 80080 40040 20020 10010 5005\0" as *const u8
                as *const libc::c_char,
            b"f:{:[x!2;x;_ x*0.5]}; (10000<) f\\640640\0" as *const u8
                as *const libc::c_char,
            s_324,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_325: S = ts(
        tp(
            tc(
                b"640640 320320 160160 80080 40040 20020 10010 5005\0" as *const u8
                    as *const libc::c_char as S,
                b"f:{:[x!2;x;_ x*0.5]}; {x>10000} f\\640640\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1359 as libc::c_int,
            b"640640 320320 160160 80080 40040 20020 10010 5005\0" as *const u8
                as *const libc::c_char,
            b"f:{:[x!2;x;_ x*0.5]}; {x>10000} f\\640640\0" as *const u8
                as *const libc::c_char,
            s_325,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_326: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"a:`a`b`c!3 3#!9;b:.((`a;0 1 2);(`b;3 4 5);(`c;6 7 8));a~b\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1361 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"a:`a`b`c!3 3#!9;b:.((`a;0 1 2);(`b;3 4 5);(`c;6 7 8));a~b\0" as *const u8
                as *const libc::c_char,
            s_326,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_327: S = ts(
        tp(
            tc(
                b"(1;\"type\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"_sin _sin (;)\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1363 as libc::c_int,
            b"(1;\"type\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"_sin _sin (;)\";:]\0" as *const u8 as *const libc::c_char,
            s_327,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_328: S = ts(
        tp(
            tc(
                b"(1;\"type\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"_sin _sin (0;)\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1364 as libc::c_int,
            b"(1;\"type\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"_sin _sin (0;)\";:]\0" as *const u8 as *const libc::c_char,
            s_328,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_329: S = ts(
        tp(
            tc(
                b"(1;\"domain\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\".`\\\"1\\\"\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1366 as libc::c_int,
            b"(1;\"domain\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\".`\\\"1\\\"\";:]\0" as *const u8 as *const libc::c_char,
            s_329,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_330: S = ts(
        tp(
            tc(
                b"(1;\"domain\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\".`\\\" \\\"\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1367 as libc::c_int,
            b"(1;\"domain\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\".`\\\" \\\"\";:]\0" as *const u8 as *const libc::c_char,
            s_330,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_331: S = ts(
        tp(
            tc(
                b"(1;\"domain\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\".`\\\"\xC5\x94\xCC\xA1\xCC\xA2\xCD\x98\xCD\x9Dx\xCC\x802\xCC\x81\xCC\x80H\xCC\xA8\xCC\x80\xCD\x98l\xCD\x98\xCD\x9E\xCD\x9E\xCD\xA0v\xCC\xB6\xCC\xA1\xCD\x98\xCD\xA1k\xCC\xB7\xCC\xA1\xCD\x9E\xCD\x9D \xCC\x9B(\xCC\xA8\xCD\x8F\xCD\x8F\xCD\x9EQ\xCC\xA7\xCD\x8Fy\xCC\xB5\xCC\xB4\xCD\x98\xCD\xA0\xCD\xA1,\xCC\xB6\xCC\xB4\xCC\xB4\xCC\x95\xCD\x9E\\\"\";:]\0"
                    as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1368 as libc::c_int,
            b"(1;\"domain\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\".`\\\"\xC5\x94\xCC\xA1\xCC\xA2\xCD\x98\xCD\x9Dx\xCC\x802\xCC\x81\xCC\x80H\xCC\xA8\xCC\x80\xCD\x98l\xCD\x98\xCD\x9E\xCD\x9E\xCD\xA0v\xCC\xB6\xCC\xA1\xCD\x98\xCD\xA1k\xCC\xB7\xCC\xA1\xCD\x9E\xCD\x9D \xCC\x9B(\xCC\xA8\xCD\x8F\xCD\x8F\xCD\x9EQ\xCC\xA7\xCD\x8Fy\xCC\xB5\xCC\xB4\xCD\x98\xCD\xA0\xCD\xA1,\xCC\xB6\xCC\xB4\xCC\xB4\xCC\x95\xCD\x9E\\\"\";:]\0"
                as *const u8 as *const libc::c_char,
            s_331,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_332: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b".`\"f\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1370 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b".`\"f\"\0" as *const u8 as *const libc::c_char,
            s_332,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_333: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b".`\".f\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1371 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b".`\".f\"\0" as *const u8 as *const libc::c_char,
            s_333,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_334: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b".`\".k.xyz\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1372 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b".`\".k.xyz\"\0" as *const u8 as *const libc::c_char,
            s_334,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_335: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b".`\".k.xyz.\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1373 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b".`\".k.xyz.\"\0" as *const u8 as *const libc::c_char,
            s_335,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_336: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b".`\".k.xyz.b\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1374 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b".`\".k.xyz.b\"\0" as *const u8 as *const libc::c_char,
            s_336,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_337: S = ts(
        tp(
            tc(
                b"_n\0" as *const u8 as *const libc::c_char as S,
                b".`\".k.xyz..b\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1375 as libc::c_int,
            b"_n\0" as *const u8 as *const libc::c_char,
            b".`\".k.xyz..b\"\0" as *const u8 as *const libc::c_char,
            s_337,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_338: S = ts(
        tp(
            tc(
                b"_ssr[1]\0" as *const u8 as *const libc::c_char as S,
                b"_ssr 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1377 as libc::c_int,
            b"_ssr[1]\0" as *const u8 as *const libc::c_char,
            b"_ssr 1\0" as *const u8 as *const libc::c_char,
            s_338,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_339: S = ts(
        tp(
            tc(
                b"_ssr[1;2]\0" as *const u8 as *const libc::c_char as S,
                b"(_ssr 1) 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1378 as libc::c_int,
            b"_ssr[1;2]\0" as *const u8 as *const libc::c_char,
            b"(_ssr 1) 2\0" as *const u8 as *const libc::c_char,
            s_339,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_340: S = ts(
        tp(
            tc(
                b"_ssr[()]\0" as *const u8 as *const libc::c_char as S,
                b"_ssr ()\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1379 as libc::c_int,
            b"_ssr[()]\0" as *const u8 as *const libc::c_char,
            b"_ssr ()\0" as *const u8 as *const libc::c_char,
            s_340,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_341: S = ts(
        tp(
            tc(
                b"_ssr[(\"this\";\"is\";,\"at\")]\0" as *const u8 as *const libc::c_char
                    as S,
                b"_ssr (\"this\";\"is\";,\"at\")\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1380 as libc::c_int,
            b"_ssr[(\"this\";\"is\";,\"at\")]\0" as *const u8 as *const libc::c_char,
            b"_ssr (\"this\";\"is\";,\"at\")\0" as *const u8 as *const libc::c_char,
            s_341,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_342: S = ts(
        tp(
            tc(
                b"_ssr[(\"this\";\"is\";,\"at\")]\0" as *const u8 as *const libc::c_char
                    as S,
                b"_ssr (\"this\";\"is\";\"at\")\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1381 as libc::c_int,
            b"_ssr[(\"this\";\"is\";,\"at\")]\0" as *const u8 as *const libc::c_char,
            b"_ssr (\"this\";\"is\";\"at\")\0" as *const u8 as *const libc::c_char,
            s_342,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_343: S = ts(
        tp(
            tc(
                b"\"that\"\0" as *const u8 as *const libc::c_char as S,
                b"_ssr[\"this\";\"is\";,\"at\"]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1382 as libc::c_int,
            b"\"that\"\0" as *const u8 as *const libc::c_char,
            b"_ssr[\"this\";\"is\";,\"at\"]\0" as *const u8 as *const libc::c_char,
            s_343,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_344: S = ts(
        tp(
            tc(
                b"\"that\"\0" as *const u8 as *const libc::c_char as S,
                b"_ssr[\"this\";\"is\";\"at\"]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1383 as libc::c_int,
            b"\"that\"\0" as *const u8 as *const libc::c_char,
            b"_ssr[\"this\";\"is\";\"at\"]\0" as *const u8 as *const libc::c_char,
            s_344,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_345: S = ts(
        tp(
            tc(
                b"\"asdf\"\0" as *const u8 as *const libc::c_char as S,
                b"_ssr[\"gsdf\";\"g\";,\"a\"]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1384 as libc::c_int,
            b"\"asdf\"\0" as *const u8 as *const libc::c_char,
            b"_ssr[\"gsdf\";\"g\";,\"a\"]\0" as *const u8 as *const libc::c_char,
            s_345,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_346: S = ts(
        tp(
            tc(
                b"\"asdf\"\0" as *const u8 as *const libc::c_char as S,
                b"_ssr[\"gsdf\";\"g\";\"a\"]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1385 as libc::c_int,
            b"\"asdf\"\0" as *const u8 as *const libc::c_char,
            b"_ssr[\"gsdf\";\"g\";\"a\"]\0" as *const u8 as *const libc::c_char,
            s_346,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_347: S = ts(
        tp(
            tc(
                b"\"ebcde\"\0" as *const u8 as *const libc::c_char as S,
                b"_ssr[\"abcda\";\"a\";\"e\"]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1386 as libc::c_int,
            b"\"ebcde\"\0" as *const u8 as *const libc::c_char,
            b"_ssr[\"abcda\";\"a\";\"e\"]\0" as *const u8 as *const libc::c_char,
            s_347,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_348: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"_2.5\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1389 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"_2.5\0" as *const u8 as *const libc::c_char,
            s_348,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_349: S = ts(
        tp(
            tc(
                b":\0" as *const u8 as *const libc::c_char as S,
                b":\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1392 as libc::c_int,
            b":\0" as *const u8 as *const libc::c_char,
            b":\0" as *const u8 as *const libc::c_char,
            s_349,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_350: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b":1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1393 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b":1\0" as *const u8 as *const libc::c_char,
            s_350,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_351: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b":1;2;3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1394 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b":1;2;3\0" as *const u8 as *const libc::c_char,
            s_351,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_352: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"1;:2;3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1395 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"1;:2;3\0" as *const u8 as *const libc::c_char,
            s_352,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_353: S = ts(
        tp(
            tc(
                b"1 2 3\0" as *const u8 as *const libc::c_char as S,
                b"(1;2;3)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1396 as libc::c_int,
            b"1 2 3\0" as *const u8 as *const libc::c_char,
            b"(1;2;3)\0" as *const u8 as *const libc::c_char,
            s_353,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_354: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"0 5 9[:2]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1397 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"0 5 9[:2]\0" as *const u8 as *const libc::c_char,
            s_354,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_355: S = ts(
        tp(
            tc(
                b"10\0" as *const u8 as *const libc::c_char as S,
                b"0 5 9[:10]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1398 as libc::c_int,
            b"10\0" as *const u8 as *const libc::c_char,
            b"0 5 9[:10]\0" as *const u8 as *const libc::c_char,
            s_355,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_356: S = ts(
        tp(
            tc(
                b"20\0" as *const u8 as *const libc::c_char as S,
                b"0 5 9[:10;:20]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1399 as libc::c_int,
            b"20\0" as *const u8 as *const libc::c_char,
            b"0 5 9[:10;:20]\0" as *const u8 as *const libc::c_char,
            s_356,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_357: S = ts(
        tp(
            tc(
                b"20\0" as *const u8 as *const libc::c_char as S,
                b"0 5 9[(:10;:20)]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1400 as libc::c_int,
            b"20\0" as *const u8 as *const libc::c_char,
            b"0 5 9[(:10;:20)]\0" as *const u8 as *const libc::c_char,
            s_357,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_358: S = ts(
        tp(
            tc(
                b"40\0" as *const u8 as *const libc::c_char as S,
                b"0 5 9[(:10;:20);(:30;:40)]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1401 as libc::c_int,
            b"40\0" as *const u8 as *const libc::c_char,
            b"0 5 9[(:10;:20);(:30;:40)]\0" as *const u8 as *const libc::c_char,
            s_358,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_359: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"(;1;:2;:3)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1402 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"(;1;:2;:3)\0" as *const u8 as *const libc::c_char,
            s_359,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_360: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b":[(:1;:2;:3);:4]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1403 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b":[(:1;:2;:3);:4]\0" as *const u8 as *const libc::c_char,
            s_360,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_361: S = ts(
        tp(
            tc(
                b"4 5 6 7\0" as *const u8 as *const libc::c_char as S,
                b":4,5,6,7\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1404 as libc::c_int,
            b"4 5 6 7\0" as *const u8 as *const libc::c_char,
            b":4,5,6,7\0" as *const u8 as *const libc::c_char,
            s_361,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_362: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"(:1\n:2\n:3\n:4)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1405 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"(:1\n:2\n:3\n:4)\0" as *const u8 as *const libc::c_char,
            s_362,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_363: S = ts(
        tp(
            tc(
                b"1\0" as *const u8 as *const libc::c_char as S,
                b"{:1\n:2\n:[:4;:5]\n:6}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1406 as libc::c_int,
            b"1\0" as *const u8 as *const libc::c_char,
            b"{:1\n:2\n:[:4;:5]\n:6}0\0" as *const u8 as *const libc::c_char,
            s_363,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_364: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"{:[:4;:5]}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1407 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"{:[:4;:5]}0\0" as *const u8 as *const libc::c_char,
            s_364,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_365: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"while[:5;:6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1408 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"while[:5;:6]\0" as *const u8 as *const libc::c_char,
            s_365,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_366: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"while[5;:6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1409 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"while[5;:6]\0" as *const u8 as *const libc::c_char,
            s_366,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_367: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"do[5;:6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1410 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"do[5;:6]\0" as *const u8 as *const libc::c_char,
            s_367,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_368: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"do[:5;:6]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1411 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"do[:5;:6]\0" as *const u8 as *const libc::c_char,
            s_368,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_369: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"if[5;:6;:7;:8]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1412 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"if[5;:6;:7;:8]\0" as *const u8 as *const libc::c_char,
            s_369,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_370: S = ts(
        tp(
            tc(
                b"6\0" as *const u8 as *const libc::c_char as S,
                b"if[5;(:3;:2;:6);:7;:8]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1413 as libc::c_int,
            b"6\0" as *const u8 as *const libc::c_char,
            b"if[5;(:3;:2;:6);:7;:8]\0" as *const u8 as *const libc::c_char,
            s_370,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_371: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"while[1;if[5;do[6;:4]]]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1414 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"while[1;if[5;do[6;:4]]]\0" as *const u8 as *const libc::c_char,
            s_371,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_372: S = ts(
        tp(
            tc(
                b"{:1}\0" as *const u8 as *const libc::c_char as S,
                b":[5;{:1}]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1415 as libc::c_int,
            b"{:1}\0" as *const u8 as *const libc::c_char,
            b":[5;{:1}]\0" as *const u8 as *const libc::c_char,
            s_372,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_373: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b":[5;:[6;5;{1+4;(:1;:2;:3)}0]]\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1416 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b":[5;:[6;5;{1+4;(:1;:2;:3)}0]]\0" as *const u8 as *const libc::c_char,
            s_373,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_374: S = ts(
        tp(
            tc(
                b"3\0" as *const u8 as *const libc::c_char as S,
                b"{1+{1;:2;1+x}0}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1417 as libc::c_int,
            b"3\0" as *const u8 as *const libc::c_char,
            b"{1+{1;:2;1+x}0}0\0" as *const u8 as *const libc::c_char,
            s_374,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_375: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"{1+{1;:2;1+x}0;7;0}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1418 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"{1+{1;:2;1+x}0;7;0}0\0" as *const u8 as *const libc::c_char,
            s_375,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_376: S = ts(
        tp(
            tc(
                b"7\0" as *const u8 as *const libc::c_char as S,
                b"{1+{1;:2;1+x}0;:7;0}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1419 as libc::c_int,
            b"7\0" as *const u8 as *const libc::c_char,
            b"{1+{1;:2;1+x}0;:7;0}0\0" as *const u8 as *const libc::c_char,
            s_376,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_377: S = ts(
        tp(
            tc(
                b"32.0\0" as *const u8 as *const libc::c_char as S,
                b"{1;:4*2^3;1}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1420 as libc::c_int,
            b"32.0\0" as *const u8 as *const libc::c_char,
            b"{1;:4*2^3;1}0\0" as *const u8 as *const libc::c_char,
            s_377,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_378: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"a:1;b:2;c:3;a;:b;c\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1421 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"a:1;b:2;c:3;a;:b;c\0" as *const u8 as *const libc::c_char,
            s_378,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_379: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"{[d]d:4;e:5;f:6;a:1;b:2;c:3;a;:b;c}0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1422 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"{[d]d:4;e:5;f:6;a:1;b:2;c:3;a;:b;c}0\0" as *const u8
                as *const libc::c_char,
            s_379,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_380: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"{[d]d:4;e:5;f:6;a:1;b:2;c:3;a;:d;c}0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1423 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"{[d]d:4;e:5;f:6;a:1;b:2;c:3;a;:d;c}0\0" as *const u8
                as *const libc::c_char,
            s_380,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_381: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"{x;4;e:5;f:6;a:1;b:2;c:3;a;:b;c}0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1424 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"{x;4;e:5;f:6;a:1;b:2;c:3;a;:b;c}0\0" as *const u8 as *const libc::c_char,
            s_381,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_382: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"{x:4;e:5;f:6;a:1;b:2;c:3;a;:x;c}0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1425 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"{x:4;e:5;f:6;a:1;b:2;c:3;a;:x;c}0\0" as *const u8 as *const libc::c_char,
            s_382,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_383: S = ts(
        tp(
            tc(
                b"4\0" as *const u8 as *const libc::c_char as S,
                b"f:{x:4;e:5;f:6;a:1;b:2;c:3;a;:x;c};f 0;f 0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1426 as libc::c_int,
            b"4\0" as *const u8 as *const libc::c_char,
            b"f:{x:4;e:5;f:6;a:1;b:2;c:3;a;:x;c};f 0;f 0\0" as *const u8
                as *const libc::c_char,
            s_383,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_384: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"f:{x;4;e;5;f:t;a:1;b:2;c:3;a;:b;c};f 0;f 0\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1427 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"f:{x;4;e;5;f:t;a:1;b:2;c:3;a;:b;c};f 0;f 0\0" as *const u8
                as *const libc::c_char,
            s_384,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_385: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"{b::4; b:5; b}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1428 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"{b::4; b:5; b}0\0" as *const u8 as *const libc::c_char,
            s_385,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_386: S = ts(
        tp(
            tc(
                b"5\0" as *const u8 as *const libc::c_char as S,
                b"{b:4; b::5; b}0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1429 as libc::c_int,
            b"5\0" as *const u8 as *const libc::c_char,
            b"{b:4; b::5; b}0\0" as *const u8 as *const libc::c_char,
            s_386,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_387: S = ts(
        tp(
            tc(
                b"2\0" as *const u8 as *const libc::c_char as S,
                b"a:1; :b:2; c:3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1430 as libc::c_int,
            b"2\0" as *const u8 as *const libc::c_char,
            b"a:1; :b:2; c:3\0" as *const u8 as *const libc::c_char,
            s_387,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_388: S = ts(
        tp(
            tc(
                b"\"abcd\"\0" as *const u8 as *const libc::c_char as S,
                b"(\"abcd\";\"efgh\")/0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1433 as libc::c_int,
            b"\"abcd\"\0" as *const u8 as *const libc::c_char,
            b"(\"abcd\";\"efgh\")/0\0" as *const u8 as *const libc::c_char,
            s_388,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_389: S = ts(
        tp(
            tc(
                b"\"efgh\"\0" as *const u8 as *const libc::c_char as S,
                b"(\"abcd\";\"efgh\")/1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1434 as libc::c_int,
            b"\"efgh\"\0" as *const u8 as *const libc::c_char,
            b"(\"abcd\";\"efgh\")/1\0" as *const u8 as *const libc::c_char,
            s_389,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_390: S = ts(
        tp(
            tc(
                b"\"b\"\0" as *const u8 as *const libc::c_char as S,
                b"(\"abcd\";\"efgh\")/0 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1435 as libc::c_int,
            b"\"b\"\0" as *const u8 as *const libc::c_char,
            b"(\"abcd\";\"efgh\")/0 1\0" as *const u8 as *const libc::c_char,
            s_390,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_391: S = ts(
        tp(
            tc(
                b"\"g\"\0" as *const u8 as *const libc::c_char as S,
                b"(\"abcd\";\"efgh\")/1 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1436 as libc::c_int,
            b"\"g\"\0" as *const u8 as *const libc::c_char,
            b"(\"abcd\";\"efgh\")/1 2\0" as *const u8 as *const libc::c_char,
            s_391,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_392: S = ts(
        tp(
            tc(
                b"0\0" as *const u8 as *const libc::c_char as S,
                b"(%[;2])/9999\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1437 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char,
            b"(%[;2])/9999\0" as *const u8 as *const libc::c_char,
            s_392,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_393: S = ts(
        tp(
            tc(
                b"0.0\0" as *const u8 as *const libc::c_char as S,
                b"(%[;2.0])/9999.0\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1438 as libc::c_int,
            b"0.0\0" as *const u8 as *const libc::c_char,
            b"(%[;2.0])/9999.0\0" as *const u8 as *const libc::c_char,
            s_393,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_394: S = ts(
        tp(
            tc(
                b"\"AAA\"\0" as *const u8 as *const libc::c_char as S,
                b"{:[3=#x;x;(1+#x)#x]}/\"A\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1439 as libc::c_int,
            b"\"AAA\"\0" as *const u8 as *const libc::c_char,
            b"{:[3=#x;x;(1+#x)#x]}/\"A\"\0" as *const u8 as *const libc::c_char,
            s_394,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_395: S = ts(
        tp(
            tc(
                b"-:/1\0" as *const u8 as *const libc::c_char as S,
                b"-1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1440 as libc::c_int,
            b"-:/1\0" as *const u8 as *const libc::c_char,
            b"-1\0" as *const u8 as *const libc::c_char,
            s_395,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_396: S = ts(
        tp(
            tc(
                b"f:{(x+1)!3}; f/2\0" as *const u8 as *const libc::c_char as S,
                b"1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1441 as libc::c_int,
            b"f:{(x+1)!3}; f/2\0" as *const u8 as *const libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char,
            s_396,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_397: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"500, {:[x>0;1+_f[x-1];0]}500\0" as *const u8 as *const libc::c_char
                    as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1444 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"500, {:[x>0;1+_f[x-1];0]}500\0" as *const u8 as *const libc::c_char,
            s_397,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_398: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"stack error, {:[x>0;1+_f[x-1];0]}500\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1445 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"stack error, {:[x>0;1+_f[x-1];0]}500\0" as *const u8
                as *const libc::c_char,
            s_398,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_399: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"500, rcr:{:[x>0;1+_f[x-1];0]}; rcr 1000\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1446 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"500, rcr:{:[x>0;1+_f[x-1];0]}; rcr 1000\0" as *const u8
                as *const libc::c_char,
            s_399,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_400: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"stack error, rcr:{:[x>0;1+_f[x-1];0]}; rcr 1001\0" as *const u8
                    as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1447 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"stack error, rcr:{:[x>0;1+_f[x-1];0]}; rcr 1001\0" as *const u8
                as *const libc::c_char,
            s_400,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_401: S = ts(
        tp(
            tc(
                b"skip\0" as *const u8 as *const libc::c_char as S,
                b"stack error, . t : \". t\"\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1448 as libc::c_int,
            b"skip\0" as *const u8 as *const libc::c_char,
            b"stack error, . t : \". t\"\0" as *const u8 as *const libc::c_char,
            s_401,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_402: S = ts(
        tp(
            tc(
                b"(1;\"syntax\")\0" as *const u8 as *const libc::c_char as S,
                b"@[.:;\"0-8^-4&1/::=-\";:]\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1450 as libc::c_int,
            b"(1;\"syntax\")\0" as *const u8 as *const libc::c_char,
            b"@[.:;\"0-8^-4&1/::=-\";:]\0" as *const u8 as *const libc::c_char,
            s_402,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_403: S = ts(
        tp(
            tc(
                b"(0 0;0 1;1 0;1 1)\0" as *const u8 as *const libc::c_char as S,
                b"!2 2\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1452 as libc::c_int,
            b"(0 0;0 1;1 0;1 1)\0" as *const u8 as *const libc::c_char,
            b"!2 2\0" as *const u8 as *const libc::c_char,
            s_403,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_404: S = ts(
        tp(
            tc(
                b"a:2 1 3; a _vs/:!*/a\0" as *const u8 as *const libc::c_char as S,
                b"!2 1 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1453 as libc::c_int,
            b"a:2 1 3; a _vs/:!*/a\0" as *const u8 as *const libc::c_char,
            b"!2 1 3\0" as *const u8 as *const libc::c_char,
            s_404,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_405: S = ts(
        tp(
            tc(
                b"a:2 1 3; a _vsx/:!*/a\0" as *const u8 as *const libc::c_char as S,
                b"!2 1 3\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1454 as libc::c_int,
            b"a:2 1 3; a _vsx/:!*/a\0" as *const u8 as *const libc::c_char,
            b"!2 1 3\0" as *const u8 as *const libc::c_char,
            s_405,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_406: S = ts(
        tp(
            tc(
                b"3 2\0" as *const u8 as *const libc::c_char as S,
                b"1 2 3 4 (2 1)\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1455 as libc::c_int,
            b"3 2\0" as *const u8 as *const libc::c_char,
            b"1 2 3 4 (2 1)\0" as *const u8 as *const libc::c_char,
            s_406,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_407: S = ts(
        tp(
            tc(
                b"2 1\0" as *const u8 as *const libc::c_char as S,
                b"x:!10; x 2 1\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1456 as libc::c_int,
            b"2 1\0" as *const u8 as *const libc::c_char,
            b"x:!10; x 2 1\0" as *const u8 as *const libc::c_char,
            s_407,
        );
    }
    test_print = 0 as libc::c_int as I;
    let mut s_408: S = ts(
        tp(
            tc(
                b"3 2\0" as *const u8 as *const libc::c_char as S,
                b"y:2 1; 1 2 3 4 y\0" as *const u8 as *const libc::c_char as S,
            ),
        ),
    );
    if test_print != 0 {
        fprintf(
            stderr,
            b"%s:%u: TC( %s , %s ) ... %s\n\0" as *const u8 as *const libc::c_char,
            b"src/tests.c\0" as *const u8 as *const libc::c_char,
            1457 as libc::c_int,
            b"3 2\0" as *const u8 as *const libc::c_char,
            b"y:2 1; 1 2 3 4 y\0" as *const u8 as *const libc::c_char,
            s_408,
        );
    }
    test_print = 0 as libc::c_int as I;
    return 0 as libc::c_int as I;
}

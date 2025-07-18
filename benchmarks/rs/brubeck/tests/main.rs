use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    fn time(__timer: *mut time_t) -> time_t;
    fn test_histogram__sampling();
    fn test_histogram__single_element();
    fn test_histogram__large_range();
    fn test_histogram__multisamples();
    fn test_histogram__with_sample_rate();
    fn test_histogram__capacity();
    fn test_mstore__save();
    fn test_atomic_spinlocks();
    fn test_ftoa();
    fn test_statsd_msg__parse_strings();
    fn test_tag_parsing();
    fn test_tag_storage();
    fn test_tag_offset();
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput {
    pub out: *mut FILE,
    pub initialized: libc::c_char,
    pub overall: sput_overall,
    pub suite: sput_suite,
    pub test: sput_test,
    pub check: sput_check,
    pub time: sput_time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput_time {
    pub start: time_t,
    pub end: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput_check {
    pub name: *const libc::c_char,
    pub cond: *const libc::c_char,
    pub type_0: *const libc::c_char,
    pub line: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput_test {
    pub name: *const libc::c_char,
    pub nr: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput_suite {
    pub name: *const libc::c_char,
    pub nr: libc::c_ulong,
    pub checks: libc::c_ulong,
    pub ok: libc::c_ulong,
    pub nok: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sput_overall {
    pub checks: libc::c_ulong,
    pub suites: libc::c_ulong,
    pub ok: libc::c_ulong,
    pub nok: libc::c_ulong,
}
pub static mut __sput: sput = sput {
    out: 0 as *const FILE as *mut FILE,
    initialized: 0,
    overall: sput_overall {
        checks: 0,
        suites: 0,
        ok: 0,
        nok: 0,
    },
    suite: sput_suite {
        name: 0 as *const libc::c_char,
        nr: 0,
        checks: 0,
        ok: 0,
        nok: 0,
    },
    test: sput_test {
        name: 0 as *const libc::c_char,
        nr: 0,
    },
    check: sput_check {
        name: 0 as *const libc::c_char,
        cond: 0 as *const libc::c_char,
        type_0: 0 as *const libc::c_char,
        line: 0,
    },
    time: sput_time { start: 0, end: 0 },
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    memset(
        &mut __sput as *mut sput as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput>() as libc::c_ulong,
    );
    __sput.out = stdout;
    __sput.time.start = time(0 as *mut time_t);
    __sput.initialized = 0x6 as libc::c_int as libc::c_char;
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if !(__sput.suite.name).is_null() {
        let mut failp: libc::c_float = 0.0f32;
        if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
            fputs(
                b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
                stderr,
            );
            exit(1 as libc::c_int);
        }
        if (__sput.suite.name).is_null() {
            fputs(
                b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        failp = if __sput.suite.checks != 0 {
            (__sput.suite.nok as libc::c_double * 100.0f64
                / __sput.suite.checks as libc::c_double) as libc::c_float
        } else {
            0.0f32
        };
        fprintf(
            __sput.out,
            b"\n--> %lu check(s), %lu ok, %lu failed (%.2f%%)\n\0" as *const u8
                as *const libc::c_char,
            __sput.suite.checks,
            __sput.suite.ok,
            __sput.suite.nok,
            failp as libc::c_double,
        );
        __sput
            .overall
            .checks = (__sput.overall.checks).wrapping_add(__sput.suite.checks);
        __sput.overall.ok = (__sput.overall.ok).wrapping_add(__sput.suite.ok);
        __sput.overall.nok = (__sput.overall.nok).wrapping_add(__sput.suite.nok);
        memset(
            &mut __sput.suite as *mut sput_suite as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sput_suite>() as libc::c_ulong,
        );
    }
    __sput
        .suite
        .name = if !(b"histogram: time/data series aggregation\0" as *const u8
        as *const libc::c_char)
        .is_null()
    {
        b"histogram: time/data series aggregation\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Suite\0" as *const u8 as *const libc::c_char
    };
    __sput.overall.suites = (__sput.overall.suites).wrapping_add(1);
    __sput.suite.nr = __sput.overall.suites;
    fprintf(
        __sput.out,
        b"\n== Entering suite #%lu, \"%s\" ==\n\n\0" as *const u8 as *const libc::c_char,
        __sput.suite.nr,
        __sput.suite.name,
    );
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput.test.name = b"test_histogram__sampling\0" as *const u8 as *const libc::c_char;
    test_histogram__sampling();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput
        .test
        .name = b"test_histogram__single_element\0" as *const u8 as *const libc::c_char;
    test_histogram__single_element();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput
        .test
        .name = b"test_histogram__large_range\0" as *const u8 as *const libc::c_char;
    test_histogram__large_range();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput
        .test
        .name = b"test_histogram__multisamples\0" as *const u8 as *const libc::c_char;
    test_histogram__multisamples();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput
        .test
        .name = b"test_histogram__with_sample_rate\0" as *const u8
        as *const libc::c_char;
    test_histogram__with_sample_rate();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput.test.name = b"test_histogram__capacity\0" as *const u8 as *const libc::c_char;
    test_histogram__capacity();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if !(__sput.suite.name).is_null() {
        let mut failp_0: libc::c_float = 0.0f32;
        if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
            fputs(
                b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
                stderr,
            );
            exit(1 as libc::c_int);
        }
        if (__sput.suite.name).is_null() {
            fputs(
                b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        failp_0 = if __sput.suite.checks != 0 {
            (__sput.suite.nok as libc::c_double * 100.0f64
                / __sput.suite.checks as libc::c_double) as libc::c_float
        } else {
            0.0f32
        };
        fprintf(
            __sput.out,
            b"\n--> %lu check(s), %lu ok, %lu failed (%.2f%%)\n\0" as *const u8
                as *const libc::c_char,
            __sput.suite.checks,
            __sput.suite.ok,
            __sput.suite.nok,
            failp_0 as libc::c_double,
        );
        __sput
            .overall
            .checks = (__sput.overall.checks).wrapping_add(__sput.suite.checks);
        __sput.overall.ok = (__sput.overall.ok).wrapping_add(__sput.suite.ok);
        __sput.overall.nok = (__sput.overall.nok).wrapping_add(__sput.suite.nok);
        memset(
            &mut __sput.suite as *mut sput_suite as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sput_suite>() as libc::c_ulong,
        );
    }
    __sput
        .suite
        .name = if !(b"mstore: concurrency test for metrics hash table\0" as *const u8
        as *const libc::c_char)
        .is_null()
    {
        b"mstore: concurrency test for metrics hash table\0" as *const u8
            as *const libc::c_char
    } else {
        b"Unlabeled Suite\0" as *const u8 as *const libc::c_char
    };
    __sput.overall.suites = (__sput.overall.suites).wrapping_add(1);
    __sput.suite.nr = __sput.overall.suites;
    fprintf(
        __sput.out,
        b"\n== Entering suite #%lu, \"%s\" ==\n\n\0" as *const u8 as *const libc::c_char,
        __sput.suite.nr,
        __sput.suite.name,
    );
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput.test.name = b"test_mstore__save\0" as *const u8 as *const libc::c_char;
    test_mstore__save();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if !(__sput.suite.name).is_null() {
        let mut failp_1: libc::c_float = 0.0f32;
        if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
            fputs(
                b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
                stderr,
            );
            exit(1 as libc::c_int);
        }
        if (__sput.suite.name).is_null() {
            fputs(
                b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        failp_1 = if __sput.suite.checks != 0 {
            (__sput.suite.nok as libc::c_double * 100.0f64
                / __sput.suite.checks as libc::c_double) as libc::c_float
        } else {
            0.0f32
        };
        fprintf(
            __sput.out,
            b"\n--> %lu check(s), %lu ok, %lu failed (%.2f%%)\n\0" as *const u8
                as *const libc::c_char,
            __sput.suite.checks,
            __sput.suite.ok,
            __sput.suite.nok,
            failp_1 as libc::c_double,
        );
        __sput
            .overall
            .checks = (__sput.overall.checks).wrapping_add(__sput.suite.checks);
        __sput.overall.ok = (__sput.overall.ok).wrapping_add(__sput.suite.ok);
        __sput.overall.nok = (__sput.overall.nok).wrapping_add(__sput.suite.nok);
        memset(
            &mut __sput.suite as *mut sput_suite as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sput_suite>() as libc::c_ulong,
        );
    }
    __sput
        .suite
        .name = if !(b"atomic: atomic primitives\0" as *const u8 as *const libc::c_char)
        .is_null()
    {
        b"atomic: atomic primitives\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Suite\0" as *const u8 as *const libc::c_char
    };
    __sput.overall.suites = (__sput.overall.suites).wrapping_add(1);
    __sput.suite.nr = __sput.overall.suites;
    fprintf(
        __sput.out,
        b"\n== Entering suite #%lu, \"%s\" ==\n\n\0" as *const u8 as *const libc::c_char,
        __sput.suite.nr,
        __sput.suite.name,
    );
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput.test.name = b"test_atomic_spinlocks\0" as *const u8 as *const libc::c_char;
    test_atomic_spinlocks();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if !(__sput.suite.name).is_null() {
        let mut failp_2: libc::c_float = 0.0f32;
        if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
            fputs(
                b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
                stderr,
            );
            exit(1 as libc::c_int);
        }
        if (__sput.suite.name).is_null() {
            fputs(
                b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        failp_2 = if __sput.suite.checks != 0 {
            (__sput.suite.nok as libc::c_double * 100.0f64
                / __sput.suite.checks as libc::c_double) as libc::c_float
        } else {
            0.0f32
        };
        fprintf(
            __sput.out,
            b"\n--> %lu check(s), %lu ok, %lu failed (%.2f%%)\n\0" as *const u8
                as *const libc::c_char,
            __sput.suite.checks,
            __sput.suite.ok,
            __sput.suite.nok,
            failp_2 as libc::c_double,
        );
        __sput
            .overall
            .checks = (__sput.overall.checks).wrapping_add(__sput.suite.checks);
        __sput.overall.ok = (__sput.overall.ok).wrapping_add(__sput.suite.ok);
        __sput.overall.nok = (__sput.overall.nok).wrapping_add(__sput.suite.nok);
        memset(
            &mut __sput.suite as *mut sput_suite as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sput_suite>() as libc::c_ulong,
        );
    }
    __sput
        .suite
        .name = if !(b"ftoa: double-to-string conversion\0" as *const u8
        as *const libc::c_char)
        .is_null()
    {
        b"ftoa: double-to-string conversion\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Suite\0" as *const u8 as *const libc::c_char
    };
    __sput.overall.suites = (__sput.overall.suites).wrapping_add(1);
    __sput.suite.nr = __sput.overall.suites;
    fprintf(
        __sput.out,
        b"\n== Entering suite #%lu, \"%s\" ==\n\n\0" as *const u8 as *const libc::c_char,
        __sput.suite.nr,
        __sput.suite.name,
    );
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput.test.name = b"test_ftoa\0" as *const u8 as *const libc::c_char;
    test_ftoa();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if !(__sput.suite.name).is_null() {
        let mut failp_3: libc::c_float = 0.0f32;
        if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
            fputs(
                b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
                stderr,
            );
            exit(1 as libc::c_int);
        }
        if (__sput.suite.name).is_null() {
            fputs(
                b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        failp_3 = if __sput.suite.checks != 0 {
            (__sput.suite.nok as libc::c_double * 100.0f64
                / __sput.suite.checks as libc::c_double) as libc::c_float
        } else {
            0.0f32
        };
        fprintf(
            __sput.out,
            b"\n--> %lu check(s), %lu ok, %lu failed (%.2f%%)\n\0" as *const u8
                as *const libc::c_char,
            __sput.suite.checks,
            __sput.suite.ok,
            __sput.suite.nok,
            failp_3 as libc::c_double,
        );
        __sput
            .overall
            .checks = (__sput.overall.checks).wrapping_add(__sput.suite.checks);
        __sput.overall.ok = (__sput.overall.ok).wrapping_add(__sput.suite.ok);
        __sput.overall.nok = (__sput.overall.nok).wrapping_add(__sput.suite.nok);
        memset(
            &mut __sput.suite as *mut sput_suite as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sput_suite>() as libc::c_ulong,
        );
    }
    __sput
        .suite
        .name = if !(b"statsd: packet parsing\0" as *const u8 as *const libc::c_char)
        .is_null()
    {
        b"statsd: packet parsing\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Suite\0" as *const u8 as *const libc::c_char
    };
    __sput.overall.suites = (__sput.overall.suites).wrapping_add(1);
    __sput.suite.nr = __sput.overall.suites;
    fprintf(
        __sput.out,
        b"\n== Entering suite #%lu, \"%s\" ==\n\n\0" as *const u8 as *const libc::c_char,
        __sput.suite.nr,
        __sput.suite.name,
    );
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput
        .test
        .name = b"test_statsd_msg__parse_strings\0" as *const u8 as *const libc::c_char;
    test_statsd_msg__parse_strings();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if !(__sput.suite.name).is_null() {
        let mut failp_4: libc::c_float = 0.0f32;
        if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
            fputs(
                b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
                stderr,
            );
            exit(1 as libc::c_int);
        }
        if (__sput.suite.name).is_null() {
            fputs(
                b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        failp_4 = if __sput.suite.checks != 0 {
            (__sput.suite.nok as libc::c_double * 100.0f64
                / __sput.suite.checks as libc::c_double) as libc::c_float
        } else {
            0.0f32
        };
        fprintf(
            __sput.out,
            b"\n--> %lu check(s), %lu ok, %lu failed (%.2f%%)\n\0" as *const u8
                as *const libc::c_char,
            __sput.suite.checks,
            __sput.suite.ok,
            __sput.suite.nok,
            failp_4 as libc::c_double,
        );
        __sput
            .overall
            .checks = (__sput.overall.checks).wrapping_add(__sput.suite.checks);
        __sput.overall.ok = (__sput.overall.ok).wrapping_add(__sput.suite.ok);
        __sput.overall.nok = (__sput.overall.nok).wrapping_add(__sput.suite.nok);
        memset(
            &mut __sput.suite as *mut sput_suite as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sput_suite>() as libc::c_ulong,
        );
    }
    __sput
        .suite
        .name = if !(b"tags: associative key value parsing\0" as *const u8
        as *const libc::c_char)
        .is_null()
    {
        b"tags: associative key value parsing\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Suite\0" as *const u8 as *const libc::c_char
    };
    __sput.overall.suites = (__sput.overall.suites).wrapping_add(1);
    __sput.suite.nr = __sput.overall.suites;
    fprintf(
        __sput.out,
        b"\n== Entering suite #%lu, \"%s\" ==\n\n\0" as *const u8 as *const libc::c_char,
        __sput.suite.nr,
        __sput.suite.name,
    );
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput.test.name = b"test_tag_parsing\0" as *const u8 as *const libc::c_char;
    test_tag_parsing();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput.test.name = b"test_tag_storage\0" as *const u8 as *const libc::c_char;
    test_tag_storage();
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if (__sput.suite.name).is_null() {
        fputs(
            b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    memset(
        &mut __sput.test as *mut sput_test as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sput_test>() as libc::c_ulong,
    );
    __sput.test.name = b"test_tag_offset\0" as *const u8 as *const libc::c_char;
    test_tag_offset();
    let mut failp_5: libc::c_float = 0.0f32;
    if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
        fputs(
            b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
            stderr,
        );
        exit(1 as libc::c_int);
    }
    if !(__sput.suite.name).is_null() {
        let mut failp_6: libc::c_float = 0.0f32;
        if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
            fputs(
                b"sput_start_testing() omitted\n\0" as *const u8 as *const libc::c_char,
                stderr,
            );
            exit(1 as libc::c_int);
        }
        if (__sput.suite.name).is_null() {
            fputs(
                b"sput_enter_suite() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        failp_6 = if __sput.suite.checks != 0 {
            (__sput.suite.nok as libc::c_double * 100.0f64
                / __sput.suite.checks as libc::c_double) as libc::c_float
        } else {
            0.0f32
        };
        fprintf(
            __sput.out,
            b"\n--> %lu check(s), %lu ok, %lu failed (%.2f%%)\n\0" as *const u8
                as *const libc::c_char,
            __sput.suite.checks,
            __sput.suite.ok,
            __sput.suite.nok,
            failp_6 as libc::c_double,
        );
        __sput
            .overall
            .checks = (__sput.overall.checks).wrapping_add(__sput.suite.checks);
        __sput.overall.ok = (__sput.overall.ok).wrapping_add(__sput.suite.ok);
        __sput.overall.nok = (__sput.overall.nok).wrapping_add(__sput.suite.nok);
        memset(
            &mut __sput.suite as *mut sput_suite as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sput_suite>() as libc::c_ulong,
        );
    }
    failp_5 = if __sput.overall.checks != 0 {
        (__sput.overall.nok as libc::c_double * 100.0f64
            / __sput.overall.checks as libc::c_double) as libc::c_float
    } else {
        0.0f32
    };
    __sput.time.end = time(0 as *mut time_t);
    fprintf(
        __sput.out,
        b"\n==> %lu check(s) in %lu suite(s) finished after %.2f second(s),\n    %lu succeeded, %lu failed (%.2f%%)\n\n[%s]\n\0"
            as *const u8 as *const libc::c_char,
        __sput.overall.checks,
        __sput.overall.suites,
        difftime(__sput.time.end, __sput.time.start),
        __sput.overall.ok,
        __sput.overall.nok,
        failp_5 as libc::c_double,
        if (if __sput.overall.nok > 0 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) == 0 as libc::c_int
        {
            b"SUCCESS\0" as *const u8 as *const libc::c_char
        } else {
            b"FAILURE\0" as *const u8 as *const libc::c_char
        },
    );
    return if __sput.overall.nok > 0 as libc::c_int as libc::c_ulong {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

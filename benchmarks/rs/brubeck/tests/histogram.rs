use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn brubeck_histo_push(
        histo: *mut brubeck_histo,
        value: value_t,
        sample_rate: value_t,
    );
    fn brubeck_histo_sample(
        sample: *mut brubeck_histo_sample,
        histo: *mut brubeck_histo,
    );
    static mut __sput: sput;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type pthread_spinlock_t = libc::c_int;
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
pub type value_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_histo {
    pub values: *mut value_t,
    pub count: uint32_t,
    pub alloc: uint16_t,
    pub size: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_histo_sample {
    pub sum: value_t,
    pub min: value_t,
    pub max: value_t,
    pub mean: value_t,
    pub median: value_t,
    pub count: value_t,
    pub percentile: [value_t; 5],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct histo_test {
    pub h: brubeck_histo,
    pub lock: pthread_spinlock_t,
}
#[inline]
unsafe extern "C" fn spawn_threads(
    mut thread_ptr: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    >,
    mut ptr: *mut libc::c_void,
) {
    let mut i: size_t = 0;
    let mut threads: [pthread_t; 16] = [0; 16];
    i = 0 as libc::c_int as size_t;
    while i < 16 as libc::c_int as libc::c_ulong {
        pthread_create(
            &mut *threads.as_mut_ptr().offset(i as isize),
            0 as *const pthread_attr_t,
            thread_ptr,
            ptr,
        );
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < 16 as libc::c_int as libc::c_ulong {
        pthread_join(threads[i as usize], 0 as *mut *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn thread_histo(mut ptr: *mut libc::c_void) -> *mut libc::c_void {
    let mut t: *mut histo_test = ptr as *mut histo_test;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (4096 as libc::c_int * 8 as libc::c_int) as libc::c_ulong {
        if rand() % 2 as libc::c_int == 0 as libc::c_int {
            let mut hsample: brubeck_histo_sample = brubeck_histo_sample {
                sum: 0.,
                min: 0.,
                max: 0.,
                mean: 0.,
                median: 0.,
                count: 0.,
                percentile: [0.; 5],
            };
            pthread_spin_lock(&mut (*t).lock);
            brubeck_histo_sample(&mut hsample, &mut (*t).h);
            pthread_spin_unlock(&mut (*t).lock);
        } else {
            pthread_spin_lock(&mut (*t).lock);
            brubeck_histo_push(&mut (*t).h, 0.42f64, 1.0f64);
            pthread_spin_unlock(&mut (*t).lock);
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn test_histogram__sampling() {
    let mut test: histo_test = histo_test {
        h: brubeck_histo {
            values: 0 as *mut value_t,
            count: 0,
            alloc: 0,
            size: 0,
        },
        lock: 0,
    };
    memset(
        &mut test.h as *mut brubeck_histo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_histo>() as libc::c_ulong,
    );
    pthread_spin_init(&mut test.lock, 0 as libc::c_int);
    spawn_threads(
        Some(
            thread_histo as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        &mut test as *mut histo_test as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn test_histogram__single_element() {
    let mut h: brubeck_histo = brubeck_histo {
        values: 0 as *mut value_t,
        count: 0,
        alloc: 0,
        size: 0,
    };
    let mut sample: brubeck_histo_sample = brubeck_histo_sample {
        sum: 0.,
        min: 0.,
        max: 0.,
        mean: 0.,
        median: 0.,
        count: 0.,
        percentile: [0.; 5],
    };
    memset(
        &mut h as *mut brubeck_histo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_histo>() as libc::c_ulong,
    );
    brubeck_histo_push(&mut h, 42.0f64, 1.0f64);
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"histogram size\0" as *const u8 as *const libc::c_char).is_null() {
        b"histogram size\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 48 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"h.size == 1\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(h.size as libc::c_int == 1 as libc::c_int) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"histogram value count\0" as *const u8 as *const libc::c_char)
        .is_null()
    {
        b"histogram value count\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 49 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"h.count == 1\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(h.count == 1 as libc::c_int as libc::c_uint) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
    brubeck_histo_sample(&mut sample, &mut h);
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.min\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.min\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 53 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.min == 42.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.min == 42.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.max\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.max\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 54 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.max == 42.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.max == 42.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.percentile[3]\0" as *const u8 as *const libc::c_char)
        .is_null()
    {
        b"sample.percentile[3]\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 55 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"sample.percentile[3] == 42.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.percentile[3 as libc::c_int as usize] == 42.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.mean\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.mean\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 56 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.mean == 42.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.mean == 42.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.count\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.count\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 57 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.count == 1\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.count == 1 as libc::c_int as libc::c_double) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.sum\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.sum\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 58 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.sum == 42.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.sum == 42.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    };
}
pub unsafe extern "C" fn test_histogram__large_range() {
    let mut h: brubeck_histo = brubeck_histo {
        values: 0 as *mut value_t,
        count: 0,
        alloc: 0,
        size: 0,
    };
    let mut sample: brubeck_histo_sample = brubeck_histo_sample {
        sum: 0.,
        min: 0.,
        max: 0.,
        mean: 0.,
        median: 0.,
        count: 0.,
        percentile: [0.; 5],
    };
    memset(
        &mut h as *mut brubeck_histo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_histo>() as libc::c_ulong,
    );
    brubeck_histo_push(&mut h, 1.3e12f64, 1.0f64);
    brubeck_histo_push(&mut h, 42.0f64, 1.0f64);
    brubeck_histo_push(&mut h, 42.0f64, 1.0f64);
    brubeck_histo_sample(&mut sample, &mut h);
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.min\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.min\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 73 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.min == 42.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.min == 42.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.max\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.max\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 74 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.max == 1.3e12\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.max == 1.3e12f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.median\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.median\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 75 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.median == 42.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.median == 42.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    };
}
pub unsafe extern "C" fn test_histogram__multisamples() {
    let mut h: brubeck_histo = brubeck_histo {
        values: 0 as *mut value_t,
        count: 0,
        alloc: 0,
        size: 0,
    };
    let mut sample: brubeck_histo_sample = brubeck_histo_sample {
        sum: 0.,
        min: 0.,
        max: 0.,
        mean: 0.,
        median: 0.,
        count: 0.,
        percentile: [0.; 5],
    };
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    memset(
        &mut h as *mut brubeck_histo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_histo>() as libc::c_ulong,
    );
    i = 0 as libc::c_int as size_t;
    while i < 8 as libc::c_int as libc::c_ulong {
        j = 0 as libc::c_int as size_t;
        while j < 128 as libc::c_int as libc::c_ulong {
            brubeck_histo_push(
                &mut h,
                j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double,
                1.0f64,
            );
            j = j.wrapping_add(1);
            j;
        }
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
        if (__sput.test.name).is_null() {
            fputs(
                b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        __sput
            .check
            .name = if !(b"histogram size\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            b"histogram size\0" as *const u8 as *const libc::c_char
        } else {
            b"Unlabeled Check\0" as *const u8 as *const libc::c_char
        };
        __sput.check.line = 89 as libc::c_int as libc::c_ulong;
        __sput.check.cond = b"h.size == 128\0" as *const u8 as *const libc::c_char;
        __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
        __sput.test.nr = (__sput.test.nr).wrapping_add(1);
        __sput.test.nr;
        __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
        __sput.suite.checks;
        if !(h.size as libc::c_int == 128 as libc::c_int) {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
            __sput.suite.nok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                    as *const u8 as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
                __sput.check.type_0,
                __sput.check.cond,
                __sput.check.line,
            );
        } else {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
            __sput.suite.ok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8
                    as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
            );
        }
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
        if (__sput.test.name).is_null() {
            fputs(
                b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        __sput
            .check
            .name = if !(b"histogram value count\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            b"histogram value count\0" as *const u8 as *const libc::c_char
        } else {
            b"Unlabeled Check\0" as *const u8 as *const libc::c_char
        };
        __sput.check.line = 90 as libc::c_int as libc::c_ulong;
        __sput.check.cond = b"h.count == 128\0" as *const u8 as *const libc::c_char;
        __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
        __sput.test.nr = (__sput.test.nr).wrapping_add(1);
        __sput.test.nr;
        __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
        __sput.suite.checks;
        if !(h.count == 128 as libc::c_int as libc::c_uint) {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
            __sput.suite.nok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                    as *const u8 as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
                __sput.check.type_0,
                __sput.check.cond,
                __sput.check.line,
            );
        } else {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
            __sput.suite.ok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8
                    as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
            );
        }
        brubeck_histo_sample(&mut sample, &mut h);
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
        if (__sput.test.name).is_null() {
            fputs(
                b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        __sput
            .check
            .name = if !(b"sample.min\0" as *const u8 as *const libc::c_char).is_null() {
            b"sample.min\0" as *const u8 as *const libc::c_char
        } else {
            b"Unlabeled Check\0" as *const u8 as *const libc::c_char
        };
        __sput.check.line = 94 as libc::c_int as libc::c_ulong;
        __sput.check.cond = b"sample.min == 1.0\0" as *const u8 as *const libc::c_char;
        __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
        __sput.test.nr = (__sput.test.nr).wrapping_add(1);
        __sput.test.nr;
        __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
        __sput.suite.checks;
        if !(sample.min == 1.0f64) {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
            __sput.suite.nok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                    as *const u8 as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
                __sput.check.type_0,
                __sput.check.cond,
                __sput.check.line,
            );
        } else {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
            __sput.suite.ok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8
                    as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
            );
        }
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
        if (__sput.test.name).is_null() {
            fputs(
                b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        __sput
            .check
            .name = if !(b"sample.max\0" as *const u8 as *const libc::c_char).is_null() {
            b"sample.max\0" as *const u8 as *const libc::c_char
        } else {
            b"Unlabeled Check\0" as *const u8 as *const libc::c_char
        };
        __sput.check.line = 95 as libc::c_int as libc::c_ulong;
        __sput.check.cond = b"sample.max == 128.0\0" as *const u8 as *const libc::c_char;
        __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
        __sput.test.nr = (__sput.test.nr).wrapping_add(1);
        __sput.test.nr;
        __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
        __sput.suite.checks;
        if !(sample.max == 128.0f64) {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
            __sput.suite.nok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                    as *const u8 as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
                __sput.check.type_0,
                __sput.check.cond,
                __sput.check.line,
            );
        } else {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
            __sput.suite.ok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8
                    as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
            );
        }
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
        if (__sput.test.name).is_null() {
            fputs(
                b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        __sput
            .check
            .name = if !(b"sample.percentile[3]\0" as *const u8 as *const libc::c_char)
            .is_null()
        {
            b"sample.percentile[3]\0" as *const u8 as *const libc::c_char
        } else {
            b"Unlabeled Check\0" as *const u8 as *const libc::c_char
        };
        __sput.check.line = 96 as libc::c_int as libc::c_ulong;
        __sput
            .check
            .cond = b"sample.percentile[3] == 127.0\0" as *const u8
            as *const libc::c_char;
        __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
        __sput.test.nr = (__sput.test.nr).wrapping_add(1);
        __sput.test.nr;
        __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
        __sput.suite.checks;
        if !(sample.percentile[3 as libc::c_int as usize] == 127.0f64) {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
            __sput.suite.nok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                    as *const u8 as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
                __sput.check.type_0,
                __sput.check.cond,
                __sput.check.line,
            );
        } else {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
            __sput.suite.ok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8
                    as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
            );
        }
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
        if (__sput.test.name).is_null() {
            fputs(
                b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        __sput
            .check
            .name = if !(b"sample.mean\0" as *const u8 as *const libc::c_char).is_null()
        {
            b"sample.mean\0" as *const u8 as *const libc::c_char
        } else {
            b"Unlabeled Check\0" as *const u8 as *const libc::c_char
        };
        __sput.check.line = 97 as libc::c_int as libc::c_ulong;
        __sput.check.cond = b"sample.mean == 64.5\0" as *const u8 as *const libc::c_char;
        __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
        __sput.test.nr = (__sput.test.nr).wrapping_add(1);
        __sput.test.nr;
        __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
        __sput.suite.checks;
        if !(sample.mean == 64.5f64) {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
            __sput.suite.nok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                    as *const u8 as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
                __sput.check.type_0,
                __sput.check.cond,
                __sput.check.line,
            );
        } else {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
            __sput.suite.ok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8
                    as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
            );
        }
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
        if (__sput.test.name).is_null() {
            fputs(
                b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        __sput
            .check
            .name = if !(b"sample.count\0" as *const u8 as *const libc::c_char).is_null()
        {
            b"sample.count\0" as *const u8 as *const libc::c_char
        } else {
            b"Unlabeled Check\0" as *const u8 as *const libc::c_char
        };
        __sput.check.line = 98 as libc::c_int as libc::c_ulong;
        __sput.check.cond = b"sample.count == 128\0" as *const u8 as *const libc::c_char;
        __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
        __sput.test.nr = (__sput.test.nr).wrapping_add(1);
        __sput.test.nr;
        __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
        __sput.suite.checks;
        if !(sample.count == 128 as libc::c_int as libc::c_double) {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
            __sput.suite.nok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                    as *const u8 as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
                __sput.check.type_0,
                __sput.check.cond,
                __sput.check.line,
            );
        } else {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
            __sput.suite.ok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8
                    as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
            );
        }
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
        if (__sput.test.name).is_null() {
            fputs(
                b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
                __sput.out,
            );
            exit(1 as libc::c_int);
        }
        __sput
            .check
            .name = if !(b"sample.sum\0" as *const u8 as *const libc::c_char).is_null() {
            b"sample.sum\0" as *const u8 as *const libc::c_char
        } else {
            b"Unlabeled Check\0" as *const u8 as *const libc::c_char
        };
        __sput.check.line = 99 as libc::c_int as libc::c_ulong;
        __sput
            .check
            .cond = b"sample.sum == 8256.0\0" as *const u8 as *const libc::c_char;
        __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
        __sput.test.nr = (__sput.test.nr).wrapping_add(1);
        __sput.test.nr;
        __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
        __sput.suite.checks;
        if !(sample.sum == 8256.0f64) {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
            __sput.suite.nok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                    as *const u8 as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
                __sput.check.type_0,
                __sput.check.cond,
                __sput.check.line,
            );
        } else {
            if __sput.initialized as libc::c_int != 0x6 as libc::c_int {
                fputs(
                    b"sput_start_testing() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    stderr,
                );
                exit(1 as libc::c_int);
            }
            if (__sput.suite.name).is_null() {
                fputs(
                    b"sput_enter_suite() omitted\n\0" as *const u8
                        as *const libc::c_char,
                    __sput.out,
                );
                exit(1 as libc::c_int);
            }
            __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
            __sput.suite.ok;
            fprintf(
                __sput.out,
                b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8
                    as *const libc::c_char,
                __sput.suite.nr,
                __sput.suite.checks,
                __sput.test.name,
                __sput.test.nr,
                __sput.check.name,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
pub unsafe extern "C" fn test_histogram__with_sample_rate() {
    let mut h: brubeck_histo = brubeck_histo {
        values: 0 as *mut value_t,
        count: 0,
        alloc: 0,
        size: 0,
    };
    let mut sample: brubeck_histo_sample = brubeck_histo_sample {
        sum: 0.,
        min: 0.,
        max: 0.,
        mean: 0.,
        median: 0.,
        count: 0.,
        percentile: [0.; 5],
    };
    let mut j: size_t = 0;
    memset(
        &mut h as *mut brubeck_histo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_histo>() as libc::c_ulong,
    );
    j = 0 as libc::c_int as size_t;
    while j < 128 as libc::c_int as libc::c_ulong {
        brubeck_histo_push(
            &mut h,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double,
            10.0f64,
        );
        j = j.wrapping_add(1);
        j;
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"histogram size\0" as *const u8 as *const libc::c_char).is_null() {
        b"histogram size\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 113 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"h.size == 128\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(h.size as libc::c_int == 128 as libc::c_int) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"histogram value count\0" as *const u8 as *const libc::c_char)
        .is_null()
    {
        b"histogram value count\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 114 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"h.count == 1280\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(h.count == 1280 as libc::c_int as libc::c_uint) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
    brubeck_histo_sample(&mut sample, &mut h);
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.min\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.min\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 118 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.min == 1.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.min == 1.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.max\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.max\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 119 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.max == 128.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.max == 128.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.percentile[3]\0" as *const u8 as *const libc::c_char)
        .is_null()
    {
        b"sample.percentile[3]\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 120 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"sample.percentile[3] == 127.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.percentile[3 as libc::c_int as usize] == 127.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.mean\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.mean\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 121 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.mean == 64.5\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.mean == 64.5f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.count\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.count\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 122 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.count == 1280\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.count == 1280 as libc::c_int as libc::c_double) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.sum\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.sum\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 123 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.sum == 8256.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.sum == 8256.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    };
}
pub unsafe extern "C" fn test_histogram__capacity() {
    static mut HISTO_CAP: size_t = (32767 as libc::c_int * 2 as libc::c_int
        + 1 as libc::c_int) as size_t;
    let mut h: brubeck_histo = brubeck_histo {
        values: 0 as *mut value_t,
        count: 0,
        alloc: 0,
        size: 0,
    };
    let mut sample: brubeck_histo_sample = brubeck_histo_sample {
        sum: 0.,
        min: 0.,
        max: 0.,
        mean: 0.,
        median: 0.,
        count: 0.,
        percentile: [0.; 5],
    };
    let mut j: size_t = 0;
    memset(
        &mut h as *mut brubeck_histo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<brubeck_histo>() as libc::c_ulong,
    );
    j = 0 as libc::c_int as size_t;
    while j < HISTO_CAP.wrapping_add(500 as libc::c_int as libc::c_ulong) {
        brubeck_histo_push(
            &mut h,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double,
            1.0f64,
        );
        j = j.wrapping_add(1);
        j;
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"histogram size\0" as *const u8 as *const libc::c_char).is_null() {
        b"histogram size\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 138 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"h.size == HISTO_CAP\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(h.size as libc::c_ulong == HISTO_CAP) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"histogram value count\0" as *const u8 as *const libc::c_char)
        .is_null()
    {
        b"histogram value count\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 139 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"h.count == (HISTO_CAP + 500)\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(h.count as libc::c_ulong
        == HISTO_CAP.wrapping_add(500 as libc::c_int as libc::c_ulong))
    {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
    brubeck_histo_sample(&mut sample, &mut h);
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.min\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.min\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 143 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.min == 1.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.min == 1.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.max\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.max\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 144 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"sample.max == (double)HISTO_CAP\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.max == HISTO_CAP as libc::c_double) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.count\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.count\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 145 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"sample.count == (HISTO_CAP + 500)\0" as *const u8
        as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.count
        == HISTO_CAP.wrapping_add(500 as libc::c_int as libc::c_ulong) as libc::c_double)
    {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
    j = 0 as libc::c_int as size_t;
    while j < HISTO_CAP.wrapping_add(500 as libc::c_int as libc::c_ulong) {
        brubeck_histo_push(
            &mut h,
            j.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_double,
            10.0f64,
        );
        j = j.wrapping_add(1);
        j;
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"histogram size\0" as *const u8 as *const libc::c_char).is_null() {
        b"histogram size\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 150 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"h.size == HISTO_CAP\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(h.size as libc::c_ulong == HISTO_CAP) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"histogram value count\0" as *const u8 as *const libc::c_char)
        .is_null()
    {
        b"histogram value count\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 152 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"h.count == ((HISTO_CAP + 500) * 10)\0" as *const u8
        as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(h.count as libc::c_ulong
        == HISTO_CAP
            .wrapping_add(500 as libc::c_int as libc::c_ulong)
            .wrapping_mul(10 as libc::c_int as libc::c_ulong))
    {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
    brubeck_histo_sample(&mut sample, &mut h);
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.min\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.min\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 156 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"sample.min == 1.0\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.min == 1.0f64) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.max\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.max\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 157 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"sample.max == (double)HISTO_CAP\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.max == HISTO_CAP as libc::c_double) {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    }
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
    if (__sput.test.name).is_null() {
        fputs(
            b"sput_run_test() omitted\n\0" as *const u8 as *const libc::c_char,
            __sput.out,
        );
        exit(1 as libc::c_int);
    }
    __sput
        .check
        .name = if !(b"sample.count\0" as *const u8 as *const libc::c_char).is_null() {
        b"sample.count\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 158 as libc::c_int as libc::c_ulong;
    __sput
        .check
        .cond = b"sample.count == ((HISTO_CAP + 500) * 10)\0" as *const u8
        as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(sample.count
        == HISTO_CAP
            .wrapping_add(500 as libc::c_int as libc::c_ulong)
            .wrapping_mul(10 as libc::c_int as libc::c_ulong) as libc::c_double)
    {
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
        __sput.suite.nok = (__sput.suite.nok).wrapping_add(1);
        __sput.suite.nok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  FAIL\n!    Type:      %s\n!    Condition: %s\n!    Line:      %lu\n\0"
                as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
            __sput.check.type_0,
            __sput.check.cond,
            __sput.check.line,
        );
    } else {
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
        __sput.suite.ok = (__sput.suite.ok).wrapping_add(1);
        __sput.suite.ok;
        fprintf(
            __sput.out,
            b"[%lu:%lu]  %s:#%lu  \"%s\"  pass\n\0" as *const u8 as *const libc::c_char,
            __sput.suite.nr,
            __sput.suite.checks,
            __sput.test.name,
            __sput.test.nr,
            __sput.check.name,
        );
    };
}

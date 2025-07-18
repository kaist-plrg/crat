use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type brubeck_hashtable_t;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn brubeck_hashtable_new(size: uint64_t) -> *mut brubeck_hashtable_t;
    fn brubeck_hashtable_find(
        ht: *mut brubeck_hashtable_t,
        key: *const libc::c_char,
        key_len: uint16_t,
    ) -> *mut brubeck_metric;
    fn brubeck_hashtable_insert(
        ht: *mut brubeck_hashtable_t,
        key: *const libc::c_char,
        key_len: uint16_t,
        val: *mut brubeck_metric,
    ) -> bool;
    static mut __sput: sput;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
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
pub struct brubeck_metric {
    pub next: *mut brubeck_metric,
    pub tags: *const brubeck_tag_set,
    pub lock: pthread_spinlock_t,
    pub key_len: uint16_t,
    pub type_0: uint8_t,
    pub private_state: uint8_t,
    pub as_0: C2RustUnnamed,
    pub key: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub gauge: C2RustUnnamed_1,
    pub meter: C2RustUnnamed_1,
    pub counter: C2RustUnnamed_0,
    pub histogram: brubeck_histo,
    pub other: *mut libc::c_void,
}
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
pub struct C2RustUnnamed_0 {
    pub value: value_t,
    pub previous: value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub value: value_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tag_set {
    pub index: uint32_t,
    pub tag_len: uint16_t,
    pub num_tags: uint16_t,
    pub tags: [brubeck_tag; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct brubeck_tag {
    pub key: *const libc::c_char,
    pub value: *const libc::c_char,
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
unsafe extern "C" fn new_metric(mut name: *const libc::c_char) -> *mut brubeck_metric {
    let mut name_len: size_t = strlen(name);
    let mut metric: *mut brubeck_metric = malloc(
        (::std::mem::size_of::<brubeck_metric>() as libc::c_ulong)
            .wrapping_add(name_len)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut brubeck_metric;
    memcpy(
        ((*metric).key).as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        name_len,
    );
    *((*metric).key)
        .as_mut_ptr()
        .offset(name_len as isize) = 0 as libc::c_int as libc::c_char;
    (*metric).key_len = name_len as uint16_t;
    return metric;
}
pub unsafe extern "C" fn test_mstore__save() {
    static mut nmetrics: libc::c_int = 15000 as libc::c_int;
    let mut store: *mut brubeck_hashtable_t = 0 as *mut brubeck_hashtable_t;
    let mut i: libc::c_int = 0;
    store = brubeck_hashtable_new(4096 as libc::c_int as uint64_t);
    i = 0 as libc::c_int;
    while i < nmetrics {
        let mut buffer: [libc::c_char; 64] = [0; 64];
        let mut metric: *mut brubeck_metric = 0 as *mut brubeck_metric;
        sprintf(
            buffer.as_mut_ptr(),
            b"github.test.metric.%d\0" as *const u8 as *const libc::c_char,
            i,
        );
        metric = new_metric(buffer.as_mut_ptr());
        if !brubeck_hashtable_insert(
            store,
            ((*metric).key).as_mut_ptr(),
            (*metric).key_len,
            metric,
        ) {
            break;
        }
        i += 1;
        i;
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
        .name = if !(b"stored 15000 metrics in table\0" as *const u8
        as *const libc::c_char)
        .is_null()
    {
        b"stored 15000 metrics in table\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 34 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"i == nmetrics\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(i == nmetrics) {
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
    i = 0 as libc::c_int;
    while i < nmetrics + 1 as libc::c_int {
        let mut buffer_0: [libc::c_char; 64] = [0; 64];
        let mut len: uint16_t = 0;
        len = sprintf(
            buffer_0.as_mut_ptr(),
            b"github.test.metric.%d\0" as *const u8 as *const libc::c_char,
            i,
        ) as uint16_t;
        if (brubeck_hashtable_find(store, buffer_0.as_mut_ptr(), len)).is_null() {
            break;
        }
        i += 1;
        i;
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
        .name = if !(b"lookup all metrics from table\0" as *const u8
        as *const libc::c_char)
        .is_null()
    {
        b"lookup all metrics from table\0" as *const u8 as *const libc::c_char
    } else {
        b"Unlabeled Check\0" as *const u8 as *const libc::c_char
    };
    __sput.check.line = 47 as libc::c_int as libc::c_ulong;
    __sput.check.cond = b"i == nmetrics\0" as *const u8 as *const libc::c_char;
    __sput.check.type_0 = b"fail-unless\0" as *const u8 as *const libc::c_char;
    __sput.test.nr = (__sput.test.nr).wrapping_add(1);
    __sput.test.nr;
    __sput.suite.checks = (__sput.suite.checks).wrapping_add(1);
    __sput.suite.checks;
    if !(i == nmetrics) {
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

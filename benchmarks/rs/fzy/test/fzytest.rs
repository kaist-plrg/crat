use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn clock() -> clock_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn match_suite();
    fn choices_suite();
    fn properties_suite();
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_suite_info {
    pub tests_run: libc::c_uint,
    pub passed: libc::c_uint,
    pub failed: libc::c_uint,
    pub skipped: libc::c_uint,
    pub pre_suite: clock_t,
    pub post_suite: clock_t,
    pub pre_test: clock_t,
    pub post_test: clock_t,
}
pub type greatest_suite_cb = unsafe extern "C" fn() -> ();
pub type greatest_setup_cb = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type greatest_teardown_cb = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type greatest_equal_cb = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
pub type greatest_printf_cb = unsafe extern "C" fn(
    *const libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_type_info {
    pub equal: Option::<greatest_equal_cb>,
    pub print: Option::<greatest_printf_cb>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_memory_cmp_env {
    pub exp: *const libc::c_uchar,
    pub got: *const libc::c_uchar,
    pub size: size_t,
}
pub type greatest_flag_t = libc::c_uint;
pub const GREATEST_FLAG_LIST_ONLY: greatest_flag_t = 2;
pub const GREATEST_FLAG_FIRST_FAIL: greatest_flag_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_prng {
    pub random_order: libc::c_uchar,
    pub initialized: libc::c_uchar,
    pub pad_0: [libc::c_uchar; 2],
    pub state: libc::c_ulong,
    pub count: libc::c_ulong,
    pub count_ceil: libc::c_ulong,
    pub count_run: libc::c_ulong,
    pub mod_0: libc::c_ulong,
    pub a: libc::c_ulong,
    pub c: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_run_info {
    pub flags: libc::c_uchar,
    pub verbosity: libc::c_uchar,
    pub pad_0: [libc::c_uchar; 2],
    pub tests_run: libc::c_uint,
    pub suite: greatest_suite_info,
    pub passed: libc::c_uint,
    pub failed: libc::c_uint,
    pub skipped: libc::c_uint,
    pub assertions: libc::c_uint,
    pub fail_line: libc::c_uint,
    pub pad_1: libc::c_uint,
    pub fail_file: *const libc::c_char,
    pub msg: *const libc::c_char,
    pub setup: Option::<greatest_setup_cb>,
    pub setup_udata: *mut libc::c_void,
    pub teardown: Option::<greatest_teardown_cb>,
    pub teardown_udata: *mut libc::c_void,
    pub col: libc::c_uint,
    pub width: libc::c_uint,
    pub suite_filter: *const libc::c_char,
    pub test_filter: *const libc::c_char,
    pub test_exclude: *const libc::c_char,
    pub prng: [greatest_prng; 2],
    pub begin: clock_t,
    pub end: clock_t,
    pub pad_jmp_buf: libc::c_int,
    pub jump_dest: jmp_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_report_t {
    pub passed: libc::c_uint,
    pub failed: libc::c_uint,
    pub skipped: libc::c_uint,
    pub assertions: libc::c_uint,
}
pub const GREATEST_TEST_RES_PASS: greatest_test_res = 0;
pub const GREATEST_TEST_RES_SKIP: greatest_test_res = 1;
pub const GREATEST_TEST_RES_FAIL: greatest_test_res = -1;
pub type greatest_test_res = libc::c_int;
pub unsafe extern "C" fn greatest_prng_init_second_pass(
    mut id: libc::c_int,
    mut seed: libc::c_ulong,
) -> libc::c_int {
    static mut primes: [libc::c_ulong; 13] = [
        11 as libc::c_int as libc::c_ulong,
        101 as libc::c_int as libc::c_ulong,
        1009 as libc::c_int as libc::c_ulong,
        10007 as libc::c_int as libc::c_ulong,
        100003 as libc::c_int as libc::c_ulong,
        1000003 as libc::c_int as libc::c_ulong,
        10000019 as libc::c_int as libc::c_ulong,
        100000007 as libc::c_int as libc::c_ulong,
        1000000007 as libc::c_int as libc::c_ulong,
        1538461 as libc::c_int as libc::c_ulong,
        1865471 as libc::c_int as libc::c_ulong,
        17471 as libc::c_int as libc::c_ulong,
        2147483647 as libc::c_int as libc::c_ulong,
    ];
    let mut prng: *mut greatest_prng = &mut *(greatest_info.prng)
        .as_mut_ptr()
        .offset(id as isize) as *mut greatest_prng;
    if (*prng).count == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    (*prng).mod_0 = 1 as libc::c_int as libc::c_ulong;
    (*prng).count_ceil = (*prng).count;
    while (*prng).mod_0 < (*prng).count {
        (*prng).mod_0 <<= 1 as libc::c_int;
    }
    (*prng).state = seed & 0x1fffffff as libc::c_int as libc::c_ulong;
    (*prng)
        .a = (4 as libc::c_ulong)
        .wrapping_mul((*prng).state)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    (*prng)
        .c = primes[seed
        .wrapping_mul(16451 as libc::c_int as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<[libc::c_ulong; 13]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong) as usize];
    (*prng).initialized = 1 as libc::c_int as libc::c_uchar;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn greatest_usage(mut name: *const libc::c_char) {
    fprintf(
        stdout,
        b"Usage: %s [--help] [-hlfv] [-s SUITE] [-t TEST]\n  -h, --help  print this Help\n  -l          List suites and tests, then exit (dry run)\n  -f          Stop runner after first failure\n  -v          Verbose output\n  -s SUITE    only run suites containing string SUITE\n  -t TEST     only run tests containing string TEST\n  -x EXCLUDE  exclude tests containing string EXCLUDE\n\0"
            as *const u8 as *const libc::c_char,
        name,
    );
}
pub unsafe extern "C" fn greatest_stop_at_first_fail() {
    greatest_info
        .flags = (greatest_info.flags as libc::c_int
        | GREATEST_FLAG_FIRST_FAIL as libc::c_int) as libc::c_uchar;
}
pub unsafe extern "C" fn greatest_set_test_filter(mut filter: *const libc::c_char) {
    greatest_info.test_filter = filter;
}
pub unsafe extern "C" fn greatest_set_flag(mut flag: greatest_flag_t) {
    greatest_info
        .flags = (greatest_info.flags as libc::c_uint | flag as libc::c_uint)
        as libc::c_uchar;
}
pub unsafe extern "C" fn greatest_set_verbosity(mut verbosity: libc::c_uint) {
    greatest_info.verbosity = verbosity as libc::c_uchar;
}
pub unsafe extern "C" fn greatest_get_verbosity() -> libc::c_uint {
    return greatest_info.verbosity as libc::c_uint;
}
pub unsafe extern "C" fn greatest_get_report(mut report: *mut greatest_report_t) {
    if !report.is_null() {
        (*report).passed = greatest_info.passed;
        (*report).failed = greatest_info.failed;
        (*report).skipped = greatest_info.skipped;
        (*report).assertions = greatest_info.assertions;
    }
}
pub unsafe extern "C" fn greatest_set_suite_filter(mut filter: *const libc::c_char) {
    greatest_info.suite_filter = filter;
}
pub unsafe extern "C" fn greatest_all_passed() -> libc::c_int {
    return (greatest_info.failed == 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
pub unsafe extern "C" fn GREATEST_SET_TEARDOWN_CB(
    mut cb: Option::<greatest_teardown_cb>,
    mut udata: *mut libc::c_void,
) {
    greatest_info.teardown = cb;
    greatest_info.teardown_udata = udata;
}
pub unsafe extern "C" fn GREATEST_SET_SETUP_CB(
    mut cb: Option::<greatest_setup_cb>,
    mut udata: *mut libc::c_void,
) {
    greatest_info.setup = cb;
    greatest_info.setup_udata = udata;
}
unsafe extern "C" fn greatest_run_suite(
    mut suite_cb: Option::<greatest_suite_cb>,
    mut suite_name: *const libc::c_char,
) {
    if greatest_suite_pre(suite_name) != 0 {
        suite_cb.unwrap()();
        greatest_suite_post();
    }
}
pub unsafe extern "C" fn greatest_prng_step(mut id: libc::c_int) {
    let mut p: *mut greatest_prng = &mut *(greatest_info.prng)
        .as_mut_ptr()
        .offset(id as isize) as *mut greatest_prng;
    loop {
        (*p)
            .state = ((*p).a).wrapping_mul((*p).state).wrapping_add((*p).c)
            & ((*p).mod_0).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        if !((*p).state >= (*p).count_ceil) {
            break;
        }
    };
}
pub unsafe extern "C" fn greatest_set_test_exclude(mut filter: *const libc::c_char) {
    greatest_info.test_exclude = filter;
}
pub unsafe extern "C" fn greatest_prng_init_first_pass(mut id: libc::c_int) {
    greatest_info.prng[id as usize].random_order = 1 as libc::c_int as libc::c_uchar;
    greatest_info.prng[id as usize].count_run = 0 as libc::c_int as libc::c_ulong;
}
pub unsafe extern "C" fn greatest_do_assert_equal_t(
    mut exp: *const libc::c_void,
    mut got: *const libc::c_void,
    mut type_info: *mut greatest_type_info,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut eq: libc::c_int = 0 as libc::c_int;
    if type_info.is_null() || ((*type_info).equal).is_none() {
        return 0 as libc::c_int;
    }
    eq = ((*type_info).equal).unwrap()(exp, got, udata);
    if eq == 0 {
        if ((*type_info).print).is_some() {
            fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
            ((*type_info).print).unwrap()(exp, udata);
            fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
            ((*type_info).print).unwrap()(got, udata);
            fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(
                stdout,
                b"GREATEST_ASSERT_EQUAL_T failure at %s:%u\n\0" as *const u8
                    as *const libc::c_char,
                greatest_info.fail_file,
                greatest_info.fail_line,
            );
        }
    }
    return eq;
}
pub static mut greatest_info: greatest_run_info = greatest_run_info {
    flags: 0,
    verbosity: 0,
    pad_0: [0; 2],
    tests_run: 0,
    suite: greatest_suite_info {
        tests_run: 0,
        passed: 0,
        failed: 0,
        skipped: 0,
        pre_suite: 0,
        post_suite: 0,
        pre_test: 0,
        post_test: 0,
    },
    passed: 0,
    failed: 0,
    skipped: 0,
    assertions: 0,
    fail_line: 0,
    pad_1: 0,
    fail_file: 0 as *const libc::c_char,
    msg: 0 as *const libc::c_char,
    setup: None,
    setup_udata: 0 as *const libc::c_void as *mut libc::c_void,
    teardown: None,
    teardown_udata: 0 as *const libc::c_void as *mut libc::c_void,
    col: 0,
    width: 0,
    suite_filter: 0 as *const libc::c_char,
    test_filter: 0 as *const libc::c_char,
    test_exclude: 0 as *const libc::c_char,
    prng: [greatest_prng {
        random_order: 0,
        initialized: 0,
        pad_0: [0; 2],
        state: 0,
        count: 0,
        count_ceil: 0,
        count_run: 0,
        mod_0: 0,
        a: 0,
        c: 0,
    }; 2],
    begin: 0,
    end: 0,
    pad_jmp_buf: 0,
    jump_dest: [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1],
};
unsafe extern "C" fn greatest_parse_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
        {
            let mut f: libc::c_char = *(*argv.offset(i as isize))
                .offset(1 as libc::c_int as isize);
            if (f as libc::c_int == 's' as i32 || f as libc::c_int == 't' as i32
                || f as libc::c_int == 'x' as i32) && argc <= i + 1 as libc::c_int
            {
                greatest_usage(*argv.offset(0 as libc::c_int as isize));
                exit(1 as libc::c_int);
            }
            let mut current_block_25: u64;
            match f as libc::c_int {
                115 => {
                    greatest_set_suite_filter(
                        *argv.offset((i + 1 as libc::c_int) as isize),
                    );
                    i += 1;
                    i;
                    current_block_25 = 7172762164747879670;
                }
                116 => {
                    greatest_set_test_filter(
                        *argv.offset((i + 1 as libc::c_int) as isize),
                    );
                    i += 1;
                    i;
                    current_block_25 = 7172762164747879670;
                }
                120 => {
                    greatest_set_test_exclude(
                        *argv.offset((i + 1 as libc::c_int) as isize),
                    );
                    i += 1;
                    i;
                    current_block_25 = 7172762164747879670;
                }
                102 => {
                    greatest_stop_at_first_fail();
                    current_block_25 = 7172762164747879670;
                }
                108 => {
                    greatest_info
                        .flags = (greatest_info.flags as libc::c_int
                        | GREATEST_FLAG_LIST_ONLY as libc::c_int) as libc::c_uchar;
                    current_block_25 = 7172762164747879670;
                }
                118 => {
                    greatest_info.verbosity = (greatest_info.verbosity).wrapping_add(1);
                    greatest_info.verbosity;
                    current_block_25 = 7172762164747879670;
                }
                104 => {
                    greatest_usage(*argv.offset(0 as libc::c_int as isize));
                    exit(0 as libc::c_int);
                }
                45 => {
                    if 0 as libc::c_int
                        == strncmp(
                            b"--help\0" as *const u8 as *const libc::c_char,
                            *argv.offset(i as isize),
                            6 as libc::c_int as libc::c_ulong,
                        )
                    {
                        greatest_usage(*argv.offset(0 as libc::c_int as isize));
                        exit(0 as libc::c_int);
                    } else if 0 as libc::c_int
                        == strncmp(
                            b"--\0" as *const u8 as *const libc::c_char,
                            *argv.offset(i as isize),
                            2 as libc::c_int as libc::c_ulong,
                        )
                    {
                        return
                    }
                    current_block_25 = 7437894006877067923;
                }
                _ => {
                    current_block_25 = 7437894006877067923;
                }
            }
            match current_block_25 {
                7172762164747879670 => {}
                _ => {
                    fprintf(
                        stdout,
                        b"Unknown argument '%s'\n\0" as *const u8 as *const libc::c_char,
                        *argv.offset(i as isize),
                    );
                    greatest_usage(*argv.offset(0 as libc::c_int as isize));
                    exit(1 as libc::c_int);
                }
            }
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn greatest_test_post(
    mut name: *const libc::c_char,
    mut res: libc::c_int,
) {
    greatest_info.suite.post_test = clock();
    if greatest_info.suite.post_test == -(1 as libc::c_int) as clock_t {
        fprintf(
            stdout,
            b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
            b"greatest_info.suite.post_test\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if (greatest_info.teardown).is_some() {
        let mut udata: *mut libc::c_void = greatest_info.teardown_udata;
        (greatest_info.teardown).unwrap()(udata);
    }
    if res <= GREATEST_TEST_RES_FAIL as libc::c_int {
        greatest_do_fail(name);
    } else if res >= GREATEST_TEST_RES_SKIP as libc::c_int {
        greatest_do_skip(name);
    } else if res == GREATEST_TEST_RES_PASS as libc::c_int {
        greatest_do_pass(name);
    }
    greatest_info.suite.tests_run = (greatest_info.suite.tests_run).wrapping_add(1);
    greatest_info.suite.tests_run;
    greatest_info.col = (greatest_info.col).wrapping_add(1);
    greatest_info.col;
    if greatest_info.verbosity as libc::c_int > 0 as libc::c_int {
        fprintf(
            stdout,
            b" (%lu ticks, %.3f sec)\0" as *const u8 as *const libc::c_char,
            (greatest_info.suite.post_test as libc::c_ulong)
                .wrapping_sub(greatest_info.suite.pre_test as libc::c_ulong),
            (greatest_info.suite.post_test - greatest_info.suite.pre_test)
                as libc::c_double
                / (1.0f64 * 1000000 as libc::c_int as __clock_t as libc::c_double),
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    } else if (greatest_info.col).wrapping_rem(greatest_info.width)
        == 0 as libc::c_int as libc::c_uint
    {
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info.col = 0 as libc::c_int as libc::c_uint;
    }
    fflush(stdout);
}
pub static mut greatest_type_info_string: greatest_type_info = unsafe {
    {
        let mut init = greatest_type_info {
            equal: Some(
                greatest_string_equal_cb
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print: Some(
                greatest_string_printf_cb
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
pub unsafe extern "C" fn greatest_test_pre(
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut g: *mut greatest_run_info = &mut greatest_info;
    let mut match_0: libc::c_int = (greatest_name_match(
        name,
        (*g).test_filter,
        1 as libc::c_int,
    ) != 0 && greatest_name_match(name, (*g).test_exclude, 0 as libc::c_int) == 0)
        as libc::c_int;
    if greatest_info.flags as libc::c_int & GREATEST_FLAG_LIST_ONLY as libc::c_int != 0 {
        if match_0 != 0 {
            fprintf(stdout, b"  %s\n\0" as *const u8 as *const libc::c_char, name);
        }
        return 0 as libc::c_int;
    }
    if match_0 != 0
        && (greatest_info.flags as libc::c_int & GREATEST_FLAG_FIRST_FAIL as libc::c_int
            == 0 || (*g).suite.failed == 0 as libc::c_int as libc::c_uint)
    {
        let mut p: *mut greatest_prng = &mut *((*g).prng)
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize) as *mut greatest_prng;
        if (*p).random_order != 0 {
            (*p).count = ((*p).count).wrapping_add(1);
            (*p).count;
            if (*p).initialized == 0
                || ((*p).count).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    != (*p).state
            {
                return 0 as libc::c_int;
            }
        }
        (*g).suite.pre_test = clock();
        if (*g).suite.pre_test == -(1 as libc::c_int) as clock_t {
            fprintf(
                stdout,
                b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
                b"g->suite.pre_test\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if ((*g).setup).is_some() {
            ((*g).setup).unwrap()((*g).setup_udata);
        }
        (*p).count_run = ((*p).count_run).wrapping_add(1);
        (*p).count_run;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn greatest_string_printf_cb(
    mut t: *const libc::c_void,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    return fprintf(
        stdout,
        b"%s\0" as *const u8 as *const libc::c_char,
        t as *const libc::c_char,
    );
}
unsafe extern "C" fn report_suite() {
    if greatest_info.suite.tests_run > 0 as libc::c_int as libc::c_uint {
        fprintf(
            stdout,
            b"\n%u test%s - %u passed, %u failed, %u skipped\0" as *const u8
                as *const libc::c_char,
            greatest_info.suite.tests_run,
            if greatest_info.suite.tests_run == 1 as libc::c_int as libc::c_uint {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
            greatest_info.suite.passed,
            greatest_info.suite.failed,
            greatest_info.suite.skipped,
        );
        fprintf(
            stdout,
            b" (%lu ticks, %.3f sec)\0" as *const u8 as *const libc::c_char,
            (greatest_info.suite.post_suite as libc::c_ulong)
                .wrapping_sub(greatest_info.suite.pre_suite as libc::c_ulong),
            (greatest_info.suite.post_suite - greatest_info.suite.pre_suite)
                as libc::c_double
                / (1.0f64 * 1000000 as libc::c_int as __clock_t as libc::c_double),
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn greatest_suite_post() {
    greatest_info.suite.post_suite = clock();
    if greatest_info.suite.post_suite == -(1 as libc::c_int) as clock_t {
        fprintf(
            stdout,
            b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
            b"greatest_info.suite.post_suite\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    report_suite();
}
unsafe extern "C" fn greatest_name_match(
    mut name: *const libc::c_char,
    mut filter: *const libc::c_char,
    mut res_if_none: libc::c_int,
) -> libc::c_int {
    let mut offset: size_t = 0 as libc::c_int as size_t;
    let mut filter_len: size_t = if !filter.is_null() {
        strlen(filter)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if filter_len == 0 as libc::c_int as libc::c_ulong {
        return res_if_none;
    }
    while *name.offset(offset as isize) as libc::c_int != '\0' as i32 {
        if *name.offset(offset as isize) as libc::c_int
            == *filter.offset(0 as libc::c_int as isize) as libc::c_int
        {
            if 0 as libc::c_int
                == strncmp(&*name.offset(offset as isize), filter, filter_len)
            {
                return 1 as libc::c_int;
            }
        }
        offset = offset.wrapping_add(1);
        offset;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn greatest_string_equal_cb(
    mut exp: *const libc::c_void,
    mut got: *const libc::c_void,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut size: *mut size_t = udata as *mut size_t;
    return if !size.is_null() {
        (0 as libc::c_int
            == strncmp(exp as *const libc::c_char, got as *const libc::c_char, *size))
            as libc::c_int
    } else {
        (0 as libc::c_int
            == strcmp(exp as *const libc::c_char, got as *const libc::c_char))
            as libc::c_int
    };
}
unsafe extern "C" fn update_counts_and_reset_suite() {
    greatest_info.setup = None;
    greatest_info.setup_udata = 0 as *mut libc::c_void;
    greatest_info.teardown = None;
    greatest_info.teardown_udata = 0 as *mut libc::c_void;
    greatest_info
        .passed = (greatest_info.passed).wrapping_add(greatest_info.suite.passed);
    greatest_info
        .failed = (greatest_info.failed).wrapping_add(greatest_info.suite.failed);
    greatest_info
        .skipped = (greatest_info.skipped).wrapping_add(greatest_info.suite.skipped);
    greatest_info
        .tests_run = (greatest_info.tests_run)
        .wrapping_add(greatest_info.suite.tests_run);
    memset(
        &mut greatest_info.suite as *mut greatest_suite_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<greatest_suite_info>() as libc::c_ulong,
    );
    greatest_info.col = 0 as libc::c_int as libc::c_uint;
}
pub static mut greatest_type_info_memory: greatest_type_info = unsafe {
    {
        let mut init = greatest_type_info {
            equal: Some(
                greatest_memory_equal_cb
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            print: Some(
                greatest_memory_printf_cb
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        };
        init
    }
};
pub unsafe extern "C" fn greatest_suite_pre(
    mut suite_name: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut greatest_prng = &mut *(greatest_info.prng)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut greatest_prng;
    if greatest_name_match(suite_name, greatest_info.suite_filter, 1 as libc::c_int) == 0
        || greatest_info.flags as libc::c_int & GREATEST_FLAG_FIRST_FAIL as libc::c_int
            != 0 && greatest_info.failed > 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if (*p).random_order != 0 {
        (*p).count = ((*p).count).wrapping_add(1);
        (*p).count;
        if (*p).initialized == 0
            || ((*p).count).wrapping_sub(1 as libc::c_int as libc::c_ulong) != (*p).state
        {
            return 0 as libc::c_int;
        }
    }
    (*p).count_run = ((*p).count_run).wrapping_add(1);
    (*p).count_run;
    update_counts_and_reset_suite();
    fprintf(
        stdout,
        b"\n* Suite %s:\n\0" as *const u8 as *const libc::c_char,
        suite_name,
    );
    greatest_info.suite.pre_suite = clock();
    if greatest_info.suite.pre_suite == -(1 as libc::c_int) as clock_t {
        fprintf(
            stdout,
            b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
            b"greatest_info.suite.pre_suite\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn greatest_do_skip(mut name: *const libc::c_char) {
    if greatest_info.verbosity as libc::c_int > 0 as libc::c_int {
        fprintf(
            stdout,
            b"SKIP %s: %s\0" as *const u8 as *const libc::c_char,
            name,
            if !(greatest_info.msg).is_null() {
                greatest_info.msg
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        fprintf(stdout, b"s\0" as *const u8 as *const libc::c_char);
    }
    greatest_info.suite.skipped = (greatest_info.suite.skipped).wrapping_add(1);
    greatest_info.suite.skipped;
}
pub unsafe extern "C" fn greatest_do_fail(mut name: *const libc::c_char) {
    if greatest_info.verbosity as libc::c_int > 0 as libc::c_int {
        fprintf(
            stdout,
            b"FAIL %s: %s (%s:%u)\0" as *const u8 as *const libc::c_char,
            name,
            if !(greatest_info.msg).is_null() {
                greatest_info.msg
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            greatest_info.fail_file,
            greatest_info.fail_line,
        );
    } else {
        fprintf(stdout, b"F\0" as *const u8 as *const libc::c_char);
        greatest_info.col = (greatest_info.col).wrapping_add(1);
        greatest_info.col;
        if greatest_info.col != 0 as libc::c_int as libc::c_uint {
            fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
            greatest_info.col = 0 as libc::c_int as libc::c_uint;
        }
        fprintf(
            stdout,
            b"FAIL %s: %s (%s:%u)\n\0" as *const u8 as *const libc::c_char,
            name,
            if !(greatest_info.msg).is_null() {
                greatest_info.msg
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            greatest_info.fail_file,
            greatest_info.fail_line,
        );
    }
    greatest_info.suite.failed = (greatest_info.suite.failed).wrapping_add(1);
    greatest_info.suite.failed;
}
unsafe extern "C" fn greatest_memory_printf_cb(
    mut t: *const libc::c_void,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut env: *mut greatest_memory_cmp_env = udata as *mut greatest_memory_cmp_env;
    let mut buf: *const libc::c_uchar = t as *const libc::c_uchar;
    let mut diff_mark: libc::c_uchar = ' ' as i32 as libc::c_uchar;
    let mut out: *mut FILE = stdout;
    let mut i: size_t = 0;
    let mut line_i: size_t = 0;
    let mut line_len: size_t = 0 as libc::c_int as size_t;
    let mut len: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < (*env).size {
        diff_mark = ' ' as i32 as libc::c_uchar;
        line_len = ((*env).size).wrapping_sub(i);
        if line_len > 16 as libc::c_int as libc::c_ulong {
            line_len = 16 as libc::c_int as size_t;
        }
        line_i = i;
        while line_i < i.wrapping_add(line_len) {
            if *((*env).exp).offset(line_i as isize) as libc::c_int
                != *((*env).got).offset(line_i as isize) as libc::c_int
            {
                diff_mark = 'X' as i32 as libc::c_uchar;
            }
            line_i = line_i.wrapping_add(1);
            line_i;
        }
        len
            += fprintf(
                out,
                b"\n%04x %c \0" as *const u8 as *const libc::c_char,
                i as libc::c_uint,
                diff_mark as libc::c_int,
            );
        line_i = i;
        while line_i < i.wrapping_add(line_len) {
            let mut m: libc::c_int = (*((*env).exp).offset(line_i as isize)
                as libc::c_int == *((*env).got).offset(line_i as isize) as libc::c_int)
                as libc::c_int;
            len
                += fprintf(
                    out,
                    b"%02x%c\0" as *const u8 as *const libc::c_char,
                    *buf.offset(line_i as isize) as libc::c_int,
                    if m != 0 { ' ' as i32 } else { '<' as i32 },
                );
            line_i = line_i.wrapping_add(1);
            line_i;
        }
        line_i = 0 as libc::c_int as size_t;
        while line_i < (16 as libc::c_int as libc::c_ulong).wrapping_sub(line_len) {
            len += fprintf(out, b"   \0" as *const u8 as *const libc::c_char);
            line_i = line_i.wrapping_add(1);
            line_i;
        }
        fprintf(out, b" \0" as *const u8 as *const libc::c_char);
        line_i = i;
        while line_i < i.wrapping_add(line_len) {
            let mut c: libc::c_uchar = *buf.offset(line_i as isize);
            len
                += fprintf(
                    out,
                    b"%c\0" as *const u8 as *const libc::c_char,
                    if *(*__ctype_b_loc()).offset(c as libc::c_int as isize)
                        as libc::c_int
                        & _ISprint as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        c as libc::c_int
                    } else {
                        '.' as i32
                    },
                );
            line_i = line_i.wrapping_add(1);
            line_i;
        }
        i = (i as libc::c_ulong).wrapping_add(line_len) as size_t as size_t;
    }
    len += fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    return len;
}
pub unsafe extern "C" fn greatest_do_pass(mut name: *const libc::c_char) {
    if greatest_info.verbosity as libc::c_int > 0 as libc::c_int {
        fprintf(
            stdout,
            b"PASS %s: %s\0" as *const u8 as *const libc::c_char,
            name,
            if !(greatest_info.msg).is_null() {
                greatest_info.msg
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
    } else {
        fprintf(stdout, b".\0" as *const u8 as *const libc::c_char);
    }
    greatest_info.suite.passed = (greatest_info.suite.passed).wrapping_add(1);
    greatest_info.suite.passed;
}
unsafe extern "C" fn greatest_memory_equal_cb(
    mut exp: *const libc::c_void,
    mut got: *const libc::c_void,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut env: *mut greatest_memory_cmp_env = udata as *mut greatest_memory_cmp_env;
    return (0 as libc::c_int == memcmp(exp, got, (*env).size)) as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    memset(
        &mut greatest_info as *mut greatest_run_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<greatest_run_info>() as libc::c_ulong,
    );
    greatest_info.width = 72 as libc::c_int as libc::c_uint;
    greatest_info.begin = clock();
    if greatest_info.begin == -(1 as libc::c_int) as clock_t {
        fprintf(
            stdout,
            b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
            b"greatest_info.begin\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    greatest_parse_options(argc, argv);
    greatest_run_suite(
        Some(match_suite as unsafe extern "C" fn() -> ()),
        b"match_suite\0" as *const u8 as *const libc::c_char,
    );
    greatest_run_suite(
        Some(choices_suite as unsafe extern "C" fn() -> ()),
        b"choices_suite\0" as *const u8 as *const libc::c_char,
    );
    greatest_run_suite(
        Some(properties_suite as unsafe extern "C" fn() -> ()),
        b"properties_suite\0" as *const u8 as *const libc::c_char,
    );
    if greatest_info.flags as libc::c_int & GREATEST_FLAG_LIST_ONLY as libc::c_int == 0 {
        update_counts_and_reset_suite();
        greatest_info.end = clock();
        if greatest_info.end == -(1 as libc::c_int) as clock_t {
            fprintf(
                stdout,
                b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
                b"greatest_info.end\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        fprintf(
            stdout,
            b"\nTotal: %u test%s\0" as *const u8 as *const libc::c_char,
            greatest_info.tests_run,
            if greatest_info.tests_run == 1 as libc::c_int as libc::c_uint {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
        );
        fprintf(
            stdout,
            b" (%lu ticks, %.3f sec)\0" as *const u8 as *const libc::c_char,
            (greatest_info.end as libc::c_ulong)
                .wrapping_sub(greatest_info.begin as libc::c_ulong),
            (greatest_info.end - greatest_info.begin) as libc::c_double
                / (1.0f64 * 1000000 as libc::c_int as __clock_t as libc::c_double),
        );
        fprintf(
            stdout,
            b", %u assertion%s\n\0" as *const u8 as *const libc::c_char,
            greatest_info.assertions,
            if greatest_info.assertions == 1 as libc::c_int as libc::c_uint {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
        );
        fprintf(
            stdout,
            b"Pass: %u, fail: %u, skip: %u.\n\0" as *const u8 as *const libc::c_char,
            greatest_info.passed,
            greatest_info.failed,
            greatest_info.skipped,
        );
    }
    return if greatest_all_passed() != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
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

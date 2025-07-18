use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn options_init(options: *mut options_t);
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn asprintf(
        __ptr: *mut *mut libc::c_char,
        __fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn choices_next(c: *mut choices_t);
    fn choices_prev(c: *mut choices_t);
    fn choices_get(c: *mut choices_t, n: size_t) -> *const libc::c_char;
    fn choices_search(c: *mut choices_t, search: *const libc::c_char);
    fn choices_add(c: *mut choices_t, choice: *const libc::c_char);
    fn choices_destroy(c: *mut choices_t);
    fn choices_init(c: *mut choices_t, options: *mut options_t);
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn GREATEST_SET_TEARDOWN_CB(
        cb: Option::<greatest_teardown_cb>,
        udata: *mut libc::c_void,
    );
    static mut greatest_type_info_string: greatest_type_info;
    static mut greatest_info: greatest_run_info;
    fn greatest_test_pre(name: *const libc::c_char) -> libc::c_int;
    fn greatest_test_post(name: *const libc::c_char, res: libc::c_int);
    fn greatest_do_assert_equal_t(
        exp: *const libc::c_void,
        got: *const libc::c_void,
        type_info: *mut greatest_type_info,
        udata: *mut libc::c_void,
    ) -> libc::c_int;
    fn GREATEST_SET_SETUP_CB(cb: Option::<greatest_setup_cb>, udata: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options_t {
    pub benchmark: libc::c_int,
    pub filter: *const libc::c_char,
    pub init_search: *const libc::c_char,
    pub tty_filename: *const libc::c_char,
    pub show_scores: libc::c_int,
    pub num_lines: libc::c_uint,
    pub scrolloff: libc::c_uint,
    pub prompt: *const libc::c_char,
    pub workers: libc::c_uint,
    pub input_delimiter: libc::c_char,
    pub show_info: libc::c_int,
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
pub type score_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scored_result {
    pub score: score_t,
    pub str_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct choices_t {
    pub buffer: *mut libc::c_char,
    pub buffer_size: size_t,
    pub capacity: size_t,
    pub size: size_t,
    pub strings: *mut *const libc::c_char,
    pub results: *mut scored_result,
    pub available: size_t,
    pub selection: size_t,
    pub worker_count: libc::c_uint,
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
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
pub type greatest_test_res = libc::c_int;
pub const GREATEST_TEST_RES_SKIP: greatest_test_res = 1;
pub const GREATEST_TEST_RES_FAIL: greatest_test_res = -1;
pub const GREATEST_TEST_RES_PASS: greatest_test_res = 0;
static mut default_options: options_t = options_t {
    benchmark: 0,
    filter: 0 as *const libc::c_char,
    init_search: 0 as *const libc::c_char,
    tty_filename: 0 as *const libc::c_char,
    show_scores: 0,
    num_lines: 0,
    scrolloff: 0,
    prompt: 0 as *const libc::c_char,
    workers: 0,
    input_delimiter: 0,
    show_info: 0,
};
static mut choices: choices_t = choices_t {
    buffer: 0 as *const libc::c_char as *mut libc::c_char,
    buffer_size: 0,
    capacity: 0,
    size: 0,
    strings: 0 as *const *const libc::c_char as *mut *const libc::c_char,
    results: 0 as *const scored_result as *mut scored_result,
    available: 0,
    selection: 0,
    worker_count: 0,
};
unsafe extern "C" fn setup(mut udata: *mut libc::c_void) {
    options_init(&mut default_options);
    choices_init(&mut choices, &mut default_options);
}
unsafe extern "C" fn teardown(mut udata: *mut libc::c_void) {
    choices_destroy(&mut choices);
}
unsafe extern "C" fn test_choices_empty() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.size {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.size);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 29 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.size)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 30 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 31 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_prev(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 34 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_next(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 37 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn test_choices_1() -> greatest_test_res {
    choices_add(&mut choices, b"tags\0" as *const u8 as *const libc::c_char);
    choices_search(&mut choices, b"\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 1 as libc::c_int as size_t != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 46 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 47 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_search(&mut choices, b"t\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 1 as libc::c_int as size_t != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 50 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 51 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_prev(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 54 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_next(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 57 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if strcmp(
        choices_get(&mut choices, 0 as libc::c_int as size_t),
        b"tags\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 59 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"!strcmp(choices_get(&choices, 0), \"tags\")\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(choices_get(&mut choices, 1 as libc::c_int as size_t)).is_null() {
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 60 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"NULL != choices_get(&choices, 1)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn test_choices_2() -> greatest_test_res {
    choices_add(&mut choices, b"tags\0" as *const u8 as *const libc::c_char);
    choices_add(&mut choices, b"test\0" as *const u8 as *const libc::c_char);
    choices_search(&mut choices, b"\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 71 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 2 as libc::c_int as size_t != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 72 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(2) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_next(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 1 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 75 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_next(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 77 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_prev(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 1 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 80 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_prev(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 82 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_search(&mut choices, b"te\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 1 as libc::c_int as size_t != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 86 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 87 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut type_info: *mut greatest_type_info = &mut greatest_type_info_string;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_do_assert_equal_t(
        b"test\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        choices_get(&mut choices, 0 as libc::c_int as size_t) as *const libc::c_void,
        type_info,
        0 as *mut libc::c_void,
    ) == 0
    {
        if type_info.is_null() || ((*type_info).equal).is_none() {
            greatest_info
                .fail_file = b"test/test_choices.c\0" as *const u8
                as *const libc::c_char;
            greatest_info.fail_line = 88 as libc::c_int as libc::c_uint;
            greatest_info
                .msg = b"type_info->equal callback missing!\0" as *const u8
                as *const libc::c_char;
            return GREATEST_TEST_RES_FAIL;
        } else {
            greatest_info
                .fail_file = b"test/test_choices.c\0" as *const u8
                as *const libc::c_char;
            greatest_info.fail_line = 88 as libc::c_int as libc::c_uint;
            greatest_info
                .msg = b"\"test\" != choices_get(&choices, 0)\0" as *const u8
                as *const libc::c_char;
            return GREATEST_TEST_RES_FAIL;
        }
    }
    choices_next(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 91 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_prev(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 94 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_search(&mut choices, b"foobar\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 98 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 99 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_search(&mut choices, b"ts\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 2 as libc::c_int as size_t != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 103 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(2) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 104 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut type_info_0: *mut greatest_type_info = &mut greatest_type_info_string;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_do_assert_equal_t(
        b"test\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        choices_get(&mut choices, 0 as libc::c_int as size_t) as *const libc::c_void,
        type_info_0,
        0 as *mut libc::c_void,
    ) == 0
    {
        if type_info_0.is_null() || ((*type_info_0).equal).is_none() {
            greatest_info
                .fail_file = b"test/test_choices.c\0" as *const u8
                as *const libc::c_char;
            greatest_info.fail_line = 105 as libc::c_int as libc::c_uint;
            greatest_info
                .msg = b"type_info->equal callback missing!\0" as *const u8
                as *const libc::c_char;
            return GREATEST_TEST_RES_FAIL;
        } else {
            greatest_info
                .fail_file = b"test/test_choices.c\0" as *const u8
                as *const libc::c_char;
            greatest_info.fail_line = 105 as libc::c_int as libc::c_uint;
            greatest_info
                .msg = b"\"test\" != choices_get(&choices, 0)\0" as *const u8
                as *const libc::c_char;
            return GREATEST_TEST_RES_FAIL;
        }
    }
    let mut type_info_1: *mut greatest_type_info = &mut greatest_type_info_string;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_do_assert_equal_t(
        b"tags\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        choices_get(&mut choices, 1 as libc::c_int as size_t) as *const libc::c_void,
        type_info_1,
        0 as *mut libc::c_void,
    ) == 0
    {
        if type_info_1.is_null() || ((*type_info_1).equal).is_none() {
            greatest_info
                .fail_file = b"test/test_choices.c\0" as *const u8
                as *const libc::c_char;
            greatest_info.fail_line = 106 as libc::c_int as libc::c_uint;
            greatest_info
                .msg = b"type_info->equal callback missing!\0" as *const u8
                as *const libc::c_char;
            return GREATEST_TEST_RES_FAIL;
        } else {
            greatest_info
                .fail_file = b"test/test_choices.c\0" as *const u8
                as *const libc::c_char;
            greatest_info.fail_line = 106 as libc::c_int as libc::c_uint;
            greatest_info
                .msg = b"\"tags\" != choices_get(&choices, 1)\0" as *const u8
                as *const libc::c_char;
            return GREATEST_TEST_RES_FAIL;
        }
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn test_choices_without_search() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 114 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 115 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.size {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.size);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 116 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.size)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(choices_get(&mut choices, 0 as libc::c_int as size_t)).is_null() {
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 117 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"NULL != choices_get(&choices, 0)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_add(&mut choices, b"test\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 121 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 122 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 1 as libc::c_int as size_t != choices.size {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.size);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 123 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.size)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(choices_get(&mut choices, 0 as libc::c_int as size_t)).is_null() {
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 124 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"NULL != choices_get(&choices, 0)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn test_choices_unicode() -> greatest_test_res {
    choices_add(
        &mut choices,
        b"Edmund Husserl - M\xC3\xA9ditations cart\xC3\xA9siennes - Introduction a la ph\xC3\xA9nom\xC3\xA9nologie.pdf\0"
            as *const u8 as *const libc::c_char,
    );
    choices_search(&mut choices, b"e\0" as *const u8 as *const libc::c_char);
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn test_choices_large_input() -> greatest_test_res {
    let N: libc::c_int = 100000 as libc::c_int;
    let mut strings: [*mut libc::c_char; 100000] = [0 as *mut libc::c_char; 100000];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < N {
        asprintf(
            &mut *strings.as_mut_ptr().offset(i as isize) as *mut *mut libc::c_char,
            b"%i\0" as *const u8 as *const libc::c_char,
            i,
        );
        choices_add(&mut choices, strings[i as usize]);
        i += 1;
        i;
    }
    choices_search(&mut choices, b"12\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 8146 as libc::c_int as size_t != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            8146 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 149 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(8146) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut type_info: *mut greatest_type_info = &mut greatest_type_info_string;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_do_assert_equal_t(
        b"12\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        choices_get(&mut choices, 0 as libc::c_int as size_t) as *const libc::c_void,
        type_info,
        0 as *mut libc::c_void,
    ) == 0
    {
        if type_info.is_null() || ((*type_info).equal).is_none() {
            greatest_info
                .fail_file = b"test/test_choices.c\0" as *const u8
                as *const libc::c_char;
            greatest_info.fail_line = 151 as libc::c_int as libc::c_uint;
            greatest_info
                .msg = b"type_info->equal callback missing!\0" as *const u8
                as *const libc::c_char;
            return GREATEST_TEST_RES_FAIL;
        } else {
            greatest_info
                .fail_file = b"test/test_choices.c\0" as *const u8
                as *const libc::c_char;
            greatest_info.fail_line = 151 as libc::c_int as libc::c_uint;
            greatest_info
                .msg = b"\"12\" != choices_get(&choices, 0)\0" as *const u8
                as *const libc::c_char;
            return GREATEST_TEST_RES_FAIL;
        }
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < N {
        free(strings[i_0 as usize] as *mut libc::c_void);
        i_0 += 1;
        i_0;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
pub unsafe extern "C" fn choices_suite() {
    GREATEST_SET_SETUP_CB(
        Some(setup as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
    GREATEST_SET_TEARDOWN_CB(
        Some(teardown as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
    if greatest_test_pre(b"test_choices_empty\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res: greatest_test_res = _setjmp((greatest_info.jump_dest).as_mut_ptr())
            as greatest_test_res;
        if res as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res = test_choices_empty();
        }
        greatest_test_post(
            b"test_choices_empty\0" as *const u8 as *const libc::c_char,
            res as libc::c_int,
        );
    }
    if greatest_test_pre(b"test_choices_1\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_0: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_0 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_0 = test_choices_1();
        }
        greatest_test_post(
            b"test_choices_1\0" as *const u8 as *const libc::c_char,
            res_0 as libc::c_int,
        );
    }
    if greatest_test_pre(b"test_choices_2\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_1: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_1 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_1 = test_choices_2();
        }
        greatest_test_post(
            b"test_choices_2\0" as *const u8 as *const libc::c_char,
            res_1 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"test_choices_without_search\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_2: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_2 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_2 = test_choices_without_search();
        }
        greatest_test_post(
            b"test_choices_without_search\0" as *const u8 as *const libc::c_char,
            res_2 as libc::c_int,
        );
    }
    if greatest_test_pre(b"test_choices_unicode\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_3: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_3 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_3 = test_choices_unicode();
        }
        greatest_test_post(
            b"test_choices_unicode\0" as *const u8 as *const libc::c_char,
            res_3 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"test_choices_large_input\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_4: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_4 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_4 = test_choices_large_input();
        }
        greatest_test_post(
            b"test_choices_large_input\0" as *const u8 as *const libc::c_char,
            res_4 as libc::c_int,
        );
    }
}

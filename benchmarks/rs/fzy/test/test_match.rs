use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[link_name = "match"]
    fn match_0(needle: *const libc::c_char, haystack: *const libc::c_char) -> score_t;
    fn match_positions(
        needle: *const libc::c_char,
        haystack: *const libc::c_char,
        positions: *mut size_t,
    ) -> score_t;
    fn has_match(
        needle: *const libc::c_char,
        haystack: *const libc::c_char,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    static mut greatest_info: greatest_run_info;
    fn greatest_test_pre(name: *const libc::c_char) -> libc::c_int;
    fn greatest_test_post(name: *const libc::c_char, res: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
pub type score_t = libc::c_double;
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
pub type greatest_setup_cb = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type greatest_teardown_cb = unsafe extern "C" fn(*mut libc::c_void) -> ();
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
unsafe extern "C" fn exact_match_should_return_true() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if has_match(
        b"a\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 14 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"a\", \"a\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn partial_match_should_return_true() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if has_match(
        b"a\0" as *const u8 as *const libc::c_char,
        b"ab\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 19 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"a\", \"ab\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if has_match(
        b"a\0" as *const u8 as *const libc::c_char,
        b"ba\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 20 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"a\", \"ba\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn match_with_delimiters_in_between() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if has_match(
        b"abc\0" as *const u8 as *const libc::c_char,
        b"a|b|c\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 25 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"abc\", \"a|b|c\")\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn non_match_should_return_false() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if has_match(
        b"a\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 30 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"!has_match(\"a\", \"\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if has_match(
        b"a\0" as *const u8 as *const libc::c_char,
        b"b\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 31 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"!has_match(\"a\", \"b\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if has_match(
        b"ass\0" as *const u8 as *const libc::c_char,
        b"tags\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 32 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"!has_match(\"ass\", \"tags\")\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn empty_query_should_always_match() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if has_match(
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 38 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"\", \"\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if has_match(
        b"\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 39 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"\", \"a\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_starts_of_words() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(match_0(
        b"amor\0" as *const u8 as *const libc::c_char,
        b"app/models/order\0" as *const u8 as *const libc::c_char,
    )
        > match_0(
            b"amor\0" as *const u8 as *const libc::c_char,
            b"app/models/zrder\0" as *const u8 as *const libc::c_char,
        ))
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 47 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"match(\"amor\", \"app/models/order\") > match(\"amor\", \"app/models/zrder\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_consecutive_letters() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(match_0(
        b"amo\0" as *const u8 as *const libc::c_char,
        b"app/m/foo\0" as *const u8 as *const libc::c_char,
    )
        < match_0(
            b"amo\0" as *const u8 as *const libc::c_char,
            b"app/models/foo\0" as *const u8 as *const libc::c_char,
        ))
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 53 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"match(\"amo\", \"app/m/foo\") < match(\"amo\", \"app/models/foo\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_contiguous_over_letter_following_period() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(match_0(
        b"gemfil\0" as *const u8 as *const libc::c_char,
        b"Gemfile.lock\0" as *const u8 as *const libc::c_char,
    )
        < match_0(
            b"gemfil\0" as *const u8 as *const libc::c_char,
            b"Gemfile\0" as *const u8 as *const libc::c_char,
        ))
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 59 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"match(\"gemfil\", \"Gemfile.lock\") < match(\"gemfil\", \"Gemfile\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_shorter_matches() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(match_0(
        b"abce\0" as *const u8 as *const libc::c_char,
        b"abcdef\0" as *const u8 as *const libc::c_char,
    )
        > match_0(
            b"abce\0" as *const u8 as *const libc::c_char,
            b"abc de\0" as *const u8 as *const libc::c_char,
        ))
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 64 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"match(\"abce\", \"abcdef\") > match(\"abce\", \"abc de\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(match_0(
        b"abc\0" as *const u8 as *const libc::c_char,
        b"    a b c \0" as *const u8 as *const libc::c_char,
    )
        > match_0(
            b"abc\0" as *const u8 as *const libc::c_char,
            b" a  b  c \0" as *const u8 as *const libc::c_char,
        ))
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 65 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"match(\"abc\", \"    a b c \") > match(\"abc\", \" a  b  c \")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(match_0(
        b"abc\0" as *const u8 as *const libc::c_char,
        b" a b c    \0" as *const u8 as *const libc::c_char,
    )
        > match_0(
            b"abc\0" as *const u8 as *const libc::c_char,
            b" a  b  c \0" as *const u8 as *const libc::c_char,
        ))
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 66 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"match(\"abc\", \" a b c    \") > match(\"abc\", \" a  b  c \")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_shorter_candidates() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(match_0(
        b"test\0" as *const u8 as *const libc::c_char,
        b"tests\0" as *const u8 as *const libc::c_char,
    )
        > match_0(
            b"test\0" as *const u8 as *const libc::c_char,
            b"testing\0" as *const u8 as *const libc::c_char,
        ))
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 71 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"match(\"test\", \"tests\") > match(\"test\", \"testing\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_start_of_candidate() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if !(match_0(
        b"test\0" as *const u8 as *const libc::c_char,
        b"testing\0" as *const u8 as *const libc::c_char,
    )
        > match_0(
            b"test\0" as *const u8 as *const libc::c_char,
            b"/testing\0" as *const u8 as *const libc::c_char,
        ))
    {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 77 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"match(\"test\", \"testing\") > match(\"test\", \"/testing\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_exact_match() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = ::std::f32::INFINITY as libc::c_double;
    let mut greatest_GOT: libc::c_double = match_0(
        b"abc\0" as *const u8 as *const libc::c_char,
        b"abc\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP > greatest_GOT && greatest_EXP - greatest_GOT > greatest_TOL
        || greatest_EXP < greatest_GOT && greatest_GOT - greatest_EXP > greatest_TOL
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 83 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"((__builtin_inff ())) != (match(\"abc\", \"abc\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_0: libc::c_double = ::std::f32::INFINITY as libc::c_double;
    let mut greatest_GOT_0: libc::c_double = match_0(
        b"aBc\0" as *const u8 as *const libc::c_char,
        b"abC\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_0: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_0 > greatest_GOT_0
        && greatest_EXP_0 - greatest_GOT_0 > greatest_TOL_0
        || greatest_EXP_0 < greatest_GOT_0
            && greatest_GOT_0 - greatest_EXP_0 > greatest_TOL_0
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_0,
            greatest_TOL_0,
            greatest_GOT_0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 84 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"((__builtin_inff ())) != (match(\"aBc\", \"abC\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_empty_query() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = -::std::f32::INFINITY as libc::c_double;
    let mut greatest_GOT: libc::c_double = match_0(
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP > greatest_GOT && greatest_EXP - greatest_GOT > greatest_TOL
        || greatest_EXP < greatest_GOT && greatest_GOT - greatest_EXP > greatest_TOL
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 90 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(\"\", \"\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_0: libc::c_double = -::std::f32::INFINITY as libc::c_double;
    let mut greatest_GOT_0: libc::c_double = match_0(
        b"\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_0: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_0 > greatest_GOT_0
        && greatest_EXP_0 - greatest_GOT_0 > greatest_TOL_0
        || greatest_EXP_0 < greatest_GOT_0
            && greatest_GOT_0 - greatest_EXP_0 > greatest_TOL_0
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_0,
            greatest_TOL_0,
            greatest_GOT_0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 91 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(\"\", \"a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_1: libc::c_double = -::std::f32::INFINITY as libc::c_double;
    let mut greatest_GOT_1: libc::c_double = match_0(
        b"\0" as *const u8 as *const libc::c_char,
        b"bb\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_1: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_1 > greatest_GOT_1
        && greatest_EXP_1 - greatest_GOT_1 > greatest_TOL_1
        || greatest_EXP_1 < greatest_GOT_1
            && greatest_GOT_1 - greatest_EXP_1 > greatest_TOL_1
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_1,
            greatest_TOL_1,
            greatest_GOT_1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 92 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(\"\", \"bb\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_gaps() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = -0.005f64;
    let mut greatest_GOT: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"*a\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP > greatest_GOT && greatest_EXP - greatest_GOT > greatest_TOL
        || greatest_EXP < greatest_GOT && greatest_GOT - greatest_EXP > greatest_TOL
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 97 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005) != (match(\"a\", \"*a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_0: libc::c_double = -0.005f64
        * 2 as libc::c_int as libc::c_double;
    let mut greatest_GOT_0: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"*ba\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_0: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_0 > greatest_GOT_0
        && greatest_EXP_0 - greatest_GOT_0 > greatest_TOL_0
        || greatest_EXP_0 < greatest_GOT_0
            && greatest_GOT_0 - greatest_EXP_0 > greatest_TOL_0
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_0,
            greatest_TOL_0,
            greatest_GOT_0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 98 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2) != (match(\"a\", \"*ba\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_1: libc::c_double = -0.005f64
        * 2 as libc::c_int as libc::c_double + -0.005f64;
    let mut greatest_GOT_1: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"**a*\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_1: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_1 > greatest_GOT_1
        && greatest_EXP_1 - greatest_GOT_1 > greatest_TOL_1
        || greatest_EXP_1 < greatest_GOT_1
            && greatest_GOT_1 - greatest_EXP_1 > greatest_TOL_1
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_1,
            greatest_TOL_1,
            greatest_GOT_1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 99 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + -0.005) != (match(\"a\", \"**a*\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_2: libc::c_double = -0.005f64
        * 2 as libc::c_int as libc::c_double
        + -0.005f64 * 2 as libc::c_int as libc::c_double;
    let mut greatest_GOT_2: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"**a**\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_2: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_2 > greatest_GOT_2
        && greatest_EXP_2 - greatest_GOT_2 > greatest_TOL_2
        || greatest_EXP_2 < greatest_GOT_2
            && greatest_GOT_2 - greatest_EXP_2 > greatest_TOL_2
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_2,
            greatest_TOL_2,
            greatest_GOT_2,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 100 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + -0.005*2) != (match(\"a\", \"**a**\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_3: libc::c_double = -0.005f64
        * 2 as libc::c_int as libc::c_double + 1.0f64
        + -0.005f64 * 2 as libc::c_int as libc::c_double;
    let mut greatest_GOT_3: libc::c_double = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        b"**aa**\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_3: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_3 > greatest_GOT_3
        && greatest_EXP_3 - greatest_GOT_3 > greatest_TOL_3
        || greatest_EXP_3 < greatest_GOT_3
            && greatest_GOT_3 - greatest_EXP_3 > greatest_TOL_3
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_3,
            greatest_TOL_3,
            greatest_GOT_3,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 101 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + 1.0 + -0.005*2) != (match(\"aa\", \"**aa**\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_4: libc::c_double = -0.005f64 + -0.005f64 + -0.01f64 + -0.005f64
        + -0.005f64;
    let mut greatest_GOT_4: libc::c_double = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        b"**a*a**\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_4: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_4 > greatest_GOT_4
        && greatest_EXP_4 - greatest_GOT_4 > greatest_TOL_4
        || greatest_EXP_4 < greatest_GOT_4
            && greatest_GOT_4 - greatest_EXP_4 > greatest_TOL_4
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_4,
            greatest_TOL_4,
            greatest_GOT_4,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 102 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + -0.005 + -0.01 + -0.005 + -0.005) != (match(\"aa\", \"**a*a**\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_consecutive() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = -0.005f64 + 1.0f64;
    let mut greatest_GOT: libc::c_double = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        b"*aa\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP > greatest_GOT && greatest_EXP - greatest_GOT > greatest_TOL
        || greatest_EXP < greatest_GOT && greatest_GOT - greatest_EXP > greatest_TOL
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 107 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + 1.0) != (match(\"aa\", \"*aa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_0: libc::c_double = -0.005f64
        + 1.0f64 * 2 as libc::c_int as libc::c_double;
    let mut greatest_GOT_0: libc::c_double = match_0(
        b"aaa\0" as *const u8 as *const libc::c_char,
        b"*aaa\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_0: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_0 > greatest_GOT_0
        && greatest_EXP_0 - greatest_GOT_0 > greatest_TOL_0
        || greatest_EXP_0 < greatest_GOT_0
            && greatest_GOT_0 - greatest_EXP_0 > greatest_TOL_0
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_0,
            greatest_TOL_0,
            greatest_GOT_0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 108 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + 1.0*2) != (match(\"aaa\", \"*aaa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_1: libc::c_double = -0.005f64 + -0.01f64 + 1.0f64;
    let mut greatest_GOT_1: libc::c_double = match_0(
        b"aaa\0" as *const u8 as *const libc::c_char,
        b"*a*aa\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_1: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_1 > greatest_GOT_1
        && greatest_EXP_1 - greatest_GOT_1 > greatest_TOL_1
        || greatest_EXP_1 < greatest_GOT_1
            && greatest_GOT_1 - greatest_EXP_1 > greatest_TOL_1
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_1,
            greatest_TOL_1,
            greatest_GOT_1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 109 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + -0.01 + 1.0) != (match(\"aaa\", \"*a*aa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_slash() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = -0.005f64 + 0.9f64;
    let mut greatest_GOT: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"/a\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP > greatest_GOT && greatest_EXP - greatest_GOT > greatest_TOL
        || greatest_EXP < greatest_GOT && greatest_GOT - greatest_EXP > greatest_TOL
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 114 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + 0.9) != (match(\"a\", \"/a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_0: libc::c_double = -0.005f64
        * 2 as libc::c_int as libc::c_double + 0.9f64;
    let mut greatest_GOT_0: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"*/a\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_0: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_0 > greatest_GOT_0
        && greatest_EXP_0 - greatest_GOT_0 > greatest_TOL_0
        || greatest_EXP_0 < greatest_GOT_0
            && greatest_GOT_0 - greatest_EXP_0 > greatest_TOL_0
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_0,
            greatest_TOL_0,
            greatest_GOT_0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 115 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + 0.9) != (match(\"a\", \"*/a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_1: libc::c_double = -0.005f64
        * 2 as libc::c_int as libc::c_double + 0.9f64 + 1.0f64;
    let mut greatest_GOT_1: libc::c_double = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        b"a/aa\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_1: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_1 > greatest_GOT_1
        && greatest_EXP_1 - greatest_GOT_1 > greatest_TOL_1
        || greatest_EXP_1 < greatest_GOT_1
            && greatest_GOT_1 - greatest_EXP_1 > greatest_TOL_1
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_1,
            greatest_TOL_1,
            greatest_GOT_1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 116 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + 0.9 + 1.0) != (match(\"aa\", \"a/aa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_capital() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = -0.005f64 + 0.7f64;
    let mut greatest_GOT: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"bA\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP > greatest_GOT && greatest_EXP - greatest_GOT > greatest_TOL
        || greatest_EXP < greatest_GOT && greatest_GOT - greatest_EXP > greatest_TOL
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 121 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + 0.7) != (match(\"a\", \"bA\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_0: libc::c_double = -0.005f64
        * 2 as libc::c_int as libc::c_double + 0.7f64;
    let mut greatest_GOT_0: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"baA\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_0: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_0 > greatest_GOT_0
        && greatest_EXP_0 - greatest_GOT_0 > greatest_TOL_0
        || greatest_EXP_0 < greatest_GOT_0
            && greatest_GOT_0 - greatest_EXP_0 > greatest_TOL_0
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_0,
            greatest_TOL_0,
            greatest_GOT_0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 122 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + 0.7) != (match(\"a\", \"baA\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_1: libc::c_double = -0.005f64
        * 2 as libc::c_int as libc::c_double + 0.7f64 + 1.0f64;
    let mut greatest_GOT_1: libc::c_double = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        b"baAa\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_1: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_1 > greatest_GOT_1
        && greatest_EXP_1 - greatest_GOT_1 > greatest_TOL_1
        || greatest_EXP_1 < greatest_GOT_1
            && greatest_GOT_1 - greatest_EXP_1 > greatest_TOL_1
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_1,
            greatest_TOL_1,
            greatest_GOT_1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 123 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + 0.7 + 1.0) != (match(\"aa\", \"baAa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_dot() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = -0.005f64 + 0.6f64;
    let mut greatest_GOT: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b".a\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP > greatest_GOT && greatest_EXP - greatest_GOT > greatest_TOL
        || greatest_EXP < greatest_GOT && greatest_GOT - greatest_EXP > greatest_TOL
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 128 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + 0.6) != (match(\"a\", \".a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_0: libc::c_double = -0.005f64
        * 3 as libc::c_int as libc::c_double + 0.6f64;
    let mut greatest_GOT_0: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"*a.a\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_0: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_0 > greatest_GOT_0
        && greatest_EXP_0 - greatest_GOT_0 > greatest_TOL_0
        || greatest_EXP_0 < greatest_GOT_0
            && greatest_GOT_0 - greatest_EXP_0 > greatest_TOL_0
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_0,
            greatest_TOL_0,
            greatest_GOT_0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 129 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*3 + 0.6) != (match(\"a\", \"*a.a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_1: libc::c_double = -0.005f64 + -0.01f64 + 0.6f64;
    let mut greatest_GOT_1: libc::c_double = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"*a.a\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_1: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_1 > greatest_GOT_1
        && greatest_EXP_1 - greatest_GOT_1 > greatest_TOL_1
        || greatest_EXP_1 < greatest_GOT_1
            && greatest_GOT_1 - greatest_EXP_1 > greatest_TOL_1
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_1,
            greatest_TOL_1,
            greatest_GOT_1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 130 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + -0.01 + 0.6) != (match(\"a\", \"*a.a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_long_string() -> greatest_test_res {
    let mut string: [libc::c_char; 4096] = [0; 4096];
    memset(
        string.as_mut_ptr() as *mut libc::c_void,
        'a' as i32,
        (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    string[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = '\0' as i32 as libc::c_char;
    let mut greatest_EXP: libc::c_double = -::std::f32::INFINITY as libc::c_double;
    let mut greatest_GOT: libc::c_double = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        string.as_mut_ptr(),
    );
    let mut greatest_TOL: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP > greatest_GOT && greatest_EXP - greatest_GOT > greatest_TOL
        || greatest_EXP < greatest_GOT && greatest_GOT - greatest_EXP > greatest_TOL
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 139 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(\"aa\", string)) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_0: libc::c_double = -::std::f32::INFINITY as libc::c_double;
    let mut greatest_GOT_0: libc::c_double = match_0(
        string.as_mut_ptr(),
        b"aa\0" as *const u8 as *const libc::c_char,
    );
    let mut greatest_TOL_0: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_0 > greatest_GOT_0
        && greatest_EXP_0 - greatest_GOT_0 > greatest_TOL_0
        || greatest_EXP_0 < greatest_GOT_0
            && greatest_GOT_0 - greatest_EXP_0 > greatest_TOL_0
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_0,
            greatest_TOL_0,
            greatest_GOT_0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 140 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(string, \"aa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    let mut greatest_EXP_1: libc::c_double = -::std::f32::INFINITY as libc::c_double;
    let mut greatest_GOT_1: libc::c_double = match_0(
        string.as_mut_ptr(),
        string.as_mut_ptr(),
    );
    let mut greatest_TOL_1: libc::c_double = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if greatest_EXP_1 > greatest_GOT_1
        && greatest_EXP_1 - greatest_GOT_1 > greatest_TOL_1
        || greatest_EXP_1 < greatest_GOT_1
            && greatest_GOT_1 - greatest_EXP_1 > greatest_TOL_1
    {
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP_1,
            greatest_TOL_1,
            greatest_GOT_1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 141 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(string, string)) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn positions_consecutive() -> greatest_test_res {
    let mut positions: [size_t; 3] = [0; 3];
    match_positions(
        b"amo\0" as *const u8 as *const libc::c_char,
        b"app/models/foo\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 149 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 4 as libc::c_int as size_t != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 150 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(4) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 5 as libc::c_int as size_t != positions[2 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[2 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 151 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(5) != (positions[2])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn positions_start_of_word() -> greatest_test_res {
    let mut positions: [size_t; 4] = [0; 4];
    match_positions(
        b"amor\0" as *const u8 as *const libc::c_char,
        b"app/models/order\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 163 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 4 as libc::c_int as size_t != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 164 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(4) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 11 as libc::c_int as size_t != positions[2 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[2 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 165 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(11) != (positions[2])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 12 as libc::c_int as size_t != positions[3 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[3 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 166 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(12) != (positions[3])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn positions_no_bonuses() -> greatest_test_res {
    let mut positions: [size_t; 2] = [0; 2];
    match_positions(
        b"as\0" as *const u8 as *const libc::c_char,
        b"tags\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 1 as libc::c_int as size_t != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 174 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 3 as libc::c_int as size_t != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 175 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(3) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    match_positions(
        b"as\0" as *const u8 as *const libc::c_char,
        b"examples.txt\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 2 as libc::c_int as size_t != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 178 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(2) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 7 as libc::c_int as size_t != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 179 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(7) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn positions_multiple_candidates_start_of_words() -> greatest_test_res {
    let mut positions: [size_t; 3] = [0; 3];
    match_positions(
        b"abc\0" as *const u8 as *const libc::c_char,
        b"a/a/b/c/c\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 2 as libc::c_int as size_t != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 187 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(2) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 4 as libc::c_int as size_t != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 188 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(4) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 6 as libc::c_int as size_t != positions[2 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[2 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 189 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(6) != (positions[2])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn positions_exact_match() -> greatest_test_res {
    let mut positions: [size_t; 3] = [0; 3];
    match_positions(
        b"foo\0" as *const u8 as *const libc::c_char,
        b"foo\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 0 as libc::c_int as size_t != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 197 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 1 as libc::c_int as size_t != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 198 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if 2 as libc::c_int as size_t != positions[2 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[2 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 199 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(2) != (positions[2])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
pub unsafe extern "C" fn match_suite() {
    if greatest_test_pre(
        b"exact_match_should_return_true\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res: greatest_test_res = _setjmp((greatest_info.jump_dest).as_mut_ptr())
            as greatest_test_res;
        if res as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res = exact_match_should_return_true();
        }
        greatest_test_post(
            b"exact_match_should_return_true\0" as *const u8 as *const libc::c_char,
            res as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"partial_match_should_return_true\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_0: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_0 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_0 = partial_match_should_return_true();
        }
        greatest_test_post(
            b"partial_match_should_return_true\0" as *const u8 as *const libc::c_char,
            res_0 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"empty_query_should_always_match\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_1: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_1 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_1 = empty_query_should_always_match();
        }
        greatest_test_post(
            b"empty_query_should_always_match\0" as *const u8 as *const libc::c_char,
            res_1 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"non_match_should_return_false\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_2: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_2 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_2 = non_match_should_return_false();
        }
        greatest_test_post(
            b"non_match_should_return_false\0" as *const u8 as *const libc::c_char,
            res_2 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"match_with_delimiters_in_between\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_3: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_3 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_3 = match_with_delimiters_in_between();
        }
        greatest_test_post(
            b"match_with_delimiters_in_between\0" as *const u8 as *const libc::c_char,
            res_3 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"should_prefer_starts_of_words\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_4: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_4 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_4 = should_prefer_starts_of_words();
        }
        greatest_test_post(
            b"should_prefer_starts_of_words\0" as *const u8 as *const libc::c_char,
            res_4 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"should_prefer_consecutive_letters\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_5: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_5 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_5 = should_prefer_consecutive_letters();
        }
        greatest_test_post(
            b"should_prefer_consecutive_letters\0" as *const u8 as *const libc::c_char,
            res_5 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"should_prefer_contiguous_over_letter_following_period\0" as *const u8
            as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_6: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_6 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_6 = should_prefer_contiguous_over_letter_following_period();
        }
        greatest_test_post(
            b"should_prefer_contiguous_over_letter_following_period\0" as *const u8
                as *const libc::c_char,
            res_6 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"should_prefer_shorter_matches\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_7: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_7 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_7 = should_prefer_shorter_matches();
        }
        greatest_test_post(
            b"should_prefer_shorter_matches\0" as *const u8 as *const libc::c_char,
            res_7 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"should_prefer_shorter_candidates\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_8: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_8 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_8 = should_prefer_shorter_candidates();
        }
        greatest_test_post(
            b"should_prefer_shorter_candidates\0" as *const u8 as *const libc::c_char,
            res_8 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"should_prefer_start_of_candidate\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_9: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_9 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_9 = should_prefer_start_of_candidate();
        }
        greatest_test_post(
            b"should_prefer_start_of_candidate\0" as *const u8 as *const libc::c_char,
            res_9 as libc::c_int,
        );
    }
    if greatest_test_pre(b"score_exact_match\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_10: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_10 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_10 = score_exact_match();
        }
        greatest_test_post(
            b"score_exact_match\0" as *const u8 as *const libc::c_char,
            res_10 as libc::c_int,
        );
    }
    if greatest_test_pre(b"score_empty_query\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_11: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_11 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_11 = score_empty_query();
        }
        greatest_test_post(
            b"score_empty_query\0" as *const u8 as *const libc::c_char,
            res_11 as libc::c_int,
        );
    }
    if greatest_test_pre(b"score_gaps\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_12: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_12 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_12 = score_gaps();
        }
        greatest_test_post(
            b"score_gaps\0" as *const u8 as *const libc::c_char,
            res_12 as libc::c_int,
        );
    }
    if greatest_test_pre(b"score_consecutive\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_13: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_13 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_13 = score_consecutive();
        }
        greatest_test_post(
            b"score_consecutive\0" as *const u8 as *const libc::c_char,
            res_13 as libc::c_int,
        );
    }
    if greatest_test_pre(b"score_slash\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_14: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_14 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_14 = score_slash();
        }
        greatest_test_post(
            b"score_slash\0" as *const u8 as *const libc::c_char,
            res_14 as libc::c_int,
        );
    }
    if greatest_test_pre(b"score_capital\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_15: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_15 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_15 = score_capital();
        }
        greatest_test_post(
            b"score_capital\0" as *const u8 as *const libc::c_char,
            res_15 as libc::c_int,
        );
    }
    if greatest_test_pre(b"score_dot\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_16: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_16 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_16 = score_dot();
        }
        greatest_test_post(
            b"score_dot\0" as *const u8 as *const libc::c_char,
            res_16 as libc::c_int,
        );
    }
    if greatest_test_pre(b"score_long_string\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_17: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_17 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_17 = score_long_string();
        }
        greatest_test_post(
            b"score_long_string\0" as *const u8 as *const libc::c_char,
            res_17 as libc::c_int,
        );
    }
    if greatest_test_pre(b"positions_consecutive\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_18: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_18 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_18 = positions_consecutive();
        }
        greatest_test_post(
            b"positions_consecutive\0" as *const u8 as *const libc::c_char,
            res_18 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"positions_start_of_word\0" as *const u8 as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_19: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_19 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_19 = positions_start_of_word();
        }
        greatest_test_post(
            b"positions_start_of_word\0" as *const u8 as *const libc::c_char,
            res_19 as libc::c_int,
        );
    }
    if greatest_test_pre(b"positions_no_bonuses\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_20: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_20 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_20 = positions_no_bonuses();
        }
        greatest_test_post(
            b"positions_no_bonuses\0" as *const u8 as *const libc::c_char,
            res_20 as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"positions_multiple_candidates_start_of_words\0" as *const u8
            as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_21: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_21 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_21 = positions_multiple_candidates_start_of_words();
        }
        greatest_test_post(
            b"positions_multiple_candidates_start_of_words\0" as *const u8
                as *const libc::c_char,
            res_21 as libc::c_int,
        );
    }
    if greatest_test_pre(b"positions_exact_match\0" as *const u8 as *const libc::c_char)
        == 1 as libc::c_int
    {
        let mut res_22: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_22 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_22 = positions_exact_match();
        }
        greatest_test_post(
            b"positions_exact_match\0" as *const u8 as *const libc::c_char,
            res_22 as libc::c_int,
        );
    }
}

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type theft_bloom;
    pub type theft_mt;
    fn strndup(__string: *const libc::c_char, __n: size_t) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn greatest_test_post(name: *const libc::c_char, res: libc::c_int);
    fn greatest_test_pre(name: *const libc::c_char) -> libc::c_int;
    static mut greatest_info: greatest_run_info;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn theft_init(bloom_bits: uint8_t) -> *mut theft;
    fn theft_free(t: *mut theft);
    fn theft_random(t: *mut theft) -> theft_hash;
    fn theft_run(t: *mut theft, cfg: *mut theft_cfg) -> theft_run_res;
    fn theft_hash_onepass(data: *mut uint8_t, bytes: size_t) -> theft_hash;
    fn has_match(
        needle: *const libc::c_char,
        haystack: *const libc::c_char,
    ) -> libc::c_int;
    fn match_positions(
        needle: *const libc::c_char,
        haystack: *const libc::c_char,
        positions: *mut size_t,
    ) -> score_t;
    #[link_name = "match"]
    fn match_0(needle: *const libc::c_char, haystack: *const libc::c_char) -> score_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type theft_seed = uint64_t;
pub type theft_hash = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft {
    pub out: *mut FILE,
    pub seed: theft_seed,
    pub requested_bloom_bits: uint8_t,
    pub bloom: *mut theft_bloom,
    pub mt: *mut theft_mt,
}
pub type theft_alloc_cb = unsafe extern "C" fn(
    *mut theft,
    theft_seed,
    *mut libc::c_void,
) -> *mut libc::c_void;
pub type theft_free_cb = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
pub type theft_hash_cb = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> theft_hash;
pub type theft_shrink_cb = unsafe extern "C" fn(
    *mut libc::c_void,
    uint32_t,
    *mut libc::c_void,
) -> *mut libc::c_void;
pub type theft_print_cb = unsafe extern "C" fn(
    *mut FILE,
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
pub type theft_trial_res = libc::c_uint;
pub const THEFT_TRIAL_ERROR: theft_trial_res = 4;
pub const THEFT_TRIAL_DUP: theft_trial_res = 3;
pub const THEFT_TRIAL_SKIP: theft_trial_res = 2;
pub const THEFT_TRIAL_FAIL: theft_trial_res = 1;
pub const THEFT_TRIAL_PASS: theft_trial_res = 0;
pub type theft_propfun = unsafe extern "C" fn() -> theft_trial_res;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_type_info {
    pub alloc: Option::<theft_alloc_cb>,
    pub free: Option::<theft_free_cb>,
    pub hash: Option::<theft_hash_cb>,
    pub shrink: Option::<theft_shrink_cb>,
    pub print: Option::<theft_print_cb>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_trial_info {
    pub name: *const libc::c_char,
    pub trial: libc::c_int,
    pub seed: theft_seed,
    pub status: theft_trial_res,
    pub arity: uint8_t,
    pub args: *mut *mut libc::c_void,
}
pub type theft_progress_callback_res = libc::c_uint;
pub const THEFT_PROGRESS_HALT: theft_progress_callback_res = 1;
pub const THEFT_PROGRESS_CONTINUE: theft_progress_callback_res = 0;
pub type theft_progress_cb = unsafe extern "C" fn(
    *mut theft_trial_info,
    *mut libc::c_void,
) -> theft_progress_callback_res;
pub type theft_run_res = libc::c_int;
pub const THEFT_RUN_ERROR_MISSING_CALLBACK: theft_run_res = -2;
pub const THEFT_RUN_ERROR_BAD_ARGS: theft_run_res = -1;
pub const THEFT_RUN_ERROR: theft_run_res = 2;
pub const THEFT_RUN_FAIL: theft_run_res = 1;
pub const THEFT_RUN_PASS: theft_run_res = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_trial_report {
    pub pass: size_t,
    pub fail: size_t,
    pub skip: size_t,
    pub dup: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_cfg {
    pub fun: Option::<theft_propfun>,
    pub type_info: [*mut theft_type_info; 10],
    pub name: *const libc::c_char,
    pub always_seed_count: libc::c_int,
    pub always_seeds: *mut theft_seed,
    pub trials: libc::c_int,
    pub progress_cb: Option::<theft_progress_cb>,
    pub env: *mut libc::c_void,
    pub report: *mut theft_trial_report,
    pub seed: theft_seed,
}
pub type score_t = libc::c_double;
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
unsafe extern "C" fn string_alloc_cb(
    mut t: *mut theft,
    mut seed: theft_hash,
    mut env: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut limit: libc::c_int = 128 as libc::c_int;
    let mut sz: size_t = seed
        .wrapping_rem(limit as libc::c_ulong)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut str: *mut libc::c_char = malloc(
        sz.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if str.is_null() {
        return -(2 as libc::c_int) as *mut libc::c_void;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < sz {
        let mut s: theft_hash = theft_random(t);
        let mut b: uint8_t = 0 as libc::c_int as uint8_t;
        while (b as libc::c_ulong) < ::std::mem::size_of::<theft_hash>() as libc::c_ulong
        {
            if i.wrapping_add(b as libc::c_ulong) >= sz {
                break;
            }
            *str
                .offset(
                    i.wrapping_add(b as libc::c_ulong) as isize,
                ) = ((s >> 8 as libc::c_int * b as libc::c_int) as uint8_t as libc::c_int
                & 0xff as libc::c_int) as libc::c_char;
            b = b.wrapping_add(1);
            b;
        }
        i = (i as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<theft_hash>() as libc::c_ulong) as size_t
            as size_t;
    }
    *str.offset(sz as isize) = 0 as libc::c_int as libc::c_char;
    return str as *mut libc::c_void;
}
unsafe extern "C" fn string_free_cb(
    mut instance: *mut libc::c_void,
    mut env: *mut libc::c_void,
) {
    free(instance);
}
unsafe extern "C" fn string_print_cb(
    mut f: *mut FILE,
    mut instance: *mut libc::c_void,
    mut env: *mut libc::c_void,
) {
    let mut str: *mut libc::c_char = instance as *mut libc::c_char;
    let mut size: size_t = strlen(str);
    fprintf(f, b"str[%zd]:\n    \0" as *const u8 as *const libc::c_char, size);
    let mut bytes: uint8_t = 0 as libc::c_int as uint8_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < size {
        fprintf(
            f,
            b"%02x\0" as *const u8 as *const libc::c_char,
            *str.offset(i as isize) as libc::c_int,
        );
        bytes = bytes.wrapping_add(1);
        bytes;
        if bytes as libc::c_int == 16 as libc::c_int {
            fprintf(f, b"\n    \0" as *const u8 as *const libc::c_char);
            bytes = 0 as libc::c_int as uint8_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    fprintf(f, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn string_hash_cb(
    mut instance: *mut libc::c_void,
    mut env: *mut libc::c_void,
) -> uint64_t {
    let mut str: *mut libc::c_char = instance as *mut libc::c_char;
    let mut size: libc::c_int = strlen(str) as libc::c_int;
    return theft_hash_onepass(str as *mut uint8_t, size as size_t);
}
unsafe extern "C" fn string_shrink_cb(
    mut instance: *mut libc::c_void,
    mut tactic: uint32_t,
    mut env: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut str: *mut libc::c_char = instance as *mut libc::c_char;
    let mut n: libc::c_int = strlen(str) as libc::c_int;
    if tactic == 0 as libc::c_int as libc::c_uint {
        return strndup(str, (n / 2 as libc::c_int) as size_t) as *mut libc::c_void
    } else if tactic == 1 as libc::c_int as libc::c_uint {
        return strndup(
            str.offset((n / 2 as libc::c_int) as isize),
            (n / 2 as libc::c_int) as size_t,
        ) as *mut libc::c_void
    } else {
        return -(3 as libc::c_int) as *mut libc::c_void
    };
}
static mut string_info: theft_type_info = unsafe {
    {
        let mut init = theft_type_info {
            alloc: Some(
                string_alloc_cb
                    as unsafe extern "C" fn(
                        *mut theft,
                        theft_hash,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            free: Some(
                string_free_cb
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
            hash: Some(
                string_hash_cb
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> uint64_t,
            ),
            shrink: Some(
                string_shrink_cb
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        uint32_t,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
            print: Some(
                string_print_cb
                    as unsafe extern "C" fn(
                        *mut FILE,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn prop_should_return_results_if_there_is_a_match(
    mut needle: *mut libc::c_char,
    mut haystack: *mut libc::c_char,
) -> theft_trial_res {
    let mut match_exists: libc::c_int = has_match(needle, haystack);
    if match_exists == 0 {
        return THEFT_TRIAL_SKIP;
    }
    let mut score: score_t = match_0(needle, haystack);
    if *needle.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return THEFT_TRIAL_SKIP;
    }
    if score == -::std::f32::INFINITY as libc::c_double {
        return THEFT_TRIAL_FAIL;
    }
    return THEFT_TRIAL_PASS;
}
unsafe extern "C" fn should_return_results_if_there_is_a_match() -> greatest_test_res {
    let mut t: *mut theft = theft_init(0 as libc::c_int as uint8_t);
    let mut cfg: theft_cfg = {
        let mut init = theft_cfg {
            fun: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> theft_trial_res,
                >,
                Option::<theft_propfun>,
            >(
                Some(
                    prop_should_return_results_if_there_is_a_match
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            *mut libc::c_char,
                        ) -> theft_trial_res,
                ),
            ),
            type_info: [
                &mut string_info,
                &mut string_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
            ],
            name: (*::std::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"should_return_results_if_there_is_a_match\0"))
                .as_ptr(),
            always_seed_count: 0,
            always_seeds: 0 as *mut theft_seed,
            trials: 100000 as libc::c_int,
            progress_cb: None,
            env: 0 as *mut libc::c_void,
            report: 0 as *mut theft_trial_report,
            seed: 0,
        };
        init
    };
    let mut res: theft_run_res = theft_run(t, &mut cfg);
    theft_free(t);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if THEFT_RUN_PASS as libc::c_int != res as libc::c_int {
        greatest_info
            .fail_file = b"test/test_properties.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 112 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"should_return_results_if_there_is_a_match\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn prop_positions_should_match_characters_in_string(
    mut needle: *mut libc::c_char,
    mut haystack: *mut libc::c_char,
) -> theft_trial_res {
    let mut match_exists: libc::c_int = has_match(needle, haystack);
    if match_exists == 0 {
        return THEFT_TRIAL_SKIP;
    }
    let mut n: libc::c_int = strlen(needle) as libc::c_int;
    let mut positions: *mut size_t = calloc(
        n as libc::c_ulong,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    ) as *mut size_t;
    if positions.is_null() {
        return THEFT_TRIAL_ERROR;
    }
    match_positions(needle, haystack, positions);
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < n {
        if *positions.offset(i as isize)
            <= *positions.offset((i - 1 as libc::c_int) as isize)
        {
            return THEFT_TRIAL_FAIL;
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        if ({
            let mut __res: libc::c_int = 0;
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_int as libc::c_ulong
            {
                if 0 != 0 {
                    let mut __c: libc::c_int = *needle.offset(i_0 as isize)
                        as libc::c_int;
                    __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                        __c
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    });
                } else {
                    __res = toupper(*needle.offset(i_0 as isize) as libc::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*needle.offset(i_0 as isize) as libc::c_int as isize);
            }
            __res
        })
            != ({
                let mut __res: libc::c_int = 0;
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *haystack
                            .offset(*positions.offset(i_0 as isize) as isize)
                            as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = toupper(
                            *haystack.offset(*positions.offset(i_0 as isize) as isize)
                                as libc::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_toupper_loc())
                        .offset(
                            *haystack.offset(*positions.offset(i_0 as isize) as isize)
                                as libc::c_int as isize,
                        );
                }
                __res
            })
        {
            return THEFT_TRIAL_FAIL;
        }
        i_0 += 1;
        i_0;
    }
    free(positions as *mut libc::c_void);
    return THEFT_TRIAL_PASS;
}
unsafe extern "C" fn positions_should_match_characters_in_string() -> greatest_test_res {
    let mut t: *mut theft = theft_init(0 as libc::c_int as uint8_t);
    let mut cfg: theft_cfg = {
        let mut init = theft_cfg {
            fun: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_char,
                        *mut libc::c_char,
                    ) -> theft_trial_res,
                >,
                Option::<theft_propfun>,
            >(
                Some(
                    prop_positions_should_match_characters_in_string
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            *mut libc::c_char,
                        ) -> theft_trial_res,
                ),
            ),
            type_info: [
                &mut string_info,
                &mut string_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
                0 as *mut theft_type_info,
            ],
            name: (*::std::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"positions_should_match_characters_in_string\0"))
                .as_ptr(),
            always_seed_count: 0,
            always_seeds: 0 as *mut theft_seed,
            trials: 100000 as libc::c_int,
            progress_cb: None,
            env: 0 as *mut libc::c_void,
            report: 0 as *mut theft_trial_report,
            seed: 0,
        };
        init
    };
    let mut res: theft_run_res = theft_run(t, &mut cfg);
    theft_free(t);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    greatest_info.assertions;
    if THEFT_RUN_PASS as libc::c_int != res as libc::c_int {
        greatest_info
            .fail_file = b"test/test_properties.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 158 as libc::c_int as libc::c_uint;
        greatest_info
            .msg = b"should_return_results_if_there_is_a_match\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
pub unsafe extern "C" fn properties_suite() {
    if greatest_test_pre(
        b"should_return_results_if_there_is_a_match\0" as *const u8
            as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res: greatest_test_res = _setjmp((greatest_info.jump_dest).as_mut_ptr())
            as greatest_test_res;
        if res as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res = should_return_results_if_there_is_a_match();
        }
        greatest_test_post(
            b"should_return_results_if_there_is_a_match\0" as *const u8
                as *const libc::c_char,
            res as libc::c_int,
        );
    }
    if greatest_test_pre(
        b"positions_should_match_characters_in_string\0" as *const u8
            as *const libc::c_char,
    ) == 1 as libc::c_int
    {
        let mut res_0: greatest_test_res = _setjmp(
            (greatest_info.jump_dest).as_mut_ptr(),
        ) as greatest_test_res;
        if res_0 as libc::c_int == GREATEST_TEST_RES_PASS as libc::c_int {
            res_0 = positions_should_match_characters_in_string();
        }
        greatest_test_post(
            b"positions_should_match_characters_in_string\0" as *const u8
                as *const libc::c_char,
            res_0 as libc::c_int,
        );
    }
}

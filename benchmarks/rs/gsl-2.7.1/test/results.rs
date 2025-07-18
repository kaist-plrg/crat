use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn gsl_isnan(x: libc::c_double) -> libc::c_int;
    fn gsl_isinf(x: libc::c_double) -> libc::c_int;
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
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    mut __arg: ::std::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
static mut tests: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut passed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut failed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut verbose: libc::c_uint = 0 as libc::c_int as libc::c_uint;
unsafe extern "C" fn initialise() {
    let mut p: *const libc::c_char = getenv(
        b"GSL_TEST_VERBOSE\0" as *const u8 as *const libc::c_char,
    );
    if p.is_null() {
        return;
    }
    if *p as libc::c_int == '\0' as i32 {
        return;
    }
    verbose = strtoul(p, 0 as *mut *mut libc::c_char, 0 as libc::c_int) as libc::c_uint;
}
unsafe extern "C" fn update(mut s: libc::c_int) {
    tests = tests.wrapping_add(1);
    tests;
    if s == 0 as libc::c_int {
        passed = passed.wrapping_add(1);
        passed;
    } else {
        failed = failed.wrapping_add(1);
        failed;
    };
}
pub unsafe extern "C" fn gsl_test(
    mut status: libc::c_int,
    mut test_description: *const libc::c_char,
    mut args: ...
) {
    if tests == 0 {
        initialise();
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const libc::c_char
            } else {
                b"PASS: \0" as *const u8 as *const libc::c_char
            },
        );
        let mut ap: ::std::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const libc::c_char, tests);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
    }
}
pub unsafe extern "C" fn gsl_test_rel(
    mut result: libc::c_double,
    mut expected: libc::c_double,
    mut relative_error: libc::c_double,
    mut test_description: *const libc::c_char,
    mut args: ...
) {
    let mut status: libc::c_int = 0;
    if tests == 0 {
        initialise();
    }
    if gsl_isnan(result) != 0 || gsl_isnan(expected) != 0 {
        status = (gsl_isnan(result) != gsl_isnan(expected)) as libc::c_int;
    } else if gsl_isinf(result) != 0 || gsl_isinf(expected) != 0 {
        status = (gsl_isinf(result) != gsl_isinf(expected)) as libc::c_int;
    } else if expected > 0 as libc::c_int as libc::c_double
        && expected < 2.2250738585072014e-308f64
        || expected < 0 as libc::c_int as libc::c_double
            && expected > -2.2250738585072014e-308f64
    {
        status = -(1 as libc::c_int);
    } else if expected != 0 as libc::c_int as libc::c_double {
        status = (fabs(result - expected) / fabs(expected) > relative_error)
            as libc::c_int;
    } else {
        status = (fabs(result) > relative_error) as libc::c_int;
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const libc::c_char
            } else {
                b"PASS: \0" as *const u8 as *const libc::c_char
            },
        );
        let mut ap: ::std::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status == 0 as libc::c_int {
            if strlen(test_description) < 45 as libc::c_int as libc::c_ulong {
                printf(
                    b" (%g observed vs %g expected)\0" as *const u8
                        as *const libc::c_char,
                    result,
                    expected,
                );
            } else {
                printf(
                    b" (%g obs vs %g exp)\0" as *const u8 as *const libc::c_char,
                    result,
                    expected,
                );
            }
        } else {
            printf(
                b" (%.18g observed vs %.18g expected)\0" as *const u8
                    as *const libc::c_char,
                result,
                expected,
            );
        }
        if status == -(1 as libc::c_int) {
            printf(
                b" [test uses subnormal value]\0" as *const u8 as *const libc::c_char,
            );
        }
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const libc::c_char, tests);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
    }
}
pub unsafe extern "C" fn gsl_test_abs(
    mut result: libc::c_double,
    mut expected: libc::c_double,
    mut absolute_error: libc::c_double,
    mut test_description: *const libc::c_char,
    mut args: ...
) {
    let mut status: libc::c_int = 0;
    if tests == 0 {
        initialise();
    }
    if gsl_isnan(result) != 0 || gsl_isnan(expected) != 0 {
        status = (gsl_isnan(result) != gsl_isnan(expected)) as libc::c_int;
    } else if gsl_isinf(result) != 0 || gsl_isinf(expected) != 0 {
        status = (gsl_isinf(result) != gsl_isinf(expected)) as libc::c_int;
    } else if expected > 0 as libc::c_int as libc::c_double
        && expected < 2.2250738585072014e-308f64
        || expected < 0 as libc::c_int as libc::c_double
            && expected > -2.2250738585072014e-308f64
    {
        status = -(1 as libc::c_int);
    } else {
        status = (fabs(result - expected) > absolute_error) as libc::c_int;
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const libc::c_char
            } else {
                b"PASS: \0" as *const u8 as *const libc::c_char
            },
        );
        let mut ap: ::std::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status == 0 as libc::c_int {
            if strlen(test_description) < 45 as libc::c_int as libc::c_ulong {
                printf(
                    b" (%g observed vs %g expected)\0" as *const u8
                        as *const libc::c_char,
                    result,
                    expected,
                );
            } else {
                printf(
                    b" (%g obs vs %g exp)\0" as *const u8 as *const libc::c_char,
                    result,
                    expected,
                );
            }
        } else {
            printf(
                b" (%.18g observed vs %.18g expected)\0" as *const u8
                    as *const libc::c_char,
                result,
                expected,
            );
        }
        if status == -(1 as libc::c_int) {
            printf(
                b" [test uses subnormal value]\0" as *const u8 as *const libc::c_char,
            );
        }
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const libc::c_char, tests);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
    }
}
pub unsafe extern "C" fn gsl_test_factor(
    mut result: libc::c_double,
    mut expected: libc::c_double,
    mut factor: libc::c_double,
    mut test_description: *const libc::c_char,
    mut args: ...
) {
    let mut status: libc::c_int = 0;
    if tests == 0 {
        initialise();
    }
    if expected > 0 as libc::c_int as libc::c_double
        && expected < 2.2250738585072014e-308f64
        || expected < 0 as libc::c_int as libc::c_double
            && expected > -2.2250738585072014e-308f64
    {
        status = -(1 as libc::c_int);
    } else if result == expected {
        status = 0 as libc::c_int;
    } else if expected == 0.0f64 {
        status = (result > expected || result < expected) as libc::c_int;
    } else {
        let mut u: libc::c_double = result / expected;
        status = (u > factor || u < 1.0f64 / factor) as libc::c_int;
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const libc::c_char
            } else {
                b"PASS: \0" as *const u8 as *const libc::c_char
            },
        );
        let mut ap: ::std::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status == 0 as libc::c_int {
            if strlen(test_description) < 45 as libc::c_int as libc::c_ulong {
                printf(
                    b" (%g observed vs %g expected)\0" as *const u8
                        as *const libc::c_char,
                    result,
                    expected,
                );
            } else {
                printf(
                    b" (%g obs vs %g exp)\0" as *const u8 as *const libc::c_char,
                    result,
                    expected,
                );
            }
        } else {
            printf(
                b" (%.18g observed vs %.18g expected)\0" as *const u8
                    as *const libc::c_char,
                result,
                expected,
            );
        }
        if status == -(1 as libc::c_int) {
            printf(
                b" [test uses subnormal value]\0" as *const u8 as *const libc::c_char,
            );
        }
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const libc::c_char, tests);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
    }
}
pub unsafe extern "C" fn gsl_test_int(
    mut result: libc::c_int,
    mut expected: libc::c_int,
    mut test_description: *const libc::c_char,
    mut args: ...
) {
    let mut status: libc::c_int = (result != expected) as libc::c_int;
    if tests == 0 {
        initialise();
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const libc::c_char
            } else {
                b"PASS: \0" as *const u8 as *const libc::c_char
            },
        );
        let mut ap: ::std::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status == 0 as libc::c_int {
            printf(
                b" (%d observed vs %d expected)\0" as *const u8 as *const libc::c_char,
                result,
                expected,
            );
        } else {
            printf(
                b" (%d observed vs %d expected)\0" as *const u8 as *const libc::c_char,
                result,
                expected,
            );
        }
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const libc::c_char, tests);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
    }
}
pub unsafe extern "C" fn gsl_test_str(
    mut result: *const libc::c_char,
    mut expected: *const libc::c_char,
    mut test_description: *const libc::c_char,
    mut args: ...
) {
    let mut status: libc::c_int = strcmp(result, expected);
    if tests == 0 {
        initialise();
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const libc::c_char
            } else {
                b"PASS: \0" as *const u8 as *const libc::c_char
            },
        );
        let mut ap: ::std::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status != 0 {
            printf(
                b" (%s observed vs %s expected)\0" as *const u8 as *const libc::c_char,
                result,
                expected,
            );
        }
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const libc::c_char, tests);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        fflush(stdout);
    }
}
pub unsafe extern "C" fn gsl_test_verbose(mut v: libc::c_int) {
    verbose = v as libc::c_uint;
}
pub unsafe extern "C" fn gsl_test_summary() -> libc::c_int {
    if verbose != 0 && 0 as libc::c_int != 0 {
        printf(
            b"%d tests, passed %d, failed %d.\n\0" as *const u8 as *const libc::c_char,
            tests,
            passed,
            failed,
        );
    }
    if failed != 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    if tests != passed.wrapping_add(failed) {
        if verbose != 0 {
            printf(
                b"TEST RESULTS DO NOT ADD UP %d != %d + %d\n\0" as *const u8
                    as *const libc::c_char,
                tests,
                passed,
                failed,
            );
        }
        return 1 as libc::c_int;
    }
    if passed == tests {
        if verbose == 0 {
            printf(
                b"Completed [%d/%d]\n\0" as *const u8 as *const libc::c_char,
                passed,
                tests,
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

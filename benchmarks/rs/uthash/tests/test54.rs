use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
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
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_string {
    pub d: *mut libc::c_char,
    pub n: size_t,
    pub i: size_t,
}
unsafe extern "C" fn utstring_printf(
    mut s: *mut UT_string,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    utstring_printf_va(s, fmt, ap.as_va_list());
}
unsafe extern "C" fn utstring_printf_va(
    mut s: *mut UT_string,
    mut fmt: *const libc::c_char,
    mut ap: ::std::ffi::VaList,
) {
    let mut n: libc::c_int = 0;
    let mut cp: ::std::ffi::VaListImpl;
    loop {
        cp = ap.clone();
        n = vsnprintf(
            &mut *((*s).d).offset((*s).i as isize),
            ((*s).n).wrapping_sub((*s).i),
            fmt,
            cp.as_va_list(),
        );
        if n > -(1 as libc::c_int) && (n as size_t) < ((*s).n).wrapping_sub((*s).i) {
            (*s)
                .i = ((*s).i as libc::c_ulong).wrapping_add(n as libc::c_ulong) as size_t
                as size_t;
            return;
        }
        if n > -(1 as libc::c_int) {
            if ((*s).n).wrapping_sub((*s).i) < (n + 1 as libc::c_int) as size_t {
                let mut utstring_tmp: *mut libc::c_char = realloc(
                    (*s).d as *mut libc::c_void,
                    ((*s).n).wrapping_add((n + 1 as libc::c_int) as libc::c_ulong),
                ) as *mut libc::c_char;
                if utstring_tmp.is_null() {
                    exit(-(1 as libc::c_int));
                }
                (*s).d = utstring_tmp;
                (*s)
                    .n = ((*s).n as libc::c_ulong)
                    .wrapping_add((n + 1 as libc::c_int) as libc::c_ulong) as size_t
                    as size_t;
            }
        } else if ((*s).n).wrapping_sub((*s).i)
            < ((*s).n).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        {
            let mut utstring_tmp_0: *mut libc::c_char = realloc(
                (*s).d as *mut libc::c_void,
                ((*s).n)
                    .wrapping_add(
                        ((*s).n).wrapping_mul(2 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut libc::c_char;
            if utstring_tmp_0.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*s).d = utstring_tmp_0;
            (*s)
                .n = ((*s).n as libc::c_ulong)
                .wrapping_add(((*s).n).wrapping_mul(2 as libc::c_int as libc::c_ulong))
                as size_t as size_t;
        }
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut s: *mut UT_string = 0 as *mut UT_string;
    let mut t: *mut UT_string = 0 as *mut UT_string;
    s = malloc(::std::mem::size_of::<UT_string>() as libc::c_ulong) as *mut UT_string;
    if s.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*s).n = 0 as libc::c_int as size_t;
    (*s).i = 0 as libc::c_int as size_t;
    (*s).d = 0 as *mut libc::c_char;
    if ((*s).n).wrapping_sub((*s).i) < 100 as libc::c_int as size_t {
        let mut utstring_tmp: *mut libc::c_char = realloc(
            (*s).d as *mut libc::c_void,
            ((*s).n).wrapping_add(100 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if utstring_tmp.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*s).d = utstring_tmp;
        (*s)
            .n = ((*s).n as libc::c_ulong)
            .wrapping_add(100 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    *((*s).d).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    t = malloc(::std::mem::size_of::<UT_string>() as libc::c_ulong) as *mut UT_string;
    if t.is_null() {
        exit(-(1 as libc::c_int));
    }
    (*t).n = 0 as libc::c_int as size_t;
    (*t).i = 0 as libc::c_int as size_t;
    (*t).d = 0 as *mut libc::c_char;
    if ((*t).n).wrapping_sub((*t).i) < 100 as libc::c_int as size_t {
        let mut utstring_tmp_0: *mut libc::c_char = realloc(
            (*t).d as *mut libc::c_void,
            ((*t).n).wrapping_add(100 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        if utstring_tmp_0.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*t).d = utstring_tmp_0;
        (*t)
            .n = ((*t).n as libc::c_ulong)
            .wrapping_add(100 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    *((*t).d).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    utstring_printf(s, b"hello \0" as *const u8 as *const libc::c_char);
    utstring_printf(s, b"world \0" as *const u8 as *const libc::c_char);
    utstring_printf(t, b"hi \0" as *const u8 as *const libc::c_char);
    utstring_printf(t, b"there \0" as *const u8 as *const libc::c_char);
    if ((*s).n).wrapping_sub((*s).i)
        < ((*t).i).wrapping_add(1 as libc::c_int as libc::c_ulong)
    {
        let mut utstring_tmp_1: *mut libc::c_char = realloc(
            (*s).d as *mut libc::c_void,
            ((*s).n)
                .wrapping_add(((*t).i).wrapping_add(1 as libc::c_int as libc::c_ulong)),
        ) as *mut libc::c_char;
        if utstring_tmp_1.is_null() {
            exit(-(1 as libc::c_int));
        }
        (*s).d = utstring_tmp_1;
        (*s)
            .n = ((*s).n as libc::c_ulong)
            .wrapping_add(((*t).i).wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
    }
    if (*t).i != 0 {
        memcpy(
            &mut *((*s).d).offset((*s).i as isize) as *mut libc::c_char
                as *mut libc::c_void,
            (*t).d as *const libc::c_void,
            (*t).i,
        );
    }
    (*s).i = ((*s).i as libc::c_ulong).wrapping_add((*t).i) as size_t as size_t;
    *((*s).d).offset((*s).i as isize) = '\0' as i32 as libc::c_char;
    printf(
        b"length: %u\n\0" as *const u8 as *const libc::c_char,
        (*s).i as libc::c_uint,
    );
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*s).d);
    if !((*s).d).is_null() {
        free((*s).d as *mut libc::c_void);
    }
    (*s).n = 0 as libc::c_int as size_t;
    free(s as *mut libc::c_void);
    if !((*t).d).is_null() {
        free((*t).d as *mut libc::c_void);
    }
    (*t).n = 0 as libc::c_int as size_t;
    free(t as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

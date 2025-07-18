use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: Option::<unsafe extern "C" fn() -> ()>,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type pthread_key_t = libc::c_uint;
pub type pthread_once_t = libc::c_int;
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
pub struct try_ball_t_ {
    pub ret: libc::c_int,
    pub code: libc::c_int,
    pub free: libc::c_int,
    pub why: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct try_s_ {
    pub env: jmp_buf,
    pub ball: try_ball_t_,
    pub next: *mut try_t_,
}
pub type try_t_ = try_s_;
pub type va_list = __builtin_va_list;
pub static mut try_key_: pthread_key_t = 0;
static mut try_once_: pthread_once_t = 0 as libc::c_int;
unsafe extern "C" fn try_create_() {
    let mut ret: libc::c_int = pthread_key_create(&mut try_key_, None);
    if ret == 0 as libc::c_int
        && !(b"try: pthread_key_create() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"ret == 0 && \"try: pthread_key_create() failed\"\0" as *const u8
                as *const libc::c_char,
            b"try.c\0" as *const u8 as *const libc::c_char,
            22 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 23],
                &[libc::c_char; 23],
            >(b"void try_create_(void)\0"))
                .as_ptr(),
        );
    }
    'c_2235: {
        if ret == 0 as libc::c_int
            && !(b"try: pthread_key_create() failed\0" as *const u8
                as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"ret == 0 && \"try: pthread_key_create() failed\"\0" as *const u8
                    as *const libc::c_char,
                b"try.c\0" as *const u8 as *const libc::c_char,
                22 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 23],
                    &[libc::c_char; 23],
                >(b"void try_create_(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn try_setup_() {
    let mut ret: libc::c_int = pthread_once(
        &mut try_once_,
        Some(try_create_ as unsafe extern "C" fn() -> ()),
    );
    if ret == 0 as libc::c_int
        && !(b"try: pthread_once() failed\0" as *const u8 as *const libc::c_char)
            .is_null()
    {} else {
        __assert_fail(
            b"ret == 0 && \"try: pthread_once() failed\"\0" as *const u8
                as *const libc::c_char,
            b"try.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 22],
                &[libc::c_char; 22],
            >(b"void try_setup_(void)\0"))
                .as_ptr(),
        );
    }
    'c_2189: {
        if ret == 0 as libc::c_int
            && !(b"try: pthread_once() failed\0" as *const u8 as *const libc::c_char)
                .is_null()
        {} else {
            __assert_fail(
                b"ret == 0 && \"try: pthread_once() failed\"\0" as *const u8
                    as *const libc::c_char,
                b"try.c\0" as *const u8 as *const libc::c_char,
                27 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 22],
                    &[libc::c_char; 22],
                >(b"void try_setup_(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
pub unsafe extern "C" fn try_throw_(
    mut code: libc::c_int,
    mut fmt: *mut libc::c_char,
    mut args: ...
) -> ! {
    try_setup_();
    if !(pthread_getspecific(try_key_) as *mut try_t_).is_null()
        && !(b"try: naked throw\0" as *const u8 as *const libc::c_char).is_null()
    {} else {
        __assert_fail(
            b"try_stack_ != NULL && \"try: naked throw\"\0" as *const u8
                as *const libc::c_char,
            b"try.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void try_throw_(int, char *, ...)\0"))
                .as_ptr(),
        );
    }
    'c_2563: {
        if !(pthread_getspecific(try_key_) as *mut try_t_).is_null()
            && !(b"try: naked throw\0" as *const u8 as *const libc::c_char).is_null()
        {} else {
            __assert_fail(
                b"try_stack_ != NULL && \"try: naked throw\"\0" as *const u8
                    as *const libc::c_char,
                b"try.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void try_throw_(int, char *, ...)\0"))
                    .as_ptr(),
            );
        }
    };
    (*(pthread_getspecific(try_key_) as *mut try_t_)).ball.ret = 1 as libc::c_int;
    (*(pthread_getspecific(try_key_) as *mut try_t_)).ball.code = code;
    (*(pthread_getspecific(try_key_) as *mut try_t_)).ball.free = 0 as libc::c_int;
    let ref mut fresh0 = (*(pthread_getspecific(try_key_) as *mut try_t_)).ball.why;
    *fresh0 = fmt;
    if !fmt.is_null() && !(strchr(fmt, '%' as i32)).is_null() {
        let mut why: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut nul: [libc::c_char; 1] = [0; 1];
        let mut len: size_t = 0;
        let mut ap1: ::std::ffi::VaListImpl;
        let mut ap2: ::std::ffi::VaListImpl;
        ap1 = args.clone();
        ap2 = ap1.clone();
        len = vsnprintf(
            nul.as_mut_ptr(),
            1 as libc::c_int as libc::c_ulong,
            fmt,
            ap1.as_va_list(),
        ) as size_t;
        why = malloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        if why.is_null() {
            let ref mut fresh1 = (*(pthread_getspecific(try_key_) as *mut try_t_))
                .ball
                .why;
            *fresh1 = b"try: out of memory\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            vsnprintf(
                why,
                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                fmt,
                ap2.as_va_list(),
            );
            (*(pthread_getspecific(try_key_) as *mut try_t_))
                .ball
                .free = 1 as libc::c_int;
            let ref mut fresh2 = (*(pthread_getspecific(try_key_) as *mut try_t_))
                .ball
                .why;
            *fresh2 = why;
        }
    }
    longjmp(
        ((*(pthread_getspecific(try_key_) as *mut try_t_)).env).as_mut_ptr(),
        1 as libc::c_int,
    );
}

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn aco_destroy(co: *mut aco_t);
    fn aco_resume(resume_co: *mut aco_t);
    #[thread_local]
    static mut aco_gtls_co: *mut aco_t;
    fn aco_create(
        main_co: *mut aco_t,
        share_stack: *mut aco_share_stack_t,
        save_stack_sz: size_t,
        fp: aco_cofuncp_t,
        arg: *mut libc::c_void,
    ) -> *mut aco_t;
    fn aco_share_stack_destroy(sstk: *mut aco_share_stack_t);
    fn aco_share_stack_new(sz: size_t) -> *mut aco_share_stack_t;
    fn acosw(from_co: *mut aco_t, to_co: *mut aco_t) -> *mut libc::c_void;
    fn aco_thread_init(last_word_co_fp: aco_cofuncp_t);
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type uintptr_t = libc::c_ulong;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aco_save_stack_t {
    pub ptr: *mut libc::c_void,
    pub sz: size_t,
    pub valid_sz: size_t,
    pub max_cpsz: size_t,
    pub ct_save: size_t,
    pub ct_restore: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aco_s {
    pub reg: [*mut libc::c_void; 9],
    pub main_co: *mut aco_t,
    pub arg: *mut libc::c_void,
    pub is_end: libc::c_char,
    pub fp: aco_cofuncp_t,
    pub save_stack: aco_save_stack_t,
    pub share_stack: *mut aco_share_stack_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aco_share_stack_t {
    pub ptr: *mut libc::c_void,
    pub sz: size_t,
    pub align_highptr: *mut libc::c_void,
    pub align_retptr: *mut libc::c_void,
    pub align_validsz: size_t,
    pub align_limit: size_t,
    pub owner: *mut aco_t,
    pub guard_page_enabled: libc::c_char,
    pub real_ptr: *mut libc::c_void,
    pub real_sz: size_t,
}
pub type aco_t = aco_s;
pub type aco_cofuncp_t = Option::<unsafe extern "C" fn() -> ()>;
pub static mut gl_co_fp: aco_cofuncp_t = None;
pub static mut gl_benchmark_print_str_buf: [libc::c_char; 64] = [0; 64];
pub unsafe extern "C" fn co_fp_alloca() {
    let mut sz: size_t = (*aco_gtls_co).arg as uintptr_t;
    let mut ptr: *mut uint8_t = 0 as *mut uint8_t;
    if (sz > 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    let mut fresh0 = ::std::vec::from_elem(0, sz as usize);
    ptr = fresh0.leak().as_mut_ptr() as *mut uint8_t;
    if !ptr.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    memset(ptr as *mut libc::c_void, 0 as libc::c_int, sz);
    loop {
        if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    };
}
pub unsafe extern "C" fn co_fp_stksz_128() {
    let mut ip: [libc::c_int; 28] = [0; 28];
    memset(
        ip.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 28]>() as libc::c_ulong,
    );
    loop {
        if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    };
}
pub unsafe extern "C" fn co_fp_stksz_64() {
    let mut ip: [libc::c_int; 12] = [0; 12];
    memset(
        ip.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 12]>() as libc::c_ulong,
    );
    loop {
        if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    };
}
pub unsafe extern "C" fn co_fp_stksz_40() {
    let mut ip: [libc::c_int; 8] = [0; 8];
    memset(
        ip.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 8]>() as libc::c_ulong,
    );
    loop {
        if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    };
}
pub unsafe extern "C" fn co_fp_stksz_24() {
    let mut ip: [libc::c_int; 4] = [0; 4];
    memset(
        ip.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong,
    );
    loop {
        if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    };
}
pub unsafe extern "C" fn co_fp_stksz_8() {
    loop {
        if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    };
}
pub unsafe extern "C" fn co_fp0() {
    loop {
        if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    };
}
pub unsafe extern "C" fn benchmark_copystack(
    mut co_amount: size_t,
    mut stksz: size_t,
    mut loopct: size_t,
) {
    let mut tstart: timespec = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: 0 as libc::c_int as __syscall_slong_t,
        };
        init
    };
    let mut tend: timespec = {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: 0 as libc::c_int as __syscall_slong_t,
        };
        init
    };
    let mut print_sz: libc::c_int = 0 as libc::c_int;
    let mut delta_t: libc::c_double = 0.;
    if (co_amount > 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long
        != 0
    {} else {
        abort();
    };
    if !(::std::mem::transmute::<aco_cofuncp_t, *mut libc::c_void>(gl_co_fp)).is_null()
        as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    let mut main_co: *mut aco_t = aco_create(
        0 as *mut aco_t,
        0 as *mut aco_share_stack_t,
        0 as libc::c_int as size_t,
        None,
        0 as *mut libc::c_void,
    );
    let mut sstk: *mut aco_share_stack_t = aco_share_stack_new(
        0 as libc::c_int as size_t,
    );
    let mut coarray: *mut *mut aco_t = malloc(
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(co_amount),
    ) as *mut *mut aco_t;
    if !coarray.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    memset(
        coarray as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(co_amount),
    );
    let mut ct: size_t = 0 as libc::c_int as size_t;
    if (0 as libc::c_int == clock_gettime(1 as libc::c_int, &mut tstart)) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    while ct < co_amount {
        let ref mut fresh1 = *coarray.offset(ct as isize);
        *fresh1 = aco_create(
            main_co,
            sstk,
            0 as libc::c_int as size_t,
            gl_co_fp,
            stksz as *mut libc::c_void,
        );
        ct = ct.wrapping_add(1);
        ct;
    }
    if (0 as libc::c_int == clock_gettime(1 as libc::c_int, &mut tend)) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    delta_t = tend.tv_sec as libc::c_double + 1.0e-9f64 * tend.tv_nsec as libc::c_double
        - (tstart.tv_sec as libc::c_double
            + 1.0e-9f64 * tstart.tv_nsec as libc::c_double);
    print_sz = snprintf(
        gl_benchmark_print_str_buf.as_mut_ptr(),
        64 as libc::c_int as libc::c_ulong,
        b"aco_create/init_save_stk_sz=64B\0" as *const u8 as *const libc::c_char,
    );
    if (print_sz > 0 as libc::c_int && print_sz < 64 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    printf(
        b"%-50s %11zu %9.3f s %11.2f ns/op %13.2f op/s\n\0" as *const u8
            as *const libc::c_char,
        gl_benchmark_print_str_buf.as_mut_ptr(),
        co_amount,
        delta_t,
        1.0e+9f64 / (co_amount as libc::c_double / delta_t),
        co_amount as libc::c_double / delta_t,
    );
    fflush(stdout);
    ct = 0 as libc::c_int as size_t;
    while ct < co_amount {
        aco_resume(*coarray.offset(ct as isize));
        ct = ct.wrapping_add(1);
        ct;
    }
    if (0 as libc::c_int == clock_gettime(1 as libc::c_int, &mut tstart)) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    let mut glct: size_t = 0 as libc::c_int as size_t;
    while glct < loopct {
        ct = 0 as libc::c_int as size_t;
        while ct < co_amount {
            aco_resume(*coarray.offset(ct as isize));
            ct = ct.wrapping_add(1);
            ct;
            glct = glct.wrapping_add(1);
            glct;
        }
    }
    if (0 as libc::c_int == clock_gettime(1 as libc::c_int, &mut tend)) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    delta_t = tend.tv_sec as libc::c_double + 1.0e-9f64 * tend.tv_nsec as libc::c_double
        - (tstart.tv_sec as libc::c_double
            + 1.0e-9f64 * tstart.tv_nsec as libc::c_double);
    print_sz = snprintf(
        gl_benchmark_print_str_buf.as_mut_ptr(),
        64 as libc::c_int as libc::c_ulong,
        b"aco_resume/co_amount=%zu/copy_stack_size=%zuB\0" as *const u8
            as *const libc::c_char,
        co_amount,
        (**coarray.offset(0 as libc::c_int as isize)).save_stack.max_cpsz,
    );
    if (print_sz > 0 as libc::c_int && print_sz < 64 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    printf(
        b"%-50s %11zu %9.3f s %11.2f ns/op %13.2f op/s\n\0" as *const u8
            as *const libc::c_char,
        gl_benchmark_print_str_buf.as_mut_ptr(),
        glct,
        delta_t,
        1.0e+9f64 / (glct as libc::c_double / delta_t),
        glct as libc::c_double / delta_t,
    );
    if co_amount == 1 as libc::c_int as libc::c_ulong
        && (**coarray.offset(0 as libc::c_int as isize)).save_stack.max_cpsz
            == 0 as libc::c_int as libc::c_ulong
    {
        printf(
            b"%-50s %11zu %9.3f s %11.2f ns/op %13.2f op/s\n\0" as *const u8
                as *const libc::c_char,
            b"  -> acosw\0" as *const u8 as *const libc::c_char,
            glct.wrapping_mul(2 as libc::c_int as libc::c_ulong),
            delta_t,
            1.0e+9f64
                / (glct.wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_double
                    / delta_t),
            glct.wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_double
                / delta_t,
        );
    }
    fflush(stdout);
    if (0 as libc::c_int == clock_gettime(1 as libc::c_int, &mut tstart)) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    ct = 0 as libc::c_int as size_t;
    while ct < co_amount {
        aco_destroy(*coarray.offset(ct as isize));
        let ref mut fresh2 = *coarray.offset(ct as isize);
        *fresh2 = 0 as *mut aco_t;
        ct = ct.wrapping_add(1);
        ct;
    }
    if (0 as libc::c_int == clock_gettime(1 as libc::c_int, &mut tend)) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    aco_share_stack_destroy(sstk);
    sstk = 0 as *mut aco_share_stack_t;
    aco_destroy(main_co);
    main_co = 0 as *mut aco_t;
    free(coarray as *mut libc::c_void);
    delta_t = tend.tv_sec as libc::c_double + 1.0e-9f64 * tend.tv_nsec as libc::c_double
        - (tstart.tv_sec as libc::c_double
            + 1.0e-9f64 * tstart.tv_nsec as libc::c_double);
    print_sz = snprintf(
        gl_benchmark_print_str_buf.as_mut_ptr(),
        64 as libc::c_int as libc::c_ulong,
        b"aco_destroy\0" as *const u8 as *const libc::c_char,
    );
    if (print_sz > 0 as libc::c_int && print_sz < 64 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    printf(
        b"%-50s %11zu %9.3f s %11.2f ns/op %13.2f op/s\n\n\0" as *const u8
            as *const libc::c_char,
        gl_benchmark_print_str_buf.as_mut_ptr(),
        co_amount,
        delta_t,
        1.0e+9f64 / (co_amount as libc::c_double / delta_t),
        co_amount as libc::c_double / delta_t,
    );
    fflush(stdout);
}
unsafe fn main_0() -> libc::c_int {
    aco_thread_init(None);
    printf(b"warm-up:\n\0" as *const u8 as *const libc::c_char);
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_stksz_8),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        10 as libc::c_int as size_t,
        20000000 as libc::c_int as size_t,
    );
    printf(b"+build:x86_64\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"+build:undefined ACO_CONFIG_SHARE_FPU_MXCSR_ENV\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"+build:each coroutine maintain each own fpu & mxcsr control words\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"+build:undefined ACO_USE_VALGRIND\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"+build:without valgrind memcheck friendly support\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"\nsizeof(aco_t)=%zu:\n\n\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<aco_t>() as libc::c_ulong,
    );
    printf(b"\nstart-test:\n\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"%-50s %15s    %15s    %15s   %15s\n\n\0" as *const u8 as *const libc::c_char,
        b"comment\0" as *const u8 as *const libc::c_char,
        b"task_amount\0" as *const u8 as *const libc::c_char,
        b"all_time_cost\0" as *const u8 as *const libc::c_char,
        b"ns_per_op\0" as *const u8 as *const libc::c_char,
        b"speed\0" as *const u8 as *const libc::c_char,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_stksz_8),
        ),
    );
    benchmark_copystack(
        1 as libc::c_int as size_t,
        10 as libc::c_int as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_stksz_8),
        ),
    );
    benchmark_copystack(
        1 as libc::c_int as size_t,
        10 as libc::c_int as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_stksz_8),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        10 as libc::c_int as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_stksz_24),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        10 as libc::c_int as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_stksz_40),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        10 as libc::c_int as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_stksz_64),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        10 as libc::c_int as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_stksz_128),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        10 as libc::c_int as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        (150 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        (158 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        (166 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        (256 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        (512 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (200 as libc::c_int * 10000 as libc::c_int) as size_t,
        (512 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (100 as libc::c_int * 10000 as libc::c_int) as size_t,
        (1024 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (100 as libc::c_int * 10000 as libc::c_int) as size_t,
        (1024 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (10 as libc::c_int * 10000 as libc::c_int) as size_t,
        (1024 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (10 as libc::c_int * 10000 as libc::c_int) as size_t,
        (2048 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (10 as libc::c_int * 10000 as libc::c_int) as size_t,
        (4096 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    gl_co_fp = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        aco_cofuncp_t,
    >(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(co_fp_alloca),
        ),
    );
    benchmark_copystack(
        (10 as libc::c_int * 10000 as libc::c_int) as size_t,
        (8012 as libc::c_int - 64 as libc::c_int) as size_t,
        20000000 as libc::c_int as size_t,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

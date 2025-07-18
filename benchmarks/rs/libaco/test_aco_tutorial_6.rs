use ::libc;
extern "C" {
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn time(__timer: *mut time_t) -> time_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type time_t = __time_t;
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
pub unsafe extern "C" fn co_fp0() {
    let mut ct: libc::c_int = 0 as libc::c_int;
    let mut loop_ct: libc::c_int = (*({ aco_gtls_co })).arg as uintptr_t as libc::c_int;
    if loop_ct < 0 as libc::c_int {
        loop_ct = 0 as libc::c_int;
    }
    while ct < loop_ct {
        if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        acosw(aco_gtls_co, (*aco_gtls_co).main_co);
        ct += 1;
        ct;
    }
    (*aco_gtls_co).is_end = 1 as libc::c_int as libc::c_char;
    if ((*(*aco_gtls_co).share_stack).owner == aco_gtls_co) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    (*(*aco_gtls_co).share_stack).owner = 0 as *mut aco_t;
    (*(*aco_gtls_co).share_stack).align_validsz = 0 as libc::c_int as size_t;
    if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    if (0 as libc::c_int != 0) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
}
unsafe fn main_0() -> libc::c_int {
    aco_thread_init(None);
    let mut seed_t: time_t = time(0 as *mut time_t);
    if (-(1 as libc::c_int) as time_t != seed_t) as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    srand(seed_t as libc::c_uint);
    let mut co_amount: size_t = 100 as libc::c_int as size_t;
    if (co_amount > 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long
        != 0
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
    while ct < co_amount {
        let ref mut fresh0 = *coarray.offset(ct as isize);
        *fresh0 = aco_create(
            main_co,
            sstk,
            0 as libc::c_int as size_t,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                aco_cofuncp_t,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(co_fp0),
                ),
            ),
            (rand() as uintptr_t).wrapping_rem(1000 as libc::c_int as libc::c_ulong)
                as *mut libc::c_void,
        );
        ct = ct.wrapping_add(1);
        ct;
    }
    printf(
        b"scheduler start: co_amount:%zu\n\0" as *const u8 as *const libc::c_char,
        co_amount,
    );
    let mut null_ct: size_t = 0 as libc::c_int as size_t;
    's_55: loop {
        ct = 0 as libc::c_int as size_t;
        while ct < co_amount {
            if !(*coarray.offset(ct as isize)).is_null() {
                aco_resume(*coarray.offset(ct as isize));
                null_ct = 0 as libc::c_int as size_t;
                if (**coarray.offset(ct as isize)).is_end as libc::c_int
                    != 0 as libc::c_int
                {
                    printf(
                        b"aco_destroy: co:%zu\n\0" as *const u8 as *const libc::c_char,
                        ct,
                    );
                    aco_destroy(*coarray.offset(ct as isize));
                    let ref mut fresh1 = *coarray.offset(ct as isize);
                    *fresh1 = 0 as *mut aco_t;
                }
            } else {
                null_ct = null_ct.wrapping_add(1);
                null_ct;
                if null_ct >= co_amount {
                    break 's_55;
                }
            }
            ct = ct.wrapping_add(1);
            ct;
        }
    }
    ct = 0 as libc::c_int as size_t;
    while ct < co_amount {
        if (*coarray.offset(ct as isize)).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            abort();
        };
        ct = ct.wrapping_add(1);
        ct;
    }
    aco_share_stack_destroy(sstk);
    sstk = 0 as *mut aco_share_stack_t;
    aco_destroy(main_co);
    main_co = 0 as *mut aco_t;
    free(coarray as *mut libc::c_void);
    printf(b"sheduler exit\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

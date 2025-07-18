use ::libc;
extern "C" {
    fn abort() -> !;
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
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn foo(mut ct: libc::c_int) {
    printf(
        b"co: %p: yield to main_co: %d\n\0" as *const u8 as *const libc::c_char,
        ({ aco_gtls_co }),
        *((*aco_gtls_co).arg as *mut libc::c_int),
    );
    if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    *((*aco_gtls_co).arg as *mut libc::c_int) = ct + 1 as libc::c_int;
}
pub unsafe extern "C" fn co_fp0() {
    printf(
        b"co: %p: entry: %d\n\0" as *const u8 as *const libc::c_char,
        ({ aco_gtls_co }),
        *((*aco_gtls_co).arg as *mut libc::c_int),
    );
    let mut ct: libc::c_int = 0 as libc::c_int;
    while ct < 6 as libc::c_int {
        foo(ct);
        ct += 1;
        ct;
    }
    printf(
        b"co: %p:  exit to main_co: %d\n\0" as *const u8 as *const libc::c_char,
        ({ aco_gtls_co }),
        *((*aco_gtls_co).arg as *mut libc::c_int),
    );
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
    let mut co_ct_arg_point_to_me: libc::c_int = 0 as libc::c_int;
    let mut co: *mut aco_t = aco_create(
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
        &mut co_ct_arg_point_to_me as *mut libc::c_int as *mut libc::c_void,
    );
    let mut ct: libc::c_int = 0 as libc::c_int;
    while ct < 6 as libc::c_int {
        if ((*co).is_end as libc::c_int == 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {} else {
            abort();
        };
        printf(
            b"main_co: yield to co: %p: %d\n\0" as *const u8 as *const libc::c_char,
            co,
            ct,
        );
        aco_resume(co);
        if (co_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        ct += 1;
        ct;
    }
    printf(
        b"main_co: yield to co: %p: %d\n\0" as *const u8 as *const libc::c_char,
        co,
        ct,
    );
    aco_resume(co);
    if (co_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    if ((*co).is_end != 0) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    printf(b"main_co: destroy and exit\n\0" as *const u8 as *const libc::c_char);
    aco_destroy(co);
    co = 0 as *mut aco_t;
    aco_share_stack_destroy(sstk);
    sstk = 0 as *mut aco_share_stack_t;
    aco_destroy(main_co);
    main_co = 0 as *mut aco_t;
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

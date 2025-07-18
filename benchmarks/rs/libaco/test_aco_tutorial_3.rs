use ::libc;
extern "C" {
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
    fn abort() -> !;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn aco_thread_init(last_word_co_fp: aco_cofuncp_t);
    fn acosw(from_co: *mut aco_t, to_co: *mut aco_t) -> *mut libc::c_void;
    fn aco_share_stack_new(sz: size_t) -> *mut aco_share_stack_t;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
pub static mut gl_race_aco_yield_ct: uint64_t = 0 as libc::c_int as uint64_t;
pub static mut gl_race_aco_yield_ct_mutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub unsafe extern "C" fn foo(mut ct: libc::c_int) {
    printf(
        b"co:%p save_stack:%p share_stack:%p yield_ct:%d\n\0" as *const u8
            as *const libc::c_char,
        ({ aco_gtls_co }),
        (*({ aco_gtls_co })).save_stack.ptr,
        (*(*({ aco_gtls_co })).share_stack).ptr,
        ct,
    );
    pthread_mutex_lock(&mut gl_race_aco_yield_ct_mutex);
    gl_race_aco_yield_ct = gl_race_aco_yield_ct.wrapping_add(1);
    gl_race_aco_yield_ct;
    pthread_mutex_unlock(&mut gl_race_aco_yield_ct_mutex);
    if !aco_gtls_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    if !((*aco_gtls_co).main_co).is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    let ref mut fresh0 = *((*aco_gtls_co).arg as *mut libc::c_int);
    *fresh0 += 1;
    *fresh0;
}
pub unsafe extern "C" fn co_fp0() {
    let mut this_co: *mut aco_t = ({ aco_gtls_co });
    if (({ ((*this_co).main_co == 0 as *mut libc::c_void as *mut aco_t) as libc::c_int })
        == 0) as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    if ((*this_co).fp
        == ::std::mem::transmute::<
            *mut libc::c_void,
            aco_cofuncp_t,
        >(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                *mut libc::c_void,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(co_fp0),
                ),
            ),
        )) as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    if ((*this_co).is_end as libc::c_int == 0 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    let mut ct: libc::c_int = 0 as libc::c_int;
    while ct < 6 as libc::c_int {
        foo(ct);
        ct += 1;
        ct;
    }
    printf(
        b"co:%p save_stack:%p share_stack:%p co_exit()\n\0" as *const u8
            as *const libc::c_char,
        this_co,
        (*this_co).save_stack.ptr,
        (*(*this_co).share_stack).ptr,
    );
    pthread_mutex_lock(&mut gl_race_aco_yield_ct_mutex);
    gl_race_aco_yield_ct = gl_race_aco_yield_ct.wrapping_add(1);
    gl_race_aco_yield_ct;
    pthread_mutex_unlock(&mut gl_race_aco_yield_ct_mutex);
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
    if (0 as libc::c_int != 0) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
}
pub unsafe extern "C" fn pmain(
    mut pthread_in_arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut t: pthread_t = pthread_self();
    let mut idx: size_t = 0 as libc::c_int as size_t;
    if (::std::mem::size_of::<pthread_t>() as libc::c_ulong
        > 0 as libc::c_int as libc::c_ulong) as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    printf(b"\ntid:0x\0" as *const u8 as *const libc::c_char);
    while idx < ::std::mem::size_of::<pthread_t>() as libc::c_ulong {
        printf(
            b"%02x\0" as *const u8 as *const libc::c_char,
            *(&mut t as *mut pthread_t as *mut uint8_t).offset(idx as isize)
                as libc::c_int,
        );
        idx = idx.wrapping_add(1);
        idx;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    aco_thread_init(None);
    let mut main_co: *mut aco_t = aco_create(
        0 as *mut aco_t,
        0 as *mut aco_share_stack_t,
        0 as libc::c_int as size_t,
        None,
        0 as *mut libc::c_void,
    );
    if !main_co.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    let mut sstk: *mut aco_share_stack_t = aco_share_stack_new(
        0 as libc::c_int as size_t,
    );
    if !sstk.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    let mut sstk2: *mut aco_share_stack_t = aco_share_stack_new(
        0 as libc::c_int as size_t,
    );
    if !sstk2.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    let mut co_ct_arg_point_to_me: libc::c_int = 0 as libc::c_int;
    let mut co2_ct_arg_point_to_me: libc::c_int = 0 as libc::c_int;
    let mut co3_ct_arg_point_to_me: libc::c_int = 0 as libc::c_int;
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
    if !co.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    let mut co2: *mut aco_t = aco_create(
        main_co,
        sstk2,
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
        &mut co2_ct_arg_point_to_me as *mut libc::c_int as *mut libc::c_void,
    );
    let mut co3: *mut aco_t = aco_create(
        main_co,
        sstk2,
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
        &mut co3_ct_arg_point_to_me as *mut libc::c_int as *mut libc::c_void,
    );
    if !co2.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    if !co3.is_null() as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    let mut ct: libc::c_int = 0 as libc::c_int;
    while ct < 6 as libc::c_int {
        if ((*co).is_end as libc::c_int == 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {} else {
            abort();
        };
        aco_resume(co);
        if (co_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        if ((*co2).is_end as libc::c_int == 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {} else {
            abort();
        };
        aco_resume(co2);
        if (co2_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        if ((*co3).is_end as libc::c_int == 0 as libc::c_int) as libc::c_int
            as libc::c_long != 0
        {} else {
            abort();
        };
        aco_resume(co3);
        if (co3_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long != 0 {} else {
            abort();
        };
        printf(b"main_co:%p\n\0" as *const u8 as *const libc::c_char, main_co);
        ct += 1;
        ct;
    }
    aco_resume(co);
    if (co_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    if ((*co).is_end != 0) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    aco_resume(co2);
    if (co2_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    if ((*co2).is_end != 0) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    aco_resume(co3);
    if (co3_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    if ((*co3).is_end != 0) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
    printf(b"main_co:%p\n\0" as *const u8 as *const libc::c_char, main_co);
    printf(
        b"\ncopy-stack co:%p:\n    max stack copy size:%zu\n    save (from share stack to save stack) counter of the private save stack:%zu\n    restore (from save stack to share stack) counter of the private save stack:%zu\n\0"
            as *const u8 as *const libc::c_char,
        co,
        (*co).save_stack.max_cpsz,
        (*co).save_stack.ct_save,
        (*co).save_stack.ct_restore,
    );
    printf(
        b"\n(Since the share stack used by the co has only one user `co`, so there is no need to save/restore the stack every time during resume & yield execution, thus you can call it a co has 'standalone stack' which just is a very special case of copy-stack.)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"\ncopy-stack co2:%p:\n    max stack copy size:%zu\n    save (from share stack to save stack) counter of the private save stack:%zu\n    restore (from save stack to share stack) counter of the private save stack:%zu\n\0"
            as *const u8 as *const libc::c_char,
        co2,
        (*co2).save_stack.max_cpsz,
        (*co2).save_stack.ct_save,
        (*co2).save_stack.ct_restore,
    );
    printf(
        b"\ncopy-stack co3:%p:\n    max stack copy size:%zu\n    save (from share stack to save stack) counter of the private save stack:%zu\n    restore (from save stack to share stack) counter of the private save stack:%zu\n\0"
            as *const u8 as *const libc::c_char,
        co3,
        (*co3).save_stack.max_cpsz,
        (*co3).save_stack.ct_save,
        (*co3).save_stack.ct_restore,
    );
    printf(
        b"\n(The co2 & co3 share the share stack sstk2, thus it is necessary to save/restore the stack every time during resume & yield execution, thus it is a ordinary case of copy-stack.)\n\0"
            as *const u8 as *const libc::c_char,
    );
    let mut tmp_gl_ct: uint64_t = 0;
    pthread_mutex_lock(&mut gl_race_aco_yield_ct_mutex);
    tmp_gl_ct = gl_race_aco_yield_ct;
    pthread_mutex_unlock(&mut gl_race_aco_yield_ct_mutex);
    printf(
        b"\ngl_race_aco_yield_ct:%lu\n\0" as *const u8 as *const libc::c_char,
        tmp_gl_ct,
    );
    aco_destroy(co);
    co = 0 as *mut aco_t;
    aco_destroy(co2);
    co2 = 0 as *mut aco_t;
    aco_destroy(co3);
    co3 = 0 as *mut aco_t;
    aco_share_stack_destroy(sstk);
    sstk = 0 as *mut aco_share_stack_t;
    aco_share_stack_destroy(sstk2);
    sstk2 = 0 as *mut aco_share_stack_t;
    aco_destroy(main_co);
    main_co = 0 as *mut aco_t;
    return 0 as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    let mut t1: pthread_t = 0;
    let mut t2: pthread_t = 0;
    if (0 as libc::c_int
        == pthread_create(
            &mut t1,
            0 as *const pthread_attr_t,
            Some(pmain as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            0 as *mut libc::c_void,
        )) as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    if (0 as libc::c_int
        == pthread_create(
            &mut t2,
            0 as *const pthread_attr_t,
            Some(pmain as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            0 as *mut libc::c_void,
        )) as libc::c_int as libc::c_long != 0
    {} else {
        abort();
    };
    if (0 as libc::c_int == pthread_join(t1, 0 as *mut *mut libc::c_void)) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    if (0 as libc::c_int == pthread_join(t2, 0 as *mut *mut libc::c_void)) as libc::c_int
        as libc::c_long != 0
    {} else {
        abort();
    };
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}

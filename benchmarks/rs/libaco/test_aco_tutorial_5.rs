use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn abort() -> !;
    static mut stderr: *mut FILE;
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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub unsafe extern "C" fn foo(mut ct: libc::c_int) {
    printf(
        b"co:%p save_stack:%p share_stack:%p yield_ct:%d\n\0" as *const u8
            as *const libc::c_char,
        ({ aco_gtls_co }),
        (*({ aco_gtls_co })).save_stack.ptr,
        (*(*({ aco_gtls_co })).share_stack).ptr,
        ct,
    );
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
        b"co:%p save_stack:%p share_stack:%p !offending return!\n\0" as *const u8
            as *const libc::c_char,
        this_co,
        (*this_co).save_stack.ptr,
        (*(*this_co).share_stack).ptr,
    );
    printf(
        b"Intended to Abort to test the aco protector :)\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn co_protector_last_word() {
    let mut co: *mut aco_t = ({ aco_gtls_co });
    fprintf(
        stderr,
        b"error: customized co_protector_last_word triggered \n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"error: co:%p should call `aco_exit(co)` instead of direct `return` in co_fp:%p to finish its execution\n\0"
            as *const u8 as *const libc::c_char,
        co,
        ::std::mem::transmute::<aco_cofuncp_t, *mut libc::c_void>((*co).fp),
    );
    if (0 as libc::c_int != 0) as libc::c_int as libc::c_long != 0 {} else {
        abort();
    };
}
unsafe fn main_0() -> libc::c_int {
    aco_thread_init(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            aco_cofuncp_t,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(co_protector_last_word),
            ),
        ),
    );
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
    if !co.is_null() as libc::c_int as libc::c_long != 0 {} else {
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

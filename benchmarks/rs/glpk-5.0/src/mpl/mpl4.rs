use ::libc;
extern "C" {
    pub type AVL;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type DMP;
    pub type glp_file;
    fn _glp_avl_create_tree(
        fcmp: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
        >,
        info: *mut libc::c_void,
    ) -> *mut AVL;
    fn _glp_avl_strcmp(
        info: *mut libc::c_void,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn _glp_avl_delete_tree(tree: *mut AVL);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_dmp_in_use(pool: *mut DMP) -> size_t;
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_get_err_msg() -> *const libc::c_char;
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_open(name: *const libc::c_char, mode: *const libc::c_char) -> *mut glp_file;
    fn _glp_ioerr(f: *mut glp_file) -> libc::c_int;
    fn _glp_getc(f: *mut glp_file) -> libc::c_int;
    fn _glp_format(f: *mut glp_file, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _glp_rng_create_rand() -> *mut RNG;
    fn _glp_rng_delete_rand(rand: *mut RNG);
    fn _glp_mpl_print_context(mpl: *mut MPL);
    fn _glp_mpl_get_char(mpl: *mut MPL);
    fn _glp_mpl_get_token(mpl: *mut MPL);
    fn _glp_mpl_is_keyword(mpl: *mut MPL, keyword: *mut libc::c_char) -> libc::c_int;
    fn _glp_mpl_end_statement(mpl: *mut MPL);
    fn _glp_mpl_model_section(mpl: *mut MPL);
    fn _glp_mpl_is_literal(mpl: *mut MPL, literal: *mut libc::c_char) -> libc::c_int;
    fn _glp_mpl_data_section(mpl: *mut MPL);
    fn _glp_mpl_format_tuple(
        mpl: *mut MPL,
        c: libc::c_int,
        tuple: *mut TUPLE,
    ) -> *mut libc::c_char;
    fn _glp_mpl_create_array(
        mpl: *mut MPL,
        type_0: libc::c_int,
        dim: libc::c_int,
    ) -> *mut ARRAY;
    fn _glp_mpl_free_dca(mpl: *mut MPL);
    fn _glp_mpl_execute_statement(mpl: *mut MPL, stmt: *mut STATEMENT);
    fn _glp_mpl_clean_statement(mpl: *mut MPL, stmt: *mut STATEMENT);
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
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
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub A: [libc::c_int; 56],
    pub fptr: *mut libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_tran {
    pub line: libc::c_int,
    pub c: libc::c_int,
    pub token: libc::c_int,
    pub imlen: libc::c_int,
    pub image: *mut libc::c_char,
    pub value: libc::c_double,
    pub b_token: libc::c_int,
    pub b_imlen: libc::c_int,
    pub b_image: *mut libc::c_char,
    pub b_value: libc::c_double,
    pub f_dots: libc::c_int,
    pub f_scan: libc::c_int,
    pub f_token: libc::c_int,
    pub f_imlen: libc::c_int,
    pub f_image: *mut libc::c_char,
    pub f_value: libc::c_double,
    pub context: *mut libc::c_char,
    pub c_ptr: libc::c_int,
    pub flag_d: libc::c_int,
    pub pool: *mut DMP,
    pub tree: *mut AVL,
    pub model: *mut STATEMENT,
    pub flag_x: libc::c_int,
    pub as_within: libc::c_int,
    pub as_in: libc::c_int,
    pub as_binary: libc::c_int,
    pub flag_s: libc::c_int,
    pub strings: *mut DMP,
    pub symbols: *mut DMP,
    pub tuples: *mut DMP,
    pub arrays: *mut DMP,
    pub members: *mut DMP,
    pub elemvars: *mut DMP,
    pub formulae: *mut DMP,
    pub elemcons: *mut DMP,
    pub a_list: *mut ARRAY,
    pub sym_buf: *mut libc::c_char,
    pub tup_buf: *mut libc::c_char,
    pub rand: *mut RNG,
    pub flag_p: libc::c_int,
    pub stmt: *mut STATEMENT,
    pub dca: *mut TABDCA,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub row: *mut *mut ELEMCON,
    pub col: *mut *mut ELEMVAR,
    pub in_fp: *mut glp_file,
    pub in_file: *mut libc::c_char,
    pub out_fp: *mut glp_file,
    pub out_file: *mut libc::c_char,
    pub prt_fp: *mut glp_file,
    pub prt_file: *mut libc::c_char,
    pub jump: jmp_buf,
    pub phase: libc::c_int,
    pub mod_file: *mut libc::c_char,
    pub mpl_buf: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMVAR {
    pub j: libc::c_int,
    pub var: *mut VARIABLE,
    pub memb: *mut MEMBER,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub temp: libc::c_double,
    pub stat: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MEMBER {
    pub tuple: *mut TUPLE,
    pub next: *mut MEMBER,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union VALUE {
    pub none: *mut libc::c_void,
    pub num: libc::c_double,
    pub sym: *mut SYMBOL,
    pub bit: libc::c_int,
    pub tuple: *mut TUPLE,
    pub set: *mut ELEMSET,
    pub var: *mut ELEMVAR,
    pub form: *mut FORMULA,
    pub con: *mut ELEMCON,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMCON {
    pub i: libc::c_int,
    pub con: *mut CONSTRAINT,
    pub memb: *mut MEMBER,
    pub form: *mut FORMULA,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub stat: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FORMULA {
    pub coef: libc::c_double,
    pub var: *mut ELEMVAR,
    pub next: *mut FORMULA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONSTRAINT {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub code: *mut CODE,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARRAY {
    pub type_0: libc::c_int,
    pub dim: libc::c_int,
    pub size: libc::c_int,
    pub head: *mut MEMBER,
    pub tail: *mut MEMBER,
    pub tree: *mut AVL,
    pub prev: *mut ARRAY,
    pub next: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CODE {
    pub op: libc::c_int,
    pub arg: OPERANDS,
    pub type_0: libc::c_int,
    pub dim: libc::c_int,
    pub up: *mut CODE,
    pub vflag: libc::c_int,
    pub valid: libc::c_int,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OPERANDS {
    pub num: libc::c_double,
    pub str_0: *mut libc::c_char,
    pub index: C2RustUnnamed_6,
    pub par: C2RustUnnamed_5,
    pub set: C2RustUnnamed_4,
    pub var: C2RustUnnamed_3,
    pub con: C2RustUnnamed_2,
    pub list: *mut ARG_LIST,
    pub slice: *mut DOMAIN_BLOCK,
    pub arg: C2RustUnnamed_1,
    pub loop_0: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub domain: *mut DOMAIN1,
    pub x: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN1 {
    pub list: *mut DOMAIN_BLOCK,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_BLOCK {
    pub list: *mut DOMAIN_SLOT,
    pub code: *mut CODE,
    pub backup: *mut TUPLE,
    pub next: *mut DOMAIN_BLOCK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TUPLE {
    pub sym: *mut SYMBOL,
    pub next: *mut TUPLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYMBOL {
    pub num: libc::c_double,
    pub str_0: *mut STRING,
}
pub type STRING = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_SLOT {
    pub name: *mut libc::c_char,
    pub code: *mut CODE,
    pub value: *mut SYMBOL,
    pub list: *mut CODE,
    pub next: *mut DOMAIN_SLOT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub x: *mut CODE,
    pub y: *mut CODE,
    pub z: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARG_LIST {
    pub x: *mut CODE,
    pub next: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub con: *mut CONSTRAINT,
    pub list: *mut ARG_LIST,
    pub suff: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub var: *mut VARIABLE,
    pub list: *mut ARG_LIST,
    pub suff: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VARIABLE {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub set: *mut SET,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SET {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub dimen: libc::c_int,
    pub within: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub gadget: *mut GADGET,
    pub data: libc::c_int,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GADGET {
    pub set: *mut SET,
    pub ind: [libc::c_int; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WITHIN {
    pub code: *mut CODE,
    pub next: *mut WITHIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub par: *mut PARAMETER,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PARAMETER {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub dim: libc::c_int,
    pub domain: *mut DOMAIN1,
    pub type_0: libc::c_int,
    pub cond: *mut CONDITION,
    pub in_0: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub data: libc::c_int,
    pub defval: *mut SYMBOL,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONDITION {
    pub rho: libc::c_int,
    pub code: *mut CODE,
    pub next: *mut CONDITION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub slot: *mut DOMAIN_SLOT,
    pub next: *mut CODE,
}
pub type ELEMSET = ARRAY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDCA {
    pub id: libc::c_int,
    pub link: *mut libc::c_void,
    pub na: libc::c_int,
    pub arg: *mut *mut libc::c_char,
    pub nf: libc::c_int,
    pub name: *mut *mut libc::c_char,
    pub type_0: *mut libc::c_int,
    pub num: *mut libc::c_double,
    pub str_0: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STATEMENT {
    pub line: libc::c_int,
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_7,
    pub next: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub tab: *mut TABLE,
    pub slv: *mut libc::c_void,
    pub chk: *mut CHECK,
    pub dpy: *mut DISPLAY,
    pub prt: *mut PRINTF,
    pub fur: *mut FOR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FOR {
    pub domain: *mut DOMAIN1,
    pub list: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF {
    pub domain: *mut DOMAIN1,
    pub fmt: *mut CODE,
    pub list: *mut PRINTF1,
    pub fname: *mut CODE,
    pub app: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF1 {
    pub code: *mut CODE,
    pub next: *mut PRINTF1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY {
    pub domain: *mut DOMAIN1,
    pub list: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY1 {
    pub type_0: libc::c_int,
    pub u: C2RustUnnamed_8,
    pub next: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub slot: *mut DOMAIN_SLOT,
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHECK {
    pub domain: *mut DOMAIN1,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABLE {
    pub name: *mut libc::c_char,
    pub alias: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub arg: *mut TABARG,
    pub u: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub in_0: C2RustUnnamed_11,
    pub out: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub domain: *mut DOMAIN1,
    pub list: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABOUT {
    pub code: *mut CODE,
    pub name: *mut libc::c_char,
    pub next: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub set: *mut SET,
    pub fld: *mut TABFLD,
    pub list: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABIN {
    pub par: *mut PARAMETER,
    pub name: *mut libc::c_char,
    pub next: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABFLD {
    pub name: *mut libc::c_char,
    pub next: *mut TABFLD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABARG {
    pub code: *mut CODE,
    pub next: *mut TABARG,
}
pub type MPL = glp_tran;
pub unsafe extern "C" fn _glp_mpl_alloc_content(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    stmt = (*mpl).model;
    while !stmt.is_null() {
        match (*stmt).type_0 {
            122 => {
                (((*(*stmt).u.set).array).is_null()
                    || {
                        glp_assert_(
                            b"stmt->u.set->array == NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                            48 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*(*stmt).u.set)
                    .array = _glp_mpl_create_array(
                    mpl,
                    106 as libc::c_int,
                    (*(*stmt).u.set).dim,
                );
            }
            120 => {
                (((*(*stmt).u.par).array).is_null()
                    || {
                        glp_assert_(
                            b"stmt->u.par->array == NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                            54 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                match (*(*stmt).u.par).type_0 {
                    118 | 113 | 101 => {
                        (*(*stmt).u.par)
                            .array = _glp_mpl_create_array(
                            mpl,
                            118 as libc::c_int,
                            (*(*stmt).u.par).dim,
                        );
                    }
                    124 => {
                        (*(*stmt).u.par)
                            .array = _glp_mpl_create_array(
                            mpl,
                            124 as libc::c_int,
                            (*(*stmt).u.par).dim,
                        );
                    }
                    _ => {
                        (stmt != stmt
                            || {
                                glp_assert_(
                                    b"stmt != stmt\0" as *const u8 as *const libc::c_char,
                                    b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                                    67 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
            }
            127 => {
                (((*(*stmt).u.var).array).is_null()
                    || {
                        glp_assert_(
                            b"stmt->u.var->array == NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                            72 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*(*stmt).u.var)
                    .array = _glp_mpl_create_array(
                    mpl,
                    107 as libc::c_int,
                    (*(*stmt).u.var).dim,
                );
            }
            103 => {
                (((*(*stmt).u.con).array).is_null()
                    || {
                        glp_assert_(
                            b"stmt->u.con->array == NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                            78 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*(*stmt).u.con)
                    .array = _glp_mpl_create_array(
                    mpl,
                    105 as libc::c_int,
                    (*(*stmt).u.con).dim,
                );
            }
            125 | 123 | 102 | 104 | 121 | 109 => {}
            _ => {
                (stmt != stmt
                    || {
                        glp_assert_(
                            b"stmt != stmt\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                            93 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        stmt = (*stmt).next;
    }
}
pub unsafe extern "C" fn _glp_mpl_generate_model(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    ((*mpl).flag_p == 0
        || {
            glp_assert_(
                b"!mpl->flag_p\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                107 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    stmt = (*mpl).model;
    while !stmt.is_null() {
        _glp_mpl_execute_statement(mpl, stmt);
        if (*(*mpl).stmt).type_0 == 123 as libc::c_int {
            break;
        }
        stmt = (*stmt).next;
    }
    (*mpl).stmt = stmt;
}
pub unsafe extern "C" fn _glp_mpl_build_problem(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    let mut v: *mut VARIABLE = 0 as *mut VARIABLE;
    let mut c: *mut CONSTRAINT = 0 as *mut CONSTRAINT;
    let mut t: *mut FORMULA = 0 as *mut FORMULA;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    ((*mpl).m == 0 as libc::c_int
        || {
            glp_assert_(
                b"mpl->m == 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                129 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*mpl).n == 0 as libc::c_int
        || {
            glp_assert_(
                b"mpl->n == 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                130 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (((*mpl).row).is_null()
        || {
            glp_assert_(
                b"mpl->row == NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                131 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (((*mpl).col).is_null()
        || {
            glp_assert_(
                b"mpl->col == NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    stmt = (*mpl).model;
    while !stmt.is_null() {
        if (*stmt).type_0 == 127 as libc::c_int {
            v = (*stmt).u.var;
            memb = (*(*v).array).head;
            while !memb.is_null() {
                ((*(*memb).value.var).j == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"memb->value.var->j == 0\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                            138 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                memb = (*memb).next;
            }
        }
        stmt = (*stmt).next;
    }
    stmt = (*mpl).model;
    while !stmt.is_null() {
        if (*stmt).type_0 == 103 as libc::c_int {
            c = (*stmt).u.con;
            memb = (*(*c).array).head;
            while !memb.is_null() {
                ((*(*memb).value.con).i == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"memb->value.con->i == 0\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                            146 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*mpl).m += 1;
                (*(*memb).value.con).i = (*mpl).m;
                t = (*(*memb).value.con).form;
                while !t.is_null() {
                    (!((*t).var).is_null()
                        || {
                            glp_assert_(
                                b"t->var != NULL\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                                151 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    (*(*(*(*t).var).memb).value.var).j = -(1 as libc::c_int);
                    t = (*t).next;
                }
                memb = (*memb).next;
            }
        }
        stmt = (*stmt).next;
    }
    stmt = (*mpl).model;
    while !stmt.is_null() {
        if (*stmt).type_0 == 127 as libc::c_int {
            v = (*stmt).u.var;
            memb = (*(*v).array).head;
            while !memb.is_null() {
                if (*(*memb).value.var).j != 0 as libc::c_int {
                    (*mpl).n += 1;
                    (*(*memb).value.var).j = (*mpl).n;
                }
                memb = (*memb).next;
            }
        }
        stmt = (*stmt).next;
    }
    (*mpl)
        .row = glp_alloc(
        1 as libc::c_int + (*mpl).m,
        ::std::mem::size_of::<*mut ELEMCON>() as libc::c_ulong as libc::c_int,
    ) as *mut *mut ELEMCON;
    i = 1 as libc::c_int;
    while i <= (*mpl).m {
        let ref mut fresh0 = *((*mpl).row).offset(i as isize);
        *fresh0 = 0 as *mut ELEMCON;
        i += 1;
        i;
    }
    stmt = (*mpl).model;
    while !stmt.is_null() {
        if (*stmt).type_0 == 103 as libc::c_int {
            c = (*stmt).u.con;
            memb = (*(*c).array).head;
            while !memb.is_null() {
                i = (*(*memb).value.con).i;
                (1 as libc::c_int <= i && i <= (*mpl).m
                    || {
                        glp_assert_(
                            b"1 <= i && i <= mpl->m\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                            174 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                ((*((*mpl).row).offset(i as isize)).is_null()
                    || {
                        glp_assert_(
                            b"mpl->row[i] == NULL\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                            175 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                let ref mut fresh1 = *((*mpl).row).offset(i as isize);
                *fresh1 = (*memb).value.con;
                memb = (*memb).next;
            }
        }
        stmt = (*stmt).next;
    }
    i = 1 as libc::c_int;
    while i <= (*mpl).m {
        (!(*((*mpl).row).offset(i as isize)).is_null()
            || {
                glp_assert_(
                    b"mpl->row[i] != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                    180 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        i += 1;
        i;
    }
    (*mpl)
        .col = glp_alloc(
        1 as libc::c_int + (*mpl).n,
        ::std::mem::size_of::<*mut ELEMVAR>() as libc::c_ulong as libc::c_int,
    ) as *mut *mut ELEMVAR;
    j = 1 as libc::c_int;
    while j <= (*mpl).n {
        let ref mut fresh2 = *((*mpl).col).offset(j as isize);
        *fresh2 = 0 as *mut ELEMVAR;
        j += 1;
        j;
    }
    stmt = (*mpl).model;
    while !stmt.is_null() {
        if (*stmt).type_0 == 127 as libc::c_int {
            v = (*stmt).u.var;
            memb = (*(*v).array).head;
            while !memb.is_null() {
                j = (*(*memb).value.var).j;
                if !(j == 0 as libc::c_int) {
                    (1 as libc::c_int <= j && j <= (*mpl).n
                        || {
                            glp_assert_(
                                b"1 <= j && j <= mpl->n\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                                190 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    ((*((*mpl).col).offset(j as isize)).is_null()
                        || {
                            glp_assert_(
                                b"mpl->col[j] == NULL\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                                191 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    let ref mut fresh3 = *((*mpl).col).offset(j as isize);
                    *fresh3 = (*memb).value.var;
                }
                memb = (*memb).next;
            }
        }
        stmt = (*stmt).next;
    }
    j = 1 as libc::c_int;
    while j <= (*mpl).n {
        (!(*((*mpl).col).offset(j as isize)).is_null()
            || {
                glp_assert_(
                    b"mpl->col[j] != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                    196 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j += 1;
        j;
    }
}
pub unsafe extern "C" fn _glp_mpl_postsolve_model(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    ((*mpl).flag_p == 0
        || {
            glp_assert_(
                b"!mpl->flag_p\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                208 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*mpl).flag_p = 1 as libc::c_int;
    stmt = (*mpl).stmt;
    while !stmt.is_null() {
        _glp_mpl_execute_statement(mpl, stmt);
        stmt = (*stmt).next;
    }
    (*mpl).stmt = 0 as *mut STATEMENT;
}
pub unsafe extern "C" fn _glp_mpl_clean_model(mut mpl: *mut MPL) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    stmt = (*mpl).model;
    while !stmt.is_null() {
        _glp_mpl_clean_statement(mpl, stmt);
        stmt = (*stmt).next;
    }
    if _glp_dmp_in_use((*mpl).strings) != 0 as libc::c_int as libc::c_ulong {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d string segment(s) were lost\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            _glp_dmp_in_use((*mpl).strings),
        );
    }
    if _glp_dmp_in_use((*mpl).symbols) != 0 as libc::c_int as libc::c_ulong {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d symbol(s) were lost\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            _glp_dmp_in_use((*mpl).symbols),
        );
    }
    if _glp_dmp_in_use((*mpl).tuples) != 0 as libc::c_int as libc::c_ulong {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d n-tuple component(s) were lost\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            _glp_dmp_in_use((*mpl).tuples),
        );
    }
    if _glp_dmp_in_use((*mpl).arrays) != 0 as libc::c_int as libc::c_ulong {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d array(s) were lost\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            _glp_dmp_in_use((*mpl).arrays),
        );
    }
    if _glp_dmp_in_use((*mpl).members) != 0 as libc::c_int as libc::c_ulong {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d array member(s) were lost\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            _glp_dmp_in_use((*mpl).members),
        );
    }
    if _glp_dmp_in_use((*mpl).elemvars) != 0 as libc::c_int as libc::c_ulong {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d elemental variable(s) were lost\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            _glp_dmp_in_use((*mpl).elemvars),
        );
    }
    if _glp_dmp_in_use((*mpl).formulae) != 0 as libc::c_int as libc::c_ulong {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d linear term(s) were lost\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            _glp_dmp_in_use((*mpl).formulae),
        );
    }
    if _glp_dmp_in_use((*mpl).elemcons) != 0 as libc::c_int as libc::c_ulong {
        _glp_mpl_error(
            mpl,
            b"internal logic error: %d elemental constraint(s) were lost\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            _glp_dmp_in_use((*mpl).elemcons),
        );
    }
}
pub unsafe extern "C" fn _glp_mpl_open_input(
    mut mpl: *mut MPL,
    mut file: *mut libc::c_char,
) {
    (*mpl).line = 0 as libc::c_int;
    (*mpl).c = '\n' as i32;
    (*mpl).token = 0 as libc::c_int;
    (*mpl).imlen = 0 as libc::c_int;
    *((*mpl).image).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*mpl).value = 0.0f64;
    (*mpl).b_token = 201 as libc::c_int;
    (*mpl).b_imlen = 0 as libc::c_int;
    *((*mpl).b_image).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*mpl).b_value = 0.0f64;
    (*mpl).f_dots = 0 as libc::c_int;
    (*mpl).f_scan = 0 as libc::c_int;
    (*mpl).f_token = 0 as libc::c_int;
    (*mpl).f_imlen = 0 as libc::c_int;
    *((*mpl).f_image).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*mpl).f_value = 0.0f64;
    memset(
        (*mpl).context as *mut libc::c_void,
        ' ' as i32,
        60 as libc::c_int as libc::c_ulong,
    );
    (*mpl).c_ptr = 0 as libc::c_int;
    (((*mpl).in_fp).is_null()
        || {
            glp_assert_(
                b"mpl->in_fp == NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                289 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*mpl).in_fp = _glp_open(file, b"r\0" as *const u8 as *const libc::c_char);
    if ((*mpl).in_fp).is_null() {
        _glp_mpl_error(
            mpl,
            b"unable to open %s - %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            file,
            _glp_get_err_msg(),
        );
    }
    (*mpl).in_file = file;
    _glp_mpl_get_char(mpl);
    _glp_mpl_get_token(mpl);
}
pub unsafe extern "C" fn _glp_mpl_read_char(mut mpl: *mut MPL) -> libc::c_int {
    let mut c: libc::c_int = 0;
    (!((*mpl).in_fp).is_null()
        || {
            glp_assert_(
                b"mpl->in_fp != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                309 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    c = _glp_getc((*mpl).in_fp);
    if c < 0 as libc::c_int {
        if _glp_ioerr((*mpl).in_fp) != 0 {
            _glp_mpl_error(
                mpl,
                b"read error on %s - %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*mpl).in_file,
                _glp_get_err_msg(),
            );
        }
        c = -(1 as libc::c_int);
    }
    return c;
}
pub unsafe extern "C" fn _glp_mpl_close_input(mut mpl: *mut MPL) {
    (!((*mpl).in_fp).is_null()
        || {
            glp_assert_(
                b"mpl->in_fp != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_close((*mpl).in_fp);
    (*mpl).in_fp = 0 as *mut glp_file;
    (*mpl).in_file = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn _glp_mpl_open_output(
    mut mpl: *mut MPL,
    mut file: *mut libc::c_char,
) {
    (((*mpl).out_fp).is_null()
        || {
            glp_assert_(
                b"mpl->out_fp == NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                340 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if file.is_null() {
        file = b"<stdout>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*mpl).out_fp = stdout as *mut libc::c_void as *mut glp_file;
    } else {
        (*mpl).out_fp = _glp_open(file, b"w\0" as *const u8 as *const libc::c_char);
        if ((*mpl).out_fp).is_null() {
            _glp_mpl_error(
                mpl,
                b"unable to create %s - %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                file,
                _glp_get_err_msg(),
            );
        }
    }
    (*mpl)
        .out_file = glp_alloc(
        1 as libc::c_int,
        (strlen(file)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_char;
    strcpy((*mpl).out_file, file);
}
pub unsafe extern "C" fn _glp_mpl_write_char(mut mpl: *mut MPL, mut c: libc::c_int) {
    (!((*mpl).out_fp).is_null()
        || {
            glp_assert_(
                b"mpl->out_fp != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                361 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*mpl).out_fp == stdout as *mut libc::c_void as *mut glp_file {
        glp_printf(b"%c\0" as *const u8 as *const libc::c_char, c);
    } else {
        _glp_format((*mpl).out_fp, b"%c\0" as *const u8 as *const libc::c_char, c);
    };
}
pub unsafe extern "C" fn _glp_mpl_write_text(
    mut mpl: *mut MPL,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut arg: ::std::ffi::VaListImpl;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    arg = args.clone();
    vsprintf(buf.as_mut_ptr(), fmt, arg.as_va_list());
    (strlen(buf.as_mut_ptr())
        < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(buf) < sizeof(buf)\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                380 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    c = buf.as_mut_ptr();
    while *c as libc::c_int != '\0' as i32 {
        _glp_mpl_write_char(mpl, *c as libc::c_int);
        c = c.offset(1);
        c;
    }
}
pub unsafe extern "C" fn _glp_mpl_flush_output(mut mpl: *mut MPL) {
    (!((*mpl).out_fp).is_null()
        || {
            glp_assert_(
                b"mpl->out_fp != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                392 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*mpl).out_fp != stdout as *mut libc::c_void as *mut glp_file {
        if _glp_ioerr((*mpl).out_fp) != 0 {
            _glp_mpl_error(
                mpl,
                b"write error on %s - %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*mpl).out_file,
                _glp_get_err_msg(),
            );
        }
    }
}
pub unsafe extern "C" fn _glp_mpl_error(
    mut mpl: *mut MPL,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut arg: ::std::ffi::VaListImpl;
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    arg = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, arg.as_va_list());
    (strlen(msg.as_mut_ptr())
        < ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(msg) < sizeof(msg)\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                420 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match (*mpl).phase {
        1 | 2 => {
            glp_printf(
                b"%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                if ((*mpl).in_file).is_null() {
                    b"(unknown)\0" as *const u8 as *const libc::c_char
                } else {
                    (*mpl).in_file as *const libc::c_char
                },
                (*mpl).line,
                msg.as_mut_ptr(),
            );
            _glp_mpl_print_context(mpl);
        }
        3 => {
            glp_printf(
                b"%s:%d: %s\n\0" as *const u8 as *const libc::c_char,
                if ((*mpl).mod_file).is_null() {
                    b"(unknown)\0" as *const u8 as *const libc::c_char
                } else {
                    (*mpl).mod_file as *const libc::c_char
                },
                if ((*mpl).stmt).is_null() {
                    0 as libc::c_int
                } else {
                    (*(*mpl).stmt).line
                },
                msg.as_mut_ptr(),
            );
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                        438 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    (*mpl).phase = 4 as libc::c_int;
    longjmp(((*mpl).jump).as_mut_ptr(), 1 as libc::c_int);
}
pub unsafe extern "C" fn _glp_mpl_warning(
    mut mpl: *mut MPL,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut arg: ::std::ffi::VaListImpl;
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    arg = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, arg.as_va_list());
    (strlen(msg.as_mut_ptr())
        < ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(msg) < sizeof(msg)\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                456 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match (*mpl).phase {
        1 | 2 => {
            glp_printf(
                b"%s:%d: warning: %s\n\0" as *const u8 as *const libc::c_char,
                if ((*mpl).in_file).is_null() {
                    b"(unknown)\0" as *const u8 as *const libc::c_char
                } else {
                    (*mpl).in_file as *const libc::c_char
                },
                (*mpl).line,
                msg.as_mut_ptr(),
            );
        }
        3 => {
            glp_printf(
                b"%s:%d: warning: %s\n\0" as *const u8 as *const libc::c_char,
                if ((*mpl).mod_file).is_null() {
                    b"(unknown)\0" as *const u8 as *const libc::c_char
                } else {
                    (*mpl).mod_file as *const libc::c_char
                },
                if ((*mpl).stmt).is_null() {
                    0 as libc::c_int
                } else {
                    (*(*mpl).stmt).line
                },
                msg.as_mut_ptr(),
            );
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                        473 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn _glp_mpl_initialize() -> *mut MPL {
    let mut mpl: *mut MPL = 0 as *mut MPL;
    mpl = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<MPL>() as libc::c_ulong as libc::c_int,
    ) as *mut MPL;
    (*mpl).line = 0 as libc::c_int;
    (*mpl).c = 0 as libc::c_int;
    (*mpl).token = 0 as libc::c_int;
    (*mpl).imlen = 0 as libc::c_int;
    (*mpl)
        .image = glp_alloc(
        100 as libc::c_int + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    *((*mpl).image).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*mpl).value = 0.0f64;
    (*mpl).b_token = 0 as libc::c_int;
    (*mpl).b_imlen = 0 as libc::c_int;
    (*mpl)
        .b_image = glp_alloc(
        100 as libc::c_int + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    *((*mpl).b_image).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*mpl).b_value = 0.0f64;
    (*mpl).f_dots = 0 as libc::c_int;
    (*mpl).f_scan = 0 as libc::c_int;
    (*mpl).f_token = 0 as libc::c_int;
    (*mpl).f_imlen = 0 as libc::c_int;
    (*mpl)
        .f_image = glp_alloc(
        100 as libc::c_int + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    *((*mpl).f_image).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*mpl).f_value = 0.0f64;
    (*mpl)
        .context = glp_alloc(
        60 as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    memset(
        (*mpl).context as *mut libc::c_void,
        ' ' as i32,
        60 as libc::c_int as libc::c_ulong,
    );
    (*mpl).c_ptr = 0 as libc::c_int;
    (*mpl).flag_d = 0 as libc::c_int;
    (*mpl).pool = _glp_dmp_create_pool();
    (*mpl)
        .tree = _glp_avl_create_tree(
        Some(
            _glp_avl_strcmp
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
    );
    (*mpl).model = 0 as *mut STATEMENT;
    (*mpl).flag_x = 0 as libc::c_int;
    (*mpl).as_within = 0 as libc::c_int;
    (*mpl).as_in = 0 as libc::c_int;
    (*mpl).as_binary = 0 as libc::c_int;
    (*mpl).flag_s = 0 as libc::c_int;
    (*mpl).strings = _glp_dmp_create_pool();
    (*mpl).symbols = _glp_dmp_create_pool();
    (*mpl).tuples = _glp_dmp_create_pool();
    (*mpl).arrays = _glp_dmp_create_pool();
    (*mpl).members = _glp_dmp_create_pool();
    (*mpl).elemvars = _glp_dmp_create_pool();
    (*mpl).formulae = _glp_dmp_create_pool();
    (*mpl).elemcons = _glp_dmp_create_pool();
    (*mpl).a_list = 0 as *mut ARRAY;
    (*mpl)
        .sym_buf = glp_alloc(
        255 as libc::c_int + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    *((*mpl).sym_buf).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*mpl)
        .tup_buf = glp_alloc(
        255 as libc::c_int + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    *((*mpl).tup_buf).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*mpl).rand = _glp_rng_create_rand();
    (*mpl).flag_p = 0 as libc::c_int;
    (*mpl).stmt = 0 as *mut STATEMENT;
    (*mpl).dca = 0 as *mut TABDCA;
    (*mpl).m = 0 as libc::c_int;
    (*mpl).n = 0 as libc::c_int;
    (*mpl).row = 0 as *mut *mut ELEMCON;
    (*mpl).col = 0 as *mut *mut ELEMVAR;
    (*mpl).in_fp = 0 as *mut glp_file;
    (*mpl).in_file = 0 as *mut libc::c_char;
    (*mpl).out_fp = 0 as *mut glp_file;
    (*mpl).out_file = 0 as *mut libc::c_char;
    (*mpl).prt_fp = 0 as *mut glp_file;
    (*mpl).prt_file = 0 as *mut libc::c_char;
    if _setjmp(((*mpl).jump).as_mut_ptr()) != 0 {
        (mpl != mpl
            || {
                glp_assert_(
                    b"mpl != mpl\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                    564 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    (*mpl).phase = 0 as libc::c_int;
    (*mpl).mod_file = 0 as *mut libc::c_char;
    (*mpl)
        .mpl_buf = glp_alloc(
        255 as libc::c_int + 1 as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    *((*mpl).mpl_buf).offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    return mpl;
}
pub unsafe extern "C" fn _glp_mpl_read_model(
    mut mpl: *mut MPL,
    mut file: *mut libc::c_char,
    mut skip_data: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    if (*mpl).phase != 0 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            611 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_read_model: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if file.is_null() {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            613 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_read_model: no input filename specified\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(_setjmp(((*mpl).jump).as_mut_ptr()) != 0) {
        (*mpl).phase = 1 as libc::c_int;
        glp_printf(
            b"Reading model section from %s...\n\0" as *const u8 as *const libc::c_char,
            file,
        );
        _glp_mpl_open_input(mpl, file);
        _glp_mpl_model_section(mpl);
        if ((*mpl).model).is_null() {
            _glp_mpl_error(
                mpl,
                b"empty model section not allowed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        (*mpl)
            .mod_file = glp_alloc(
            (strlen(file)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_char;
        strcpy((*mpl).mod_file, (*mpl).in_file);
        _glp_mpl_alloc_content(mpl);
        if _glp_mpl_is_keyword(
            mpl,
            b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            if skip_data != 0 {
                _glp_mpl_warning(
                    mpl,
                    b"data section ignored\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                current_block = 970721835176407203;
            } else {
                (*mpl).flag_d = 1 as libc::c_int;
                _glp_mpl_get_token(mpl);
                if (*mpl).token != 241 as libc::c_int {
                    _glp_mpl_error(
                        mpl,
                        b"semicolon missing where expected\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                _glp_mpl_get_token(mpl);
                (*mpl).phase = 2 as libc::c_int;
                glp_printf(
                    b"Reading data section from %s...\n\0" as *const u8
                        as *const libc::c_char,
                    file,
                );
                _glp_mpl_data_section(mpl);
                current_block = 5948590327928692120;
            }
        } else {
            current_block = 5948590327928692120;
        }
        match current_block {
            5948590327928692120 => {
                _glp_mpl_end_statement(mpl);
            }
            _ => {}
        }
        glp_printf(
            b"%d line%s were read\n\0" as *const u8 as *const libc::c_char,
            (*mpl).line,
            if (*mpl).line == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
        );
        _glp_mpl_close_input(mpl);
    }
    return (*mpl).phase;
}
pub unsafe extern "C" fn _glp_mpl_read_data(
    mut mpl: *mut MPL,
    mut file: *mut libc::c_char,
) -> libc::c_int {
    if !((*mpl).phase == 1 as libc::c_int || (*mpl).phase == 2 as libc::c_int) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            686 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_read_data: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if file.is_null() {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            688 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_read_data: no input filename specified\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(_setjmp(((*mpl).jump).as_mut_ptr()) != 0) {
        (*mpl).phase = 2 as libc::c_int;
        glp_printf(
            b"Reading data section from %s...\n\0" as *const u8 as *const libc::c_char,
            file,
        );
        (*mpl).flag_d = 1 as libc::c_int;
        _glp_mpl_open_input(mpl, file);
        if _glp_mpl_is_literal(
            mpl,
            b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            _glp_mpl_get_token(mpl);
            if (*mpl).token != 241 as libc::c_int {
                _glp_mpl_error(
                    mpl,
                    b"semicolon missing where expected\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            _glp_mpl_get_token(mpl);
        }
        _glp_mpl_data_section(mpl);
        _glp_mpl_end_statement(mpl);
        glp_printf(
            b"%d line%s were read\n\0" as *const u8 as *const libc::c_char,
            (*mpl).line,
            if (*mpl).line == 1 as libc::c_int {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b"s\0" as *const u8 as *const libc::c_char
            },
        );
        _glp_mpl_close_input(mpl);
    }
    return (*mpl).phase;
}
pub unsafe extern "C" fn _glp_mpl_generate(
    mut mpl: *mut MPL,
    mut file: *mut libc::c_char,
) -> libc::c_int {
    if !((*mpl).phase == 1 as libc::c_int || (*mpl).phase == 2 as libc::c_int) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            750 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_generate: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(_setjmp(((*mpl).jump).as_mut_ptr()) != 0) {
        (*mpl).phase = 3 as libc::c_int;
        _glp_mpl_open_output(mpl, file);
        _glp_mpl_generate_model(mpl);
        _glp_mpl_flush_output(mpl);
        _glp_mpl_build_problem(mpl);
        glp_printf(
            b"Model has been successfully generated\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return (*mpl).phase;
}
pub unsafe extern "C" fn _glp_mpl_get_prob_name(mut mpl: *mut MPL) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = (*mpl).mpl_buf;
    let mut file: *mut libc::c_char = (*mpl).mod_file;
    let mut k: libc::c_int = 0;
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            791 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_prob_name: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    loop {
        if !(strchr(file, '/' as i32)).is_null() {
            file = (strchr(file, '/' as i32)).offset(1 as libc::c_int as isize);
        } else if !(strchr(file, '\\' as i32)).is_null() {
            file = (strchr(file, '\\' as i32)).offset(1 as libc::c_int as isize);
        } else {
            if (strchr(file, ':' as i32)).is_null() {
                break;
            }
            file = (strchr(file, ':' as i32)).offset(1 as libc::c_int as isize);
        }
    }
    k = 0 as libc::c_int;
    while !(k == 255 as libc::c_int) {
        if !(*(*__ctype_b_loc()).offset(*file as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            != 0 || *file as libc::c_int == '_' as i32)
        {
            break;
        }
        let fresh4 = file;
        file = file.offset(1);
        *name.offset(k as isize) = *fresh4;
        k += 1;
        k;
    }
    if k == 0 as libc::c_int {
        strcpy(name, b"Unknown\0" as *const u8 as *const libc::c_char);
    } else {
        *name.offset(k as isize) = '\0' as i32 as libc::c_char;
    }
    (strlen(name) <= 255 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(name) <= 255\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                811 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return name;
}
pub unsafe extern "C" fn _glp_mpl_get_num_rows(mut mpl: *mut MPL) -> libc::c_int {
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            830 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_num_rows: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return (*mpl).m;
}
pub unsafe extern "C" fn _glp_mpl_get_num_cols(mut mpl: *mut MPL) -> libc::c_int {
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            849 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_num_cols: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return (*mpl).n;
}
pub unsafe extern "C" fn _glp_mpl_get_row_name(
    mut mpl: *mut MPL,
    mut i: libc::c_int,
) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = (*mpl).mpl_buf;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            870 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_row_name: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= i && i <= (*mpl).m) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            872 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_row_name: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    strcpy(name, (*(**((*mpl).row).offset(i as isize)).con).name);
    len = strlen(name) as libc::c_int;
    (len <= 255 as libc::c_int
        || {
            glp_assert_(
                b"len <= 255\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                876 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    t = _glp_mpl_format_tuple(
        mpl,
        '[' as i32,
        (*(**((*mpl).row).offset(i as isize)).memb).tuple,
    );
    while *t != 0 {
        if len == 255 as libc::c_int {
            break;
        }
        let fresh5 = t;
        t = t.offset(1);
        let fresh6 = len;
        len = len + 1;
        *name.offset(fresh6 as isize) = *fresh5;
    }
    *name.offset(len as isize) = '\0' as i32 as libc::c_char;
    if len == 255 as libc::c_int {
        strcpy(
            name.offset(252 as libc::c_int as isize),
            b"...\0" as *const u8 as *const libc::c_char,
        );
    }
    (strlen(name) <= 255 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(name) <= 255\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                884 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return name;
}
pub unsafe extern "C" fn _glp_mpl_get_row_kind(
    mut mpl: *mut MPL,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut kind: libc::c_int = 0;
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            908 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_row_kind: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= i && i <= (*mpl).m) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            910 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_row_kind: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    match (*(**((*mpl).row).offset(i as isize)).con).type_0 {
        103 => {
            kind = 411 as libc::c_int;
        }
        116 => {
            kind = 412 as libc::c_int;
        }
        115 => {
            kind = 413 as libc::c_int;
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                        920 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return kind;
}
pub unsafe extern "C" fn _glp_mpl_get_row_bnds(
    mut mpl: *mut MPL,
    mut i: libc::c_int,
    mut _lb: *mut libc::c_double,
    mut _ub: *mut libc::c_double,
) -> libc::c_int {
    let mut con: *mut ELEMCON = 0 as *mut ELEMCON;
    let mut type_0: libc::c_int = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            969 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_row_bnds: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= i && i <= (*mpl).m) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            971 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_row_bnds: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    con = *((*mpl).row).offset(i as isize);
    lb = if ((*(*con).con).lbnd).is_null() {
        -1.7976931348623157e+308f64
    } else {
        (*con).lbnd
    };
    ub = if ((*(*con).con).ubnd).is_null() {
        1.7976931348623157e+308f64
    } else {
        (*con).ubnd
    };
    if lb == -1.7976931348623157e+308f64 && ub == 1.7976931348623157e+308f64 {
        type_0 = 401 as libc::c_int;
        ub = 0.0f64;
        lb = ub;
    } else if ub == 1.7976931348623157e+308f64 {
        type_0 = 402 as libc::c_int;
        ub = 0.0f64;
    } else if lb == -1.7976931348623157e+308f64 {
        type_0 = 403 as libc::c_int;
        lb = 0.0f64;
    } else if (*(*con).con).lbnd != (*(*con).con).ubnd {
        type_0 = 404 as libc::c_int;
    } else {
        type_0 = 405 as libc::c_int;
    }
    if !_lb.is_null() {
        *_lb = lb;
    }
    if !_ub.is_null() {
        *_ub = ub;
    }
    return type_0;
}
pub unsafe extern "C" fn _glp_mpl_get_mat_row(
    mut mpl: *mut MPL,
    mut i: libc::c_int,
    mut ndx: *mut libc::c_int,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut term: *mut FORMULA = 0 as *mut FORMULA;
    let mut len: libc::c_int = 0 as libc::c_int;
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1037 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_mat_row: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= i && i <= (*mpl).m) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1039 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_mat_row: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    term = (**((*mpl).row).offset(i as isize)).form;
    while !term.is_null() {
        (!((*term).var).is_null()
            || {
                glp_assert_(
                    b"term->var != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                    1042 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        len += 1;
        len;
        (len <= (*mpl).n
            || {
                glp_assert_(
                    b"len <= mpl->n\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                    1044 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !ndx.is_null() {
            *ndx.offset(len as isize) = (*(*term).var).j;
        }
        if !val.is_null() {
            *val.offset(len as isize) = (*term).coef;
        }
        term = (*term).next;
    }
    return len;
}
pub unsafe extern "C" fn _glp_mpl_get_row_c0(
    mut mpl: *mut MPL,
    mut i: libc::c_int,
) -> libc::c_double {
    let mut con: *mut ELEMCON = 0 as *mut ELEMCON;
    let mut c0: libc::c_double = 0.;
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1071 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_row_c0: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= i && i <= (*mpl).m) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1073 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_row_c0: i = %d; row number out of range\n\0" as *const u8
                as *const libc::c_char,
            i,
        );
    }
    con = *((*mpl).row).offset(i as isize);
    if ((*(*con).con).lbnd).is_null() && ((*(*con).con).ubnd).is_null() {
        c0 = -(*con).lbnd;
    } else {
        c0 = 0.0f64;
    }
    return c0;
}
pub unsafe extern "C" fn _glp_mpl_get_col_name(
    mut mpl: *mut MPL,
    mut j: libc::c_int,
) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = (*mpl).mpl_buf;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1100 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_col_name: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= j && j <= (*mpl).n) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1102 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_col_name: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    strcpy(name, (*(**((*mpl).col).offset(j as isize)).var).name);
    len = strlen(name) as libc::c_int;
    (len <= 255 as libc::c_int
        || {
            glp_assert_(
                b"len <= 255\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                1106 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    t = _glp_mpl_format_tuple(
        mpl,
        '[' as i32,
        (*(**((*mpl).col).offset(j as isize)).memb).tuple,
    );
    while *t != 0 {
        if len == 255 as libc::c_int {
            break;
        }
        let fresh7 = t;
        t = t.offset(1);
        let fresh8 = len;
        len = len + 1;
        *name.offset(fresh8 as isize) = *fresh7;
    }
    *name.offset(len as isize) = '\0' as i32 as libc::c_char;
    if len == 255 as libc::c_int {
        strcpy(
            name.offset(252 as libc::c_int as isize),
            b"...\0" as *const u8 as *const libc::c_char,
        );
    }
    (strlen(name) <= 255 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(name) <= 255\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                1114 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return name;
}
pub unsafe extern "C" fn _glp_mpl_get_col_kind(
    mut mpl: *mut MPL,
    mut j: libc::c_int,
) -> libc::c_int {
    let mut kind: libc::c_int = 0;
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1144 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_col_kind: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= j && j <= (*mpl).n) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1146 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_col_kind: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    match (*(**((*mpl).col).offset(j as isize)).var).type_0 {
        118 => {
            kind = 421 as libc::c_int;
        }
        113 => {
            kind = 422 as libc::c_int;
        }
        101 => {
            kind = 423 as libc::c_int;
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                        1156 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return kind;
}
pub unsafe extern "C" fn _glp_mpl_get_col_bnds(
    mut mpl: *mut MPL,
    mut j: libc::c_int,
    mut _lb: *mut libc::c_double,
    mut _ub: *mut libc::c_double,
) -> libc::c_int {
    let mut var: *mut ELEMVAR = 0 as *mut ELEMVAR;
    let mut type_0: libc::c_int = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1205 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_col_bnds: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(1 as libc::c_int <= j && j <= (*mpl).n) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1207 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_get_col_bnds: j = %d; column number out of range\n\0" as *const u8
                as *const libc::c_char,
            j,
        );
    }
    var = *((*mpl).col).offset(j as isize);
    lb = if ((*(*var).var).lbnd).is_null() {
        -1.7976931348623157e+308f64
    } else {
        (*var).lbnd
    };
    ub = if ((*(*var).var).ubnd).is_null() {
        1.7976931348623157e+308f64
    } else {
        (*var).ubnd
    };
    if lb == -1.7976931348623157e+308f64 && ub == 1.7976931348623157e+308f64 {
        type_0 = 401 as libc::c_int;
        ub = 0.0f64;
        lb = ub;
    } else if ub == 1.7976931348623157e+308f64 {
        type_0 = 402 as libc::c_int;
        ub = 0.0f64;
    } else if lb == -1.7976931348623157e+308f64 {
        type_0 = 403 as libc::c_int;
        lb = 0.0f64;
    } else if (*(*var).var).lbnd != (*(*var).var).ubnd {
        type_0 = 404 as libc::c_int;
    } else {
        type_0 = 405 as libc::c_int;
    }
    if !_lb.is_null() {
        *_lb = lb;
    }
    if !_ub.is_null() {
        *_ub = ub;
    }
    return type_0;
}
pub unsafe extern "C" fn _glp_mpl_has_solve_stmt(mut mpl: *mut MPL) -> libc::c_int {
    if (*mpl).phase != 3 as libc::c_int {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1255 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_has_solve_stmt: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return (*mpl).flag_s;
}
pub unsafe extern "C" fn _glp_mpl_put_row_soln(
    mut mpl: *mut MPL,
    mut i: libc::c_int,
    mut stat: libc::c_int,
    mut prim: libc::c_double,
    mut dual: libc::c_double,
) {
    ((*mpl).phase == 3 as libc::c_int
        || {
            glp_assert_(
                b"mpl->phase == 3\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                1263 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= i && i <= (*mpl).m
        || {
            glp_assert_(
                b"1 <= i && i <= mpl->m\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                1264 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (**((*mpl).row).offset(i as isize)).stat = stat;
    (**((*mpl).row).offset(i as isize)).prim = prim;
    (**((*mpl).row).offset(i as isize)).dual = dual;
}
pub unsafe extern "C" fn _glp_mpl_put_col_soln(
    mut mpl: *mut MPL,
    mut j: libc::c_int,
    mut stat: libc::c_int,
    mut prim: libc::c_double,
    mut dual: libc::c_double,
) {
    ((*mpl).phase == 3 as libc::c_int
        || {
            glp_assert_(
                b"mpl->phase == 3\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                1276 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (1 as libc::c_int <= j && j <= (*mpl).n
        || {
            glp_assert_(
                b"1 <= j && j <= mpl->n\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                1277 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (**((*mpl).col).offset(j as isize)).stat = stat;
    (**((*mpl).col).offset(j as isize)).prim = prim;
    (**((*mpl).col).offset(j as isize)).dual = dual;
}
pub unsafe extern "C" fn _glp_mpl_postsolve(mut mpl: *mut MPL) -> libc::c_int {
    if !((*mpl).phase == 3 as libc::c_int && (*mpl).flag_p == 0) {
        (glp_error_(
            b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
            1339 as libc::c_int,
        ))
            .unwrap()(
            b"mpl_postsolve: invalid call sequence\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !(_setjmp(((*mpl).jump).as_mut_ptr()) != 0) {
        _glp_mpl_postsolve_model(mpl);
        _glp_mpl_flush_output(mpl);
        glp_printf(
            b"Model has been successfully processed\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    return (*mpl).phase;
}
pub unsafe extern "C" fn _glp_mpl_terminate(mut mpl: *mut MPL) {
    if _setjmp(((*mpl).jump).as_mut_ptr()) != 0 {
        (mpl != mpl
            || {
                glp_assert_(
                    b"mpl != mpl\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                    1365 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    match (*mpl).phase {
        0 | 1 | 2 | 3 => {
            _glp_mpl_clean_model(mpl);
            (((*mpl).a_list).is_null()
                || {
                    glp_assert_(
                        b"mpl->a_list == NULL\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                        1373 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (((*mpl).dca).is_null()
                || {
                    glp_assert_(
                        b"mpl->dca == NULL\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                        1375 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        4 => {
            let mut a: *mut ARRAY = 0 as *mut ARRAY;
            a = (*mpl).a_list;
            while !a.is_null() {
                if !((*a).tree).is_null() {
                    _glp_avl_delete_tree((*a).tree);
                }
                a = (*a).next;
            }
            _glp_mpl_free_dca(mpl);
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl4.c\0" as *const u8 as *const libc::c_char,
                        1390 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    glp_free((*mpl).image as *mut libc::c_void);
    glp_free((*mpl).b_image as *mut libc::c_void);
    glp_free((*mpl).f_image as *mut libc::c_void);
    glp_free((*mpl).context as *mut libc::c_void);
    _glp_dmp_delete_pool((*mpl).pool);
    _glp_avl_delete_tree((*mpl).tree);
    _glp_dmp_delete_pool((*mpl).strings);
    _glp_dmp_delete_pool((*mpl).symbols);
    _glp_dmp_delete_pool((*mpl).tuples);
    _glp_dmp_delete_pool((*mpl).arrays);
    _glp_dmp_delete_pool((*mpl).members);
    _glp_dmp_delete_pool((*mpl).elemvars);
    _glp_dmp_delete_pool((*mpl).formulae);
    _glp_dmp_delete_pool((*mpl).elemcons);
    glp_free((*mpl).sym_buf as *mut libc::c_void);
    glp_free((*mpl).tup_buf as *mut libc::c_void);
    _glp_rng_delete_rand((*mpl).rand);
    if !((*mpl).row).is_null() {
        glp_free((*mpl).row as *mut libc::c_void);
    }
    if !((*mpl).col).is_null() {
        glp_free((*mpl).col as *mut libc::c_void);
    }
    if !((*mpl).in_fp).is_null() {
        _glp_close((*mpl).in_fp);
    }
    if !((*mpl).out_fp).is_null()
        && (*mpl).out_fp != stdout as *mut libc::c_void as *mut glp_file
    {
        _glp_close((*mpl).out_fp);
    }
    if !((*mpl).out_file).is_null() {
        glp_free((*mpl).out_file as *mut libc::c_void);
    }
    if !((*mpl).prt_fp).is_null() {
        _glp_close((*mpl).prt_fp);
    }
    if !((*mpl).prt_file).is_null() {
        glp_free((*mpl).prt_file as *mut libc::c_void);
    }
    if !((*mpl).mod_file).is_null() {
        glp_free((*mpl).mod_file as *mut libc::c_void);
    }
    glp_free((*mpl).mpl_buf as *mut libc::c_void);
    glp_free(mpl as *mut libc::c_void);
}

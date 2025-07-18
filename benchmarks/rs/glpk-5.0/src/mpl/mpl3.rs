use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type DMP;
    pub type glp_file;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn _glp_avl_delete_tree(tree: *mut AVL);
    fn _glp_avl_get_node_link(node: *mut AVLNODE) -> *mut libc::c_void;
    fn _glp_avl_find_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_set_node_link(node: *mut AVLNODE, link: *mut libc::c_void);
    fn _glp_avl_insert_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: libc::c_int);
    fn glp_printf(fmt: *const libc::c_char, _: ...);
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
    fn _glp_write(
        f: *mut glp_file,
        buf: *const libc::c_void,
        nnn: libc::c_int,
    ) -> libc::c_int;
    fn _glp_close(f: *mut glp_file) -> libc::c_int;
    fn _glp_str2num(str: *const libc::c_char, val: *mut libc::c_double) -> libc::c_int;
    fn _glp_rng_next_rand(rand: *mut RNG) -> libc::c_int;
    fn _glp_rng_unif_rand(rand: *mut RNG, m: libc::c_int) -> libc::c_int;
    fn _glp_mpl_error(mpl: *mut MPL, fmt: *mut libc::c_char, _: ...);
    fn _glp_mpl_fn_gmtime(mpl: *mut MPL) -> libc::c_double;
    fn _glp_mpl_fn_str2time(
        mpl: *mut MPL,
        str: *const libc::c_char,
        fmt: *const libc::c_char,
    ) -> libc::c_double;
    fn _glp_mpl_fn_time2str(
        mpl: *mut MPL,
        str: *mut libc::c_char,
        t: libc::c_double,
        fmt: *const libc::c_char,
    );
    fn _glp_mpl_write_char(mpl: *mut MPL, c: libc::c_int);
    fn _glp_mpl_write_text(mpl: *mut MPL, fmt: *mut libc::c_char, _: ...);
    fn _glp_mpl_tab_drv_close(mpl: *mut MPL);
    fn _glp_mpl_tab_drv_write(mpl: *mut MPL);
    fn _glp_mpl_tab_drv_read(mpl: *mut MPL) -> libc::c_int;
    fn _glp_mpl_tab_drv_open(mpl: *mut MPL, mode: libc::c_int);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eval_domain_info {
    pub domain: *mut DOMAIN1,
    pub block: *mut DOMAIN_BLOCK,
    pub tuple: *mut TUPLE,
    pub info: *mut libc::c_void,
    pub func: Option::<unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> ()>,
    pub failure: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iter_log_info {
    pub code: *mut CODE,
    pub value: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loop_domain_info {
    pub domain: *mut DOMAIN1,
    pub block: *mut DOMAIN_BLOCK,
    pub looping: libc::c_int,
    pub info: *mut libc::c_void,
    pub func: Option::<unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iter_set_info {
    pub code: *mut CODE,
    pub value: *mut ELEMSET,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iter_num_info {
    pub code: *mut CODE,
    pub value: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eval_con_info {
    pub con: *mut CONSTRAINT,
    pub tuple: *mut TUPLE,
    pub refer: *mut ELEMCON,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iter_form_info {
    pub code: *mut CODE,
    pub value: *mut FORMULA,
    pub tail: *mut FORMULA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eval_var_info {
    pub var: *mut VARIABLE,
    pub tuple: *mut TUPLE,
    pub refer: *mut ELEMVAR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eval_num_info {
    pub par: *mut PARAMETER,
    pub tuple: *mut TUPLE,
    pub memb: *mut MEMBER,
    pub value: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eval_sym_info {
    pub par: *mut PARAMETER,
    pub tuple: *mut TUPLE,
    pub memb: *mut MEMBER,
    pub value: *mut SYMBOL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eval_set_info {
    pub set: *mut SET,
    pub tuple: *mut TUPLE,
    pub memb: *mut MEMBER,
    pub refer: *mut ELEMSET,
}
pub unsafe extern "C" fn _glp_mpl_fp_add(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    if x > 0.0f64 && y > 0.0f64 && x > 0.999f64 * 1.7976931348623157e+308f64 - y
        || x < 0.0f64 && y < 0.0f64 && x < -0.999f64 * 1.7976931348623157e+308f64 - y
    {
        _glp_mpl_error(
            mpl,
            b"%.*g + %.*g; floating-point overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            y,
        );
    }
    return x + y;
}
pub unsafe extern "C" fn _glp_mpl_fp_sub(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    if x > 0.0f64 && y < 0.0f64 && x > 0.999f64 * 1.7976931348623157e+308f64 + y
        || x < 0.0f64 && y > 0.0f64 && x < -0.999f64 * 1.7976931348623157e+308f64 + y
    {
        _glp_mpl_error(
            mpl,
            b"%.*g - %.*g; floating-point overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            y,
        );
    }
    return x - y;
}
pub unsafe extern "C" fn _glp_mpl_fp_less(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    if x < y {
        return 0.0f64;
    }
    if x > 0.0f64 && y < 0.0f64 && x > 0.999f64 * 1.7976931348623157e+308f64 + y {
        _glp_mpl_error(
            mpl,
            b"%.*g less %.*g; floating-point overflow\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            y,
        );
    }
    return x - y;
}
pub unsafe extern "C" fn _glp_mpl_fp_mul(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    if fabs(y) > 1.0f64 && fabs(x) > 0.999f64 * 1.7976931348623157e+308f64 / fabs(y) {
        _glp_mpl_error(
            mpl,
            b"%.*g * %.*g; floating-point overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            y,
        );
    }
    return x * y;
}
pub unsafe extern "C" fn _glp_mpl_fp_div(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    if fabs(y) < 2.2250738585072014e-308f64 {
        _glp_mpl_error(
            mpl,
            b"%.*g / %.*g; floating-point zero divide\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            y,
        );
    }
    if fabs(y) < 1.0f64 && fabs(x) > 0.999f64 * 1.7976931348623157e+308f64 * fabs(y) {
        _glp_mpl_error(
            mpl,
            b"%.*g / %.*g; floating-point overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            y,
        );
    }
    return x / y;
}
pub unsafe extern "C" fn _glp_mpl_fp_idiv(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    if fabs(y) < 2.2250738585072014e-308f64 {
        _glp_mpl_error(
            mpl,
            b"%.*g div %.*g; floating-point zero divide\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            y,
        );
    }
    if fabs(y) < 1.0f64 && fabs(x) > 0.999f64 * 1.7976931348623157e+308f64 * fabs(y) {
        _glp_mpl_error(
            mpl,
            b"%.*g div %.*g; floating-point overflow\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            y,
        );
    }
    x /= y;
    return if x > 0.0f64 { floor(x) } else if x < 0.0f64 { ceil(x) } else { 0.0f64 };
}
pub unsafe extern "C" fn _glp_mpl_fp_mod(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    let mut r: libc::c_double = 0.;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if x == 0.0f64 {
        r = 0.0f64;
    } else if y == 0.0f64 {
        r = x;
    } else {
        r = fmod(fabs(x), fabs(y));
        if r != 0.0f64 {
            if x < 0.0f64 {
                r = -r;
            }
            if x > 0.0f64 && y < 0.0f64 || x < 0.0f64 && y > 0.0f64 {
                r += y;
            }
        }
    }
    return r;
}
pub unsafe extern "C" fn _glp_mpl_fp_power(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    let mut current_block: u64;
    let mut r: libc::c_double = 0.;
    if x == 0.0f64 && y <= 0.0f64 || x < 0.0f64 && y != floor(y) {
        _glp_mpl_error(
            mpl,
            b"%.*g ** %.*g; result undefined\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            y,
        );
    }
    if x == 0.0f64 {
        current_block = 16521808255277263772;
    } else {
        if fabs(x) > 1.0f64 && y > 1.0f64
            && log(fabs(x)) > 0.999f64 * log(1.7976931348623157e+308f64) / y
            || fabs(x) < 1.0f64 && y < -1.0f64
                && log(fabs(x)) < 0.999f64 * log(1.7976931348623157e+308f64) / y
        {
            _glp_mpl_error(
                mpl,
                b"%.*g ** %.*g; floating-point overflow\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                15 as libc::c_int,
                x,
                15 as libc::c_int,
                y,
            );
        }
        if fabs(x) > 1.0f64 && y < -1.0f64
            && -log(fabs(x)) < 0.999f64 * log(1.7976931348623157e+308f64) / y
            || fabs(x) < 1.0f64 && y > 1.0f64
                && -log(fabs(x)) > 0.999f64 * log(1.7976931348623157e+308f64) / y
        {
            r = 0.0f64;
            current_block = 10879442775620481940;
        } else {
            current_block = 16521808255277263772;
        }
    }
    match current_block {
        16521808255277263772 => {
            r = pow(x, y);
        }
        _ => {}
    }
    return r;
}
pub unsafe extern "C" fn _glp_mpl_fp_exp(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
) -> libc::c_double {
    if x > 0.999f64 * log(1.7976931348623157e+308f64) {
        _glp_mpl_error(
            mpl,
            b"exp(%.*g); floating-point overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
        );
    }
    return exp(x);
}
pub unsafe extern "C" fn _glp_mpl_fp_log(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
) -> libc::c_double {
    if x <= 0.0f64 {
        _glp_mpl_error(
            mpl,
            b"log(%.*g); non-positive argument\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
        );
    }
    return log(x);
}
pub unsafe extern "C" fn _glp_mpl_fp_log10(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
) -> libc::c_double {
    if x <= 0.0f64 {
        _glp_mpl_error(
            mpl,
            b"log10(%.*g); non-positive argument\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
        );
    }
    return log10(x);
}
pub unsafe extern "C" fn _glp_mpl_fp_sqrt(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
) -> libc::c_double {
    if x < 0.0f64 {
        _glp_mpl_error(
            mpl,
            b"sqrt(%.*g); negative argument\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
        );
    }
    return sqrt(x);
}
pub unsafe extern "C" fn _glp_mpl_fp_sin(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
) -> libc::c_double {
    if !(-1e6f64 <= x && x <= 1e6f64) {
        _glp_mpl_error(
            mpl,
            b"sin(%.*g); argument too large\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
        );
    }
    return sin(x);
}
pub unsafe extern "C" fn _glp_mpl_fp_cos(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
) -> libc::c_double {
    if !(-1e6f64 <= x && x <= 1e6f64) {
        _glp_mpl_error(
            mpl,
            b"cos(%.*g); argument too large\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
        );
    }
    return cos(x);
}
pub unsafe extern "C" fn _glp_mpl_fp_tan(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
) -> libc::c_double {
    if !(-1e6f64 <= x && x <= 1e6f64) {
        _glp_mpl_error(
            mpl,
            b"tan(%.*g); argument too large\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            x,
        );
    }
    return tan(x);
}
pub unsafe extern "C" fn _glp_mpl_fp_atan(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
) -> libc::c_double {
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                244 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return atan(x);
}
pub unsafe extern "C" fn _glp_mpl_fp_atan2(
    mut mpl: *mut MPL,
    mut y: libc::c_double,
    mut x: libc::c_double,
) -> libc::c_double {
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                254 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return atan2(y, x);
}
pub unsafe extern "C" fn _glp_mpl_fp_round(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
    mut n: libc::c_double,
) -> libc::c_double {
    let mut ten_to_n: libc::c_double = 0.;
    if n != floor(n) {
        _glp_mpl_error(
            mpl,
            b"round(%.*g, %.*g); non-integer second argument\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            n,
        );
    }
    if n <= (15 as libc::c_int + 2 as libc::c_int) as libc::c_double {
        ten_to_n = pow(10.0f64, n);
        if fabs(x) < 0.999f64 * 1.7976931348623157e+308f64 / ten_to_n {
            x = floor(x * ten_to_n + 0.5f64);
            if x != 0.0f64 {
                x /= ten_to_n;
            }
        }
    }
    return x;
}
pub unsafe extern "C" fn _glp_mpl_fp_trunc(
    mut mpl: *mut MPL,
    mut x: libc::c_double,
    mut n: libc::c_double,
) -> libc::c_double {
    let mut ten_to_n: libc::c_double = 0.;
    if n != floor(n) {
        _glp_mpl_error(
            mpl,
            b"trunc(%.*g, %.*g); non-integer second argument\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
            x,
            15 as libc::c_int,
            n,
        );
    }
    if n <= (15 as libc::c_int + 2 as libc::c_int) as libc::c_double {
        ten_to_n = pow(10.0f64, n);
        if fabs(x) < 0.999f64 * 1.7976931348623157e+308f64 / ten_to_n {
            x = if x >= 0.0f64 { floor(x * ten_to_n) } else { ceil(x * ten_to_n) };
            if x != 0.0f64 {
                x /= ten_to_n;
            }
        }
    }
    return x;
}
pub unsafe extern "C" fn _glp_mpl_fp_irand224(mut mpl: *mut MPL) -> libc::c_double {
    return _glp_rng_unif_rand((*mpl).rand, 0x1000000 as libc::c_int) as libc::c_double;
}
pub unsafe extern "C" fn _glp_mpl_fp_uniform01(mut mpl: *mut MPL) -> libc::c_double {
    return _glp_rng_next_rand((*mpl).rand) as libc::c_double
        / 0x80000000 as libc::c_uint as libc::c_double;
}
pub unsafe extern "C" fn _glp_mpl_uniform(
    mut mpl: *mut MPL,
    mut a: libc::c_double,
    mut b: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    if a >= b {
        _glp_mpl_error(
            mpl,
            b"Uniform(%.*g, %.*g); invalid range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            a,
            15 as libc::c_int,
            b,
        );
    }
    x = _glp_mpl_fp_uniform01(mpl);
    x = _glp_mpl_fp_add(mpl, a * (1.0f64 - x), b * x);
    return x;
}
pub unsafe extern "C" fn _glp_mpl_fp_normal01(mut mpl: *mut MPL) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut r2: libc::c_double = 0.;
    loop {
        x = -1.0f64 + 2.0f64 * _glp_mpl_fp_uniform01(mpl);
        y = -1.0f64 + 2.0f64 * _glp_mpl_fp_uniform01(mpl);
        r2 = x * x + y * y;
        if !(r2 > 1.0f64 || r2 == 0.0f64) {
            break;
        }
    }
    return y * sqrt(-2.0f64 * log(r2) / r2);
}
pub unsafe extern "C" fn _glp_mpl_fp_normal(
    mut mpl: *mut MPL,
    mut mu: libc::c_double,
    mut sigma: libc::c_double,
) -> libc::c_double {
    let mut x: libc::c_double = 0.;
    x = _glp_mpl_fp_add(mpl, mu, _glp_mpl_fp_mul(mpl, sigma, _glp_mpl_fp_normal01(mpl)));
    return x;
}
pub unsafe extern "C" fn _glp_mpl_create_string(
    mut mpl: *mut MPL,
    mut buf: *mut libc::c_char,
) -> *mut STRING {
    let mut str: *mut STRING = 0 as *mut STRING;
    (strlen(buf as *const libc::c_char) <= 100 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(buf) <= MAX_LENGTH\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                429 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    str = _glp_dmp_get_atom(
        (*mpl).strings,
        (strlen(buf as *const libc::c_char))
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) as *mut STRING;
    strcpy(str, buf as *const libc::c_char);
    return str;
}
pub unsafe extern "C" fn _glp_mpl_copy_string(
    mut mpl: *mut MPL,
    mut str: *mut STRING,
) -> *mut STRING {
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                458 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return _glp_mpl_create_string(mpl, str);
}
pub unsafe extern "C" fn _glp_mpl_compare_strings(
    mut mpl: *mut MPL,
    mut str1: *mut STRING,
    mut str2: *mut STRING,
) -> libc::c_int {
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                495 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return strcmp(str1, str2);
}
pub unsafe extern "C" fn _glp_mpl_fetch_string(
    mut mpl: *mut MPL,
    mut str: *mut STRING,
    mut buf: *mut libc::c_char,
) -> *mut libc::c_char {
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                524 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return strcpy(buf, str);
}
pub unsafe extern "C" fn _glp_mpl_delete_string(
    mut mpl: *mut MPL,
    mut str: *mut STRING,
) {
    _glp_dmp_free_atom(
        (*mpl).strings,
        str as *mut libc::c_void,
        (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_mpl_create_symbol_num(
    mut mpl: *mut MPL,
    mut num: libc::c_double,
) -> *mut SYMBOL {
    let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
    sym = _glp_dmp_get_atom(
        (*mpl).symbols,
        ::std::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int,
    ) as *mut SYMBOL;
    (*sym).num = num;
    (*sym).str_0 = 0 as *mut STRING;
    return sym;
}
pub unsafe extern "C" fn _glp_mpl_create_symbol_str(
    mut mpl: *mut MPL,
    mut str: *mut STRING,
) -> *mut SYMBOL {
    let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
    (!str.is_null()
        || {
            glp_assert_(
                b"str != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                583 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    sym = _glp_dmp_get_atom(
        (*mpl).symbols,
        ::std::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int,
    ) as *mut SYMBOL;
    (*sym).num = 0.0f64;
    (*sym).str_0 = str;
    return sym;
}
pub unsafe extern "C" fn _glp_mpl_copy_symbol(
    mut mpl: *mut MPL,
    mut sym: *mut SYMBOL,
) -> *mut SYMBOL {
    let mut copy: *mut SYMBOL = 0 as *mut SYMBOL;
    (!sym.is_null()
        || {
            glp_assert_(
                b"sym != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                600 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    copy = _glp_dmp_get_atom(
        (*mpl).symbols,
        ::std::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int,
    ) as *mut SYMBOL;
    if ((*sym).str_0).is_null() {
        (*copy).num = (*sym).num;
        (*copy).str_0 = 0 as *mut STRING;
    } else {
        (*copy).num = 0.0f64;
        (*copy).str_0 = _glp_mpl_copy_string(mpl, (*sym).str_0);
    }
    return copy;
}
pub unsafe extern "C" fn _glp_mpl_compare_symbols(
    mut mpl: *mut MPL,
    mut sym1: *mut SYMBOL,
    mut sym2: *mut SYMBOL,
) -> libc::c_int {
    (!sym1.is_null()
        || {
            glp_assert_(
                b"sym1 != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                631 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!sym2.is_null()
        || {
            glp_assert_(
                b"sym2 != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                632 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ((*sym1).str_0).is_null() && ((*sym2).str_0).is_null() {
        if (*sym1).num < (*sym2).num {
            return -(1 as libc::c_int);
        }
        if (*sym1).num > (*sym2).num {
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    if ((*sym1).str_0).is_null() {
        return -(1 as libc::c_int);
    }
    if ((*sym2).str_0).is_null() {
        return 1 as libc::c_int;
    }
    return _glp_mpl_compare_strings(mpl, (*sym1).str_0, (*sym2).str_0);
}
pub unsafe extern "C" fn _glp_mpl_delete_symbol(
    mut mpl: *mut MPL,
    mut sym: *mut SYMBOL,
) {
    (!sym.is_null()
        || {
            glp_assert_(
                b"sym != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                653 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !((*sym).str_0).is_null() {
        _glp_mpl_delete_string(mpl, (*sym).str_0);
    }
    _glp_dmp_free_atom(
        (*mpl).symbols,
        sym as *mut libc::c_void,
        ::std::mem::size_of::<SYMBOL>() as libc::c_ulong as libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_mpl_format_symbol(
    mut mpl: *mut MPL,
    mut sym: *mut SYMBOL,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = (*mpl).sym_buf;
    (!sym.is_null()
        || {
            glp_assert_(
                b"sym != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                673 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ((*sym).str_0).is_null() {
        sprintf(
            buf,
            b"%.*g\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int,
            (*sym).num,
        );
    } else {
        let mut str: [libc::c_char; 101] = [0; 101];
        let mut quoted: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut len: libc::c_int = 0;
        _glp_mpl_fetch_string(mpl, (*sym).str_0, str.as_mut_ptr());
        if !(*(*__ctype_b_loc())
            .offset(
                str[0 as libc::c_int as usize] as libc::c_uchar as libc::c_int as isize,
            ) as libc::c_int & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0 || str[0 as libc::c_int as usize] as libc::c_int == '_' as i32)
        {
            quoted = 1 as libc::c_int;
        } else {
            quoted = 0 as libc::c_int;
            j = 1 as libc::c_int;
            while str[j as usize] as libc::c_int != '\0' as i32 {
                if !(*(*__ctype_b_loc())
                    .offset(str[j as usize] as libc::c_uchar as libc::c_int as isize)
                    as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || !(strchr(
                        b"+-._\0" as *const u8 as *const libc::c_char,
                        str[j as usize] as libc::c_uchar as libc::c_int,
                    ))
                        .is_null())
                {
                    quoted = 1 as libc::c_int;
                    break;
                } else {
                    j += 1;
                    j;
                }
            }
        }
        *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        len = 0 as libc::c_int;
        if quoted != 0 {
            if len < 255 as libc::c_int {
                let fresh0 = len;
                len = len + 1;
                *buf.offset(fresh0 as isize) = '\'' as i32 as libc::c_char;
            } else {};
        }
        j = 0 as libc::c_int;
        while str[j as usize] as libc::c_int != '\0' as i32 {
            if quoted != 0 && str[j as usize] as libc::c_int == '\'' as i32 {
                if len < 255 as libc::c_int {
                    let fresh1 = len;
                    len = len + 1;
                    *buf.offset(fresh1 as isize) = '\'' as i32 as libc::c_char;
                } else {};
            }
            if len < 255 as libc::c_int {
                let fresh2 = len;
                len = len + 1;
                *buf.offset(fresh2 as isize) = str[j as usize];
            } else {};
            j += 1;
            j;
        }
        if quoted != 0 {
            if len < 255 as libc::c_int {
                let fresh3 = len;
                len = len + 1;
                *buf.offset(fresh3 as isize) = '\'' as i32 as libc::c_char;
            } else {};
        }
        *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
        if len == 255 as libc::c_int {
            strcpy(
                buf.offset(252 as libc::c_int as isize),
                b"...\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (strlen(buf) <= 255 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(buf) <= 255\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                705 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return buf;
}
pub unsafe extern "C" fn _glp_mpl_concat_symbols(
    mut mpl: *mut MPL,
    mut sym1: *mut SYMBOL,
    mut sym2: *mut SYMBOL,
) -> *mut SYMBOL {
    let mut str1: [libc::c_char; 101] = [0; 101];
    let mut str2: [libc::c_char; 101] = [0; 101];
    (100 as libc::c_int >= 15 as libc::c_int + 15 as libc::c_int
        || {
            glp_assert_(
                b"MAX_LENGTH >= DBL_DIG + DBL_DIG\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                722 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if ((*sym1).str_0).is_null() {
        sprintf(
            str1.as_mut_ptr(),
            b"%.*g\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int,
            (*sym1).num,
        );
    } else {
        _glp_mpl_fetch_string(mpl, (*sym1).str_0, str1.as_mut_ptr());
    }
    if ((*sym2).str_0).is_null() {
        sprintf(
            str2.as_mut_ptr(),
            b"%.*g\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int,
            (*sym2).num,
        );
    } else {
        _glp_mpl_fetch_string(mpl, (*sym2).str_0, str2.as_mut_ptr());
    }
    if (strlen(str1.as_mut_ptr())).wrapping_add(strlen(str2.as_mut_ptr()))
        > 100 as libc::c_int as libc::c_ulong
    {
        let mut buf: [libc::c_char; 256] = [0; 256];
        strcpy(buf.as_mut_ptr(), _glp_mpl_format_symbol(mpl, sym1));
        (strlen(buf.as_mut_ptr())
            < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
            || {
                glp_assert_(
                    b"strlen(buf) < sizeof(buf)\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    734 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_mpl_error(
            mpl,
            b"%s & %s; resultant symbol exceeds %d characters\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr(),
            _glp_mpl_format_symbol(mpl, sym2),
            100 as libc::c_int,
        );
    }
    _glp_mpl_delete_symbol(mpl, sym1);
    _glp_mpl_delete_symbol(mpl, sym2);
    return _glp_mpl_create_symbol_str(
        mpl,
        _glp_mpl_create_string(mpl, strcat(str1.as_mut_ptr(), str2.as_mut_ptr())),
    );
}
pub unsafe extern "C" fn _glp_mpl_create_tuple(mut mpl: *mut MPL) -> *mut TUPLE {
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                756 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    tuple = 0 as *mut TUPLE;
    return tuple;
}
pub unsafe extern "C" fn _glp_mpl_expand_tuple(
    mut mpl: *mut MPL,
    mut tuple: *mut TUPLE,
    mut sym: *mut SYMBOL,
) -> *mut TUPLE {
    let mut tail: *mut TUPLE = 0 as *mut TUPLE;
    let mut temp: *mut TUPLE = 0 as *mut TUPLE;
    (!sym.is_null()
        || {
            glp_assert_(
                b"sym != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                773 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    tail = _glp_dmp_get_atom(
        (*mpl).tuples,
        ::std::mem::size_of::<TUPLE>() as libc::c_ulong as libc::c_int,
    ) as *mut TUPLE;
    (*tail).sym = sym;
    (*tail).next = 0 as *mut TUPLE;
    if tuple.is_null() {
        tuple = tail;
    } else {
        temp = tuple;
        while !((*temp).next).is_null() {
            temp = (*temp).next;
        }
        (*temp).next = tail;
    }
    return tuple;
}
pub unsafe extern "C" fn _glp_mpl_tuple_dimen(
    mut mpl: *mut MPL,
    mut tuple: *mut TUPLE,
) -> libc::c_int {
    let mut temp: *mut TUPLE = 0 as *mut TUPLE;
    let mut dim: libc::c_int = 0 as libc::c_int;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                800 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    temp = tuple;
    while !temp.is_null() {
        dim += 1;
        dim;
        temp = (*temp).next;
    }
    return dim;
}
pub unsafe extern "C" fn _glp_mpl_copy_tuple(
    mut mpl: *mut MPL,
    mut tuple: *mut TUPLE,
) -> *mut TUPLE {
    let mut head: *mut TUPLE = 0 as *mut TUPLE;
    let mut tail: *mut TUPLE = 0 as *mut TUPLE;
    if tuple.is_null() {
        head = 0 as *mut TUPLE;
    } else {
        tail = _glp_dmp_get_atom(
            (*mpl).tuples,
            ::std::mem::size_of::<TUPLE>() as libc::c_ulong as libc::c_int,
        ) as *mut TUPLE;
        head = tail;
        while !tuple.is_null() {
            (!((*tuple).sym).is_null()
                || {
                    glp_assert_(
                        b"tuple->sym != NULL\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        820 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*tail).sym = _glp_mpl_copy_symbol(mpl, (*tuple).sym);
            if !((*tuple).next).is_null() {
                (*tail)
                    .next = _glp_dmp_get_atom(
                    (*mpl).tuples,
                    ::std::mem::size_of::<TUPLE>() as libc::c_ulong as libc::c_int,
                ) as *mut TUPLE;
                tail = (*tail).next;
            }
            tuple = (*tuple).next;
        }
        (*tail).next = 0 as *mut TUPLE;
    }
    return head;
}
pub unsafe extern "C" fn _glp_mpl_compare_tuples(
    mut mpl: *mut MPL,
    mut tuple1: *mut TUPLE,
    mut tuple2: *mut TUPLE,
) -> libc::c_int {
    let mut item1: *mut TUPLE = 0 as *mut TUPLE;
    let mut item2: *mut TUPLE = 0 as *mut TUPLE;
    let mut ret: libc::c_int = 0;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                851 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    item1 = tuple1;
    item2 = tuple2;
    while !item1.is_null() {
        (!item2.is_null()
            || {
                glp_assert_(
                    b"item2 != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    854 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (!((*item1).sym).is_null()
            || {
                glp_assert_(
                    b"item1->sym != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    855 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (!((*item2).sym).is_null()
            || {
                glp_assert_(
                    b"item2->sym != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    856 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ret = _glp_mpl_compare_symbols(mpl, (*item1).sym, (*item2).sym);
        if ret != 0 as libc::c_int {
            return ret;
        }
        item1 = (*item1).next;
        item2 = (*item2).next;
    }
    (item2.is_null()
        || {
            glp_assert_(
                b"item2 == NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                860 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_build_subtuple(
    mut mpl: *mut MPL,
    mut tuple: *mut TUPLE,
    mut dim: libc::c_int,
) -> *mut TUPLE {
    let mut head: *mut TUPLE = 0 as *mut TUPLE;
    let mut temp: *mut TUPLE = 0 as *mut TUPLE;
    let mut j: libc::c_int = 0;
    head = _glp_mpl_create_tuple(mpl);
    j = 1 as libc::c_int;
    temp = tuple;
    while j <= dim {
        (!temp.is_null()
            || {
                glp_assert_(
                    b"temp != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    879 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        head = _glp_mpl_expand_tuple(mpl, head, _glp_mpl_copy_symbol(mpl, (*temp).sym));
        j += 1;
        j;
        temp = (*temp).next;
    }
    return head;
}
pub unsafe extern "C" fn _glp_mpl_delete_tuple(
    mut mpl: *mut MPL,
    mut tuple: *mut TUPLE,
) {
    let mut temp: *mut TUPLE = 0 as *mut TUPLE;
    while !tuple.is_null() {
        temp = tuple;
        tuple = (*temp).next;
        (!((*temp).sym).is_null()
            || {
                glp_assert_(
                    b"temp->sym != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    898 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_mpl_delete_symbol(mpl, (*temp).sym);
        _glp_dmp_free_atom(
            (*mpl).tuples,
            temp as *mut libc::c_void,
            ::std::mem::size_of::<TUPLE>() as libc::c_ulong as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn _glp_mpl_format_tuple(
    mut mpl: *mut MPL,
    mut c: libc::c_int,
    mut tuple: *mut TUPLE,
) -> *mut libc::c_char {
    let mut temp: *mut TUPLE = 0 as *mut TUPLE;
    let mut dim: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut buf: *mut libc::c_char = (*mpl).tup_buf;
    let mut str: [libc::c_char; 256] = [0; 256];
    let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
    *buf.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    len = 0 as libc::c_int;
    dim = _glp_mpl_tuple_dimen(mpl, tuple);
    if c == '[' as i32 && dim > 0 as libc::c_int {
        if len < 255 as libc::c_int {
            let fresh4 = len;
            len = len + 1;
            *buf.offset(fresh4 as isize) = '[' as i32 as libc::c_char;
        } else {};
    }
    if c == '(' as i32 && dim > 1 as libc::c_int {
        if len < 255 as libc::c_int {
            let fresh5 = len;
            len = len + 1;
            *buf.offset(fresh5 as isize) = '(' as i32 as libc::c_char;
        } else {};
    }
    temp = tuple;
    while !temp.is_null() {
        if temp != tuple {
            if len < 255 as libc::c_int {
                let fresh6 = len;
                len = len + 1;
                *buf.offset(fresh6 as isize) = ',' as i32 as libc::c_char;
            } else {};
        }
        (!((*temp).sym).is_null()
            || {
                glp_assert_(
                    b"temp->sym != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    930 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        save = (*mpl).sym_buf;
        (*mpl).sym_buf = str.as_mut_ptr();
        _glp_mpl_format_symbol(mpl, (*temp).sym);
        (*mpl).sym_buf = save;
        (strlen(str.as_mut_ptr())
            < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
            || {
                glp_assert_(
                    b"strlen(str) < sizeof(str)\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    935 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        j = 0 as libc::c_int;
        while str[j as usize] as libc::c_int != '\0' as i32 {
            if len < 255 as libc::c_int {
                let fresh7 = len;
                len = len + 1;
                *buf.offset(fresh7 as isize) = str[j as usize];
            } else {};
            j += 1;
            j;
        }
        temp = (*temp).next;
    }
    if c == '[' as i32 && dim > 0 as libc::c_int {
        if len < 255 as libc::c_int {
            let fresh8 = len;
            len = len + 1;
            *buf.offset(fresh8 as isize) = ']' as i32 as libc::c_char;
        } else {};
    }
    if c == '(' as i32 && dim > 1 as libc::c_int {
        if len < 255 as libc::c_int {
            let fresh9 = len;
            len = len + 1;
            *buf.offset(fresh9 as isize) = ')' as i32 as libc::c_char;
        } else {};
    }
    *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
    if len == 255 as libc::c_int {
        strcpy(
            buf.offset(252 as libc::c_int as isize),
            b"...\0" as *const u8 as *const libc::c_char,
        );
    }
    (strlen(buf) <= 255 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(buf) <= 255\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                943 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return buf;
}
pub unsafe extern "C" fn _glp_mpl_create_elemset(
    mut mpl: *mut MPL,
    mut dim: libc::c_int,
) -> *mut ELEMSET {
    let mut set: *mut ELEMSET = 0 as *mut ELEMSET;
    (dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                959 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    set = _glp_mpl_create_array(mpl, 117 as libc::c_int, dim);
    return set;
}
pub unsafe extern "C" fn _glp_mpl_find_tuple(
    mut mpl: *mut MPL,
    mut set: *mut ELEMSET,
    mut tuple: *mut TUPLE,
) -> *mut MEMBER {
    (!set.is_null()
        || {
            glp_assert_(
                b"set != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                977 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*set).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"set->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                978 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*set).dim == _glp_mpl_tuple_dimen(mpl, tuple)
        || {
            glp_assert_(
                b"set->dim == tuple_dimen(mpl, tuple)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                979 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return _glp_mpl_find_member(mpl, set, tuple);
}
pub unsafe extern "C" fn _glp_mpl_add_tuple(
    mut mpl: *mut MPL,
    mut set: *mut ELEMSET,
    mut tuple: *mut TUPLE,
) -> *mut MEMBER {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    (!set.is_null()
        || {
            glp_assert_(
                b"set != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1000 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*set).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"set->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1001 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*set).dim == _glp_mpl_tuple_dimen(mpl, tuple)
        || {
            glp_assert_(
                b"set->dim == tuple_dimen(mpl, tuple)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1002 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    memb = _glp_mpl_add_member(mpl, set, tuple);
    (*memb).value.none = 0 as *mut libc::c_void;
    return memb;
}
pub unsafe extern "C" fn _glp_mpl_check_then_add(
    mut mpl: *mut MPL,
    mut set: *mut ELEMSET,
    mut tuple: *mut TUPLE,
) -> *mut MEMBER {
    if !(_glp_mpl_find_tuple(mpl, set, tuple)).is_null() {
        _glp_mpl_error(
            mpl,
            b"duplicate tuple %s detected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            _glp_mpl_format_tuple(mpl, '(' as i32, tuple),
        );
    }
    return _glp_mpl_add_tuple(mpl, set, tuple);
}
pub unsafe extern "C" fn _glp_mpl_copy_elemset(
    mut mpl: *mut MPL,
    mut set: *mut ELEMSET,
) -> *mut ELEMSET {
    let mut copy: *mut ELEMSET = 0 as *mut ELEMSET;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    (!set.is_null()
        || {
            glp_assert_(
                b"set != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1036 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*set).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"set->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1037 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*set).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"set->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1038 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    copy = _glp_mpl_create_elemset(mpl, (*set).dim);
    memb = (*set).head;
    while !memb.is_null() {
        _glp_mpl_add_tuple(mpl, copy, _glp_mpl_copy_tuple(mpl, (*memb).tuple));
        memb = (*memb).next;
    }
    return copy;
}
pub unsafe extern "C" fn _glp_mpl_delete_elemset(
    mut mpl: *mut MPL,
    mut set: *mut ELEMSET,
) {
    (!set.is_null()
        || {
            glp_assert_(
                b"set != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1054 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*set).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"set->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1055 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_mpl_delete_array(mpl, set);
}
pub unsafe extern "C" fn _glp_mpl_arelset_size(
    mut mpl: *mut MPL,
    mut t0: libc::c_double,
    mut tf: libc::c_double,
    mut dt: libc::c_double,
) -> libc::c_int {
    let mut temp: libc::c_double = 0.;
    if dt == 0.0f64 {
        _glp_mpl_error(
            mpl,
            b"%.*g .. %.*g by %.*g; zero stride not allowed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            15 as libc::c_int,
            t0,
            15 as libc::c_int,
            tf,
            15 as libc::c_int,
            dt,
        );
    }
    if tf > 0.0f64 && t0 < 0.0f64 && tf > 0.999f64 * 1.7976931348623157e+308f64 + t0 {
        temp = 1.7976931348623157e+308f64;
    } else if tf < 0.0f64 && t0 > 0.0f64
        && tf < -0.999f64 * 1.7976931348623157e+308f64 + t0
    {
        temp = -1.7976931348623157e+308f64;
    } else {
        temp = tf - t0;
    }
    if fabs(dt) < 1.0f64 && fabs(temp) > 0.999f64 * 1.7976931348623157e+308f64 * fabs(dt)
    {
        if temp > 0.0f64 && dt > 0.0f64 || temp < 0.0f64 && dt < 0.0f64 {
            temp = 1.7976931348623157e+308f64;
        } else {
            temp = 0.0f64;
        }
    } else {
        temp = floor(temp / dt) + 1.0f64;
        if temp < 0.0f64 {
            temp = 0.0f64;
        }
    }
    (temp >= 0.0f64
        || {
            glp_assert_(
                b"temp >= 0.0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1093 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if temp > (2147483647 as libc::c_int - 1 as libc::c_int) as libc::c_double {
        _glp_mpl_error(
            mpl,
            b"%.*g .. %.*g by %.*g; set too large\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            15 as libc::c_int,
            t0,
            15 as libc::c_int,
            tf,
            15 as libc::c_int,
            dt,
        );
    }
    return (temp + 0.5f64) as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_arelset_member(
    mut mpl: *mut MPL,
    mut t0: libc::c_double,
    mut tf: libc::c_double,
    mut dt: libc::c_double,
    mut j: libc::c_int,
) -> libc::c_double {
    (1 as libc::c_int <= j && j <= _glp_mpl_arelset_size(mpl, t0, tf, dt)
        || {
            glp_assert_(
                b"1 <= j && j <= arelset_size(mpl, t0, tf, dt)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1117 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return t0 + (j - 1 as libc::c_int) as libc::c_double * dt;
}
pub unsafe extern "C" fn _glp_mpl_create_arelset(
    mut mpl: *mut MPL,
    mut t0: libc::c_double,
    mut tf: libc::c_double,
    mut dt: libc::c_double,
) -> *mut ELEMSET {
    let mut set: *mut ELEMSET = 0 as *mut ELEMSET;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    set = _glp_mpl_create_elemset(mpl, 1 as libc::c_int);
    n = _glp_mpl_arelset_size(mpl, t0, tf, dt);
    j = 1 as libc::c_int;
    while j <= n {
        _glp_mpl_add_tuple(
            mpl,
            set,
            _glp_mpl_expand_tuple(
                mpl,
                _glp_mpl_create_tuple(mpl),
                _glp_mpl_create_symbol_num(
                    mpl,
                    _glp_mpl_arelset_member(mpl, t0, tf, dt, j),
                ),
            ),
        );
        j += 1;
        j;
    }
    return set;
}
pub unsafe extern "C" fn _glp_mpl_set_union(
    mut mpl: *mut MPL,
    mut X: *mut ELEMSET,
    mut Y: *mut ELEMSET,
) -> *mut ELEMSET {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    (!X.is_null()
        || {
            glp_assert_(
                b"X != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1168 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"X->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1169 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"X->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1170 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!Y.is_null()
        || {
            glp_assert_(
                b"Y != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1171 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*Y).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"Y->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1172 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*Y).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"Y->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1173 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).dim == (*Y).dim
        || {
            glp_assert_(
                b"X->dim == Y->dim\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1174 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    memb = (*Y).head;
    while !memb.is_null() {
        if (_glp_mpl_find_tuple(mpl, X, (*memb).tuple)).is_null() {
            _glp_mpl_add_tuple(mpl, X, _glp_mpl_copy_tuple(mpl, (*memb).tuple));
        }
        memb = (*memb).next;
    }
    _glp_mpl_delete_elemset(mpl, Y);
    return X;
}
pub unsafe extern "C" fn _glp_mpl_set_diff(
    mut mpl: *mut MPL,
    mut X: *mut ELEMSET,
    mut Y: *mut ELEMSET,
) -> *mut ELEMSET {
    let mut Z: *mut ELEMSET = 0 as *mut ELEMSET;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    (!X.is_null()
        || {
            glp_assert_(
                b"X != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1199 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"X->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1200 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"X->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1201 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!Y.is_null()
        || {
            glp_assert_(
                b"Y != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1202 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*Y).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"Y->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1203 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*Y).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"Y->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1204 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).dim == (*Y).dim
        || {
            glp_assert_(
                b"X->dim == Y->dim\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1205 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    Z = _glp_mpl_create_elemset(mpl, (*X).dim);
    memb = (*X).head;
    while !memb.is_null() {
        if (_glp_mpl_find_tuple(mpl, Y, (*memb).tuple)).is_null() {
            _glp_mpl_add_tuple(mpl, Z, _glp_mpl_copy_tuple(mpl, (*memb).tuple));
        }
        memb = (*memb).next;
    }
    _glp_mpl_delete_elemset(mpl, X);
    _glp_mpl_delete_elemset(mpl, Y);
    return Z;
}
pub unsafe extern "C" fn _glp_mpl_set_symdiff(
    mut mpl: *mut MPL,
    mut X: *mut ELEMSET,
    mut Y: *mut ELEMSET,
) -> *mut ELEMSET {
    let mut Z: *mut ELEMSET = 0 as *mut ELEMSET;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    (!X.is_null()
        || {
            glp_assert_(
                b"X != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1232 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"X->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1233 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"X->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1234 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!Y.is_null()
        || {
            glp_assert_(
                b"Y != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1235 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*Y).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"Y->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1236 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*Y).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"Y->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1237 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).dim == (*Y).dim
        || {
            glp_assert_(
                b"X->dim == Y->dim\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1238 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    Z = _glp_mpl_create_elemset(mpl, (*X).dim);
    memb = (*X).head;
    while !memb.is_null() {
        if (_glp_mpl_find_tuple(mpl, Y, (*memb).tuple)).is_null() {
            _glp_mpl_add_tuple(mpl, Z, _glp_mpl_copy_tuple(mpl, (*memb).tuple));
        }
        memb = (*memb).next;
    }
    memb = (*Y).head;
    while !memb.is_null() {
        if (_glp_mpl_find_tuple(mpl, X, (*memb).tuple)).is_null() {
            _glp_mpl_add_tuple(mpl, Z, _glp_mpl_copy_tuple(mpl, (*memb).tuple));
        }
        memb = (*memb).next;
    }
    _glp_mpl_delete_elemset(mpl, X);
    _glp_mpl_delete_elemset(mpl, Y);
    return Z;
}
pub unsafe extern "C" fn _glp_mpl_set_inter(
    mut mpl: *mut MPL,
    mut X: *mut ELEMSET,
    mut Y: *mut ELEMSET,
) -> *mut ELEMSET {
    let mut Z: *mut ELEMSET = 0 as *mut ELEMSET;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    (!X.is_null()
        || {
            glp_assert_(
                b"X != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1271 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"X->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1272 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"X->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1273 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!Y.is_null()
        || {
            glp_assert_(
                b"Y != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1274 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*Y).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"Y->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1275 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*Y).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"Y->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1276 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).dim == (*Y).dim
        || {
            glp_assert_(
                b"X->dim == Y->dim\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1277 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    Z = _glp_mpl_create_elemset(mpl, (*X).dim);
    memb = (*X).head;
    while !memb.is_null() {
        if !(_glp_mpl_find_tuple(mpl, Y, (*memb).tuple)).is_null() {
            _glp_mpl_add_tuple(mpl, Z, _glp_mpl_copy_tuple(mpl, (*memb).tuple));
        }
        memb = (*memb).next;
    }
    _glp_mpl_delete_elemset(mpl, X);
    _glp_mpl_delete_elemset(mpl, Y);
    return Z;
}
pub unsafe extern "C" fn _glp_mpl_set_cross(
    mut mpl: *mut MPL,
    mut X: *mut ELEMSET,
    mut Y: *mut ELEMSET,
) -> *mut ELEMSET {
    let mut Z: *mut ELEMSET = 0 as *mut ELEMSET;
    let mut memx: *mut MEMBER = 0 as *mut MEMBER;
    let mut memy: *mut MEMBER = 0 as *mut MEMBER;
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    let mut temp: *mut TUPLE = 0 as *mut TUPLE;
    (!X.is_null()
        || {
            glp_assert_(
                b"X != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1305 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"X->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1306 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*X).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"X->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1307 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!Y.is_null()
        || {
            glp_assert_(
                b"Y != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1308 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*Y).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"Y->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1309 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*Y).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"Y->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1310 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    Z = _glp_mpl_create_elemset(mpl, (*X).dim + (*Y).dim);
    memx = (*X).head;
    while !memx.is_null() {
        memy = (*Y).head;
        while !memy.is_null() {
            tuple = _glp_mpl_copy_tuple(mpl, (*memx).tuple);
            temp = (*memy).tuple;
            while !temp.is_null() {
                tuple = _glp_mpl_expand_tuple(
                    mpl,
                    tuple,
                    _glp_mpl_copy_symbol(mpl, (*temp).sym),
                );
                temp = (*temp).next;
            }
            _glp_mpl_add_tuple(mpl, Z, tuple);
            memy = (*memy).next;
        }
        memx = (*memx).next;
    }
    _glp_mpl_delete_elemset(mpl, X);
    _glp_mpl_delete_elemset(mpl, Y);
    return Z;
}
pub unsafe extern "C" fn _glp_mpl_constant_term(
    mut mpl: *mut MPL,
    mut coef: libc::c_double,
) -> *mut FORMULA {
    let mut form: *mut FORMULA = 0 as *mut FORMULA;
    if coef == 0.0f64 {
        form = 0 as *mut FORMULA;
    } else {
        form = _glp_dmp_get_atom(
            (*mpl).formulae,
            ::std::mem::size_of::<FORMULA>() as libc::c_ulong as libc::c_int,
        ) as *mut FORMULA;
        (*form).coef = coef;
        (*form).var = 0 as *mut ELEMVAR;
        (*form).next = 0 as *mut FORMULA;
    }
    return form;
}
pub unsafe extern "C" fn _glp_mpl_single_variable(
    mut mpl: *mut MPL,
    mut var: *mut ELEMVAR,
) -> *mut FORMULA {
    let mut form: *mut FORMULA = 0 as *mut FORMULA;
    (!var.is_null()
        || {
            glp_assert_(
                b"var != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1365 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    form = _glp_dmp_get_atom(
        (*mpl).formulae,
        ::std::mem::size_of::<FORMULA>() as libc::c_ulong as libc::c_int,
    ) as *mut FORMULA;
    (*form).coef = 1.0f64;
    (*form).var = var;
    (*form).next = 0 as *mut FORMULA;
    return form;
}
pub unsafe extern "C" fn _glp_mpl_copy_formula(
    mut mpl: *mut MPL,
    mut form: *mut FORMULA,
) -> *mut FORMULA {
    let mut head: *mut FORMULA = 0 as *mut FORMULA;
    let mut tail: *mut FORMULA = 0 as *mut FORMULA;
    if form.is_null() {
        head = 0 as *mut FORMULA;
    } else {
        tail = _glp_dmp_get_atom(
            (*mpl).formulae,
            ::std::mem::size_of::<FORMULA>() as libc::c_ulong as libc::c_int,
        ) as *mut FORMULA;
        head = tail;
        while !form.is_null() {
            (*tail).coef = (*form).coef;
            (*tail).var = (*form).var;
            if !((*form).next).is_null() {
                (*tail)
                    .next = _glp_dmp_get_atom(
                    (*mpl).formulae,
                    ::std::mem::size_of::<FORMULA>() as libc::c_ulong as libc::c_int,
                ) as *mut FORMULA;
                tail = (*tail).next;
            }
            form = (*form).next;
        }
        (*tail).next = 0 as *mut FORMULA;
    }
    return head;
}
pub unsafe extern "C" fn _glp_mpl_delete_formula(
    mut mpl: *mut MPL,
    mut form: *mut FORMULA,
) {
    let mut temp: *mut FORMULA = 0 as *mut FORMULA;
    while !form.is_null() {
        temp = form;
        form = (*form).next;
        _glp_dmp_free_atom(
            (*mpl).formulae,
            temp as *mut libc::c_void,
            ::std::mem::size_of::<FORMULA>() as libc::c_ulong as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn _glp_mpl_linear_comb(
    mut mpl: *mut MPL,
    mut a: libc::c_double,
    mut fx: *mut FORMULA,
    mut b: libc::c_double,
    mut fy: *mut FORMULA,
) -> *mut FORMULA {
    let mut form: *mut FORMULA = 0 as *mut FORMULA;
    let mut term: *mut FORMULA = 0 as *mut FORMULA;
    let mut temp: *mut FORMULA = 0 as *mut FORMULA;
    let mut c0: libc::c_double = 0.0f64;
    term = fx;
    while !term.is_null() {
        if ((*term).var).is_null() {
            c0 = _glp_mpl_fp_add(mpl, c0, _glp_mpl_fp_mul(mpl, a, (*term).coef));
        } else {
            (*(*term).var)
                .temp = _glp_mpl_fp_add(
                mpl,
                (*(*term).var).temp,
                _glp_mpl_fp_mul(mpl, a, (*term).coef),
            );
        }
        term = (*term).next;
    }
    term = fy;
    while !term.is_null() {
        if ((*term).var).is_null() {
            c0 = _glp_mpl_fp_add(mpl, c0, _glp_mpl_fp_mul(mpl, b, (*term).coef));
        } else {
            (*(*term).var)
                .temp = _glp_mpl_fp_add(
                mpl,
                (*(*term).var).temp,
                _glp_mpl_fp_mul(mpl, b, (*term).coef),
            );
        }
        term = (*term).next;
    }
    term = fx;
    while !term.is_null() {
        if !((*term).var).is_null() && (*(*term).var).temp != 0.0f64 {
            temp = _glp_dmp_get_atom(
                (*mpl).formulae,
                ::std::mem::size_of::<FORMULA>() as libc::c_ulong as libc::c_int,
            ) as *mut FORMULA;
            (*temp).coef = (*(*term).var).temp;
            (*temp).var = (*term).var;
            (*temp).next = form;
            form = temp;
            (*(*term).var).temp = 0.0f64;
        }
        term = (*term).next;
    }
    term = fy;
    while !term.is_null() {
        if !((*term).var).is_null() && (*(*term).var).temp != 0.0f64 {
            temp = _glp_dmp_get_atom(
                (*mpl).formulae,
                ::std::mem::size_of::<FORMULA>() as libc::c_ulong as libc::c_int,
            ) as *mut FORMULA;
            (*temp).coef = (*(*term).var).temp;
            (*temp).var = (*term).var;
            (*temp).next = form;
            form = temp;
            (*(*term).var).temp = 0.0f64;
        }
        term = (*term).next;
    }
    if c0 != 0.0f64 {
        temp = _glp_dmp_get_atom(
            (*mpl).formulae,
            ::std::mem::size_of::<FORMULA>() as libc::c_ulong as libc::c_int,
        ) as *mut FORMULA;
        (*temp).coef = c0;
        (*temp).var = 0 as *mut ELEMVAR;
        (*temp).next = form;
        form = temp;
    }
    _glp_mpl_delete_formula(mpl, fx);
    _glp_mpl_delete_formula(mpl, fy);
    return form;
}
pub unsafe extern "C" fn _glp_mpl_remove_constant(
    mut mpl: *mut MPL,
    mut form: *mut FORMULA,
    mut coef: *mut libc::c_double,
) -> *mut FORMULA {
    let mut head: *mut FORMULA = 0 as *mut FORMULA;
    let mut temp: *mut FORMULA = 0 as *mut FORMULA;
    *coef = 0.0f64;
    while !form.is_null() {
        temp = form;
        form = (*form).next;
        if ((*temp).var).is_null() {
            *coef = _glp_mpl_fp_add(mpl, *coef, (*temp).coef);
            _glp_dmp_free_atom(
                (*mpl).formulae,
                temp as *mut libc::c_void,
                ::std::mem::size_of::<FORMULA>() as libc::c_ulong as libc::c_int,
            );
        } else {
            (*temp).next = head;
            head = temp;
        }
    }
    return head;
}
pub unsafe extern "C" fn _glp_mpl_reduce_terms(
    mut mpl: *mut MPL,
    mut form: *mut FORMULA,
) -> *mut FORMULA {
    let mut term: *mut FORMULA = 0 as *mut FORMULA;
    let mut next_term: *mut FORMULA = 0 as *mut FORMULA;
    let mut c0: libc::c_double = 0.0f64;
    term = form;
    while !term.is_null() {
        if ((*term).var).is_null() {
            c0 = _glp_mpl_fp_add(mpl, c0, (*term).coef);
        } else {
            (*(*term).var)
                .temp = _glp_mpl_fp_add(mpl, (*(*term).var).temp, (*term).coef);
        }
        term = (*term).next;
    }
    next_term = form;
    form = 0 as *mut FORMULA;
    term = next_term;
    while !term.is_null() {
        next_term = (*term).next;
        if ((*term).var).is_null() && c0 != 0.0f64 {
            (*term).coef = c0;
            c0 = 0.0f64;
            (*term).next = form;
            form = term;
        } else if !((*term).var).is_null() && (*(*term).var).temp != 0.0f64 {
            (*term).coef = (*(*term).var).temp;
            (*(*term).var).temp = 0.0f64;
            (*term).next = form;
            form = term;
        } else {
            _glp_dmp_free_atom(
                (*mpl).formulae,
                term as *mut libc::c_void,
                ::std::mem::size_of::<FORMULA>() as libc::c_ulong as libc::c_int,
            );
        }
        term = next_term;
    }
    return form;
}
pub unsafe extern "C" fn _glp_mpl_delete_value(
    mut mpl: *mut MPL,
    mut type_0: libc::c_int,
    mut value: *mut VALUE,
) {
    (!value.is_null()
        || {
            glp_assert_(
                b"value != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1559 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match type_0 {
        117 => {
            (*value).none = 0 as *mut libc::c_void;
        }
        118 => {
            (*value).num = 0.0f64;
        }
        124 => {
            _glp_mpl_delete_symbol(mpl, (*value).sym);
            (*value).sym = 0 as *mut SYMBOL;
        }
        114 => {
            (*value).bit = 0 as libc::c_int;
        }
        126 => {
            _glp_mpl_delete_tuple(mpl, (*value).tuple);
            (*value).tuple = 0 as *mut TUPLE;
        }
        106 => {
            _glp_mpl_delete_elemset(mpl, (*value).set);
            (*value).set = 0 as *mut ELEMSET;
        }
        107 => {
            (*value).var = 0 as *mut ELEMVAR;
        }
        110 => {
            _glp_mpl_delete_formula(mpl, (*value).form);
            (*value).form = 0 as *mut FORMULA;
        }
        105 => {
            (*value).con = 0 as *mut ELEMCON;
        }
        _ => {
            (type_0 != type_0
                || {
                    glp_assert_(
                        b"type != type\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        1589 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn _glp_mpl_create_array(
    mut mpl: *mut MPL,
    mut type_0: libc::c_int,
    mut dim: libc::c_int,
) -> *mut ARRAY {
    let mut array: *mut ARRAY = 0 as *mut ARRAY;
    (type_0 == 117 as libc::c_int || type_0 == 118 as libc::c_int
        || type_0 == 124 as libc::c_int || type_0 == 106 as libc::c_int
        || type_0 == 107 as libc::c_int || type_0 == 105 as libc::c_int
        || {
            glp_assert_(
                b"type == A_NONE || type == A_NUMERIC || type == A_SYMBOLIC || type == A_ELEMSET || type == A_ELEMVAR || type == A_ELEMCON\0"
                    as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1621 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (dim >= 0 as libc::c_int
        || {
            glp_assert_(
                b"dim >= 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1622 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    array = _glp_dmp_get_atom(
        (*mpl).arrays,
        ::std::mem::size_of::<ARRAY>() as libc::c_ulong as libc::c_int,
    ) as *mut ARRAY;
    (*array).type_0 = type_0;
    (*array).dim = dim;
    (*array).size = 0 as libc::c_int;
    (*array).head = 0 as *mut MEMBER;
    (*array).tail = 0 as *mut MEMBER;
    (*array).tree = 0 as *mut AVL;
    (*array).prev = 0 as *mut ARRAY;
    (*array).next = (*mpl).a_list;
    if !((*array).next).is_null() {
        (*(*array).next).prev = array;
    }
    (*mpl).a_list = array;
    return array;
}
unsafe extern "C" fn compare_member_tuples(
    mut info: *mut libc::c_void,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    return _glp_mpl_compare_tuples(
        info as *mut MPL,
        key1 as *mut TUPLE,
        key2 as *mut TUPLE,
    );
}
pub unsafe extern "C" fn _glp_mpl_find_member(
    mut mpl: *mut MPL,
    mut array: *mut ARRAY,
    mut tuple: *mut TUPLE,
) -> *mut MEMBER {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    (!array.is_null()
        || {
            glp_assert_(
                b"array != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1659 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (_glp_mpl_tuple_dimen(mpl, tuple) == (*array).dim
        || {
            glp_assert_(
                b"tuple_dimen(mpl, tuple) == array->dim\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1661 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*array).size > 30 as libc::c_int && ((*array).tree).is_null() {
        (*array)
            .tree = _glp_avl_create_tree(
            Some(
                compare_member_tuples
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            mpl as *mut libc::c_void,
        );
        memb = (*array).head;
        while !memb.is_null() {
            _glp_avl_set_node_link(
                _glp_avl_insert_node(
                    (*array).tree,
                    (*memb).tuple as *const libc::c_void,
                ),
                memb as *mut libc::c_void,
            );
            memb = (*memb).next;
        }
    }
    if ((*array).tree).is_null() {
        memb = (*array).head;
        while !memb.is_null() {
            if _glp_mpl_compare_tuples(mpl, (*memb).tuple, tuple) == 0 as libc::c_int {
                break;
            }
            memb = (*memb).next;
        }
    } else {
        let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
        node = _glp_avl_find_node((*array).tree, tuple as *const libc::c_void);
        memb = (if node.is_null() {
            0 as *mut libc::c_void
        } else {
            _glp_avl_get_node_link(node)
        }) as *mut MEMBER;
    }
    return memb;
}
pub unsafe extern "C" fn _glp_mpl_add_member(
    mut mpl: *mut MPL,
    mut array: *mut ARRAY,
    mut tuple: *mut TUPLE,
) -> *mut MEMBER {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    (!array.is_null()
        || {
            glp_assert_(
                b"array != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1706 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (_glp_mpl_tuple_dimen(mpl, tuple) == (*array).dim
        || {
            glp_assert_(
                b"tuple_dimen(mpl, tuple) == array->dim\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1708 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    memb = _glp_dmp_get_atom(
        (*mpl).members,
        ::std::mem::size_of::<MEMBER>() as libc::c_ulong as libc::c_int,
    ) as *mut MEMBER;
    (*memb).tuple = tuple;
    (*memb).next = 0 as *mut MEMBER;
    memset(
        &mut (*memb).value as *mut VALUE as *mut libc::c_void,
        '?' as i32,
        ::std::mem::size_of::<VALUE>() as libc::c_ulong,
    );
    (*array).size += 1;
    (*array).size;
    if ((*array).head).is_null() {
        (*array).head = memb;
    } else {
        (*(*array).tail).next = memb;
    }
    (*array).tail = memb;
    if !((*array).tree).is_null() {
        _glp_avl_set_node_link(
            _glp_avl_insert_node((*array).tree, (*memb).tuple as *const libc::c_void),
            memb as *mut libc::c_void,
        );
    }
    return memb;
}
pub unsafe extern "C" fn _glp_mpl_delete_array(
    mut mpl: *mut MPL,
    mut array: *mut ARRAY,
) {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    (!array.is_null()
        || {
            glp_assert_(
                b"array != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1742 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    while !((*array).head).is_null() {
        memb = (*array).head;
        (*array).head = (*memb).next;
        _glp_mpl_delete_tuple(mpl, (*memb).tuple);
        _glp_dmp_free_atom(
            (*mpl).members,
            memb as *mut libc::c_void,
            ::std::mem::size_of::<MEMBER>() as libc::c_ulong as libc::c_int,
        );
    }
    if !((*array).tree).is_null() {
        _glp_avl_delete_tree((*array).tree);
    }
    if ((*array).prev).is_null() {
        (*mpl).a_list = (*array).next;
    } else {
        (*(*array).prev).next = (*array).next;
    }
    if !((*array).next).is_null() {
        (*(*array).next).prev = (*array).prev;
    }
    _glp_dmp_free_atom(
        (*mpl).arrays,
        array as *mut libc::c_void,
        ::std::mem::size_of::<ARRAY>() as libc::c_ulong as libc::c_int,
    );
}
pub unsafe extern "C" fn _glp_mpl_assign_dummy_index(
    mut mpl: *mut MPL,
    mut slot: *mut DOMAIN_SLOT,
    mut value: *mut SYMBOL,
) {
    let mut current_block: u64;
    let mut leaf: *mut CODE = 0 as *mut CODE;
    let mut code: *mut CODE = 0 as *mut CODE;
    (!slot.is_null()
        || {
            glp_assert_(
                b"slot != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1783 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!value.is_null()
        || {
            glp_assert_(
                b"value != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1784 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !((*slot).value).is_null() {
        if _glp_mpl_compare_symbols(mpl, (*slot).value, value) == 0 as libc::c_int {
            current_block = 8882875065143129722;
        } else {
            _glp_mpl_delete_symbol(mpl, (*slot).value);
            (*slot).value = 0 as *mut SYMBOL;
            current_block = 715039052867723359;
        }
    } else {
        current_block = 715039052867723359;
    }
    match current_block {
        715039052867723359 => {
            leaf = (*slot).list;
            while !leaf.is_null() {
                ((*leaf).op == 303 as libc::c_int
                    || {
                        glp_assert_(
                            b"leaf->op == O_INDEX\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            1798 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                code = leaf;
                while !code.is_null() {
                    if (*code).valid != 0 {
                        (*code).valid = 0 as libc::c_int;
                        _glp_mpl_delete_value(mpl, (*code).type_0, &mut (*code).value);
                    }
                    code = (*code).up;
                }
                leaf = (*leaf).arg.index.next;
            }
            (*slot).value = _glp_mpl_copy_symbol(mpl, value);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn _glp_mpl_update_dummy_indices(
    mut mpl: *mut MPL,
    mut block: *mut DOMAIN_BLOCK,
) {
    let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
    let mut temp: *mut TUPLE = 0 as *mut TUPLE;
    if !((*block).backup).is_null() {
        slot = (*block).list;
        temp = (*block).backup;
        while !slot.is_null() {
            (!temp.is_null()
                || {
                    glp_assert_(
                        b"temp != NULL\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        1831 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (!((*temp).sym).is_null()
                || {
                    glp_assert_(
                        b"temp->sym != NULL\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        1832 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            _glp_mpl_assign_dummy_index(mpl, slot, (*temp).sym);
            slot = (*slot).next;
            temp = (*temp).next;
        }
    }
}
pub unsafe extern "C" fn _glp_mpl_enter_domain_block(
    mut mpl: *mut MPL,
    mut block: *mut DOMAIN_BLOCK,
    mut tuple: *mut TUPLE,
    mut info: *mut libc::c_void,
    mut func: Option::<unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> ()>,
) -> libc::c_int {
    let mut backup: *mut TUPLE = 0 as *mut TUPLE;
    let mut ret: libc::c_int = 0 as libc::c_int;
    (!((*block).code).is_null()
        || {
            glp_assert_(
                b"block->code != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                1880 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if _glp_mpl_is_member(mpl, (*block).code, tuple) == 0 {
        ret = 1 as libc::c_int;
    } else {
        backup = (*block).backup;
        (*block).backup = tuple;
        _glp_mpl_update_dummy_indices(mpl, block);
        func.unwrap()(mpl, info);
        (*block).backup = backup;
        _glp_mpl_update_dummy_indices(mpl, block);
    }
    return ret;
}
unsafe extern "C" fn eval_domain_func(
    mut mpl: *mut MPL,
    mut _my_info: *mut libc::c_void,
) {
    let mut my_info: *mut eval_domain_info = _my_info as *mut eval_domain_info;
    if !((*my_info).block).is_null() {
        let mut block: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
        let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
        let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
        let mut temp: *mut TUPLE = 0 as *mut TUPLE;
        block = (*my_info).block;
        (*my_info).block = (*block).next;
        slot = (*block).list;
        while !slot.is_null() {
            if tuple.is_null() {
                temp = _glp_dmp_get_atom(
                    (*mpl).tuples,
                    ::std::mem::size_of::<TUPLE>() as libc::c_ulong as libc::c_int,
                ) as *mut TUPLE;
                tuple = temp;
            } else {
                (*temp)
                    .next = _glp_dmp_get_atom(
                    (*mpl).tuples,
                    ::std::mem::size_of::<TUPLE>() as libc::c_ulong as libc::c_int,
                ) as *mut TUPLE;
                temp = (*temp).next;
            }
            if ((*slot).code).is_null() {
                (!((*my_info).tuple).is_null()
                    || {
                        glp_assert_(
                            b"my_info->tuple != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            1987 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*temp).sym = (*(*my_info).tuple).sym;
                (!((*temp).sym).is_null()
                    || {
                        glp_assert_(
                            b"temp->sym != NULL\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            1989 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*my_info).tuple = (*(*my_info).tuple).next;
            } else {
                (*temp).sym = _glp_mpl_eval_symbolic(mpl, (*slot).code);
            }
            slot = (*slot).next;
        }
        (*temp).next = 0 as *mut TUPLE;
        if _glp_mpl_enter_domain_block(
            mpl,
            block,
            tuple,
            my_info as *mut libc::c_void,
            Some(
                eval_domain_func
                    as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> (),
            ),
        ) != 0
        {
            (*my_info).failure = 1 as libc::c_int;
        }
        slot = (*block).list;
        while !slot.is_null() {
            (!tuple.is_null()
                || {
                    glp_assert_(
                        b"tuple != NULL\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        2005 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            temp = tuple;
            tuple = (*tuple).next;
            if !((*slot).code).is_null() {
                _glp_mpl_delete_symbol(mpl, (*temp).sym);
            }
            _glp_dmp_free_atom(
                (*mpl).tuples,
                temp as *mut libc::c_void,
                ::std::mem::size_of::<TUPLE>() as libc::c_ulong as libc::c_int,
            );
            slot = (*slot).next;
        }
    } else {
        (((*my_info).tuple).is_null()
            || {
                glp_assert_(
                    b"my_info->tuple == NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2019 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if !((*(*my_info).domain).code).is_null()
            && _glp_mpl_eval_logical(mpl, (*(*my_info).domain).code) == 0
        {
            (*my_info).failure = 2 as libc::c_int;
        } else {
            ((*my_info).func).unwrap()(mpl, (*my_info).info);
        }
    };
}
pub unsafe extern "C" fn _glp_mpl_eval_within_domain(
    mut mpl: *mut MPL,
    mut domain: *mut DOMAIN1,
    mut tuple: *mut TUPLE,
    mut info: *mut libc::c_void,
    mut func: Option::<unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> ()>,
) -> libc::c_int {
    let mut _my_info: eval_domain_info = eval_domain_info {
        domain: 0 as *mut DOMAIN1,
        block: 0 as *mut DOMAIN_BLOCK,
        tuple: 0 as *mut TUPLE,
        info: 0 as *mut libc::c_void,
        func: None,
        failure: 0,
    };
    let mut my_info: *mut eval_domain_info = &mut _my_info;
    if domain.is_null() {
        (tuple.is_null()
            || {
                glp_assert_(
                    b"tuple == NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2043 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        func.unwrap()(mpl, info);
        (*my_info).failure = 0 as libc::c_int;
    } else {
        (!tuple.is_null()
            || {
                glp_assert_(
                    b"tuple != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2048 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*my_info).domain = domain;
        (*my_info).block = (*domain).list;
        (*my_info).tuple = tuple;
        (*my_info).info = info;
        (*my_info).func = func;
        (*my_info).failure = 0 as libc::c_int;
        eval_domain_func(mpl, my_info as *mut libc::c_void);
    }
    return (*my_info).failure;
}
unsafe extern "C" fn loop_domain_func(
    mut mpl: *mut MPL,
    mut _my_info: *mut libc::c_void,
) {
    let mut my_info: *mut loop_domain_info = _my_info as *mut loop_domain_info;
    if !((*my_info).block).is_null() {
        let mut block: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
        let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
        let mut bound: *mut TUPLE = 0 as *mut TUPLE;
        block = (*my_info).block;
        (*my_info).block = (*block).next;
        bound = _glp_mpl_create_tuple(mpl);
        slot = (*block).list;
        while !slot.is_null() {
            if !((*slot).code).is_null() {
                bound = _glp_mpl_expand_tuple(
                    mpl,
                    bound,
                    _glp_mpl_eval_symbolic(mpl, (*slot).code),
                );
            }
            slot = (*slot).next;
        }
        (!((*block).code).is_null()
            || {
                glp_assert_(
                    b"block->code != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2118 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        if (*(*block).code).op == 373 as libc::c_int {
            let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
            let mut n: libc::c_int = 0;
            let mut j: libc::c_int = 0;
            let mut t0: libc::c_double = 0.;
            let mut tf: libc::c_double = 0.;
            let mut dt: libc::c_double = 0.;
            t0 = _glp_mpl_eval_numeric(mpl, (*(*block).code).arg.arg.x);
            tf = _glp_mpl_eval_numeric(mpl, (*(*block).code).arg.arg.y);
            if ((*(*block).code).arg.arg.z).is_null() {
                dt = 1.0f64;
            } else {
                dt = _glp_mpl_eval_numeric(mpl, (*(*block).code).arg.arg.z);
            }
            n = _glp_mpl_arelset_size(mpl, t0, tf, dt);
            tuple = _glp_mpl_expand_tuple(
                mpl,
                _glp_mpl_create_tuple(mpl),
                _glp_mpl_create_symbol_num(mpl, 0.0f64),
            );
            (bound.is_null()
                || {
                    glp_assert_(
                        b"bound == NULL\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        2139 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            j = 1 as libc::c_int;
            while j <= n && (*my_info).looping != 0 {
                (*(*tuple).sym).num = _glp_mpl_arelset_member(mpl, t0, tf, dt, j);
                _glp_mpl_enter_domain_block(
                    mpl,
                    block,
                    tuple,
                    my_info as *mut libc::c_void,
                    Some(
                        loop_domain_func
                            as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> (),
                    ),
                );
                j += 1;
                j;
            }
            _glp_mpl_delete_tuple(mpl, tuple);
        } else {
            let mut set: *mut ELEMSET = 0 as *mut ELEMSET;
            let mut memb: *mut MEMBER = 0 as *mut MEMBER;
            let mut temp1: *mut TUPLE = 0 as *mut TUPLE;
            let mut temp2: *mut TUPLE = 0 as *mut TUPLE;
            set = _glp_mpl_eval_elemset(mpl, (*block).code);
            memb = (*set).head;
            while !memb.is_null() && (*my_info).looping != 0 {
                let mut current_block_36: u64;
                temp1 = (*memb).tuple;
                temp2 = bound;
                slot = (*block).list;
                loop {
                    if slot.is_null() {
                        current_block_36 = 9520865839495247062;
                        break;
                    }
                    (!temp1.is_null()
                        || {
                            glp_assert_(
                                b"temp1 != NULL\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                2168 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if !((*slot).code).is_null() {
                        (!temp2.is_null()
                            || {
                                glp_assert_(
                                    b"temp2 != NULL\0" as *const u8 as *const libc::c_char,
                                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                    2171 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        if _glp_mpl_compare_symbols(mpl, (*temp1).sym, (*temp2).sym)
                            != 0 as libc::c_int
                        {
                            current_block_36 = 14832935472441733737;
                            break;
                        }
                        temp2 = (*temp2).next;
                    }
                    temp1 = (*temp1).next;
                    slot = (*slot).next;
                }
                match current_block_36 {
                    9520865839495247062 => {
                        (temp1.is_null()
                            || {
                                glp_assert_(
                                    b"temp1 == NULL\0" as *const u8 as *const libc::c_char,
                                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                    2181 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        (temp2.is_null()
                            || {
                                glp_assert_(
                                    b"temp2 == NULL\0" as *const u8 as *const libc::c_char,
                                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                    2182 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                        _glp_mpl_enter_domain_block(
                            mpl,
                            block,
                            (*memb).tuple,
                            my_info as *mut libc::c_void,
                            Some(
                                loop_domain_func
                                    as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> (),
                            ),
                        );
                    }
                    _ => {}
                }
                memb = (*memb).next;
            }
            _glp_mpl_delete_elemset(mpl, set);
        }
        _glp_mpl_delete_tuple(mpl, bound);
        (*my_info).block = block;
    } else if !(!((*(*my_info).domain).code).is_null()
        && _glp_mpl_eval_logical(mpl, (*(*my_info).domain).code) == 0)
    {
        (*my_info)
            .looping = (((*my_info).func).unwrap()(mpl, (*my_info).info) == 0)
            as libc::c_int;
    }
}
pub unsafe extern "C" fn _glp_mpl_loop_within_domain(
    mut mpl: *mut MPL,
    mut domain: *mut DOMAIN1,
    mut info: *mut libc::c_void,
    mut func: Option::<unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> libc::c_int>,
) {
    let mut _my_info: loop_domain_info = loop_domain_info {
        domain: 0 as *mut DOMAIN1,
        block: 0 as *mut DOMAIN_BLOCK,
        looping: 0,
        info: 0 as *mut libc::c_void,
        func: None,
    };
    let mut my_info: *mut loop_domain_info = &mut _my_info;
    if domain.is_null() {
        func.unwrap()(mpl, info);
    } else {
        (*my_info).domain = domain;
        (*my_info).block = (*domain).list;
        (*my_info).looping = 1 as libc::c_int;
        (*my_info).info = info;
        (*my_info).func = func;
        loop_domain_func(mpl, my_info as *mut libc::c_void);
    };
}
pub unsafe extern "C" fn _glp_mpl_out_of_domain(
    mut mpl: *mut MPL,
    mut name: *mut libc::c_char,
    mut tuple: *mut TUPLE,
) {
    (!name.is_null()
        || {
            glp_assert_(
                b"name != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                2245 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!tuple.is_null()
        || {
            glp_assert_(
                b"tuple != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                2246 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_mpl_error(
        mpl,
        b"%s%s out of domain\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name,
        _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
    );
}
pub unsafe extern "C" fn _glp_mpl_get_domain_tuple(
    mut mpl: *mut MPL,
    mut domain: *mut DOMAIN1,
) -> *mut TUPLE {
    let mut block: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
    let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    tuple = _glp_mpl_create_tuple(mpl);
    if !domain.is_null() {
        block = (*domain).list;
        while !block.is_null() {
            slot = (*block).list;
            while !slot.is_null() {
                if ((*slot).code).is_null() {
                    (!((*slot).value).is_null()
                        || {
                            glp_assert_(
                                b"slot->value != NULL\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                2275 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    tuple = _glp_mpl_expand_tuple(
                        mpl,
                        tuple,
                        _glp_mpl_copy_symbol(mpl, (*slot).value),
                    );
                }
                slot = (*slot).next;
            }
            block = (*block).next;
        }
    }
    return tuple;
}
pub unsafe extern "C" fn _glp_mpl_clean_domain(
    mut mpl: *mut MPL,
    mut domain: *mut DOMAIN1,
) {
    let mut block: *mut DOMAIN_BLOCK = 0 as *mut DOMAIN_BLOCK;
    let mut slot: *mut DOMAIN_SLOT = 0 as *mut DOMAIN_SLOT;
    if !domain.is_null() {
        block = (*domain).list;
        while !block.is_null() {
            slot = (*block).list;
            while !slot.is_null() {
                _glp_mpl_clean_code(mpl, (*slot).code);
                if !((*slot).value).is_null() {
                    _glp_mpl_delete_symbol(mpl, (*slot).value);
                    (*slot).value = 0 as *mut SYMBOL;
                }
                slot = (*slot).next;
            }
            _glp_mpl_clean_code(mpl, (*block).code);
            block = (*block).next;
        }
        _glp_mpl_clean_code(mpl, (*domain).code);
    }
}
pub unsafe extern "C" fn _glp_mpl_check_elem_set(
    mut mpl: *mut MPL,
    mut set: *mut SET,
    mut tuple: *mut TUPLE,
    mut refer: *mut ELEMSET,
) {
    let mut within: *mut WITHIN = 0 as *mut WITHIN;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    let mut eqno: libc::c_int = 0;
    within = (*set).within;
    eqno = 1 as libc::c_int;
    while !within.is_null() {
        (!((*within).code).is_null()
            || {
                glp_assert_(
                    b"within->code != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2338 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        memb = (*refer).head;
        while !memb.is_null() {
            if _glp_mpl_is_member(mpl, (*within).code, (*memb).tuple) == 0 {
                let mut buf: [libc::c_char; 256] = [0; 256];
                strcpy(
                    buf.as_mut_ptr(),
                    _glp_mpl_format_tuple(mpl, '(' as i32, (*memb).tuple),
                );
                (strlen(buf.as_mut_ptr())
                    < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                    || {
                        glp_assert_(
                            b"strlen(buf) < sizeof(buf)\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            2343 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                _glp_mpl_error(
                    mpl,
                    b"%s%s contains %s which not within specified set; see (%d)\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*set).name,
                    _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                    buf.as_mut_ptr(),
                    eqno,
                );
            }
            memb = (*memb).next;
        }
        within = (*within).next;
        eqno += 1;
        eqno;
    }
}
pub unsafe extern "C" fn _glp_mpl_take_member_set(
    mut mpl: *mut MPL,
    mut set: *mut SET,
    mut tuple: *mut TUPLE,
) -> *mut ELEMSET {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    let mut refer: *mut ELEMSET = 0 as *mut ELEMSET;
    memb = _glp_mpl_find_member(mpl, (*set).array, tuple);
    if !memb.is_null() {
        refer = (*memb).value.set;
    } else {
        let mut current_block_9: u64;
        if !((*set).assign).is_null() {
            refer = _glp_mpl_eval_elemset(mpl, (*set).assign);
            current_block_9 = 11933412257165561125;
        } else if !((*set).option).is_null() {
            refer = _glp_mpl_eval_elemset(mpl, (*set).option);
            current_block_9 = 11933412257165561125;
        } else {
            _glp_mpl_error(
                mpl,
                b"no value for %s%s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*set).name,
                _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
            );
            current_block_9 = 13536709405535804910;
        }
        match current_block_9 {
            11933412257165561125 => {
                _glp_mpl_check_elem_set(mpl, set, tuple, refer);
                memb = _glp_mpl_add_member(
                    mpl,
                    (*set).array,
                    _glp_mpl_copy_tuple(mpl, tuple),
                );
                (*memb).value.set = refer;
            }
            _ => {}
        }
    }
    return refer;
}
unsafe extern "C" fn eval_set_func(mut mpl: *mut MPL, mut _info: *mut libc::c_void) {
    let mut info: *mut eval_set_info = _info as *mut eval_set_info;
    if !((*info).memb).is_null() {
        _glp_mpl_check_elem_set(
            mpl,
            (*info).set,
            (*(*info).memb).tuple,
            (*(*info).memb).value.set,
        );
    } else {
        (*info).refer = _glp_mpl_take_member_set(mpl, (*info).set, (*info).tuple);
    };
}
unsafe extern "C" fn saturate_set(mut mpl: *mut MPL, mut set: *mut SET) {
    let mut gadget: *mut GADGET = (*set).gadget;
    let mut data: *mut ELEMSET = 0 as *mut ELEMSET;
    let mut elem: *mut MEMBER = 0 as *mut MEMBER;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    let mut work: [*mut TUPLE; 20] = [0 as *mut TUPLE; 20];
    let mut i: libc::c_int = 0;
    glp_printf(b"Generating %s...\n\0" as *const u8 as *const libc::c_char, (*set).name);
    _glp_mpl_eval_whole_set(mpl, (*gadget).set);
    (!((*(*gadget).set).array).is_null()
        || {
            glp_assert_(
                b"gadget->set->array != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                2443 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!((*(*(*gadget).set).array).head).is_null()
        || {
            glp_assert_(
                b"gadget->set->array->head != NULL\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                2444 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*(*(*gadget).set).array).head == (*(*(*gadget).set).array).tail
        || {
            glp_assert_(
                b"gadget->set->array->head == gadget->set->array->tail\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                2445 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    data = (*(*(*(*gadget).set).array).head).value.set;
    ((*data).type_0 == 117 as libc::c_int
        || {
            glp_assert_(
                b"data->type == A_NONE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                2447 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*data).dim == (*(*gadget).set).dimen
        || {
            glp_assert_(
                b"data->dim == gadget->set->dimen\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                2448 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    elem = (*data).head;
    while !elem.is_null() {
        tuple = _glp_mpl_copy_tuple(mpl, (*elem).tuple);
        i = 0 as libc::c_int;
        while i < (*(*gadget).set).dimen {
            work[i as usize] = 0 as *mut TUPLE;
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while !tuple.is_null() {
            let fresh10 = i;
            i = i + 1;
            work[((*gadget).ind[fresh10 as usize] - 1 as libc::c_int) as usize] = tuple;
            tuple = (*tuple).next;
        }
        (i == (*(*gadget).set).dimen
            || {
                glp_assert_(
                    b"i == gadget->set->dimen\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2458 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*(*gadget).set).dimen {
            (!(work[i as usize]).is_null()
                || {
                    glp_assert_(
                        b"work[i] != NULL\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        2460 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*work[i as usize]).next = work[(i + 1 as libc::c_int) as usize];
            i += 1;
            i;
        }
        if (*set).dim == 0 as libc::c_int {
            tuple = 0 as *mut TUPLE;
        } else {
            tuple = work[0 as libc::c_int as usize];
            (*work[((*set).dim - 1 as libc::c_int) as usize]).next = 0 as *mut TUPLE;
        }
        memb = _glp_mpl_find_member(mpl, (*set).array, tuple);
        if memb.is_null() {
            memb = _glp_mpl_add_member(mpl, (*set).array, tuple);
            (*memb).value.set = _glp_mpl_create_elemset(mpl, (*set).dimen);
        } else {
            _glp_mpl_delete_tuple(mpl, tuple);
        }
        tuple = work[(*set).dim as usize];
        ((*set).dim + (*set).dimen == (*(*gadget).set).dimen
            || {
                glp_assert_(
                    b"set->dim + set->dimen == gadget->set->dimen\0" as *const u8
                        as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2482 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*work[((*(*gadget).set).dimen - 1 as libc::c_int) as usize])
            .next = 0 as *mut TUPLE;
        _glp_mpl_add_tuple(mpl, (*memb).value.set, tuple);
        elem = (*elem).next;
    }
    (*set).data = 1 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_eval_member_set(
    mut mpl: *mut MPL,
    mut set: *mut SET,
    mut tuple: *mut TUPLE,
) -> *mut ELEMSET {
    let mut _info: eval_set_info = eval_set_info {
        set: 0 as *mut SET,
        tuple: 0 as *mut TUPLE,
        memb: 0 as *mut MEMBER,
        refer: 0 as *mut ELEMSET,
    };
    let mut info: *mut eval_set_info = &mut _info;
    ((*set).dim == _glp_mpl_tuple_dimen(mpl, tuple)
        || {
            glp_assert_(
                b"set->dim == tuple_dimen(mpl, tuple)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                2501 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*info).set = set;
    (*info).tuple = tuple;
    if !((*set).gadget).is_null() && (*set).data == 0 as libc::c_int {
        saturate_set(mpl, set);
    }
    if (*set).data == 1 as libc::c_int {
        let mut tail: *mut MEMBER = (*(*set).array).tail;
        (*set).data = 2 as libc::c_int;
        (*info).memb = (*(*set).array).head;
        while !((*info).memb).is_null() {
            if _glp_mpl_eval_within_domain(
                mpl,
                (*set).domain,
                (*(*info).memb).tuple,
                info as *mut libc::c_void,
                Some(
                    eval_set_func
                        as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> (),
                ),
            ) != 0
            {
                _glp_mpl_out_of_domain(mpl, (*set).name, (*(*info).memb).tuple);
            }
            if (*info).memb == tail {
                break;
            }
            (*info).memb = (*(*info).memb).next;
        }
    }
    (*info).memb = 0 as *mut MEMBER;
    if _glp_mpl_eval_within_domain(
        mpl,
        (*(*info).set).domain,
        (*info).tuple,
        info as *mut libc::c_void,
        Some(eval_set_func as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> ()),
    ) != 0
    {
        _glp_mpl_out_of_domain(mpl, (*set).name, (*info).tuple);
    }
    return (*info).refer;
}
unsafe extern "C" fn whole_set_func(
    mut mpl: *mut MPL,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut set: *mut SET = info as *mut SET;
    let mut tuple: *mut TUPLE = _glp_mpl_get_domain_tuple(mpl, (*set).domain);
    _glp_mpl_eval_member_set(mpl, set, tuple);
    _glp_mpl_delete_tuple(mpl, tuple);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_eval_whole_set(mut mpl: *mut MPL, mut set: *mut SET) {
    _glp_mpl_loop_within_domain(
        mpl,
        (*set).domain,
        set as *mut libc::c_void,
        Some(
            whole_set_func
                as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn _glp_mpl_clean_set(mut mpl: *mut MPL, mut set: *mut SET) {
    let mut within: *mut WITHIN = 0 as *mut WITHIN;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    _glp_mpl_clean_domain(mpl, (*set).domain);
    within = (*set).within;
    while !within.is_null() {
        _glp_mpl_clean_code(mpl, (*within).code);
        within = (*within).next;
    }
    _glp_mpl_clean_code(mpl, (*set).assign);
    _glp_mpl_clean_code(mpl, (*set).option);
    (*set).data = 0 as libc::c_int;
    memb = (*(*set).array).head;
    while !memb.is_null() {
        _glp_mpl_delete_value(mpl, (*(*set).array).type_0, &mut (*memb).value);
        memb = (*memb).next;
    }
    _glp_mpl_delete_array(mpl, (*set).array);
    (*set).array = 0 as *mut ARRAY;
}
pub unsafe extern "C" fn _glp_mpl_check_value_num(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut tuple: *mut TUPLE,
    mut value: libc::c_double,
) {
    let mut current_block: u64;
    let mut cond: *mut CONDITION = 0 as *mut CONDITION;
    let mut in_0: *mut WITHIN = 0 as *mut WITHIN;
    let mut eqno: libc::c_int = 0;
    match (*par).type_0 {
        118 => {}
        113 => {
            if value != floor(value) {
                _glp_mpl_error(
                    mpl,
                    b"%s%s = %.*g not integer\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*par).name,
                    _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                    15 as libc::c_int,
                    value,
                );
            }
        }
        101 => {
            if !(value == 0.0f64 || value == 1.0f64) {
                _glp_mpl_error(
                    mpl,
                    b"%s%s = %.*g not binary\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*par).name,
                    _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                    15 as libc::c_int,
                    value,
                );
            }
        }
        _ => {
            (par != par
                || {
                    glp_assert_(
                        b"par != par\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        2626 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    cond = (*par).cond;
    eqno = 1 as libc::c_int;
    while !cond.is_null() {
        let mut bound: libc::c_double = 0.;
        let mut rho: *mut libc::c_char = 0 as *mut libc::c_char;
        (!((*cond).code).is_null()
            || {
                glp_assert_(
                    b"cond->code != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2633 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        bound = _glp_mpl_eval_numeric(mpl, (*cond).code);
        match (*cond).rho {
            353 => {
                if !(value < bound) {
                    rho = b"<\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    current_block = 7028964386627940608;
                } else {
                    current_block = 3640593987805443782;
                }
            }
            354 => {
                if !(value <= bound) {
                    rho = b"<=\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    current_block = 7028964386627940608;
                } else {
                    current_block = 3640593987805443782;
                }
            }
            355 => {
                if !(value == bound) {
                    rho = b"=\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    current_block = 7028964386627940608;
                } else {
                    current_block = 3640593987805443782;
                }
            }
            356 => {
                if !(value >= bound) {
                    rho = b">=\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    current_block = 7028964386627940608;
                } else {
                    current_block = 3640593987805443782;
                }
            }
            357 => {
                if !(value > bound) {
                    rho = b">\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    current_block = 7028964386627940608;
                } else {
                    current_block = 3640593987805443782;
                }
            }
            358 => {
                if !(value != bound) {
                    rho = b"<>\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    current_block = 7028964386627940608;
                } else {
                    current_block = 3640593987805443782;
                }
            }
            _ => {
                (cond != cond
                    || {
                        glp_assert_(
                            b"cond != cond\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            2660 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                current_block = 3640593987805443782;
            }
        }
        match current_block {
            7028964386627940608 => {
                _glp_mpl_error(
                    mpl,
                    b"%s%s = %.*g not %s %.*g; see (%d)\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (*par).name,
                    _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                    15 as libc::c_int,
                    value,
                    rho,
                    15 as libc::c_int,
                    bound,
                    eqno,
                );
            }
            _ => {}
        }
        cond = (*cond).next;
        eqno += 1;
        eqno;
    }
    in_0 = (*par).in_0;
    eqno = 1 as libc::c_int;
    while !in_0.is_null() {
        let mut dummy: *mut TUPLE = 0 as *mut TUPLE;
        (!((*in_0).code).is_null()
            || {
                glp_assert_(
                    b"in->code != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2666 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ((*(*in_0).code).dim == 1 as libc::c_int
            || {
                glp_assert_(
                    b"in->code->dim == 1\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2667 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        dummy = _glp_mpl_expand_tuple(
            mpl,
            _glp_mpl_create_tuple(mpl),
            _glp_mpl_create_symbol_num(mpl, value),
        );
        if _glp_mpl_is_member(mpl, (*in_0).code, dummy) == 0 {
            _glp_mpl_error(
                mpl,
                b"%s%s = %.*g not in specified set; see (%d)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*par).name,
                _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                15 as libc::c_int,
                value,
                eqno,
            );
        }
        _glp_mpl_delete_tuple(mpl, dummy);
        in_0 = (*in_0).next;
        eqno += 1;
        eqno;
    }
}
pub unsafe extern "C" fn _glp_mpl_take_member_num(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut tuple: *mut TUPLE,
) -> libc::c_double {
    let mut current_block: u64;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    let mut value: libc::c_double = 0.;
    memb = _glp_mpl_find_member(mpl, (*par).array, tuple);
    if !memb.is_null() {
        value = (*memb).value.num;
    } else {
        if !((*par).assign).is_null() {
            value = _glp_mpl_eval_numeric(mpl, (*par).assign);
            current_block = 10031169459552537827;
        } else if !((*par).option).is_null() {
            value = _glp_mpl_eval_numeric(mpl, (*par).option);
            current_block = 10031169459552537827;
        } else if !((*par).defval).is_null() {
            if !((*(*par).defval).str_0).is_null() {
                _glp_mpl_error(
                    mpl,
                    b"cannot convert %s to floating-point number\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    _glp_mpl_format_symbol(mpl, (*par).defval),
                );
            }
            value = (*(*par).defval).num;
            current_block = 10031169459552537827;
        } else {
            _glp_mpl_error(
                mpl,
                b"no value for %s%s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*par).name,
                _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
            );
            current_block = 9606288038608642794;
        }
        match current_block {
            9606288038608642794 => {}
            _ => {
                _glp_mpl_check_value_num(mpl, par, tuple, value);
                memb = _glp_mpl_add_member(
                    mpl,
                    (*par).array,
                    _glp_mpl_copy_tuple(mpl, tuple),
                );
                (*memb).value.num = value;
            }
        }
    }
    return value;
}
unsafe extern "C" fn eval_num_func(mut mpl: *mut MPL, mut _info: *mut libc::c_void) {
    let mut info: *mut eval_num_info = _info as *mut eval_num_info;
    if !((*info).memb).is_null() {
        _glp_mpl_check_value_num(
            mpl,
            (*info).par,
            (*(*info).memb).tuple,
            (*(*info).memb).value.num,
        );
    } else {
        (*info).value = _glp_mpl_take_member_num(mpl, (*info).par, (*info).tuple);
    };
}
pub unsafe extern "C" fn _glp_mpl_eval_member_num(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut tuple: *mut TUPLE,
) -> libc::c_double {
    let mut _info: eval_num_info = eval_num_info {
        par: 0 as *mut PARAMETER,
        tuple: 0 as *mut TUPLE,
        memb: 0 as *mut MEMBER,
        value: 0.,
    };
    let mut info: *mut eval_num_info = &mut _info;
    ((*par).type_0 == 118 as libc::c_int || (*par).type_0 == 113 as libc::c_int
        || (*par).type_0 == 101 as libc::c_int
        || {
            glp_assert_(
                b"par->type == A_NUMERIC || par->type == A_INTEGER || par->type == A_BINARY\0"
                    as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                2775 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*par).dim == _glp_mpl_tuple_dimen(mpl, tuple)
        || {
            glp_assert_(
                b"par->dim == tuple_dimen(mpl, tuple)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                2776 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*info).par = par;
    (*info).tuple = tuple;
    if (*par).data == 1 as libc::c_int {
        let mut tail: *mut MEMBER = (*(*par).array).tail;
        (*par).data = 2 as libc::c_int;
        (*info).memb = (*(*par).array).head;
        while !((*info).memb).is_null() {
            if _glp_mpl_eval_within_domain(
                mpl,
                (*par).domain,
                (*(*info).memb).tuple,
                info as *mut libc::c_void,
                Some(
                    eval_num_func
                        as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> (),
                ),
            ) != 0
            {
                _glp_mpl_out_of_domain(mpl, (*par).name, (*(*info).memb).tuple);
            }
            if (*info).memb == tail {
                break;
            }
            (*info).memb = (*(*info).memb).next;
        }
    }
    (*info).memb = 0 as *mut MEMBER;
    if _glp_mpl_eval_within_domain(
        mpl,
        (*(*info).par).domain,
        (*info).tuple,
        info as *mut libc::c_void,
        Some(eval_num_func as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> ()),
    ) != 0
    {
        _glp_mpl_out_of_domain(mpl, (*par).name, (*info).tuple);
    }
    return (*info).value;
}
pub unsafe extern "C" fn _glp_mpl_check_value_sym(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut tuple: *mut TUPLE,
    mut value: *mut SYMBOL,
) {
    let mut cond: *mut CONDITION = 0 as *mut CONDITION;
    let mut in_0: *mut WITHIN = 0 as *mut WITHIN;
    let mut eqno: libc::c_int = 0;
    cond = (*par).cond;
    eqno = 1 as libc::c_int;
    while !cond.is_null() {
        let mut bound: *mut SYMBOL = 0 as *mut SYMBOL;
        let mut buf: [libc::c_char; 256] = [0; 256];
        (!((*cond).code).is_null()
            || {
                glp_assert_(
                    b"cond->code != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2834 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        bound = _glp_mpl_eval_symbolic(mpl, (*cond).code);
        match (*cond).rho {
            353 => {
                if !(_glp_mpl_compare_symbols(mpl, value, bound) < 0 as libc::c_int) {
                    strcpy(buf.as_mut_ptr(), _glp_mpl_format_symbol(mpl, bound));
                    (strlen(buf.as_mut_ptr())
                        < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        || {
                            glp_assert_(
                                b"strlen(buf) < sizeof(buf)\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                2842 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    _glp_mpl_error(
                        mpl,
                        b"%s%s = %s not < %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*par).name,
                        _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                        _glp_mpl_format_symbol(mpl, value),
                        buf.as_mut_ptr(),
                        eqno,
                    );
                }
            }
            354 => {
                if !(_glp_mpl_compare_symbols(mpl, value, bound) <= 0 as libc::c_int) {
                    strcpy(buf.as_mut_ptr(), _glp_mpl_format_symbol(mpl, bound));
                    (strlen(buf.as_mut_ptr())
                        < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        || {
                            glp_assert_(
                                b"strlen(buf) < sizeof(buf)\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                2851 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    _glp_mpl_error(
                        mpl,
                        b"%s%s = %s not <= %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*par).name,
                        _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                        _glp_mpl_format_symbol(mpl, value),
                        buf.as_mut_ptr(),
                        eqno,
                    );
                }
            }
            355 => {
                if !(_glp_mpl_compare_symbols(mpl, value, bound) == 0 as libc::c_int) {
                    strcpy(buf.as_mut_ptr(), _glp_mpl_format_symbol(mpl, bound));
                    (strlen(buf.as_mut_ptr())
                        < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        || {
                            glp_assert_(
                                b"strlen(buf) < sizeof(buf)\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                2861 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    _glp_mpl_error(
                        mpl,
                        b"%s%s = %s not = %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*par).name,
                        _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                        _glp_mpl_format_symbol(mpl, value),
                        buf.as_mut_ptr(),
                        eqno,
                    );
                }
            }
            356 => {
                if !(_glp_mpl_compare_symbols(mpl, value, bound) >= 0 as libc::c_int) {
                    strcpy(buf.as_mut_ptr(), _glp_mpl_format_symbol(mpl, bound));
                    (strlen(buf.as_mut_ptr())
                        < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        || {
                            glp_assert_(
                                b"strlen(buf) < sizeof(buf)\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                2871 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    _glp_mpl_error(
                        mpl,
                        b"%s%s = %s not >= %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*par).name,
                        _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                        _glp_mpl_format_symbol(mpl, value),
                        buf.as_mut_ptr(),
                        eqno,
                    );
                }
            }
            357 => {
                if !(_glp_mpl_compare_symbols(mpl, value, bound) > 0 as libc::c_int) {
                    strcpy(buf.as_mut_ptr(), _glp_mpl_format_symbol(mpl, bound));
                    (strlen(buf.as_mut_ptr())
                        < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        || {
                            glp_assert_(
                                b"strlen(buf) < sizeof(buf)\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                2880 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    _glp_mpl_error(
                        mpl,
                        b"%s%s = %s not > %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*par).name,
                        _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                        _glp_mpl_format_symbol(mpl, value),
                        buf.as_mut_ptr(),
                        eqno,
                    );
                }
            }
            358 => {
                if !(_glp_mpl_compare_symbols(mpl, value, bound) != 0 as libc::c_int) {
                    strcpy(buf.as_mut_ptr(), _glp_mpl_format_symbol(mpl, bound));
                    (strlen(buf.as_mut_ptr())
                        < ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                        || {
                            glp_assert_(
                                b"strlen(buf) < sizeof(buf)\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                2890 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    _glp_mpl_error(
                        mpl,
                        b"%s%s = %s not <> %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*par).name,
                        _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                        _glp_mpl_format_symbol(mpl, value),
                        buf.as_mut_ptr(),
                        eqno,
                    );
                }
            }
            _ => {
                (cond != cond
                    || {
                        glp_assert_(
                            b"cond != cond\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            2897 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        _glp_mpl_delete_symbol(mpl, bound);
        cond = (*cond).next;
        eqno += 1;
        eqno;
    }
    in_0 = (*par).in_0;
    eqno = 1 as libc::c_int;
    while !in_0.is_null() {
        let mut dummy: *mut TUPLE = 0 as *mut TUPLE;
        (!((*in_0).code).is_null()
            || {
                glp_assert_(
                    b"in->code != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2904 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        ((*(*in_0).code).dim == 1 as libc::c_int
            || {
                glp_assert_(
                    b"in->code->dim == 1\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    2905 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        dummy = _glp_mpl_expand_tuple(
            mpl,
            _glp_mpl_create_tuple(mpl),
            _glp_mpl_copy_symbol(mpl, value),
        );
        if _glp_mpl_is_member(mpl, (*in_0).code, dummy) == 0 {
            _glp_mpl_error(
                mpl,
                b"%s%s = %s not in specified set; see (%d)\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*par).name,
                _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
                _glp_mpl_format_symbol(mpl, value),
                eqno,
            );
        }
        _glp_mpl_delete_tuple(mpl, dummy);
        in_0 = (*in_0).next;
        eqno += 1;
        eqno;
    }
}
pub unsafe extern "C" fn _glp_mpl_take_member_sym(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut tuple: *mut TUPLE,
) -> *mut SYMBOL {
    let mut current_block: u64;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    let mut value: *mut SYMBOL = 0 as *mut SYMBOL;
    memb = _glp_mpl_find_member(mpl, (*par).array, tuple);
    if !memb.is_null() {
        value = _glp_mpl_copy_symbol(mpl, (*memb).value.sym);
    } else {
        if !((*par).assign).is_null() {
            value = _glp_mpl_eval_symbolic(mpl, (*par).assign);
            current_block = 8085621643516336470;
        } else if !((*par).option).is_null() {
            value = _glp_mpl_eval_symbolic(mpl, (*par).option);
            current_block = 8085621643516336470;
        } else if !((*par).defval).is_null() {
            value = _glp_mpl_copy_symbol(mpl, (*par).defval);
            current_block = 8085621643516336470;
        } else {
            _glp_mpl_error(
                mpl,
                b"no value for %s%s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*par).name,
                _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
            );
            current_block = 4166486009154926805;
        }
        match current_block {
            4166486009154926805 => {}
            _ => {
                _glp_mpl_check_value_sym(mpl, par, tuple, value);
                memb = _glp_mpl_add_member(
                    mpl,
                    (*par).array,
                    _glp_mpl_copy_tuple(mpl, tuple),
                );
                (*memb).value.sym = _glp_mpl_copy_symbol(mpl, value);
            }
        }
    }
    return value;
}
unsafe extern "C" fn eval_sym_func(mut mpl: *mut MPL, mut _info: *mut libc::c_void) {
    let mut info: *mut eval_sym_info = _info as *mut eval_sym_info;
    if !((*info).memb).is_null() {
        _glp_mpl_check_value_sym(
            mpl,
            (*info).par,
            (*(*info).memb).tuple,
            (*(*info).memb).value.sym,
        );
    } else {
        (*info).value = _glp_mpl_take_member_sym(mpl, (*info).par, (*info).tuple);
    };
}
pub unsafe extern "C" fn _glp_mpl_eval_member_sym(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut tuple: *mut TUPLE,
) -> *mut SYMBOL {
    let mut _info: eval_sym_info = eval_sym_info {
        par: 0 as *mut PARAMETER,
        tuple: 0 as *mut TUPLE,
        memb: 0 as *mut MEMBER,
        value: 0 as *mut SYMBOL,
    };
    let mut info: *mut eval_sym_info = &mut _info;
    ((*par).type_0 == 124 as libc::c_int
        || {
            glp_assert_(
                b"par->type == A_SYMBOLIC\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                3009 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*par).dim == _glp_mpl_tuple_dimen(mpl, tuple)
        || {
            glp_assert_(
                b"par->dim == tuple_dimen(mpl, tuple)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                3010 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*info).par = par;
    (*info).tuple = tuple;
    if (*par).data == 1 as libc::c_int {
        let mut tail: *mut MEMBER = (*(*par).array).tail;
        (*par).data = 2 as libc::c_int;
        (*info).memb = (*(*par).array).head;
        while !((*info).memb).is_null() {
            if _glp_mpl_eval_within_domain(
                mpl,
                (*par).domain,
                (*(*info).memb).tuple,
                info as *mut libc::c_void,
                Some(
                    eval_sym_func
                        as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> (),
                ),
            ) != 0
            {
                _glp_mpl_out_of_domain(mpl, (*par).name, (*(*info).memb).tuple);
            }
            if (*info).memb == tail {
                break;
            }
            (*info).memb = (*(*info).memb).next;
        }
    }
    (*info).memb = 0 as *mut MEMBER;
    if _glp_mpl_eval_within_domain(
        mpl,
        (*(*info).par).domain,
        (*info).tuple,
        info as *mut libc::c_void,
        Some(eval_sym_func as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> ()),
    ) != 0
    {
        _glp_mpl_out_of_domain(mpl, (*par).name, (*info).tuple);
    }
    return (*info).value;
}
unsafe extern "C" fn whole_par_func(
    mut mpl: *mut MPL,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut par: *mut PARAMETER = info as *mut PARAMETER;
    let mut tuple: *mut TUPLE = _glp_mpl_get_domain_tuple(mpl, (*par).domain);
    match (*par).type_0 {
        118 | 113 | 101 => {
            _glp_mpl_eval_member_num(mpl, par, tuple);
        }
        124 => {
            _glp_mpl_delete_symbol(mpl, _glp_mpl_eval_member_sym(mpl, par, tuple));
        }
        _ => {
            (par != par
                || {
                    glp_assert_(
                        b"par != par\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        3066 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    _glp_mpl_delete_tuple(mpl, tuple);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_eval_whole_par(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
) {
    _glp_mpl_loop_within_domain(
        mpl,
        (*par).domain,
        par as *mut libc::c_void,
        Some(
            whole_par_func
                as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn _glp_mpl_clean_parameter(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
) {
    let mut cond: *mut CONDITION = 0 as *mut CONDITION;
    let mut in_0: *mut WITHIN = 0 as *mut WITHIN;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    _glp_mpl_clean_domain(mpl, (*par).domain);
    cond = (*par).cond;
    while !cond.is_null() {
        _glp_mpl_clean_code(mpl, (*cond).code);
        cond = (*cond).next;
    }
    in_0 = (*par).in_0;
    while !in_0.is_null() {
        _glp_mpl_clean_code(mpl, (*in_0).code);
        in_0 = (*in_0).next;
    }
    _glp_mpl_clean_code(mpl, (*par).assign);
    _glp_mpl_clean_code(mpl, (*par).option);
    (*par).data = 0 as libc::c_int;
    if !((*par).defval).is_null() {
        _glp_mpl_delete_symbol(mpl, (*par).defval);
        (*par).defval = 0 as *mut SYMBOL;
    }
    memb = (*(*par).array).head;
    while !memb.is_null() {
        _glp_mpl_delete_value(mpl, (*(*par).array).type_0, &mut (*memb).value);
        memb = (*memb).next;
    }
    _glp_mpl_delete_array(mpl, (*par).array);
    (*par).array = 0 as *mut ARRAY;
}
pub unsafe extern "C" fn _glp_mpl_take_member_var(
    mut mpl: *mut MPL,
    mut var: *mut VARIABLE,
    mut tuple: *mut TUPLE,
) -> *mut ELEMVAR {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    let mut refer: *mut ELEMVAR = 0 as *mut ELEMVAR;
    memb = _glp_mpl_find_member(mpl, (*var).array, tuple);
    if !memb.is_null() {
        refer = (*memb).value.var;
    } else {
        memb = _glp_mpl_add_member(mpl, (*var).array, _glp_mpl_copy_tuple(mpl, tuple));
        (*memb)
            .value
            .var = _glp_dmp_get_atom(
            (*mpl).elemvars,
            ::std::mem::size_of::<ELEMVAR>() as libc::c_ulong as libc::c_int,
        ) as *mut ELEMVAR;
        refer = (*memb).value.var;
        (*refer).j = 0 as libc::c_int;
        (*refer).var = var;
        (*refer).memb = memb;
        if ((*var).lbnd).is_null() {
            (*refer).lbnd = 0.0f64;
        } else {
            (*refer).lbnd = _glp_mpl_eval_numeric(mpl, (*var).lbnd);
        }
        if ((*var).ubnd).is_null() {
            (*refer).ubnd = 0.0f64;
        } else if (*var).ubnd == (*var).lbnd {
            (*refer).ubnd = (*refer).lbnd;
        } else {
            (*refer).ubnd = _glp_mpl_eval_numeric(mpl, (*var).ubnd);
        }
        (*refer).temp = 0.0f64;
        (*refer).stat = 0 as libc::c_int;
        (*refer).dual = 0.0f64;
        (*refer).prim = (*refer).dual;
    }
    return refer;
}
unsafe extern "C" fn eval_var_func(mut mpl: *mut MPL, mut _info: *mut libc::c_void) {
    let mut info: *mut eval_var_info = _info as *mut eval_var_info;
    (*info).refer = _glp_mpl_take_member_var(mpl, (*info).var, (*info).tuple);
}
pub unsafe extern "C" fn _glp_mpl_eval_member_var(
    mut mpl: *mut MPL,
    mut var: *mut VARIABLE,
    mut tuple: *mut TUPLE,
) -> *mut ELEMVAR {
    let mut _info: eval_var_info = eval_var_info {
        var: 0 as *mut VARIABLE,
        tuple: 0 as *mut TUPLE,
        refer: 0 as *mut ELEMVAR,
    };
    let mut info: *mut eval_var_info = &mut _info;
    ((*var).dim == _glp_mpl_tuple_dimen(mpl, tuple)
        || {
            glp_assert_(
                b"var->dim == tuple_dimen(mpl, tuple)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                3200 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*info).var = var;
    (*info).tuple = tuple;
    if _glp_mpl_eval_within_domain(
        mpl,
        (*(*info).var).domain,
        (*info).tuple,
        info as *mut libc::c_void,
        Some(eval_var_func as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> ()),
    ) != 0
    {
        _glp_mpl_out_of_domain(mpl, (*var).name, (*info).tuple);
    }
    return (*info).refer;
}
unsafe extern "C" fn whole_var_func(
    mut mpl: *mut MPL,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut var: *mut VARIABLE = info as *mut VARIABLE;
    let mut tuple: *mut TUPLE = _glp_mpl_get_domain_tuple(mpl, (*var).domain);
    _glp_mpl_eval_member_var(mpl, var, tuple);
    _glp_mpl_delete_tuple(mpl, tuple);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_eval_whole_var(
    mut mpl: *mut MPL,
    mut var: *mut VARIABLE,
) {
    _glp_mpl_loop_within_domain(
        mpl,
        (*var).domain,
        var as *mut libc::c_void,
        Some(
            whole_var_func
                as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn _glp_mpl_clean_variable(
    mut mpl: *mut MPL,
    mut var: *mut VARIABLE,
) {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    _glp_mpl_clean_domain(mpl, (*var).domain);
    _glp_mpl_clean_code(mpl, (*var).lbnd);
    if (*var).ubnd != (*var).lbnd {
        _glp_mpl_clean_code(mpl, (*var).ubnd);
    }
    memb = (*(*var).array).head;
    while !memb.is_null() {
        _glp_dmp_free_atom(
            (*mpl).elemvars,
            (*memb).value.var as *mut libc::c_void,
            ::std::mem::size_of::<ELEMVAR>() as libc::c_ulong as libc::c_int,
        );
        memb = (*memb).next;
    }
    _glp_mpl_delete_array(mpl, (*var).array);
    (*var).array = 0 as *mut ARRAY;
}
pub unsafe extern "C" fn _glp_mpl_take_member_con(
    mut mpl: *mut MPL,
    mut con: *mut CONSTRAINT,
    mut tuple: *mut TUPLE,
) -> *mut ELEMCON {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    let mut refer: *mut ELEMCON = 0 as *mut ELEMCON;
    memb = _glp_mpl_find_member(mpl, (*con).array, tuple);
    if !memb.is_null() {
        refer = (*memb).value.con;
    } else {
        memb = _glp_mpl_add_member(mpl, (*con).array, _glp_mpl_copy_tuple(mpl, tuple));
        (*memb)
            .value
            .con = _glp_dmp_get_atom(
            (*mpl).elemcons,
            ::std::mem::size_of::<ELEMCON>() as libc::c_ulong as libc::c_int,
        ) as *mut ELEMCON;
        refer = (*memb).value.con;
        (*refer).i = 0 as libc::c_int;
        (*refer).con = con;
        (*refer).memb = memb;
        (!((*con).code).is_null()
            || {
                glp_assert_(
                    b"con->code != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    3289 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*refer).form = _glp_mpl_eval_formula(mpl, (*con).code);
        if ((*con).lbnd).is_null() && ((*con).ubnd).is_null() {
            let mut temp: libc::c_double = 0.;
            ((*con).type_0 == 116 as libc::c_int || (*con).type_0 == 115 as libc::c_int
                || {
                    glp_assert_(
                        b"con->type == A_MINIMIZE || con->type == A_MAXIMIZE\0"
                            as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        3295 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*refer).form = _glp_mpl_remove_constant(mpl, (*refer).form, &mut temp);
            (*refer).ubnd = -temp;
            (*refer).lbnd = (*refer).ubnd;
        } else if !((*con).lbnd).is_null() && ((*con).ubnd).is_null() {
            let mut temp_0: libc::c_double = 0.;
            ((*con).type_0 == 103 as libc::c_int
                || {
                    glp_assert_(
                        b"con->type == A_CONSTRAINT\0" as *const u8
                            as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        3304 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*refer)
                .form = _glp_mpl_linear_comb(
                mpl,
                1.0f64,
                (*refer).form,
                -1.0f64,
                _glp_mpl_eval_formula(mpl, (*con).lbnd),
            );
            (*refer).form = _glp_mpl_remove_constant(mpl, (*refer).form, &mut temp_0);
            (*refer).lbnd = -temp_0;
            (*refer).ubnd = 0.0f64;
        } else if ((*con).lbnd).is_null() && !((*con).ubnd).is_null() {
            let mut temp_1: libc::c_double = 0.;
            ((*con).type_0 == 103 as libc::c_int
                || {
                    glp_assert_(
                        b"con->type == A_CONSTRAINT\0" as *const u8
                            as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        3316 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*refer)
                .form = _glp_mpl_linear_comb(
                mpl,
                1.0f64,
                (*refer).form,
                -1.0f64,
                _glp_mpl_eval_formula(mpl, (*con).ubnd),
            );
            (*refer).form = _glp_mpl_remove_constant(mpl, (*refer).form, &mut temp_1);
            (*refer).lbnd = 0.0f64;
            (*refer).ubnd = -temp_1;
        } else if (*con).lbnd == (*con).ubnd {
            let mut temp_2: libc::c_double = 0.;
            ((*con).type_0 == 103 as libc::c_int
                || {
                    glp_assert_(
                        b"con->type == A_CONSTRAINT\0" as *const u8
                            as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        3328 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*refer)
                .form = _glp_mpl_linear_comb(
                mpl,
                1.0f64,
                (*refer).form,
                -1.0f64,
                _glp_mpl_eval_formula(mpl, (*con).lbnd),
            );
            (*refer).form = _glp_mpl_remove_constant(mpl, (*refer).form, &mut temp_2);
            (*refer).ubnd = -temp_2;
            (*refer).lbnd = (*refer).ubnd;
        } else {
            let mut temp_3: libc::c_double = 0.;
            let mut temp1: libc::c_double = 0.;
            let mut temp2: libc::c_double = 0.;
            ((*con).type_0 == 103 as libc::c_int
                || {
                    glp_assert_(
                        b"con->type == A_CONSTRAINT\0" as *const u8
                            as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        3339 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*refer).form = _glp_mpl_remove_constant(mpl, (*refer).form, &mut temp_3);
            ((_glp_mpl_remove_constant(
                mpl,
                _glp_mpl_eval_formula(mpl, (*con).lbnd),
                &mut temp1,
            ))
                .is_null()
                || {
                    glp_assert_(
                        b"remove_constant(mpl, eval_formula(mpl, con->lbnd), &temp1) == NULL\0"
                            as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        3342 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            ((_glp_mpl_remove_constant(
                mpl,
                _glp_mpl_eval_formula(mpl, (*con).ubnd),
                &mut temp2,
            ))
                .is_null()
                || {
                    glp_assert_(
                        b"remove_constant(mpl, eval_formula(mpl, con->ubnd), &temp2) == NULL\0"
                            as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        3344 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            (*refer).lbnd = _glp_mpl_fp_sub(mpl, temp1, temp_3);
            (*refer).ubnd = _glp_mpl_fp_sub(mpl, temp2, temp_3);
        }
        (*refer).stat = 0 as libc::c_int;
        (*refer).dual = 0.0f64;
        (*refer).prim = (*refer).dual;
    }
    return refer;
}
unsafe extern "C" fn eval_con_func(mut mpl: *mut MPL, mut _info: *mut libc::c_void) {
    let mut info: *mut eval_con_info = _info as *mut eval_con_info;
    (*info).refer = _glp_mpl_take_member_con(mpl, (*info).con, (*info).tuple);
}
pub unsafe extern "C" fn _glp_mpl_eval_member_con(
    mut mpl: *mut MPL,
    mut con: *mut CONSTRAINT,
    mut tuple: *mut TUPLE,
) -> *mut ELEMCON {
    let mut _info: eval_con_info = eval_con_info {
        con: 0 as *mut CONSTRAINT,
        tuple: 0 as *mut TUPLE,
        refer: 0 as *mut ELEMCON,
    };
    let mut info: *mut eval_con_info = &mut _info;
    ((*con).dim == _glp_mpl_tuple_dimen(mpl, tuple)
        || {
            glp_assert_(
                b"con->dim == tuple_dimen(mpl, tuple)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                3387 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*info).con = con;
    (*info).tuple = tuple;
    if _glp_mpl_eval_within_domain(
        mpl,
        (*(*info).con).domain,
        (*info).tuple,
        info as *mut libc::c_void,
        Some(eval_con_func as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> ()),
    ) != 0
    {
        _glp_mpl_out_of_domain(mpl, (*con).name, (*info).tuple);
    }
    return (*info).refer;
}
unsafe extern "C" fn whole_con_func(
    mut mpl: *mut MPL,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut con: *mut CONSTRAINT = info as *mut CONSTRAINT;
    let mut tuple: *mut TUPLE = _glp_mpl_get_domain_tuple(mpl, (*con).domain);
    _glp_mpl_eval_member_con(mpl, con, tuple);
    _glp_mpl_delete_tuple(mpl, tuple);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_eval_whole_con(
    mut mpl: *mut MPL,
    mut con: *mut CONSTRAINT,
) {
    _glp_mpl_loop_within_domain(
        mpl,
        (*con).domain,
        con as *mut libc::c_void,
        Some(
            whole_con_func
                as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn _glp_mpl_clean_constraint(
    mut mpl: *mut MPL,
    mut con: *mut CONSTRAINT,
) {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    _glp_mpl_clean_domain(mpl, (*con).domain);
    _glp_mpl_clean_code(mpl, (*con).code);
    _glp_mpl_clean_code(mpl, (*con).lbnd);
    if (*con).ubnd != (*con).lbnd {
        _glp_mpl_clean_code(mpl, (*con).ubnd);
    }
    memb = (*(*con).array).head;
    while !memb.is_null() {
        _glp_mpl_delete_formula(mpl, (*(*memb).value.con).form);
        _glp_dmp_free_atom(
            (*mpl).elemcons,
            (*memb).value.con as *mut libc::c_void,
            ::std::mem::size_of::<ELEMCON>() as libc::c_ulong as libc::c_int,
        );
        memb = (*memb).next;
    }
    _glp_mpl_delete_array(mpl, (*con).array);
    (*con).array = 0 as *mut ARRAY;
}
unsafe extern "C" fn iter_num_func(
    mut mpl: *mut MPL,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut iter_num_info = _info as *mut iter_num_info;
    let mut temp: libc::c_double = 0.;
    temp = _glp_mpl_eval_numeric(mpl, (*(*info).code).arg.loop_0.x);
    match (*(*info).code).op {
        378 => {
            (*info).value = _glp_mpl_fp_add(mpl, (*info).value, temp);
        }
        379 => {
            (*info).value = _glp_mpl_fp_mul(mpl, (*info).value, temp);
        }
        380 => {
            if (*info).value > temp {
                (*info).value = temp;
            }
        }
        381 => {
            if (*info).value < temp {
                (*info).value = temp;
            }
        }
        _ => {
            (info != info
                || {
                    glp_assert_(
                        b"info != info\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        3485 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_eval_numeric(
    mut mpl: *mut MPL,
    mut code: *mut CODE,
) -> libc::c_double {
    let mut value: libc::c_double = 0.;
    (!code.is_null()
        || {
            glp_assert_(
                b"code != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                3492 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).type_0 == 118 as libc::c_int
        || {
            glp_assert_(
                b"code->type == A_NUMERIC\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                3493 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).dim == 0 as libc::c_int
        || {
            glp_assert_(
                b"code->dim == 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                3494 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*code).vflag != 0 && (*code).valid != 0 {
        (*code).valid = 0 as libc::c_int;
        _glp_mpl_delete_value(mpl, (*code).type_0, &mut (*code).value);
    }
    if (*code).valid != 0 {
        value = (*code).value.num;
    } else {
        match (*code).op {
            301 => {
                value = (*code).arg.num;
            }
            304 => {
                let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
                let mut e: *mut ARG_LIST = 0 as *mut ARG_LIST;
                tuple = _glp_mpl_create_tuple(mpl);
                e = (*code).arg.par.list;
                while !e.is_null() {
                    tuple = _glp_mpl_expand_tuple(
                        mpl,
                        tuple,
                        _glp_mpl_eval_symbolic(mpl, (*e).x),
                    );
                    e = (*e).next;
                }
                value = _glp_mpl_eval_member_num(mpl, (*code).arg.par.par, tuple);
                _glp_mpl_delete_tuple(mpl, tuple);
            }
            307 => {
                let mut tuple_0: *mut TUPLE = 0 as *mut TUPLE;
                let mut e_0: *mut ARG_LIST = 0 as *mut ARG_LIST;
                let mut var: *mut ELEMVAR = 0 as *mut ELEMVAR;
                tuple_0 = _glp_mpl_create_tuple(mpl);
                e_0 = (*code).arg.var.list;
                while !e_0.is_null() {
                    tuple_0 = _glp_mpl_expand_tuple(
                        mpl,
                        tuple_0,
                        _glp_mpl_eval_symbolic(mpl, (*e_0).x),
                    );
                    e_0 = (*e_0).next;
                }
                var = _glp_mpl_eval_member_var(mpl, (*code).arg.var.var, tuple_0);
                match (*code).arg.var.suff {
                    1 => {
                        if ((*(*var).var).lbnd).is_null() {
                            value = -1.7976931348623157e+308f64;
                        } else {
                            value = (*var).lbnd;
                        }
                    }
                    2 => {
                        if ((*(*var).var).ubnd).is_null() {
                            value = 1.7976931348623157e+308f64;
                        } else {
                            value = (*var).ubnd;
                        }
                    }
                    3 => {
                        value = (*var).stat as libc::c_double;
                    }
                    4 => {
                        value = (*var).prim;
                    }
                    5 => {
                        value = (*var).dual;
                    }
                    _ => {
                        (code != code
                            || {
                                glp_assert_(
                                    b"code != code\0" as *const u8 as *const libc::c_char,
                                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                    3563 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
                _glp_mpl_delete_tuple(mpl, tuple_0);
            }
            308 => {
                let mut tuple_1: *mut TUPLE = 0 as *mut TUPLE;
                let mut e_1: *mut ARG_LIST = 0 as *mut ARG_LIST;
                let mut con: *mut ELEMCON = 0 as *mut ELEMCON;
                tuple_1 = _glp_mpl_create_tuple(mpl);
                e_1 = (*code).arg.con.list;
                while !e_1.is_null() {
                    tuple_1 = _glp_mpl_expand_tuple(
                        mpl,
                        tuple_1,
                        _glp_mpl_eval_symbolic(mpl, (*e_1).x),
                    );
                    e_1 = (*e_1).next;
                }
                con = _glp_mpl_eval_member_con(mpl, (*code).arg.con.con, tuple_1);
                match (*code).arg.con.suff {
                    1 => {
                        if ((*(*con).con).lbnd).is_null() {
                            value = -1.7976931348623157e+308f64;
                        } else {
                            value = (*con).lbnd;
                        }
                    }
                    2 => {
                        if ((*(*con).con).ubnd).is_null() {
                            value = 1.7976931348623157e+308f64;
                        } else {
                            value = (*con).ubnd;
                        }
                    }
                    3 => {
                        value = (*con).stat as libc::c_double;
                    }
                    4 => {
                        value = (*con).prim;
                    }
                    5 => {
                        value = (*con).dual;
                    }
                    _ => {
                        (code != code
                            || {
                                glp_assert_(
                                    b"code != code\0" as *const u8 as *const libc::c_char,
                                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                    3603 as libc::c_int,
                                );
                                1 as libc::c_int != 0
                            }) as libc::c_int;
                    }
                }
                _glp_mpl_delete_tuple(mpl, tuple_1);
            }
            312 => {
                value = _glp_mpl_fp_irand224(mpl);
            }
            313 => {
                value = _glp_mpl_fp_uniform01(mpl);
            }
            314 => {
                value = _glp_mpl_fp_normal01(mpl);
            }
            315 => {
                value = _glp_mpl_fn_gmtime(mpl);
            }
            316 => {
                let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
                sym = _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.x);
                if ((*sym).str_0).is_null() {
                    value = (*sym).num;
                } else if _glp_str2num((*sym).str_0, &mut value) != 0 {
                    _glp_mpl_error(
                        mpl,
                        b"cannot convert %s to floating-point number\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        _glp_mpl_format_symbol(mpl, sym),
                    );
                }
                _glp_mpl_delete_symbol(mpl, sym);
            }
            321 => {
                value = _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x);
            }
            322 => {
                value = -_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x);
            }
            324 => {
                value = fabs(_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x));
            }
            325 => {
                value = ceil(_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x));
            }
            326 => {
                value = floor(_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x));
            }
            327 => {
                value = _glp_mpl_fp_exp(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                );
            }
            328 => {
                value = _glp_mpl_fp_log(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                );
            }
            329 => {
                value = _glp_mpl_fp_log10(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                );
            }
            330 => {
                value = _glp_mpl_fp_sqrt(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                );
            }
            331 => {
                value = _glp_mpl_fp_sin(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                );
            }
            332 => {
                value = _glp_mpl_fp_cos(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                );
            }
            333 => {
                value = _glp_mpl_fp_tan(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                );
            }
            334 => {
                value = _glp_mpl_fp_atan(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                );
            }
            347 => {
                value = _glp_mpl_fp_atan2(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            335 => {
                value = _glp_mpl_fp_round(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    0.0f64,
                );
            }
            348 => {
                value = _glp_mpl_fp_round(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            336 => {
                value = _glp_mpl_fp_trunc(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    0.0f64,
                );
            }
            349 => {
                value = _glp_mpl_fp_trunc(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            339 => {
                value = _glp_mpl_fp_add(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            340 => {
                value = _glp_mpl_fp_sub(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            341 => {
                value = _glp_mpl_fp_less(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            342 => {
                value = _glp_mpl_fp_mul(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            343 => {
                value = _glp_mpl_fp_div(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            344 => {
                value = _glp_mpl_fp_idiv(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            345 => {
                value = _glp_mpl_fp_mod(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            346 => {
                value = _glp_mpl_fp_power(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            350 => {
                value = _glp_mpl_uniform(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            351 => {
                value = _glp_mpl_fp_normal(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                );
            }
            337 => {
                let mut set: *mut ELEMSET = 0 as *mut ELEMSET;
                set = _glp_mpl_eval_elemset(mpl, (*code).arg.arg.x);
                value = (*set).size as libc::c_double;
                _glp_mpl_delete_array(mpl, set);
            }
            338 => {
                let mut sym_0: *mut SYMBOL = 0 as *mut SYMBOL;
                let mut str: [libc::c_char; 101] = [0; 101];
                sym_0 = _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.x);
                if ((*sym_0).str_0).is_null() {
                    sprintf(
                        str.as_mut_ptr(),
                        b"%.*g\0" as *const u8 as *const libc::c_char,
                        15 as libc::c_int,
                        (*sym_0).num,
                    );
                } else {
                    _glp_mpl_fetch_string(mpl, (*sym_0).str_0, str.as_mut_ptr());
                }
                _glp_mpl_delete_symbol(mpl, sym_0);
                value = strlen(str.as_mut_ptr()) as libc::c_double;
            }
            371 => {
                let mut sym_1: *mut SYMBOL = 0 as *mut SYMBOL;
                let mut str_0: [libc::c_char; 101] = [0; 101];
                let mut fmt: [libc::c_char; 101] = [0; 101];
                sym_1 = _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.x);
                if ((*sym_1).str_0).is_null() {
                    sprintf(
                        str_0.as_mut_ptr(),
                        b"%.*g\0" as *const u8 as *const libc::c_char,
                        15 as libc::c_int,
                        (*sym_1).num,
                    );
                } else {
                    _glp_mpl_fetch_string(mpl, (*sym_1).str_0, str_0.as_mut_ptr());
                }
                _glp_mpl_delete_symbol(mpl, sym_1);
                sym_1 = _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.y);
                if ((*sym_1).str_0).is_null() {
                    sprintf(
                        fmt.as_mut_ptr(),
                        b"%.*g\0" as *const u8 as *const libc::c_char,
                        15 as libc::c_int,
                        (*sym_1).num,
                    );
                } else {
                    _glp_mpl_fetch_string(mpl, (*sym_1).str_0, fmt.as_mut_ptr());
                }
                _glp_mpl_delete_symbol(mpl, sym_1);
                value = _glp_mpl_fn_str2time(mpl, str_0.as_mut_ptr(), fmt.as_mut_ptr());
            }
            374 => {
                if _glp_mpl_eval_logical(mpl, (*code).arg.arg.x) != 0 {
                    value = _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y);
                } else if ((*code).arg.arg.z).is_null() {
                    value = 0.0f64;
                } else {
                    value = _glp_mpl_eval_numeric(mpl, (*code).arg.arg.z);
                }
            }
            376 => {
                let mut e_2: *mut ARG_LIST = 0 as *mut ARG_LIST;
                let mut temp: libc::c_double = 0.;
                value = 1.7976931348623157e+308f64;
                e_2 = (*code).arg.list;
                while !e_2.is_null() {
                    temp = _glp_mpl_eval_numeric(mpl, (*e_2).x);
                    if value > temp {
                        value = temp;
                    }
                    e_2 = (*e_2).next;
                }
            }
            377 => {
                let mut e_3: *mut ARG_LIST = 0 as *mut ARG_LIST;
                let mut temp_0: libc::c_double = 0.;
                value = -1.7976931348623157e+308f64;
                e_3 = (*code).arg.list;
                while !e_3.is_null() {
                    temp_0 = _glp_mpl_eval_numeric(mpl, (*e_3).x);
                    if value < temp_0 {
                        value = temp_0;
                    }
                    e_3 = (*e_3).next;
                }
            }
            378 => {
                let mut _info: iter_num_info = iter_num_info {
                    code: 0 as *mut CODE,
                    value: 0.,
                };
                let mut info: *mut iter_num_info = &mut _info;
                (*info).code = code;
                (*info).value = 0.0f64;
                _glp_mpl_loop_within_domain(
                    mpl,
                    (*code).arg.loop_0.domain,
                    info as *mut libc::c_void,
                    Some(
                        iter_num_func
                            as unsafe extern "C" fn(
                                *mut MPL,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                value = (*info).value;
            }
            379 => {
                let mut _info_0: iter_num_info = iter_num_info {
                    code: 0 as *mut CODE,
                    value: 0.,
                };
                let mut info_0: *mut iter_num_info = &mut _info_0;
                (*info_0).code = code;
                (*info_0).value = 1.0f64;
                _glp_mpl_loop_within_domain(
                    mpl,
                    (*code).arg.loop_0.domain,
                    info_0 as *mut libc::c_void,
                    Some(
                        iter_num_func
                            as unsafe extern "C" fn(
                                *mut MPL,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                value = (*info_0).value;
            }
            380 => {
                let mut _info_1: iter_num_info = iter_num_info {
                    code: 0 as *mut CODE,
                    value: 0.,
                };
                let mut info_1: *mut iter_num_info = &mut _info_1;
                (*info_1).code = code;
                (*info_1).value = 1.7976931348623157e+308f64;
                _glp_mpl_loop_within_domain(
                    mpl,
                    (*code).arg.loop_0.domain,
                    info_1 as *mut libc::c_void,
                    Some(
                        iter_num_func
                            as unsafe extern "C" fn(
                                *mut MPL,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if (*info_1).value == 1.7976931348623157e+308f64 {
                    _glp_mpl_error(
                        mpl,
                        b"min{} over empty set; result undefined\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                value = (*info_1).value;
            }
            381 => {
                let mut _info_2: iter_num_info = iter_num_info {
                    code: 0 as *mut CODE,
                    value: 0.,
                };
                let mut info_2: *mut iter_num_info = &mut _info_2;
                (*info_2).code = code;
                (*info_2).value = -1.7976931348623157e+308f64;
                _glp_mpl_loop_within_domain(
                    mpl,
                    (*code).arg.loop_0.domain,
                    info_2 as *mut libc::c_void,
                    Some(
                        iter_num_func
                            as unsafe extern "C" fn(
                                *mut MPL,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                if (*info_2).value == -1.7976931348623157e+308f64 {
                    _glp_mpl_error(
                        mpl,
                        b"max{} over empty set; result undefined\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                value = (*info_2).value;
            }
            _ => {
                (code != code
                    || {
                        glp_assert_(
                            b"code != code\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            3899 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        ((*code).valid == 0
            || {
                glp_assert_(
                    b"!code->valid\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    3902 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*code).valid = 1 as libc::c_int;
        (*code).value.num = value;
    }
    return value;
}
pub unsafe extern "C" fn _glp_mpl_eval_symbolic(
    mut mpl: *mut MPL,
    mut code: *mut CODE,
) -> *mut SYMBOL {
    let mut value: *mut SYMBOL = 0 as *mut SYMBOL;
    (!code.is_null()
        || {
            glp_assert_(
                b"code != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                3916 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).type_0 == 124 as libc::c_int
        || {
            glp_assert_(
                b"code->type == A_SYMBOLIC\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                3917 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).dim == 0 as libc::c_int
        || {
            glp_assert_(
                b"code->dim == 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                3918 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*code).vflag != 0 && (*code).valid != 0 {
        (*code).valid = 0 as libc::c_int;
        _glp_mpl_delete_value(mpl, (*code).type_0, &mut (*code).value);
    }
    if (*code).valid != 0 {
        value = _glp_mpl_copy_symbol(mpl, (*code).value.sym);
    } else {
        match (*code).op {
            302 => {
                value = _glp_mpl_create_symbol_str(
                    mpl,
                    _glp_mpl_create_string(mpl, (*code).arg.str_0),
                );
            }
            303 => {
                (!((*(*code).arg.index.slot).value).is_null()
                    || {
                        glp_assert_(
                            b"code->arg.index.slot->value != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            3939 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                value = _glp_mpl_copy_symbol(mpl, (*(*code).arg.index.slot).value);
            }
            305 => {
                let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
                let mut e: *mut ARG_LIST = 0 as *mut ARG_LIST;
                tuple = _glp_mpl_create_tuple(mpl);
                e = (*code).arg.par.list;
                while !e.is_null() {
                    tuple = _glp_mpl_expand_tuple(
                        mpl,
                        tuple,
                        _glp_mpl_eval_symbolic(mpl, (*e).x),
                    );
                    e = (*e).next;
                }
                value = _glp_mpl_eval_member_sym(mpl, (*code).arg.par.par, tuple);
                _glp_mpl_delete_tuple(mpl, tuple);
            }
            317 => {
                value = _glp_mpl_create_symbol_num(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                );
            }
            352 => {
                value = _glp_mpl_concat_symbols(
                    mpl,
                    _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.y),
                );
            }
            374 => {
                if _glp_mpl_eval_logical(mpl, (*code).arg.arg.x) != 0 {
                    value = _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.y);
                } else if ((*code).arg.arg.z).is_null() {
                    value = _glp_mpl_create_symbol_num(mpl, 0.0f64);
                } else {
                    value = _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.z);
                }
            }
            370 | 375 => {
                let mut pos: libc::c_double = 0.;
                let mut len: libc::c_double = 0.;
                let mut str: [libc::c_char; 101] = [0; 101];
                value = _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.x);
                if ((*value).str_0).is_null() {
                    sprintf(
                        str.as_mut_ptr(),
                        b"%.*g\0" as *const u8 as *const libc::c_char,
                        15 as libc::c_int,
                        (*value).num,
                    );
                } else {
                    _glp_mpl_fetch_string(mpl, (*value).str_0, str.as_mut_ptr());
                }
                _glp_mpl_delete_symbol(mpl, value);
                if (*code).op == 370 as libc::c_int {
                    pos = _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y);
                    if pos != floor(pos) {
                        _glp_mpl_error(
                            mpl,
                            b"substr('...', %.*g); non-integer second argument\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            15 as libc::c_int,
                            pos,
                        );
                    }
                    if pos < 1 as libc::c_int as libc::c_double
                        || pos
                            > (strlen(str.as_mut_ptr()))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_double
                    {
                        _glp_mpl_error(
                            mpl,
                            b"substr('...', %.*g); substring out of range\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            15 as libc::c_int,
                            pos,
                        );
                    }
                } else {
                    pos = _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y);
                    len = _glp_mpl_eval_numeric(mpl, (*code).arg.arg.z);
                    if pos != floor(pos) || len != floor(len) {
                        _glp_mpl_error(
                            mpl,
                            b"substr('...', %.*g, %.*g); non-integer second and/or third argument\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            15 as libc::c_int,
                            pos,
                            15 as libc::c_int,
                            len,
                        );
                    }
                    if pos < 1 as libc::c_int as libc::c_double
                        || len < 0 as libc::c_int as libc::c_double
                        || pos + len
                            > (strlen(str.as_mut_ptr()))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_double
                    {
                        _glp_mpl_error(
                            mpl,
                            b"substr('...', %.*g, %.*g); substring out of range\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            15 as libc::c_int,
                            pos,
                            15 as libc::c_int,
                            len,
                        );
                    }
                    str[(pos as libc::c_int + len as libc::c_int - 1 as libc::c_int)
                        as usize] = '\0' as i32 as libc::c_char;
                }
                value = _glp_mpl_create_symbol_str(
                    mpl,
                    _glp_mpl_create_string(
                        mpl,
                        str
                            .as_mut_ptr()
                            .offset(pos as libc::c_int as isize)
                            .offset(-(1 as libc::c_int as isize)),
                    ),
                );
            }
            372 => {
                let mut num: libc::c_double = 0.;
                let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
                let mut str_0: [libc::c_char; 101] = [0; 101];
                let mut fmt: [libc::c_char; 101] = [0; 101];
                num = _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x);
                sym = _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.y);
                if ((*sym).str_0).is_null() {
                    sprintf(
                        fmt.as_mut_ptr(),
                        b"%.*g\0" as *const u8 as *const libc::c_char,
                        15 as libc::c_int,
                        (*sym).num,
                    );
                } else {
                    _glp_mpl_fetch_string(mpl, (*sym).str_0, fmt.as_mut_ptr());
                }
                _glp_mpl_delete_symbol(mpl, sym);
                _glp_mpl_fn_time2str(mpl, str_0.as_mut_ptr(), num, fmt.as_mut_ptr());
                value = _glp_mpl_create_symbol_str(
                    mpl,
                    _glp_mpl_create_string(mpl, str_0.as_mut_ptr()),
                );
            }
            _ => {
                (code != code
                    || {
                        glp_assert_(
                            b"code != code\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4025 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        ((*code).valid == 0
            || {
                glp_assert_(
                    b"!code->valid\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    4028 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*code).valid = 1 as libc::c_int;
        (*code).value.sym = _glp_mpl_copy_symbol(mpl, value);
    }
    return value;
}
unsafe extern "C" fn iter_log_func(
    mut mpl: *mut MPL,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut iter_log_info = _info as *mut iter_log_info;
    let mut ret: libc::c_int = 0 as libc::c_int;
    match (*(*info).code).op {
        382 => {
            (*info).value &= _glp_mpl_eval_logical(mpl, (*(*info).code).arg.loop_0.x);
            if (*info).value == 0 {
                ret = 1 as libc::c_int;
            }
        }
        383 => {
            (*info).value |= _glp_mpl_eval_logical(mpl, (*(*info).code).arg.loop_0.x);
            if (*info).value != 0 {
                ret = 1 as libc::c_int;
            }
        }
        _ => {
            (info != info
                || {
                    glp_assert_(
                        b"info != info\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        4065 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return ret;
}
pub unsafe extern "C" fn _glp_mpl_eval_logical(
    mut mpl: *mut MPL,
    mut code: *mut CODE,
) -> libc::c_int {
    let mut value: libc::c_int = 0;
    ((*code).type_0 == 114 as libc::c_int
        || {
            glp_assert_(
                b"code->type == A_LOGICAL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4072 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).dim == 0 as libc::c_int
        || {
            glp_assert_(
                b"code->dim == 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4073 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*code).vflag != 0 && (*code).valid != 0 {
        (*code).valid = 0 as libc::c_int;
        _glp_mpl_delete_value(mpl, (*code).type_0, &mut (*code).value);
    }
    if (*code).valid != 0 {
        value = (*code).value.bit;
    } else {
        match (*code).op {
            318 => {
                value = (_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x) != 0.0f64)
                    as libc::c_int;
            }
            323 => {
                value = (_glp_mpl_eval_logical(mpl, (*code).arg.arg.x) == 0)
                    as libc::c_int;
            }
            353 => {
                (!((*code).arg.arg.x).is_null()
                    || {
                        glp_assert_(
                            b"code->arg.arg.x != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4101 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if (*(*code).arg.arg.x).type_0 == 118 as libc::c_int {
                    value = (_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x)
                        < _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y)) as libc::c_int;
                } else {
                    let mut sym1: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.x,
                    );
                    let mut sym2: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.y,
                    );
                    value = (_glp_mpl_compare_symbols(mpl, sym1, sym2)
                        < 0 as libc::c_int) as libc::c_int;
                    _glp_mpl_delete_symbol(mpl, sym1);
                    _glp_mpl_delete_symbol(mpl, sym2);
                }
            }
            354 => {
                (!((*code).arg.arg.x).is_null()
                    || {
                        glp_assert_(
                            b"code->arg.arg.x != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4120 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if (*(*code).arg.arg.x).type_0 == 118 as libc::c_int {
                    value = (_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x)
                        <= _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y)) as libc::c_int;
                } else {
                    let mut sym1_0: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.x,
                    );
                    let mut sym2_0: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.y,
                    );
                    value = (_glp_mpl_compare_symbols(mpl, sym1_0, sym2_0)
                        <= 0 as libc::c_int) as libc::c_int;
                    _glp_mpl_delete_symbol(mpl, sym1_0);
                    _glp_mpl_delete_symbol(mpl, sym2_0);
                }
            }
            355 => {
                (!((*code).arg.arg.x).is_null()
                    || {
                        glp_assert_(
                            b"code->arg.arg.x != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4135 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if (*(*code).arg.arg.x).type_0 == 118 as libc::c_int {
                    value = (_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x)
                        == _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y)) as libc::c_int;
                } else {
                    let mut sym1_1: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.x,
                    );
                    let mut sym2_1: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.y,
                    );
                    value = (_glp_mpl_compare_symbols(mpl, sym1_1, sym2_1)
                        == 0 as libc::c_int) as libc::c_int;
                    _glp_mpl_delete_symbol(mpl, sym1_1);
                    _glp_mpl_delete_symbol(mpl, sym2_1);
                }
            }
            356 => {
                (!((*code).arg.arg.x).is_null()
                    || {
                        glp_assert_(
                            b"code->arg.arg.x != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4153 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if (*(*code).arg.arg.x).type_0 == 118 as libc::c_int {
                    value = (_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x)
                        >= _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y)) as libc::c_int;
                } else {
                    let mut sym1_2: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.x,
                    );
                    let mut sym2_2: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.y,
                    );
                    value = (_glp_mpl_compare_symbols(mpl, sym1_2, sym2_2)
                        >= 0 as libc::c_int) as libc::c_int;
                    _glp_mpl_delete_symbol(mpl, sym1_2);
                    _glp_mpl_delete_symbol(mpl, sym2_2);
                }
            }
            357 => {
                (!((*code).arg.arg.x).is_null()
                    || {
                        glp_assert_(
                            b"code->arg.arg.x != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4172 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if (*(*code).arg.arg.x).type_0 == 118 as libc::c_int {
                    value = (_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x)
                        > _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y)) as libc::c_int;
                } else {
                    let mut sym1_3: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.x,
                    );
                    let mut sym2_3: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.y,
                    );
                    value = (_glp_mpl_compare_symbols(mpl, sym1_3, sym2_3)
                        > 0 as libc::c_int) as libc::c_int;
                    _glp_mpl_delete_symbol(mpl, sym1_3);
                    _glp_mpl_delete_symbol(mpl, sym2_3);
                }
            }
            358 => {
                (!((*code).arg.arg.x).is_null()
                    || {
                        glp_assert_(
                            b"code->arg.arg.x != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4187 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if (*(*code).arg.arg.x).type_0 == 118 as libc::c_int {
                    value = (_glp_mpl_eval_numeric(mpl, (*code).arg.arg.x)
                        != _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y)) as libc::c_int;
                } else {
                    let mut sym1_4: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.x,
                    );
                    let mut sym2_4: *mut SYMBOL = _glp_mpl_eval_symbolic(
                        mpl,
                        (*code).arg.arg.y,
                    );
                    value = (_glp_mpl_compare_symbols(mpl, sym1_4, sym2_4)
                        != 0 as libc::c_int) as libc::c_int;
                    _glp_mpl_delete_symbol(mpl, sym1_4);
                    _glp_mpl_delete_symbol(mpl, sym2_4);
                }
            }
            359 => {
                value = (_glp_mpl_eval_logical(mpl, (*code).arg.arg.x) != 0
                    && _glp_mpl_eval_logical(mpl, (*code).arg.arg.y) != 0)
                    as libc::c_int;
            }
            360 => {
                value = (_glp_mpl_eval_logical(mpl, (*code).arg.arg.x) != 0
                    || _glp_mpl_eval_logical(mpl, (*code).arg.arg.y) != 0)
                    as libc::c_int;
            }
            366 => {
                let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
                tuple = _glp_mpl_eval_tuple(mpl, (*code).arg.arg.x);
                value = _glp_mpl_is_member(mpl, (*code).arg.arg.y, tuple);
                _glp_mpl_delete_tuple(mpl, tuple);
            }
            367 => {
                let mut tuple_0: *mut TUPLE = 0 as *mut TUPLE;
                tuple_0 = _glp_mpl_eval_tuple(mpl, (*code).arg.arg.x);
                value = (_glp_mpl_is_member(mpl, (*code).arg.arg.y, tuple_0) == 0)
                    as libc::c_int;
                _glp_mpl_delete_tuple(mpl, tuple_0);
            }
            368 => {
                let mut set: *mut ELEMSET = 0 as *mut ELEMSET;
                let mut memb: *mut MEMBER = 0 as *mut MEMBER;
                set = _glp_mpl_eval_elemset(mpl, (*code).arg.arg.x);
                value = 1 as libc::c_int;
                memb = (*set).head;
                while !memb.is_null() {
                    if _glp_mpl_is_member(mpl, (*code).arg.arg.y, (*memb).tuple) == 0 {
                        value = 0 as libc::c_int;
                        break;
                    } else {
                        memb = (*memb).next;
                    }
                }
                _glp_mpl_delete_elemset(mpl, set);
            }
            369 => {
                let mut set_0: *mut ELEMSET = 0 as *mut ELEMSET;
                let mut memb_0: *mut MEMBER = 0 as *mut MEMBER;
                set_0 = _glp_mpl_eval_elemset(mpl, (*code).arg.arg.x);
                value = 1 as libc::c_int;
                memb_0 = (*set_0).head;
                while !memb_0.is_null() {
                    if _glp_mpl_is_member(mpl, (*code).arg.arg.y, (*memb_0).tuple) != 0 {
                        value = 0 as libc::c_int;
                        break;
                    } else {
                        memb_0 = (*memb_0).next;
                    }
                }
                _glp_mpl_delete_elemset(mpl, set_0);
            }
            382 => {
                let mut _info: iter_log_info = iter_log_info {
                    code: 0 as *mut CODE,
                    value: 0,
                };
                let mut info: *mut iter_log_info = &mut _info;
                (*info).code = code;
                (*info).value = 1 as libc::c_int;
                _glp_mpl_loop_within_domain(
                    mpl,
                    (*code).arg.loop_0.domain,
                    info as *mut libc::c_void,
                    Some(
                        iter_log_func
                            as unsafe extern "C" fn(
                                *mut MPL,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                value = (*info).value;
            }
            383 => {
                let mut _info_0: iter_log_info = iter_log_info {
                    code: 0 as *mut CODE,
                    value: 0,
                };
                let mut info_0: *mut iter_log_info = &mut _info_0;
                (*info_0).code = code;
                (*info_0).value = 0 as libc::c_int;
                _glp_mpl_loop_within_domain(
                    mpl,
                    (*code).arg.loop_0.domain,
                    info_0 as *mut libc::c_void,
                    Some(
                        iter_log_func
                            as unsafe extern "C" fn(
                                *mut MPL,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                value = (*info_0).value;
            }
            _ => {
                (code != code
                    || {
                        glp_assert_(
                            b"code != code\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4276 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        ((*code).valid == 0
            || {
                glp_assert_(
                    b"!code->valid\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    4279 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*code).valid = 1 as libc::c_int;
        (*code).value.bit = value;
    }
    return value;
}
pub unsafe extern "C" fn _glp_mpl_eval_tuple(
    mut mpl: *mut MPL,
    mut code: *mut CODE,
) -> *mut TUPLE {
    let mut value: *mut TUPLE = 0 as *mut TUPLE;
    (!code.is_null()
        || {
            glp_assert_(
                b"code != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4293 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).type_0 == 126 as libc::c_int
        || {
            glp_assert_(
                b"code->type == A_TUPLE\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4294 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"code->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4295 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*code).vflag != 0 && (*code).valid != 0 {
        (*code).valid = 0 as libc::c_int;
        _glp_mpl_delete_value(mpl, (*code).type_0, &mut (*code).value);
    }
    if (*code).valid != 0 {
        value = _glp_mpl_copy_tuple(mpl, (*code).value.tuple);
    } else {
        match (*code).op {
            309 => {
                let mut e: *mut ARG_LIST = 0 as *mut ARG_LIST;
                value = _glp_mpl_create_tuple(mpl);
                e = (*code).arg.list;
                while !e.is_null() {
                    value = _glp_mpl_expand_tuple(
                        mpl,
                        value,
                        _glp_mpl_eval_symbolic(mpl, (*e).x),
                    );
                    e = (*e).next;
                }
            }
            319 => {
                value = _glp_mpl_expand_tuple(
                    mpl,
                    _glp_mpl_create_tuple(mpl),
                    _glp_mpl_eval_symbolic(mpl, (*code).arg.arg.x),
                );
            }
            _ => {
                (code != code
                    || {
                        glp_assert_(
                            b"code != code\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4324 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        ((*code).valid == 0
            || {
                glp_assert_(
                    b"!code->valid\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    4327 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*code).valid = 1 as libc::c_int;
        (*code).value.tuple = _glp_mpl_copy_tuple(mpl, value);
    }
    return value;
}
unsafe extern "C" fn iter_set_func(
    mut mpl: *mut MPL,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut iter_set_info = _info as *mut iter_set_info;
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    match (*(*info).code).op {
        384 => {
            tuple = _glp_mpl_eval_tuple(mpl, (*(*info).code).arg.loop_0.x);
            if (_glp_mpl_find_tuple(mpl, (*info).value, tuple)).is_null() {
                _glp_mpl_add_tuple(mpl, (*info).value, tuple);
            } else {
                _glp_mpl_delete_tuple(mpl, tuple);
            }
        }
        385 => {
            _glp_mpl_add_tuple(
                mpl,
                (*info).value,
                _glp_mpl_get_domain_tuple(mpl, (*(*info).code).arg.loop_0.domain),
            );
        }
        _ => {
            (info != info
                || {
                    glp_assert_(
                        b"info != info\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        4370 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_eval_elemset(
    mut mpl: *mut MPL,
    mut code: *mut CODE,
) -> *mut ELEMSET {
    let mut value: *mut ELEMSET = 0 as *mut ELEMSET;
    (!code.is_null()
        || {
            glp_assert_(
                b"code != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4377 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).type_0 == 106 as libc::c_int
        || {
            glp_assert_(
                b"code->type == A_ELEMSET\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4378 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"code->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4379 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*code).vflag != 0 && (*code).valid != 0 {
        (*code).valid = 0 as libc::c_int;
        _glp_mpl_delete_value(mpl, (*code).type_0, &mut (*code).value);
    }
    if (*code).valid != 0 {
        value = _glp_mpl_copy_elemset(mpl, (*code).value.set);
    } else {
        match (*code).op {
            306 => {
                let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
                let mut e: *mut ARG_LIST = 0 as *mut ARG_LIST;
                tuple = _glp_mpl_create_tuple(mpl);
                e = (*code).arg.set.list;
                while !e.is_null() {
                    tuple = _glp_mpl_expand_tuple(
                        mpl,
                        tuple,
                        _glp_mpl_eval_symbolic(mpl, (*e).x),
                    );
                    e = (*e).next;
                }
                value = _glp_mpl_copy_elemset(
                    mpl,
                    _glp_mpl_eval_member_set(mpl, (*code).arg.set.set, tuple),
                );
                _glp_mpl_delete_tuple(mpl, tuple);
            }
            310 => {
                let mut e_0: *mut ARG_LIST = 0 as *mut ARG_LIST;
                value = _glp_mpl_create_elemset(mpl, (*code).dim);
                e_0 = (*code).arg.list;
                while !e_0.is_null() {
                    _glp_mpl_check_then_add(
                        mpl,
                        value,
                        _glp_mpl_eval_tuple(mpl, (*e_0).x),
                    );
                    e_0 = (*e_0).next;
                }
            }
            361 => {
                value = _glp_mpl_set_union(
                    mpl,
                    _glp_mpl_eval_elemset(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_elemset(mpl, (*code).arg.arg.y),
                );
            }
            362 => {
                value = _glp_mpl_set_diff(
                    mpl,
                    _glp_mpl_eval_elemset(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_elemset(mpl, (*code).arg.arg.y),
                );
            }
            363 => {
                value = _glp_mpl_set_symdiff(
                    mpl,
                    _glp_mpl_eval_elemset(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_elemset(mpl, (*code).arg.arg.y),
                );
            }
            364 => {
                value = _glp_mpl_set_inter(
                    mpl,
                    _glp_mpl_eval_elemset(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_elemset(mpl, (*code).arg.arg.y),
                );
            }
            365 => {
                value = _glp_mpl_set_cross(
                    mpl,
                    _glp_mpl_eval_elemset(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_elemset(mpl, (*code).arg.arg.y),
                );
            }
            373 => {
                value = _glp_mpl_create_arelset(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                    if ((*code).arg.arg.z).is_null() {
                        1.0f64
                    } else {
                        _glp_mpl_eval_numeric(mpl, (*code).arg.arg.z)
                    },
                );
            }
            374 => {
                if _glp_mpl_eval_logical(mpl, (*code).arg.arg.x) != 0 {
                    value = _glp_mpl_eval_elemset(mpl, (*code).arg.arg.y);
                } else {
                    value = _glp_mpl_eval_elemset(mpl, (*code).arg.arg.z);
                }
            }
            384 => {
                let mut _info: iter_set_info = iter_set_info {
                    code: 0 as *mut CODE,
                    value: 0 as *mut ELEMSET,
                };
                let mut info: *mut iter_set_info = &mut _info;
                (*info).code = code;
                (*info).value = _glp_mpl_create_elemset(mpl, (*code).dim);
                _glp_mpl_loop_within_domain(
                    mpl,
                    (*code).arg.loop_0.domain,
                    info as *mut libc::c_void,
                    Some(
                        iter_set_func
                            as unsafe extern "C" fn(
                                *mut MPL,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                value = (*info).value;
            }
            385 => {
                let mut _info_0: iter_set_info = iter_set_info {
                    code: 0 as *mut CODE,
                    value: 0 as *mut ELEMSET,
                };
                let mut info_0: *mut iter_set_info = &mut _info_0;
                (*info_0).code = code;
                (*info_0).value = _glp_mpl_create_elemset(mpl, (*code).dim);
                _glp_mpl_loop_within_domain(
                    mpl,
                    (*code).arg.loop_0.domain,
                    info_0 as *mut libc::c_void,
                    Some(
                        iter_set_func
                            as unsafe extern "C" fn(
                                *mut MPL,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                value = (*info_0).value;
            }
            _ => {
                (code != code
                    || {
                        glp_assert_(
                            b"code != code\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4480 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        ((*code).valid == 0
            || {
                glp_assert_(
                    b"!code->valid\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    4483 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*code).valid = 1 as libc::c_int;
        (*code).value.set = _glp_mpl_copy_elemset(mpl, value);
    }
    return value;
}
unsafe extern "C" fn null_func(mut mpl: *mut MPL, mut info: *mut libc::c_void) {
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4500 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (info.is_null()
        || {
            glp_assert_(
                b"info == NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4501 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_is_member(
    mut mpl: *mut MPL,
    mut code: *mut CODE,
    mut tuple: *mut TUPLE,
) -> libc::c_int {
    let mut value: libc::c_int = 0;
    (!code.is_null()
        || {
            glp_assert_(
                b"code != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4507 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).type_0 == 106 as libc::c_int
        || {
            glp_assert_(
                b"code->type == A_ELEMSET\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4508 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).dim > 0 as libc::c_int
        || {
            glp_assert_(
                b"code->dim > 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4509 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!tuple.is_null()
        || {
            glp_assert_(
                b"tuple != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4510 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match (*code).op {
        306 => {
            let mut e: *mut ARG_LIST = 0 as *mut ARG_LIST;
            let mut temp: *mut TUPLE = 0 as *mut TUPLE;
            let mut set: *mut ELEMSET = 0 as *mut ELEMSET;
            temp = _glp_mpl_create_tuple(mpl);
            e = (*code).arg.set.list;
            while !e.is_null() {
                temp = _glp_mpl_expand_tuple(
                    mpl,
                    temp,
                    _glp_mpl_eval_symbolic(mpl, (*e).x),
                );
                e = (*e).next;
            }
            set = _glp_mpl_eval_member_set(mpl, (*code).arg.set.set, temp);
            _glp_mpl_delete_tuple(mpl, temp);
            temp = _glp_mpl_build_subtuple(mpl, tuple, (*set).dim);
            value = (_glp_mpl_find_tuple(mpl, set, temp)
                != 0 as *mut libc::c_void as *mut MEMBER) as libc::c_int;
            _glp_mpl_delete_tuple(mpl, temp);
        }
        310 => {
            let mut e_0: *mut ARG_LIST = 0 as *mut ARG_LIST;
            let mut temp_0: *mut TUPLE = 0 as *mut TUPLE;
            let mut that: *mut TUPLE = 0 as *mut TUPLE;
            value = 0 as libc::c_int;
            temp_0 = _glp_mpl_build_subtuple(mpl, tuple, (*code).dim);
            e_0 = (*code).arg.list;
            while !e_0.is_null() {
                that = _glp_mpl_eval_tuple(mpl, (*e_0).x);
                value = (_glp_mpl_compare_tuples(mpl, temp_0, that) == 0 as libc::c_int)
                    as libc::c_int;
                _glp_mpl_delete_tuple(mpl, that);
                if value != 0 {
                    break;
                }
                e_0 = (*e_0).next;
            }
            _glp_mpl_delete_tuple(mpl, temp_0);
        }
        361 => {
            value = (_glp_mpl_is_member(mpl, (*code).arg.arg.x, tuple) != 0
                || _glp_mpl_is_member(mpl, (*code).arg.arg.y, tuple) != 0)
                as libc::c_int;
        }
        362 => {
            value = (_glp_mpl_is_member(mpl, (*code).arg.arg.x, tuple) != 0
                && _glp_mpl_is_member(mpl, (*code).arg.arg.y, tuple) == 0)
                as libc::c_int;
        }
        363 => {
            let mut in1: libc::c_int = _glp_mpl_is_member(mpl, (*code).arg.arg.x, tuple);
            let mut in2: libc::c_int = _glp_mpl_is_member(mpl, (*code).arg.arg.y, tuple);
            value = (in1 != 0 && in2 == 0 || in1 == 0 && in2 != 0) as libc::c_int;
        }
        364 => {
            value = (_glp_mpl_is_member(mpl, (*code).arg.arg.x, tuple) != 0
                && _glp_mpl_is_member(mpl, (*code).arg.arg.y, tuple) != 0)
                as libc::c_int;
        }
        365 => {
            let mut j: libc::c_int = 0;
            value = _glp_mpl_is_member(mpl, (*code).arg.arg.x, tuple);
            if value != 0 {
                j = 1 as libc::c_int;
                while j <= (*(*code).arg.arg.x).dim {
                    (!tuple.is_null()
                        || {
                            glp_assert_(
                                b"tuple != NULL\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                4569 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    tuple = (*tuple).next;
                    j += 1;
                    j;
                }
                value = _glp_mpl_is_member(mpl, (*code).arg.arg.y, tuple);
            }
        }
        373 => {
            let mut j_0: libc::c_int = 0;
            let mut x: libc::c_double = 0.;
            let mut t0: libc::c_double = 0.;
            let mut tf: libc::c_double = 0.;
            let mut dt: libc::c_double = 0.;
            ((*code).dim == 1 as libc::c_int
                || {
                    glp_assert_(
                        b"code->dim == 1\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        4580 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            t0 = _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x);
            tf = _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y);
            if ((*code).arg.arg.z).is_null() {
                dt = 1.0f64;
            } else {
                dt = _glp_mpl_eval_numeric(mpl, (*code).arg.arg.z);
            }
            _glp_mpl_arelset_size(mpl, t0, tf, dt);
            (!((*tuple).sym).is_null()
                || {
                    glp_assert_(
                        b"tuple->sym != NULL\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        4592 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if !((*(*tuple).sym).str_0).is_null() {
                value = 0 as libc::c_int;
            } else {
                x = (*(*tuple).sym).num;
                if dt > 0.0f64 && !(t0 <= x && x <= tf)
                    || dt < 0.0f64 && !(tf <= x && x <= t0)
                {
                    value = 0 as libc::c_int;
                } else {
                    j_0 = ((x - t0) / dt + 0.5f64) as libc::c_int + 1 as libc::c_int;
                    value = (_glp_mpl_arelset_member(mpl, t0, tf, dt, j_0) == x)
                        as libc::c_int;
                }
            }
        }
        374 => {
            if _glp_mpl_eval_logical(mpl, (*code).arg.arg.x) != 0 {
                value = _glp_mpl_is_member(mpl, (*code).arg.arg.y, tuple);
            } else {
                value = _glp_mpl_is_member(mpl, (*code).arg.arg.z, tuple);
            }
        }
        384 => {
            _glp_mpl_error(
                mpl,
                b"implementation restriction; in/within setof{} not allowed\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        385 => {
            let mut temp_1: *mut TUPLE = 0 as *mut TUPLE;
            temp_1 = _glp_mpl_build_subtuple(mpl, tuple, (*code).dim);
            value = (_glp_mpl_eval_within_domain(
                mpl,
                (*code).arg.loop_0.domain,
                temp_1,
                0 as *mut libc::c_void,
                Some(
                    null_func as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> (),
                ),
            ) == 0 as libc::c_int) as libc::c_int;
            _glp_mpl_delete_tuple(mpl, temp_1);
        }
        _ => {
            (code != code
                || {
                    glp_assert_(
                        b"code != code\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        4638 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return value;
}
unsafe extern "C" fn iter_form_func(
    mut mpl: *mut MPL,
    mut _info: *mut libc::c_void,
) -> libc::c_int {
    let mut info: *mut iter_form_info = _info as *mut iter_form_info;
    match (*(*info).code).op {
        378 => {
            let mut form: *mut FORMULA = 0 as *mut FORMULA;
            let mut term: *mut FORMULA = 0 as *mut FORMULA;
            form = _glp_mpl_eval_formula(mpl, (*(*info).code).arg.loop_0.x);
            if ((*info).value).is_null() {
                (((*info).tail).is_null()
                    || {
                        glp_assert_(
                            b"info->tail == NULL\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4682 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*info).value = form;
            } else {
                (!((*info).tail).is_null()
                    || {
                        glp_assert_(
                            b"info->tail != NULL\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4686 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (*(*info).tail).next = form;
            }
            term = form;
            while !term.is_null() {
                (*info).tail = term;
                term = (*term).next;
            }
        }
        _ => {
            (info != info
                || {
                    glp_assert_(
                        b"info != info\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        4695 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_eval_formula(
    mut mpl: *mut MPL,
    mut code: *mut CODE,
) -> *mut FORMULA {
    let mut value: *mut FORMULA = 0 as *mut FORMULA;
    (!code.is_null()
        || {
            glp_assert_(
                b"code != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4702 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).type_0 == 110 as libc::c_int
        || {
            glp_assert_(
                b"code->type == A_FORMULA\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4703 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*code).dim == 0 as libc::c_int
        || {
            glp_assert_(
                b"code->dim == 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4704 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if (*code).vflag != 0 && (*code).valid != 0 {
        (*code).valid = 0 as libc::c_int;
        _glp_mpl_delete_value(mpl, (*code).type_0, &mut (*code).value);
    }
    if (*code).valid != 0 {
        value = _glp_mpl_copy_formula(mpl, (*code).value.form);
    } else {
        match (*code).op {
            307 => {
                let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
                let mut e: *mut ARG_LIST = 0 as *mut ARG_LIST;
                tuple = _glp_mpl_create_tuple(mpl);
                e = (*code).arg.var.list;
                while !e.is_null() {
                    tuple = _glp_mpl_expand_tuple(
                        mpl,
                        tuple,
                        _glp_mpl_eval_symbolic(mpl, (*e).x),
                    );
                    e = (*e).next;
                }
                ((*code).arg.var.suff == 0 as libc::c_int
                    || {
                        glp_assert_(
                            b"code->arg.var.suff == DOT_NONE\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4727 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                value = _glp_mpl_single_variable(
                    mpl,
                    _glp_mpl_eval_member_var(mpl, (*code).arg.var.var, tuple),
                );
                _glp_mpl_delete_tuple(mpl, tuple);
            }
            320 => {
                value = _glp_mpl_constant_term(
                    mpl,
                    _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                );
            }
            321 => {
                value = _glp_mpl_linear_comb(
                    mpl,
                    0.0f64,
                    _glp_mpl_constant_term(mpl, 0.0f64),
                    1.0f64,
                    _glp_mpl_eval_formula(mpl, (*code).arg.arg.x),
                );
            }
            322 => {
                value = _glp_mpl_linear_comb(
                    mpl,
                    0.0f64,
                    _glp_mpl_constant_term(mpl, 0.0f64),
                    -1.0f64,
                    _glp_mpl_eval_formula(mpl, (*code).arg.arg.x),
                );
            }
            339 => {
                value = _glp_mpl_linear_comb(
                    mpl,
                    1.0f64,
                    _glp_mpl_eval_formula(mpl, (*code).arg.arg.x),
                    1.0f64,
                    _glp_mpl_eval_formula(mpl, (*code).arg.arg.y),
                );
            }
            340 => {
                value = _glp_mpl_linear_comb(
                    mpl,
                    1.0f64,
                    _glp_mpl_eval_formula(mpl, (*code).arg.arg.x),
                    -1.0f64,
                    _glp_mpl_eval_formula(mpl, (*code).arg.arg.y),
                );
            }
            342 => {
                (!((*code).arg.arg.x).is_null()
                    || {
                        glp_assert_(
                            b"code->arg.arg.x != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4765 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (!((*code).arg.arg.y).is_null()
                    || {
                        glp_assert_(
                            b"code->arg.arg.y != NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4766 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if (*(*code).arg.arg.x).type_0 == 118 as libc::c_int {
                    ((*(*code).arg.arg.y).type_0 == 110 as libc::c_int
                        || {
                            glp_assert_(
                                b"code->arg.arg.y->type == A_FORMULA\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                4768 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    value = _glp_mpl_linear_comb(
                        mpl,
                        _glp_mpl_eval_numeric(mpl, (*code).arg.arg.x),
                        _glp_mpl_eval_formula(mpl, (*code).arg.arg.y),
                        0.0f64,
                        _glp_mpl_constant_term(mpl, 0.0f64),
                    );
                } else {
                    ((*(*code).arg.arg.x).type_0 == 110 as libc::c_int
                        || {
                            glp_assert_(
                                b"code->arg.arg.x->type == A_FORMULA\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                4775 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    ((*(*code).arg.arg.y).type_0 == 118 as libc::c_int
                        || {
                            glp_assert_(
                                b"code->arg.arg.y->type == A_NUMERIC\0" as *const u8
                                    as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                4776 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    value = _glp_mpl_linear_comb(
                        mpl,
                        _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                        _glp_mpl_eval_formula(mpl, (*code).arg.arg.x),
                        0.0f64,
                        _glp_mpl_constant_term(mpl, 0.0f64),
                    );
                }
            }
            343 => {
                value = _glp_mpl_linear_comb(
                    mpl,
                    _glp_mpl_fp_div(
                        mpl,
                        1.0f64,
                        _glp_mpl_eval_numeric(mpl, (*code).arg.arg.y),
                    ),
                    _glp_mpl_eval_formula(mpl, (*code).arg.arg.x),
                    0.0f64,
                    _glp_mpl_constant_term(mpl, 0.0f64),
                );
            }
            374 => {
                if _glp_mpl_eval_logical(mpl, (*code).arg.arg.x) != 0 {
                    value = _glp_mpl_eval_formula(mpl, (*code).arg.arg.y);
                } else if ((*code).arg.arg.z).is_null() {
                    value = _glp_mpl_constant_term(mpl, 0.0f64);
                } else {
                    value = _glp_mpl_eval_formula(mpl, (*code).arg.arg.z);
                }
            }
            378 => {
                let mut _info: iter_form_info = iter_form_info {
                    code: 0 as *mut CODE,
                    value: 0 as *mut FORMULA,
                    tail: 0 as *mut FORMULA,
                };
                let mut info: *mut iter_form_info = &mut _info;
                (*info).code = code;
                (*info).value = _glp_mpl_constant_term(mpl, 0.0f64);
                (*info).tail = 0 as *mut FORMULA;
                _glp_mpl_loop_within_domain(
                    mpl,
                    (*code).arg.loop_0.domain,
                    info as *mut libc::c_void,
                    Some(
                        iter_form_func
                            as unsafe extern "C" fn(
                                *mut MPL,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                );
                value = _glp_mpl_reduce_terms(mpl, (*info).value);
            }
            _ => {
                (code != code
                    || {
                        glp_assert_(
                            b"code != code\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4811 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        ((*code).valid == 0
            || {
                glp_assert_(
                    b"!code->valid\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    4814 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        (*code).valid = 1 as libc::c_int;
        (*code).value.form = _glp_mpl_copy_formula(mpl, value);
    }
    return value;
}
pub unsafe extern "C" fn _glp_mpl_clean_code(mut mpl: *mut MPL, mut code: *mut CODE) {
    let mut e: *mut ARG_LIST = 0 as *mut ARG_LIST;
    if !code.is_null() {
        if (*code).valid != 0 {
            (*code).valid = 0 as libc::c_int;
            _glp_mpl_delete_value(mpl, (*code).type_0, &mut (*code).value);
        }
        match (*code).op {
            304 | 305 => {
                e = (*code).arg.par.list;
                while !e.is_null() {
                    _glp_mpl_clean_code(mpl, (*e).x);
                    e = (*e).next;
                }
            }
            306 => {
                e = (*code).arg.set.list;
                while !e.is_null() {
                    _glp_mpl_clean_code(mpl, (*e).x);
                    e = (*e).next;
                }
            }
            307 => {
                e = (*code).arg.var.list;
                while !e.is_null() {
                    _glp_mpl_clean_code(mpl, (*e).x);
                    e = (*e).next;
                }
            }
            308 => {
                e = (*code).arg.con.list;
                while !e.is_null() {
                    _glp_mpl_clean_code(mpl, (*e).x);
                    e = (*e).next;
                }
            }
            309 | 310 => {
                e = (*code).arg.list;
                while !e.is_null() {
                    _glp_mpl_clean_code(mpl, (*e).x);
                    e = (*e).next;
                }
            }
            311 => {
                (code != code
                    || {
                        glp_assert_(
                            b"code != code\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4866 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
            301 | 302 | 303 | 312 | 313 | 314 | 315 => {}
            316 | 317 | 318 | 319 | 320 | 321 | 322 | 323 | 324 | 325 | 326 | 327 | 328
            | 329 | 330 | 331 | 332 | 333 | 334 | 335 | 336 | 337 | 338 => {
                _glp_mpl_clean_code(mpl, (*code).arg.arg.x);
            }
            339 | 340 | 341 | 342 | 343 | 344 | 345 | 346 | 347 | 348 | 349 | 350 | 351
            | 352 | 353 | 354 | 355 | 356 | 357 | 358 | 359 | 360 | 361 | 362 | 363 | 364
            | 365 | 366 | 367 | 368 | 369 | 370 | 371 | 372 => {
                _glp_mpl_clean_code(mpl, (*code).arg.arg.x);
                _glp_mpl_clean_code(mpl, (*code).arg.arg.y);
            }
            373 | 374 | 375 => {
                _glp_mpl_clean_code(mpl, (*code).arg.arg.x);
                _glp_mpl_clean_code(mpl, (*code).arg.arg.y);
                _glp_mpl_clean_code(mpl, (*code).arg.arg.z);
            }
            376 | 377 => {
                e = (*code).arg.list;
                while !e.is_null() {
                    _glp_mpl_clean_code(mpl, (*e).x);
                    e = (*e).next;
                }
            }
            378 | 379 | 380 | 381 | 382 | 383 | 384 | 385 => {
                _glp_mpl_clean_domain(mpl, (*code).arg.loop_0.domain);
                _glp_mpl_clean_code(mpl, (*code).arg.loop_0.x);
            }
            _ => {
                ((*code).op != (*code).op
                    || {
                        glp_assert_(
                            b"code->op != code->op\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            4963 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    }
}
pub unsafe extern "C" fn _glp_mpl_tab_num_args(mut dca: *mut TABDCA) -> libc::c_int {
    return (*dca).na;
}
pub unsafe extern "C" fn _glp_mpl_tab_get_arg(
    mut dca: *mut TABDCA,
    mut k: libc::c_int,
) -> *const libc::c_char {
    (1 as libc::c_int <= k && k <= (*dca).na
        || {
            glp_assert_(
                b"1 <= k && k <= dca->na\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4980 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return *((*dca).arg).offset(k as isize);
}
pub unsafe extern "C" fn _glp_mpl_tab_num_flds(mut dca: *mut TABDCA) -> libc::c_int {
    return (*dca).nf;
}
pub unsafe extern "C" fn _glp_mpl_tab_get_name(
    mut dca: *mut TABDCA,
    mut k: libc::c_int,
) -> *const libc::c_char {
    (1 as libc::c_int <= k && k <= (*dca).nf
        || {
            glp_assert_(
                b"1 <= k && k <= dca->nf\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4991 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return *((*dca).name).offset(k as isize);
}
pub unsafe extern "C" fn _glp_mpl_tab_get_type(
    mut dca: *mut TABDCA,
    mut k: libc::c_int,
) -> libc::c_int {
    (1 as libc::c_int <= k && k <= (*dca).nf
        || {
            glp_assert_(
                b"1 <= k && k <= dca->nf\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                4997 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return *((*dca).type_0).offset(k as isize);
}
pub unsafe extern "C" fn _glp_mpl_tab_get_num(
    mut dca: *mut TABDCA,
    mut k: libc::c_int,
) -> libc::c_double {
    (1 as libc::c_int <= k && k <= (*dca).nf
        || {
            glp_assert_(
                b"1 <= k && k <= dca->nf\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5003 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*((*dca).type_0).offset(k as isize) == 'N' as i32
        || {
            glp_assert_(
                b"dca->type[k] == 'N'\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5004 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return *((*dca).num).offset(k as isize);
}
pub unsafe extern "C" fn _glp_mpl_tab_get_str(
    mut dca: *mut TABDCA,
    mut k: libc::c_int,
) -> *const libc::c_char {
    (1 as libc::c_int <= k && k <= (*dca).nf
        || {
            glp_assert_(
                b"1 <= k && k <= dca->nf\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5010 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*((*dca).type_0).offset(k as isize) == 'S' as i32
        || {
            glp_assert_(
                b"dca->type[k] == 'S'\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5011 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!(*((*dca).str_0).offset(k as isize)).is_null()
        || {
            glp_assert_(
                b"dca->str[k] != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5012 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return *((*dca).str_0).offset(k as isize);
}
pub unsafe extern "C" fn _glp_mpl_tab_set_num(
    mut dca: *mut TABDCA,
    mut k: libc::c_int,
    mut num: libc::c_double,
) {
    (1 as libc::c_int <= k && k <= (*dca).nf
        || {
            glp_assert_(
                b"1 <= k && k <= dca->nf\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5018 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*((*dca).type_0).offset(k as isize) == '?' as i32
        || {
            glp_assert_(
                b"dca->type[k] == '?'\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5019 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    *((*dca).type_0).offset(k as isize) = 'N' as i32;
    *((*dca).num).offset(k as isize) = num;
}
pub unsafe extern "C" fn _glp_mpl_tab_set_str(
    mut dca: *mut TABDCA,
    mut k: libc::c_int,
    mut str: *const libc::c_char,
) {
    (1 as libc::c_int <= k && k <= (*dca).nf
        || {
            glp_assert_(
                b"1 <= k && k <= dca->nf\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5027 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*((*dca).type_0).offset(k as isize) == '?' as i32
        || {
            glp_assert_(
                b"dca->type[k] == '?'\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5028 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (strlen(str) <= 100 as libc::c_int as libc::c_ulong
        || {
            glp_assert_(
                b"strlen(str) <= MAX_LENGTH\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5029 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!(*((*dca).str_0).offset(k as isize)).is_null()
        || {
            glp_assert_(
                b"dca->str[k] != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5030 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    *((*dca).type_0).offset(k as isize) = 'S' as i32;
    strcpy(*((*dca).str_0).offset(k as isize), str);
}
unsafe extern "C" fn write_func(
    mut mpl: *mut MPL,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut tab: *mut TABLE = info as *mut TABLE;
    let mut dca: *mut TABDCA = (*mpl).dca;
    let mut out: *mut TABOUT = 0 as *mut TABOUT;
    let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut k: libc::c_int = 0;
    let mut buf: [libc::c_char; 101] = [0; 101];
    k = 0 as libc::c_int;
    out = (*tab).u.out.list;
    while !out.is_null() {
        k += 1;
        k;
        match (*(*out).code).type_0 {
            118 => {
                *((*dca).type_0).offset(k as isize) = 'N' as i32;
                *((*dca).num)
                    .offset(k as isize) = _glp_mpl_eval_numeric(mpl, (*out).code);
                *(*((*dca).str_0).offset(k as isize))
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            }
            124 => {
                sym = _glp_mpl_eval_symbolic(mpl, (*out).code);
                if ((*sym).str_0).is_null() {
                    *((*dca).type_0).offset(k as isize) = 'N' as i32;
                    *((*dca).num).offset(k as isize) = (*sym).num;
                    *(*((*dca).str_0).offset(k as isize))
                        .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                } else {
                    *((*dca).type_0).offset(k as isize) = 'S' as i32;
                    *((*dca).num).offset(k as isize) = 0.0f64;
                    _glp_mpl_fetch_string(mpl, (*sym).str_0, buf.as_mut_ptr());
                    strcpy(*((*dca).str_0).offset(k as isize), buf.as_mut_ptr());
                }
                _glp_mpl_delete_symbol(mpl, sym);
            }
            _ => {
                (out != out
                    || {
                        glp_assert_(
                            b"out != out\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            5070 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
        out = (*out).next;
    }
    _glp_mpl_tab_drv_write(mpl);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_execute_table(mut mpl: *mut MPL, mut tab: *mut TABLE) {
    let mut current_block: u64;
    let mut arg: *mut TABARG = 0 as *mut TABARG;
    let mut fld: *mut TABFLD = 0 as *mut TABFLD;
    let mut in_0: *mut TABIN = 0 as *mut TABIN;
    let mut out: *mut TABOUT = 0 as *mut TABOUT;
    let mut dca: *mut TABDCA = 0 as *mut TABDCA;
    let mut set: *mut SET = 0 as *mut SET;
    let mut k: libc::c_int = 0;
    let mut buf: [libc::c_char; 101] = [0; 101];
    (((*mpl).dca).is_null()
        || {
            glp_assert_(
                b"mpl->dca == NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5089 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    dca = glp_alloc(
        1 as libc::c_int,
        ::std::mem::size_of::<TABDCA>() as libc::c_ulong as libc::c_int,
    ) as *mut TABDCA;
    (*mpl).dca = dca;
    (*dca).id = 0 as libc::c_int;
    (*dca).link = 0 as *mut libc::c_void;
    (*dca).na = 0 as libc::c_int;
    (*dca).arg = 0 as *mut *mut libc::c_char;
    (*dca).nf = 0 as libc::c_int;
    (*dca).name = 0 as *mut *mut libc::c_char;
    (*dca).type_0 = 0 as *mut libc::c_int;
    (*dca).num = 0 as *mut libc::c_double;
    (*dca).str_0 = 0 as *mut *mut libc::c_char;
    ((*dca).na == 0 as libc::c_int
        || {
            glp_assert_(
                b"dca->na == 0\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5101 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    arg = (*tab).arg;
    while !arg.is_null() {
        (*dca).na += 1;
        (*dca).na;
        arg = (*arg).next;
    }
    (*dca)
        .arg = glp_alloc(
        1 as libc::c_int + (*dca).na,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong as libc::c_int,
    ) as *mut *mut libc::c_char;
    k = 1 as libc::c_int;
    while k <= (*dca).na {
        let ref mut fresh11 = *((*dca).arg).offset(k as isize);
        *fresh11 = 0 as *mut libc::c_char;
        k += 1;
        k;
    }
    k = 0 as libc::c_int;
    arg = (*tab).arg;
    while !arg.is_null() {
        let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
        k += 1;
        k;
        ((*(*arg).code).type_0 == 124 as libc::c_int
            || {
                glp_assert_(
                    b"arg->code->type == A_SYMBOLIC\0" as *const u8
                        as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    5113 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        sym = _glp_mpl_eval_symbolic(mpl, (*arg).code);
        if ((*sym).str_0).is_null() {
            sprintf(
                buf.as_mut_ptr(),
                b"%.*g\0" as *const u8 as *const libc::c_char,
                15 as libc::c_int,
                (*sym).num,
            );
        } else {
            _glp_mpl_fetch_string(mpl, (*sym).str_0, buf.as_mut_ptr());
        }
        _glp_mpl_delete_symbol(mpl, sym);
        let ref mut fresh12 = *((*dca).arg).offset(k as isize);
        *fresh12 = glp_alloc(
            1 as libc::c_int,
            (strlen(buf.as_mut_ptr())).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
        ) as *mut libc::c_char;
        strcpy(*((*dca).arg).offset(k as isize), buf.as_mut_ptr());
        arg = (*arg).next;
    }
    match (*tab).type_0 {
        112 => {
            current_block = 11404888101622655496;
        }
        119 => {
            ((*dca).nf == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"dca->nf == 0\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        5265 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            out = (*tab).u.out.list;
            while !out.is_null() {
                (*dca).nf += 1;
                (*dca).nf;
                out = (*out).next;
            }
            (*dca)
                .name = glp_alloc(
                1 as libc::c_int + (*dca).nf,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                    as libc::c_int,
            ) as *mut *mut libc::c_char;
            (*dca)
                .type_0 = glp_alloc(
                1 as libc::c_int + (*dca).nf,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_int;
            (*dca)
                .num = glp_alloc(
                1 as libc::c_int + (*dca).nf,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_double;
            (*dca)
                .str_0 = glp_alloc(
                1 as libc::c_int + (*dca).nf,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                    as libc::c_int,
            ) as *mut *mut libc::c_char;
            k = 0 as libc::c_int;
            out = (*tab).u.out.list;
            while !out.is_null() {
                k += 1;
                k;
                let ref mut fresh18 = *((*dca).name).offset(k as isize);
                *fresh18 = (*out).name;
                *((*dca).type_0).offset(k as isize) = '?' as i32;
                *((*dca).num).offset(k as isize) = 0.0f64;
                let ref mut fresh19 = *((*dca).str_0).offset(k as isize);
                *fresh19 = glp_alloc(
                    1 as libc::c_int,
                    100 as libc::c_int + 1 as libc::c_int,
                ) as *mut libc::c_char;
                *(*((*dca).str_0).offset(k as isize))
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                out = (*out).next;
            }
            _glp_mpl_tab_drv_open(mpl, 'W' as i32);
            _glp_mpl_loop_within_domain(
                mpl,
                (*tab).u.out.domain,
                tab as *mut libc::c_void,
                Some(
                    write_func
                        as unsafe extern "C" fn(
                            *mut MPL,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            _glp_mpl_tab_drv_close(mpl);
            current_block = 10722830739387830766;
        }
        _ => {
            (tab != tab
                || {
                    glp_assert_(
                        b"tab != tab\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        5127 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            current_block = 11404888101622655496;
        }
    }
    match current_block {
        11404888101622655496 => {
            set = (*tab).u.in_0.set;
            if !set.is_null() {
                if (*set).data != 0 {
                    _glp_mpl_error(
                        mpl,
                        b"%s already provided with data\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*set).name,
                    );
                }
                (((*(*set).array).head).is_null()
                    || {
                        glp_assert_(
                            b"set->array->head == NULL\0" as *const u8
                                as *const libc::c_char,
                            b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                            5137 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                let ref mut fresh13 = (*_glp_mpl_add_member(
                    mpl,
                    (*set).array,
                    0 as *mut TUPLE,
                ))
                    .value
                    .set;
                *fresh13 = _glp_mpl_create_elemset(mpl, (*set).dimen);
                (*set).data = 1 as libc::c_int;
            }
            in_0 = (*tab).u.in_0.list;
            while !in_0.is_null() {
                if (*(*in_0).par).data != 0 {
                    _glp_mpl_error(
                        mpl,
                        b"%s already provided with data\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*(*in_0).par).name,
                    );
                }
                (*(*in_0).par).data = 1 as libc::c_int;
                in_0 = (*in_0).next;
            }
            ((*dca).nf == 0 as libc::c_int
                || {
                    glp_assert_(
                        b"dca->nf == 0\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        5149 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            fld = (*tab).u.in_0.fld;
            while !fld.is_null() {
                (*dca).nf += 1;
                (*dca).nf;
                fld = (*fld).next;
            }
            in_0 = (*tab).u.in_0.list;
            while !in_0.is_null() {
                (*dca).nf += 1;
                (*dca).nf;
                in_0 = (*in_0).next;
            }
            (*dca)
                .name = glp_alloc(
                1 as libc::c_int + (*dca).nf,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                    as libc::c_int,
            ) as *mut *mut libc::c_char;
            (*dca)
                .type_0 = glp_alloc(
                1 as libc::c_int + (*dca).nf,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_int;
            (*dca)
                .num = glp_alloc(
                1 as libc::c_int + (*dca).nf,
                ::std::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_double;
            (*dca)
                .str_0 = glp_alloc(
                1 as libc::c_int + (*dca).nf,
                ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong
                    as libc::c_int,
            ) as *mut *mut libc::c_char;
            k = 0 as libc::c_int;
            fld = (*tab).u.in_0.fld;
            while !fld.is_null() {
                k += 1;
                k;
                let ref mut fresh14 = *((*dca).name).offset(k as isize);
                *fresh14 = (*fld).name;
                *((*dca).type_0).offset(k as isize) = '?' as i32;
                *((*dca).num).offset(k as isize) = 0.0f64;
                let ref mut fresh15 = *((*dca).str_0).offset(k as isize);
                *fresh15 = glp_alloc(
                    1 as libc::c_int,
                    100 as libc::c_int + 1 as libc::c_int,
                ) as *mut libc::c_char;
                *(*((*dca).str_0).offset(k as isize))
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                fld = (*fld).next;
            }
            in_0 = (*tab).u.in_0.list;
            while !in_0.is_null() {
                k += 1;
                k;
                let ref mut fresh16 = *((*dca).name).offset(k as isize);
                *fresh16 = (*in_0).name;
                *((*dca).type_0).offset(k as isize) = '?' as i32;
                *((*dca).num).offset(k as isize) = 0.0f64;
                let ref mut fresh17 = *((*dca).str_0).offset(k as isize);
                *fresh17 = glp_alloc(
                    1 as libc::c_int,
                    100 as libc::c_int + 1 as libc::c_int,
                ) as *mut libc::c_char;
                *(*((*dca).str_0).offset(k as isize))
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                in_0 = (*in_0).next;
            }
            _glp_mpl_tab_drv_open(mpl, 'R' as i32);
            loop {
                let mut tup: *mut TUPLE = 0 as *mut TUPLE;
                k = 1 as libc::c_int;
                while k <= (*dca).nf {
                    *((*dca).type_0).offset(k as isize) = '?' as i32;
                    k += 1;
                    k;
                }
                if _glp_mpl_tab_drv_read(mpl) != 0 {
                    break;
                }
                k = 1 as libc::c_int;
                while k <= (*dca).nf {
                    if *((*dca).type_0).offset(k as isize) == '?' as i32 {
                        _glp_mpl_error(
                            mpl,
                            b"field %s missing in input table\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            *((*dca).name).offset(k as isize),
                        );
                    }
                    k += 1;
                    k;
                }
                tup = _glp_mpl_create_tuple(mpl);
                k = 0 as libc::c_int;
                fld = (*tab).u.in_0.fld;
                while !fld.is_null() {
                    k += 1;
                    k;
                    (k <= (*dca).nf
                        || {
                            glp_assert_(
                                b"k <= dca->nf\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                5196 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    match *((*dca).type_0).offset(k as isize) {
                        78 => {
                            tup = _glp_mpl_expand_tuple(
                                mpl,
                                tup,
                                _glp_mpl_create_symbol_num(
                                    mpl,
                                    *((*dca).num).offset(k as isize),
                                ),
                            );
                        }
                        83 => {
                            (strlen(*((*dca).str_0).offset(k as isize))
                                <= 100 as libc::c_int as libc::c_ulong
                                || {
                                    glp_assert_(
                                        b"strlen(dca->str[k]) <= MAX_LENGTH\0" as *const u8
                                            as *const libc::c_char,
                                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                        5203 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                            tup = _glp_mpl_expand_tuple(
                                mpl,
                                tup,
                                _glp_mpl_create_symbol_str(
                                    mpl,
                                    _glp_mpl_create_string(
                                        mpl,
                                        *((*dca).str_0).offset(k as isize),
                                    ),
                                ),
                            );
                        }
                        _ => {
                            (dca != dca
                                || {
                                    glp_assert_(
                                        b"dca != dca\0" as *const u8 as *const libc::c_char,
                                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                        5208 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                    fld = (*fld).next;
                }
                if !((*tab).u.in_0.set).is_null() {
                    _glp_mpl_check_then_add(
                        mpl,
                        (*(*(*(*tab).u.in_0.set).array).head).value.set,
                        _glp_mpl_copy_tuple(mpl, tup),
                    );
                }
                in_0 = (*tab).u.in_0.list;
                while !in_0.is_null() {
                    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
                    k += 1;
                    k;
                    (k <= (*dca).nf
                        || {
                            glp_assert_(
                                b"k <= dca->nf\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                5219 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if !(_glp_mpl_find_member(mpl, (*(*in_0).par).array, tup)).is_null()
                    {
                        _glp_mpl_error(
                            mpl,
                            b"%s%s already defined\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*(*in_0).par).name,
                            _glp_mpl_format_tuple(mpl, '[' as i32, tup),
                        );
                    }
                    memb = _glp_mpl_add_member(
                        mpl,
                        (*(*in_0).par).array,
                        _glp_mpl_copy_tuple(mpl, tup),
                    );
                    match (*(*in_0).par).type_0 {
                        118 | 113 | 101 => {
                            if *((*dca).type_0).offset(k as isize) != 'N' as i32 {
                                _glp_mpl_error(
                                    mpl,
                                    b"%s requires numeric data\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    (*(*in_0).par).name,
                                );
                            }
                            (*memb).value.num = *((*dca).num).offset(k as isize);
                        }
                        124 => {
                            match *((*dca).type_0).offset(k as isize) {
                                78 => {
                                    (*memb)
                                        .value
                                        .sym = _glp_mpl_create_symbol_num(
                                        mpl,
                                        *((*dca).num).offset(k as isize),
                                    );
                                }
                                83 => {
                                    (strlen(*((*dca).str_0).offset(k as isize))
                                        <= 100 as libc::c_int as libc::c_ulong
                                        || {
                                            glp_assert_(
                                                b"strlen(dca->str[k]) <= MAX_LENGTH\0" as *const u8
                                                    as *const libc::c_char,
                                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                                5244 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                    (*memb)
                                        .value
                                        .sym = _glp_mpl_create_symbol_str(
                                        mpl,
                                        _glp_mpl_create_string(
                                            mpl,
                                            *((*dca).str_0).offset(k as isize),
                                        ),
                                    );
                                }
                                _ => {
                                    (dca != dca
                                        || {
                                            glp_assert_(
                                                b"dca != dca\0" as *const u8 as *const libc::c_char,
                                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                                5249 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                }
                            }
                        }
                        _ => {
                            (in_0 != in_0
                                || {
                                    glp_assert_(
                                        b"in != in\0" as *const u8 as *const libc::c_char,
                                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                        5253 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                    in_0 = (*in_0).next;
                }
                _glp_mpl_delete_tuple(mpl, tup);
            }
            _glp_mpl_tab_drv_close(mpl);
        }
        _ => {}
    }
    _glp_mpl_free_dca(mpl);
}
pub unsafe extern "C" fn _glp_mpl_free_dca(mut mpl: *mut MPL) {
    let mut dca: *mut TABDCA = (*mpl).dca;
    let mut k: libc::c_int = 0;
    if !dca.is_null() {
        if !((*dca).link).is_null() {
            _glp_mpl_tab_drv_close(mpl);
        }
        if !((*dca).arg).is_null() {
            k = 1 as libc::c_int;
            while k <= (*dca).na {
                if !(*((*dca).arg).offset(k as isize)).is_null() {
                    glp_free(*((*dca).arg).offset(k as isize) as *mut libc::c_void);
                }
                k += 1;
                k;
            }
            glp_free((*dca).arg as *mut libc::c_void);
        }
        if !((*dca).name).is_null() {
            glp_free((*dca).name as *mut libc::c_void);
        }
        if !((*dca).type_0).is_null() {
            glp_free((*dca).type_0 as *mut libc::c_void);
        }
        if !((*dca).num).is_null() {
            glp_free((*dca).num as *mut libc::c_void);
        }
        if !((*dca).str_0).is_null() {
            k = 1 as libc::c_int;
            while k <= (*dca).nf {
                glp_free(*((*dca).str_0).offset(k as isize) as *mut libc::c_void);
                k += 1;
                k;
            }
            glp_free((*dca).str_0 as *mut libc::c_void);
        }
        glp_free(dca as *mut libc::c_void);
        (*mpl).dca = 0 as *mut TABDCA;
    }
}
pub unsafe extern "C" fn _glp_mpl_clean_table(mut mpl: *mut MPL, mut tab: *mut TABLE) {
    let mut arg: *mut TABARG = 0 as *mut TABARG;
    let mut out: *mut TABOUT = 0 as *mut TABOUT;
    arg = (*tab).arg;
    while !arg.is_null() {
        _glp_mpl_clean_code(mpl, (*arg).code);
        arg = (*arg).next;
    }
    match (*tab).type_0 {
        112 => {}
        119 => {
            _glp_mpl_clean_domain(mpl, (*tab).u.out.domain);
            out = (*tab).u.out.list;
            while !out.is_null() {
                _glp_mpl_clean_code(mpl, (*out).code);
                out = (*out).next;
            }
        }
        _ => {
            (tab != tab
                || {
                    glp_assert_(
                        b"tab != tab\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        5338 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    };
}
unsafe extern "C" fn check_func(
    mut mpl: *mut MPL,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut chk: *mut CHECK = info as *mut CHECK;
    if _glp_mpl_eval_logical(mpl, (*chk).code) == 0 {
        _glp_mpl_error(
            mpl,
            b"check%s failed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            _glp_mpl_format_tuple(
                mpl,
                '[' as i32,
                _glp_mpl_get_domain_tuple(mpl, (*chk).domain),
            ),
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_execute_check(mut mpl: *mut MPL, mut chk: *mut CHECK) {
    _glp_mpl_loop_within_domain(
        mpl,
        (*chk).domain,
        chk as *mut libc::c_void,
        Some(
            check_func
                as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn _glp_mpl_clean_check(mut mpl: *mut MPL, mut chk: *mut CHECK) {
    _glp_mpl_clean_domain(mpl, (*chk).domain);
    _glp_mpl_clean_code(mpl, (*chk).code);
}
unsafe extern "C" fn display_set(
    mut mpl: *mut MPL,
    mut set: *mut SET,
    mut memb: *mut MEMBER,
) {
    let mut s: *mut ELEMSET = (*memb).value.set;
    let mut m: *mut MEMBER = 0 as *mut MEMBER;
    _glp_mpl_write_text(
        mpl,
        b"%s%s%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*set).name,
        _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
        if ((*s).head).is_null() {
            b" is empty\0" as *const u8 as *const libc::c_char
        } else {
            b":\0" as *const u8 as *const libc::c_char
        },
    );
    m = (*s).head;
    while !m.is_null() {
        _glp_mpl_write_text(
            mpl,
            b"   %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            _glp_mpl_format_tuple(mpl, '(' as i32, (*m).tuple),
        );
        m = (*m).next;
    }
}
unsafe extern "C" fn display_par(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut memb: *mut MEMBER,
) {
    match (*par).type_0 {
        118 | 113 | 101 => {
            _glp_mpl_write_text(
                mpl,
                b"%s%s = %.*g\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*par).name,
                _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
                15 as libc::c_int,
                (*memb).value.num,
            );
        }
        124 => {
            _glp_mpl_write_text(
                mpl,
                b"%s%s = %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*par).name,
                _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
                _glp_mpl_format_symbol(mpl, (*memb).value.sym),
            );
        }
        _ => {
            (par != par
                || {
                    glp_assert_(
                        b"par != par\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        5414 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    };
}
unsafe extern "C" fn display_var(
    mut mpl: *mut MPL,
    mut var: *mut VARIABLE,
    mut memb: *mut MEMBER,
    mut suff: libc::c_int,
) {
    if suff == 0 as libc::c_int || suff == 0x4 as libc::c_int {
        _glp_mpl_write_text(
            mpl,
            b"%s%s.val = %.*g\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*var).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
            15 as libc::c_int,
            (*(*memb).value.var).prim,
        );
    } else if suff == 0x1 as libc::c_int {
        _glp_mpl_write_text(
            mpl,
            b"%s%s.lb = %.*g\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*var).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
            15 as libc::c_int,
            if ((*(*(*memb).value.var).var).lbnd).is_null() {
                -1.7976931348623157e+308f64
            } else {
                (*(*memb).value.var).lbnd
            },
        );
    } else if suff == 0x2 as libc::c_int {
        _glp_mpl_write_text(
            mpl,
            b"%s%s.ub = %.*g\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*var).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
            15 as libc::c_int,
            if ((*(*(*memb).value.var).var).ubnd).is_null() {
                1.7976931348623157e+308f64
            } else {
                (*(*memb).value.var).ubnd
            },
        );
    } else if suff == 0x3 as libc::c_int {
        _glp_mpl_write_text(
            mpl,
            b"%s%s.status = %d\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*var).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
            (*(*memb).value.var).stat,
        );
    } else if suff == 0x5 as libc::c_int {
        _glp_mpl_write_text(
            mpl,
            b"%s%s.dual = %.*g\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*var).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
            15 as libc::c_int,
            (*(*memb).value.var).dual,
        );
    } else {
        (suff != suff
            || {
                glp_assert_(
                    b"suff != suff\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    5445 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    };
}
unsafe extern "C" fn display_con(
    mut mpl: *mut MPL,
    mut con: *mut CONSTRAINT,
    mut memb: *mut MEMBER,
    mut suff: libc::c_int,
) {
    if suff == 0 as libc::c_int || suff == 0x4 as libc::c_int {
        _glp_mpl_write_text(
            mpl,
            b"%s%s.val = %.*g\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*con).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
            15 as libc::c_int,
            (*(*memb).value.con).prim,
        );
    } else if suff == 0x1 as libc::c_int {
        _glp_mpl_write_text(
            mpl,
            b"%s%s.lb = %.*g\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*con).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
            15 as libc::c_int,
            if ((*(*(*memb).value.con).con).lbnd).is_null() {
                -1.7976931348623157e+308f64
            } else {
                (*(*memb).value.con).lbnd
            },
        );
    } else if suff == 0x2 as libc::c_int {
        _glp_mpl_write_text(
            mpl,
            b"%s%s.ub = %.*g\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*con).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
            15 as libc::c_int,
            if ((*(*(*memb).value.con).con).ubnd).is_null() {
                1.7976931348623157e+308f64
            } else {
                (*(*memb).value.con).ubnd
            },
        );
    } else if suff == 0x3 as libc::c_int {
        _glp_mpl_write_text(
            mpl,
            b"%s%s.status = %d\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*con).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
            (*(*memb).value.con).stat,
        );
    } else if suff == 0x5 as libc::c_int {
        _glp_mpl_write_text(
            mpl,
            b"%s%s.dual = %.*g\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*con).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, (*memb).tuple),
            15 as libc::c_int,
            (*(*memb).value.con).dual,
        );
    } else {
        (suff != suff
            || {
                glp_assert_(
                    b"suff != suff\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                    5476 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    };
}
unsafe extern "C" fn display_memb(mut mpl: *mut MPL, mut code: *mut CODE) {
    let mut memb: MEMBER = MEMBER {
        tuple: 0 as *mut TUPLE,
        next: 0 as *mut MEMBER,
        value: VALUE {
            none: 0 as *mut libc::c_void,
        },
    };
    let mut e: *mut ARG_LIST = 0 as *mut ARG_LIST;
    ((*code).op == 304 as libc::c_int || (*code).op == 305 as libc::c_int
        || (*code).op == 306 as libc::c_int || (*code).op == 307 as libc::c_int
        || (*code).op == 308 as libc::c_int
        || {
            glp_assert_(
                b"code->op == O_MEMNUM || code->op == O_MEMSYM || code->op == O_MEMSET || code->op == O_MEMVAR || code->op == O_MEMCON\0"
                    as *const u8 as *const libc::c_char,
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5487 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    memb.tuple = _glp_mpl_create_tuple(mpl);
    e = (*code).arg.par.list;
    while !e.is_null() {
        memb
            .tuple = _glp_mpl_expand_tuple(
            mpl,
            memb.tuple,
            _glp_mpl_eval_symbolic(mpl, (*e).x),
        );
        e = (*e).next;
    }
    match (*code).op {
        304 => {
            memb
                .value
                .num = _glp_mpl_eval_member_num(mpl, (*code).arg.par.par, memb.tuple);
            display_par(mpl, (*code).arg.par.par, &mut memb);
        }
        305 => {
            memb
                .value
                .sym = _glp_mpl_eval_member_sym(mpl, (*code).arg.par.par, memb.tuple);
            display_par(mpl, (*code).arg.par.par, &mut memb);
            _glp_mpl_delete_symbol(mpl, memb.value.sym);
        }
        306 => {
            memb
                .value
                .set = _glp_mpl_eval_member_set(mpl, (*code).arg.set.set, memb.tuple);
            display_set(mpl, (*code).arg.set.set, &mut memb);
        }
        307 => {
            memb
                .value
                .var = _glp_mpl_eval_member_var(mpl, (*code).arg.var.var, memb.tuple);
            display_var(mpl, (*code).arg.var.var, &mut memb, (*code).arg.var.suff);
        }
        308 => {
            memb
                .value
                .con = _glp_mpl_eval_member_con(mpl, (*code).arg.con.con, memb.tuple);
            display_con(mpl, (*code).arg.con.con, &mut memb, (*code).arg.con.suff);
        }
        _ => {
            (code != code
                || {
                    glp_assert_(
                        b"code != code\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        5522 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    _glp_mpl_delete_tuple(mpl, memb.tuple);
}
unsafe extern "C" fn display_code(mut mpl: *mut MPL, mut code: *mut CODE) {
    match (*code).type_0 {
        118 => {
            let mut num: libc::c_double = 0.;
            num = _glp_mpl_eval_numeric(mpl, code);
            _glp_mpl_write_text(
                mpl,
                b"%.*g\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                15 as libc::c_int,
                num,
            );
        }
        124 => {
            let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
            sym = _glp_mpl_eval_symbolic(mpl, code);
            _glp_mpl_write_text(
                mpl,
                b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                _glp_mpl_format_symbol(mpl, sym),
            );
            _glp_mpl_delete_symbol(mpl, sym);
        }
        114 => {
            let mut bit: libc::c_int = 0;
            bit = _glp_mpl_eval_logical(mpl, code);
            _glp_mpl_write_text(
                mpl,
                b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                if bit != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
        }
        126 => {
            let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
            tuple = _glp_mpl_eval_tuple(mpl, code);
            _glp_mpl_write_text(
                mpl,
                b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                _glp_mpl_format_tuple(mpl, '(' as i32, tuple),
            );
            _glp_mpl_delete_tuple(mpl, tuple);
        }
        106 => {
            let mut set: *mut ELEMSET = 0 as *mut ELEMSET;
            let mut memb: *mut MEMBER = 0 as *mut MEMBER;
            set = _glp_mpl_eval_elemset(mpl, code);
            if ((*set).head).is_null() {
                _glp_mpl_write_text(
                    mpl,
                    b"set is empty\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            memb = (*set).head;
            while !memb.is_null() {
                _glp_mpl_write_text(
                    mpl,
                    b"   %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    _glp_mpl_format_tuple(mpl, '(' as i32, (*memb).tuple),
                );
                memb = (*memb).next;
            }
            _glp_mpl_delete_elemset(mpl, set);
        }
        110 => {
            let mut form: *mut FORMULA = 0 as *mut FORMULA;
            let mut term: *mut FORMULA = 0 as *mut FORMULA;
            form = _glp_mpl_eval_formula(mpl, code);
            if form.is_null() {
                _glp_mpl_write_text(
                    mpl,
                    b"linear form is empty\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            term = form;
            while !term.is_null() {
                if ((*term).var).is_null() {
                    _glp_mpl_write_text(
                        mpl,
                        b"   %.*g\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (*term).coef,
                    );
                } else {
                    _glp_mpl_write_text(
                        mpl,
                        b"   %.*g %s%s\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        15 as libc::c_int,
                        (*term).coef,
                        (*(*(*term).var).var).name,
                        _glp_mpl_format_tuple(
                            mpl,
                            '[' as i32,
                            (*(*(*term).var).memb).tuple,
                        ),
                    );
                }
                term = (*term).next;
            }
            _glp_mpl_delete_formula(mpl, form);
        }
        _ => {
            (code != code
                || {
                    glp_assert_(
                        b"code != code\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        5592 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    };
}
unsafe extern "C" fn display_func(
    mut mpl: *mut MPL,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut dpy: *mut DISPLAY = info as *mut DISPLAY;
    let mut entry: *mut DISPLAY1 = 0 as *mut DISPLAY1;
    entry = (*dpy).list;
    while !entry.is_null() {
        if (*entry).type_0 == 111 as libc::c_int {
            let mut slot: *mut DOMAIN_SLOT = (*entry).u.slot;
            _glp_mpl_write_text(
                mpl,
                b"%s = %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*slot).name,
                _glp_mpl_format_symbol(mpl, (*slot).value),
            );
        } else if (*entry).type_0 == 122 as libc::c_int {
            let mut set: *mut SET = (*entry).u.set;
            let mut memb: *mut MEMBER = 0 as *mut MEMBER;
            if !((*set).assign).is_null() {
                _glp_mpl_eval_whole_set(mpl, set);
            } else {
                if !((*set).gadget).is_null() && (*set).data == 0 as libc::c_int {
                    saturate_set(mpl, set);
                }
                if !((*(*set).array).head).is_null() {
                    _glp_mpl_eval_member_set(mpl, set, (*(*(*set).array).head).tuple);
                }
            }
            if ((*(*set).array).head).is_null() {
                _glp_mpl_write_text(
                    mpl,
                    b"%s has empty content\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*set).name,
                );
            }
            memb = (*(*set).array).head;
            while !memb.is_null() {
                display_set(mpl, set, memb);
                memb = (*memb).next;
            }
        } else if (*entry).type_0 == 120 as libc::c_int {
            let mut par: *mut PARAMETER = (*entry).u.par;
            let mut memb_0: *mut MEMBER = 0 as *mut MEMBER;
            if !((*par).assign).is_null() {
                _glp_mpl_eval_whole_par(mpl, par);
            } else if !((*(*par).array).head).is_null() {
                if (*par).type_0 != 124 as libc::c_int {
                    _glp_mpl_eval_member_num(mpl, par, (*(*(*par).array).head).tuple);
                } else {
                    _glp_mpl_delete_symbol(
                        mpl,
                        _glp_mpl_eval_member_sym(mpl, par, (*(*(*par).array).head).tuple),
                    );
                }
            }
            if ((*(*par).array).head).is_null() {
                _glp_mpl_write_text(
                    mpl,
                    b"%s has empty content\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*par).name,
                );
            }
            memb_0 = (*(*par).array).head;
            while !memb_0.is_null() {
                display_par(mpl, par, memb_0);
                memb_0 = (*memb_0).next;
            }
        } else if (*entry).type_0 == 127 as libc::c_int {
            let mut var: *mut VARIABLE = (*entry).u.var;
            let mut memb_1: *mut MEMBER = 0 as *mut MEMBER;
            ((*mpl).flag_p != 0
                || {
                    glp_assert_(
                        b"mpl->flag_p\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        5667 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if ((*(*var).array).head).is_null() {
                _glp_mpl_write_text(
                    mpl,
                    b"%s has empty content\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*var).name,
                );
            }
            memb_1 = (*(*var).array).head;
            while !memb_1.is_null() {
                display_var(mpl, var, memb_1, 0 as libc::c_int);
                memb_1 = (*memb_1).next;
            }
        } else if (*entry).type_0 == 103 as libc::c_int {
            let mut con: *mut CONSTRAINT = (*entry).u.con;
            let mut memb_2: *mut MEMBER = 0 as *mut MEMBER;
            ((*mpl).flag_p != 0
                || {
                    glp_assert_(
                        b"mpl->flag_p\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        5678 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            if ((*(*con).array).head).is_null() {
                _glp_mpl_write_text(
                    mpl,
                    b"%s has empty content\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*con).name,
                );
            }
            memb_2 = (*(*con).array).head;
            while !memb_2.is_null() {
                display_con(mpl, con, memb_2, 0 as libc::c_int);
                memb_2 = (*memb_2).next;
            }
        } else if (*entry).type_0 == 108 as libc::c_int {
            let mut code: *mut CODE = (*entry).u.code;
            if (*code).op == 304 as libc::c_int || (*code).op == 305 as libc::c_int
                || (*code).op == 306 as libc::c_int || (*code).op == 307 as libc::c_int
                || (*code).op == 308 as libc::c_int
            {
                display_memb(mpl, code);
            } else {
                display_code(mpl, code);
            }
        } else {
            (entry != entry
                || {
                    glp_assert_(
                        b"entry != entry\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        5696 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
        entry = (*entry).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_execute_display(
    mut mpl: *mut MPL,
    mut dpy: *mut DISPLAY,
) {
    _glp_mpl_loop_within_domain(
        mpl,
        (*dpy).domain,
        dpy as *mut libc::c_void,
        Some(
            display_func
                as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn _glp_mpl_clean_display(
    mut mpl: *mut MPL,
    mut dpy: *mut DISPLAY,
) {
    let mut d: *mut DISPLAY1 = 0 as *mut DISPLAY1;
    _glp_mpl_clean_domain(mpl, (*dpy).domain);
    d = (*dpy).list;
    while !d.is_null() {
        if (*d).type_0 == 108 as libc::c_int {
            _glp_mpl_clean_code(mpl, (*d).u.code);
        }
        d = (*d).next;
    }
}
unsafe extern "C" fn print_char(mut mpl: *mut MPL, mut c: libc::c_int) {
    if ((*mpl).prt_fp).is_null() {
        _glp_mpl_write_char(mpl, c);
    } else {
        let mut buf: [libc::c_uchar; 1] = [0; 1];
        buf[0 as libc::c_int as usize] = c as libc::c_uchar;
        _glp_write(
            (*mpl).prt_fp,
            buf.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int,
        );
    };
}
unsafe extern "C" fn print_text(
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
                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                5759 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    c = buf.as_mut_ptr();
    while *c as libc::c_int != '\0' as i32 {
        print_char(mpl, *c as libc::c_int);
        c = c.offset(1);
        c;
    }
}
unsafe extern "C" fn printf_func(
    mut mpl: *mut MPL,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut prt: *mut PRINTF = info as *mut PRINTF;
    let mut entry: *mut PRINTF1 = 0 as *mut PRINTF1;
    let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut fmt: [libc::c_char; 101] = [0; 101];
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: libc::c_char = 0;
    sym = _glp_mpl_eval_symbolic(mpl, (*prt).fmt);
    if ((*sym).str_0).is_null() {
        sprintf(
            fmt.as_mut_ptr(),
            b"%.*g\0" as *const u8 as *const libc::c_char,
            15 as libc::c_int,
            (*sym).num,
        );
    } else {
        _glp_mpl_fetch_string(mpl, (*sym).str_0, fmt.as_mut_ptr());
    }
    _glp_mpl_delete_symbol(mpl, sym);
    entry = (*prt).list;
    c = fmt.as_mut_ptr();
    while *c as libc::c_int != '\0' as i32 {
        if *c as libc::c_int == '%' as i32 {
            let fresh20 = c;
            c = c.offset(1);
            from = fresh20;
            if *c as libc::c_int == '%' as i32 {
                print_char(mpl, '%' as i32);
            } else {
                if entry.is_null() {
                    break;
                }
                while *c as libc::c_int == '-' as i32 || *c as libc::c_int == '+' as i32
                    || *c as libc::c_int == ' ' as i32 || *c as libc::c_int == '#' as i32
                    || *c as libc::c_int == '0' as i32
                {
                    c = c.offset(1);
                    c;
                }
                while *(*__ctype_b_loc())
                    .offset(*c as libc::c_uchar as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    c = c.offset(1);
                    c;
                }
                if *c as libc::c_int == '.' as i32 {
                    c = c.offset(1);
                    c;
                    while *(*__ctype_b_loc())
                        .offset(*c as libc::c_uchar as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    {
                        c = c.offset(1);
                        c;
                    }
                }
                save = *c.offset(1 as libc::c_int as isize);
                *c.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                if *c as libc::c_int == 'd' as i32 || *c as libc::c_int == 'i' as i32
                    || *c as libc::c_int == 'e' as i32 || *c as libc::c_int == 'E' as i32
                    || *c as libc::c_int == 'f' as i32 || *c as libc::c_int == 'F' as i32
                    || *c as libc::c_int == 'g' as i32 || *c as libc::c_int == 'G' as i32
                {
                    let mut value: libc::c_double = 0.;
                    (!entry.is_null()
                        || {
                            glp_assert_(
                                b"entry != NULL\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                5806 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    match (*(*entry).code).type_0 {
                        118 => {
                            value = _glp_mpl_eval_numeric(mpl, (*entry).code);
                        }
                        124 => {
                            sym = _glp_mpl_eval_symbolic(mpl, (*entry).code);
                            if !((*sym).str_0).is_null() {
                                _glp_mpl_error(
                                    mpl,
                                    b"cannot convert %s to floating-point number\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    _glp_mpl_format_symbol(mpl, sym),
                                );
                            }
                            value = (*sym).num;
                            _glp_mpl_delete_symbol(mpl, sym);
                        }
                        114 => {
                            if _glp_mpl_eval_logical(mpl, (*entry).code) != 0 {
                                value = 1.0f64;
                            } else {
                                value = 0.0f64;
                            }
                        }
                        _ => {
                            (entry != entry
                                || {
                                    glp_assert_(
                                        b"entry != entry\0" as *const u8 as *const libc::c_char,
                                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                        5826 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                    if *c as libc::c_int == 'd' as i32 || *c as libc::c_int == 'i' as i32
                    {
                        let mut int_max: libc::c_double = 2147483647 as libc::c_int
                            as libc::c_double;
                        if !(-int_max <= value && value <= int_max) {
                            _glp_mpl_error(
                                mpl,
                                b"cannot convert %.*g to integer\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                15 as libc::c_int,
                                value,
                            );
                        }
                        print_text(mpl, from, floor(value + 0.5f64) as libc::c_int);
                    } else {
                        print_text(mpl, from, value);
                    }
                } else if *c as libc::c_int == 's' as i32 {
                    let mut value_0: [libc::c_char; 101] = [0; 101];
                    match (*(*entry).code).type_0 {
                        118 => {
                            sprintf(
                                value_0.as_mut_ptr(),
                                b"%.*g\0" as *const u8 as *const libc::c_char,
                                15 as libc::c_int,
                                _glp_mpl_eval_numeric(mpl, (*entry).code),
                            );
                        }
                        114 => {
                            if _glp_mpl_eval_logical(mpl, (*entry).code) != 0 {
                                strcpy(
                                    value_0.as_mut_ptr(),
                                    b"T\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                strcpy(
                                    value_0.as_mut_ptr(),
                                    b"F\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                        124 => {
                            sym = _glp_mpl_eval_symbolic(mpl, (*entry).code);
                            if ((*sym).str_0).is_null() {
                                sprintf(
                                    value_0.as_mut_ptr(),
                                    b"%.*g\0" as *const u8 as *const libc::c_char,
                                    15 as libc::c_int,
                                    (*sym).num,
                                );
                            } else {
                                _glp_mpl_fetch_string(
                                    mpl,
                                    (*sym).str_0,
                                    value_0.as_mut_ptr(),
                                );
                            }
                            _glp_mpl_delete_symbol(mpl, sym);
                        }
                        _ => {
                            (entry != entry
                                || {
                                    glp_assert_(
                                        b"entry != entry\0" as *const u8 as *const libc::c_char,
                                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                        5861 as libc::c_int,
                                    );
                                    1 as libc::c_int != 0
                                }) as libc::c_int;
                        }
                    }
                    print_text(mpl, from, value_0.as_mut_ptr());
                } else {
                    _glp_mpl_error(
                        mpl,
                        b"format specifier missing or invalid\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                *c.offset(1 as libc::c_int as isize) = save;
                entry = (*entry).next;
            }
        } else if *c as libc::c_int == '\\' as i32 {
            c = c.offset(1);
            c;
            if *c as libc::c_int == 't' as i32 {
                print_char(mpl, '\t' as i32);
            } else if *c as libc::c_int == 'n' as i32 {
                print_char(mpl, '\n' as i32);
            } else if *c as libc::c_int == '\0' as i32 {
                _glp_mpl_error(
                    mpl,
                    b"invalid use of escape character \\ in format control string\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                print_char(mpl, *c as libc::c_int);
            }
        } else {
            print_char(mpl, *c as libc::c_int);
        }
        c = c.offset(1);
        c;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_execute_printf(
    mut mpl: *mut MPL,
    mut prt: *mut PRINTF,
) {
    if ((*prt).fname).is_null() {
        if !((*mpl).prt_fp).is_null() {
            _glp_close((*mpl).prt_fp);
            (*mpl).prt_fp = 0 as *mut glp_file;
            glp_free((*mpl).prt_file as *mut libc::c_void);
            (*mpl).prt_file = 0 as *mut libc::c_char;
        }
    } else {
        let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
        let mut fname: [libc::c_char; 101] = [0; 101];
        sym = _glp_mpl_eval_symbolic(mpl, (*prt).fname);
        if ((*sym).str_0).is_null() {
            sprintf(
                fname.as_mut_ptr(),
                b"%.*g\0" as *const u8 as *const libc::c_char,
                15 as libc::c_int,
                (*sym).num,
            );
        } else {
            _glp_mpl_fetch_string(mpl, (*sym).str_0, fname.as_mut_ptr());
        }
        _glp_mpl_delete_symbol(mpl, sym);
        if !((*mpl).prt_fp).is_null()
            && ((*prt).app == 0
                || strcmp((*mpl).prt_file, fname.as_mut_ptr()) != 0 as libc::c_int)
        {
            _glp_close((*mpl).prt_fp);
            (*mpl).prt_fp = 0 as *mut glp_file;
            glp_free((*mpl).prt_file as *mut libc::c_void);
            (*mpl).prt_file = 0 as *mut libc::c_char;
        }
        if ((*mpl).prt_fp).is_null() {
            (*mpl)
                .prt_fp = _glp_open(
                fname.as_mut_ptr(),
                if (*prt).app != 0 {
                    b"a\0" as *const u8 as *const libc::c_char
                } else {
                    b"w\0" as *const u8 as *const libc::c_char
                },
            );
            if ((*mpl).prt_fp).is_null() {
                _glp_mpl_error(
                    mpl,
                    b"unable to open '%s' for writing - %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    fname.as_mut_ptr(),
                    _glp_get_err_msg(),
                );
            }
            (*mpl)
                .prt_file = glp_alloc(
                1 as libc::c_int,
                (strlen(fname.as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            ) as *mut libc::c_char;
            strcpy((*mpl).prt_file, fname.as_mut_ptr());
        }
    }
    _glp_mpl_loop_within_domain(
        mpl,
        (*prt).domain,
        prt as *mut libc::c_void,
        Some(
            printf_func
                as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> libc::c_int,
        ),
    );
    if !((*mpl).prt_fp).is_null() {
        if _glp_ioerr((*mpl).prt_fp) != 0 {
            _glp_mpl_error(
                mpl,
                b"writing error to '%s' - %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*mpl).prt_file,
                _glp_get_err_msg(),
            );
        }
    }
}
pub unsafe extern "C" fn _glp_mpl_clean_printf(mut mpl: *mut MPL, mut prt: *mut PRINTF) {
    let mut p: *mut PRINTF1 = 0 as *mut PRINTF1;
    _glp_mpl_clean_domain(mpl, (*prt).domain);
    _glp_mpl_clean_code(mpl, (*prt).fmt);
    p = (*prt).list;
    while !p.is_null() {
        _glp_mpl_clean_code(mpl, (*p).code);
        p = (*p).next;
    }
    _glp_mpl_clean_code(mpl, (*prt).fname);
}
unsafe extern "C" fn for_func(
    mut mpl: *mut MPL,
    mut info: *mut libc::c_void,
) -> libc::c_int {
    let mut fur: *mut FOR = info as *mut FOR;
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    let mut save: *mut STATEMENT = 0 as *mut STATEMENT;
    save = (*mpl).stmt;
    stmt = (*fur).list;
    while !stmt.is_null() {
        _glp_mpl_execute_statement(mpl, stmt);
        stmt = (*stmt).next;
    }
    (*mpl).stmt = save;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_execute_for(mut mpl: *mut MPL, mut fur: *mut FOR) {
    _glp_mpl_loop_within_domain(
        mpl,
        (*fur).domain,
        fur as *mut libc::c_void,
        Some(
            for_func as unsafe extern "C" fn(*mut MPL, *mut libc::c_void) -> libc::c_int,
        ),
    );
}
pub unsafe extern "C" fn _glp_mpl_clean_for(mut mpl: *mut MPL, mut fur: *mut FOR) {
    let mut stmt: *mut STATEMENT = 0 as *mut STATEMENT;
    _glp_mpl_clean_domain(mpl, (*fur).domain);
    stmt = (*fur).list;
    while !stmt.is_null() {
        _glp_mpl_clean_statement(mpl, stmt);
        stmt = (*stmt).next;
    }
}
pub unsafe extern "C" fn _glp_mpl_execute_statement(
    mut mpl: *mut MPL,
    mut stmt: *mut STATEMENT,
) {
    (*mpl).stmt = stmt;
    match (*stmt).type_0 {
        103 => {
            glp_printf(
                b"Generating %s...\n\0" as *const u8 as *const libc::c_char,
                (*(*stmt).u.con).name,
            );
            _glp_mpl_eval_whole_con(mpl, (*stmt).u.con);
        }
        125 => {
            match (*(*stmt).u.tab).type_0 {
                112 => {
                    glp_printf(
                        b"Reading %s...\n\0" as *const u8 as *const libc::c_char,
                        (*(*stmt).u.tab).name,
                    );
                }
                119 => {
                    glp_printf(
                        b"Writing %s...\n\0" as *const u8 as *const libc::c_char,
                        (*(*stmt).u.tab).name,
                    );
                }
                _ => {
                    (stmt != stmt
                        || {
                            glp_assert_(
                                b"stmt != stmt\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                                6035 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                }
            }
            _glp_mpl_execute_table(mpl, (*stmt).u.tab);
        }
        122 | 120 | 127 | 123 => {}
        102 => {
            glp_printf(
                b"Checking (line %d)...\n\0" as *const u8 as *const libc::c_char,
                (*stmt).line,
            );
            _glp_mpl_execute_check(mpl, (*stmt).u.chk);
        }
        104 => {
            _glp_mpl_write_text(
                mpl,
                b"Display statement at line %d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*stmt).line,
            );
            _glp_mpl_execute_display(mpl, (*stmt).u.dpy);
        }
        121 => {
            _glp_mpl_execute_printf(mpl, (*stmt).u.prt);
        }
        109 => {
            _glp_mpl_execute_for(mpl, (*stmt).u.fur);
        }
        _ => {
            (stmt != stmt
                || {
                    glp_assert_(
                        b"stmt != stmt\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        6057 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn _glp_mpl_clean_statement(
    mut mpl: *mut MPL,
    mut stmt: *mut STATEMENT,
) {
    match (*stmt).type_0 {
        122 => {
            _glp_mpl_clean_set(mpl, (*stmt).u.set);
        }
        120 => {
            _glp_mpl_clean_parameter(mpl, (*stmt).u.par);
        }
        127 => {
            _glp_mpl_clean_variable(mpl, (*stmt).u.var);
        }
        103 => {
            _glp_mpl_clean_constraint(mpl, (*stmt).u.con);
        }
        125 => {
            _glp_mpl_clean_table(mpl, (*stmt).u.tab);
        }
        123 => {}
        102 => {
            _glp_mpl_clean_check(mpl, (*stmt).u.chk);
        }
        104 => {
            _glp_mpl_clean_display(mpl, (*stmt).u.dpy);
        }
        121 => {
            _glp_mpl_clean_printf(mpl, (*stmt).u.prt);
        }
        109 => {
            _glp_mpl_clean_for(mpl, (*stmt).u.fur);
        }
        _ => {
            (stmt != stmt
                || {
                    glp_assert_(
                        b"stmt != stmt\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl3.c\0" as *const u8 as *const libc::c_char,
                        6093 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    };
}

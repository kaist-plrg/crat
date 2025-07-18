use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type DMP;
    pub type glp_file;
    fn _glp_avl_get_node_link(node: *mut AVLNODE) -> *mut libc::c_void;
    fn _glp_avl_get_node_type(node: *mut AVLNODE) -> libc::c_int;
    fn _glp_avl_find_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn _glp_dmp_get_atom(pool: *mut DMP, size: libc::c_int) -> *mut libc::c_void;
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: libc::c_int);
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _glp_mpl_get_token(mpl: *mut MPL);
    fn _glp_mpl_unget_token(mpl: *mut MPL);
    fn _glp_mpl_delete_symbol(mpl: *mut MPL, sym: *mut SYMBOL);
    fn _glp_mpl_create_string(mpl: *mut MPL, buf: *mut libc::c_char) -> *mut STRING;
    fn _glp_mpl_create_symbol_str(mpl: *mut MPL, str: *mut STRING) -> *mut SYMBOL;
    fn _glp_mpl_create_symbol_num(mpl: *mut MPL, num: libc::c_double) -> *mut SYMBOL;
    fn _glp_mpl_error(mpl: *mut MPL, fmt: *mut libc::c_char, _: ...);
    fn _glp_mpl_check_then_add(
        mpl: *mut MPL,
        set: *mut ELEMSET,
        tuple: *mut TUPLE,
    ) -> *mut MEMBER;
    fn _glp_mpl_expand_tuple(
        mpl: *mut MPL,
        tuple: *mut TUPLE,
        sym: *mut SYMBOL,
    ) -> *mut TUPLE;
    fn _glp_mpl_copy_symbol(mpl: *mut MPL, sym: *mut SYMBOL) -> *mut SYMBOL;
    fn _glp_mpl_format_symbol(mpl: *mut MPL, sym: *mut SYMBOL) -> *mut libc::c_char;
    fn _glp_mpl_create_tuple(mpl: *mut MPL) -> *mut TUPLE;
    fn _glp_mpl_create_elemset(mpl: *mut MPL, dim: libc::c_int) -> *mut ELEMSET;
    fn _glp_mpl_add_member(
        mpl: *mut MPL,
        array: *mut ARRAY,
        tuple: *mut TUPLE,
    ) -> *mut MEMBER;
    fn _glp_mpl_format_tuple(
        mpl: *mut MPL,
        c: libc::c_int,
        tuple: *mut TUPLE,
    ) -> *mut libc::c_char;
    fn _glp_mpl_find_member(
        mpl: *mut MPL,
        array: *mut ARRAY,
        tuple: *mut TUPLE,
    ) -> *mut MEMBER;
    fn _glp_mpl_tuple_dimen(mpl: *mut MPL, tuple: *mut TUPLE) -> libc::c_int;
    fn _glp_mpl_delete_tuple(mpl: *mut MPL, tuple: *mut TUPLE);
    fn _glp_mpl_copy_tuple(mpl: *mut MPL, tuple: *mut TUPLE) -> *mut TUPLE;
}
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
    pub index: C2RustUnnamed_5,
    pub par: C2RustUnnamed_4,
    pub set: C2RustUnnamed_3,
    pub var: C2RustUnnamed_2,
    pub con: C2RustUnnamed_1,
    pub list: *mut ARG_LIST,
    pub slice: *mut DOMAIN_BLOCK,
    pub arg: C2RustUnnamed_0,
    pub loop_0: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
pub struct C2RustUnnamed_0 {
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
pub struct C2RustUnnamed_1 {
    pub con: *mut CONSTRAINT,
    pub list: *mut ARG_LIST,
    pub suff: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
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
pub struct C2RustUnnamed_4 {
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
pub struct C2RustUnnamed_5 {
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
    pub u: C2RustUnnamed_6,
    pub next: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
    pub u: C2RustUnnamed_7,
    pub next: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub u: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub in_0: C2RustUnnamed_10,
    pub out: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct C2RustUnnamed_10 {
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
pub type SLICE = TUPLE;
pub unsafe extern "C" fn _glp_mpl_create_slice(mut mpl: *mut MPL) -> *mut SLICE {
    let mut slice: *mut SLICE = 0 as *mut SLICE;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                35 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    slice = 0 as *mut SLICE;
    return slice;
}
pub unsafe extern "C" fn _glp_mpl_expand_slice(
    mut mpl: *mut MPL,
    mut slice: *mut SLICE,
    mut sym: *mut SYMBOL,
) -> *mut SLICE {
    let mut tail: *mut SLICE = 0 as *mut SLICE;
    let mut temp: *mut SLICE = 0 as *mut SLICE;
    tail = _glp_dmp_get_atom(
        (*mpl).tuples,
        ::std::mem::size_of::<SLICE>() as libc::c_ulong as libc::c_int,
    ) as *mut SLICE;
    (*tail).sym = sym;
    (*tail).next = 0 as *mut TUPLE;
    if slice.is_null() {
        slice = tail;
    } else {
        temp = slice;
        while !((*temp).next).is_null() {
            temp = (*temp).next;
        }
        (*temp).next = tail;
    }
    return slice;
}
pub unsafe extern "C" fn _glp_mpl_slice_dimen(
    mut mpl: *mut MPL,
    mut slice: *mut SLICE,
) -> libc::c_int {
    let mut temp: *mut SLICE = 0 as *mut SLICE;
    let mut dim: libc::c_int = 0;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    dim = 0 as libc::c_int;
    temp = slice;
    while !temp.is_null() {
        dim += 1;
        dim;
        temp = (*temp).next;
    }
    return dim;
}
pub unsafe extern "C" fn _glp_mpl_slice_arity(
    mut mpl: *mut MPL,
    mut slice: *mut SLICE,
) -> libc::c_int {
    let mut temp: *mut SLICE = 0 as *mut SLICE;
    let mut arity: libc::c_int = 0;
    (mpl == mpl
        || {
            glp_assert_(
                b"mpl == mpl\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                96 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    arity = 0 as libc::c_int;
    temp = slice;
    while !temp.is_null() {
        if ((*temp).sym).is_null() {
            arity += 1;
            arity;
        }
        temp = (*temp).next;
    }
    return arity;
}
pub unsafe extern "C" fn _glp_mpl_fake_slice(
    mut mpl: *mut MPL,
    mut dim: libc::c_int,
) -> *mut SLICE {
    let mut slice: *mut SLICE = 0 as *mut SLICE;
    slice = _glp_mpl_create_slice(mpl);
    loop {
        let fresh0 = dim;
        dim = dim - 1;
        if !(fresh0 > 0 as libc::c_int) {
            break;
        }
        slice = _glp_mpl_expand_slice(mpl, slice, 0 as *mut SYMBOL);
    }
    return slice;
}
pub unsafe extern "C" fn _glp_mpl_delete_slice(
    mut mpl: *mut MPL,
    mut slice: *mut SLICE,
) {
    let mut temp: *mut SLICE = 0 as *mut SLICE;
    while !slice.is_null() {
        temp = slice;
        slice = (*temp).next;
        if !((*temp).sym).is_null() {
            _glp_mpl_delete_symbol(mpl, (*temp).sym);
        }
        (::std::mem::size_of::<SLICE>() as libc::c_ulong
            == ::std::mem::size_of::<TUPLE>() as libc::c_ulong
            || {
                glp_assert_(
                    b"sizeof(SLICE) == sizeof(TUPLE)\0" as *const u8
                        as *const libc::c_char,
                    b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                    130 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_dmp_free_atom(
            (*mpl).tuples,
            temp as *mut libc::c_void,
            ::std::mem::size_of::<TUPLE>() as libc::c_ulong as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn _glp_mpl_is_number(mut mpl: *mut MPL) -> libc::c_int {
    return ((*mpl).token == 204 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_is_symbol(mut mpl: *mut MPL) -> libc::c_int {
    return ((*mpl).token == 204 as libc::c_int || (*mpl).token == 203 as libc::c_int
        || (*mpl).token == 205 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_is_literal(
    mut mpl: *mut MPL,
    mut literal: *mut libc::c_char,
) -> libc::c_int {
    return (_glp_mpl_is_symbol(mpl) != 0
        && strcmp((*mpl).image, literal) == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn _glp_mpl_read_number(mut mpl: *mut MPL) -> libc::c_double {
    let mut num: libc::c_double = 0.;
    (_glp_mpl_is_number(mpl) != 0
        || {
            glp_assert_(
                b"is_number(mpl)\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                182 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    num = (*mpl).value;
    _glp_mpl_get_token(mpl);
    return num;
}
pub unsafe extern "C" fn _glp_mpl_read_symbol(mut mpl: *mut MPL) -> *mut SYMBOL {
    let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
    (_glp_mpl_is_symbol(mpl) != 0
        || {
            glp_assert_(
                b"is_symbol(mpl)\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                196 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if _glp_mpl_is_number(mpl) != 0 {
        sym = _glp_mpl_create_symbol_num(mpl, (*mpl).value);
    } else {
        sym = _glp_mpl_create_symbol_str(mpl, _glp_mpl_create_string(mpl, (*mpl).image));
    }
    _glp_mpl_get_token(mpl);
    return sym;
}
pub unsafe extern "C" fn _glp_mpl_read_slice(
    mut mpl: *mut MPL,
    mut name: *mut libc::c_char,
    mut dim: libc::c_int,
) -> *mut SLICE {
    let mut slice: *mut SLICE = 0 as *mut SLICE;
    let mut close: libc::c_int = 0;
    (!name.is_null()
        || {
            glp_assert_(
                b"name != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                227 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    match (*mpl).token {
        246 => {
            close = 247 as libc::c_int;
        }
        244 => {
            (dim > 0 as libc::c_int
                || {
                    glp_assert_(
                        b"dim > 0\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                        233 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            close = 245 as libc::c_int;
        }
        _ => {
            (mpl != mpl
                || {
                    glp_assert_(
                        b"mpl != mpl\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                        237 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    if dim == 0 as libc::c_int {
        _glp_mpl_error(
            mpl,
            b"%s cannot be subscripted\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
    }
    _glp_mpl_get_token(mpl);
    slice = _glp_mpl_create_slice(mpl);
    loop {
        if _glp_mpl_is_symbol(mpl) != 0 {
            slice = _glp_mpl_expand_slice(mpl, slice, _glp_mpl_read_symbol(mpl));
        } else if (*mpl).token == 227 as libc::c_int {
            slice = _glp_mpl_expand_slice(mpl, slice, 0 as *mut SYMBOL);
            _glp_mpl_get_token(mpl);
        } else {
            _glp_mpl_error(
                mpl,
                b"number, symbol, or asterisk missing where expected\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if (*mpl).token == 239 as libc::c_int {
            _glp_mpl_get_token(mpl);
        } else {
            if (*mpl).token == close {
                break;
            }
            _glp_mpl_error(
                mpl,
                b"syntax error in slice\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    if _glp_mpl_slice_dimen(mpl, slice) != dim {
        match close {
            247 => {
                _glp_mpl_error(
                    mpl,
                    b"%s must have %d subscript%s, not %d\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    name,
                    dim,
                    if dim == 1 as libc::c_int {
                        b"\0" as *const u8 as *const libc::c_char
                    } else {
                        b"s\0" as *const u8 as *const libc::c_char
                    },
                    _glp_mpl_slice_dimen(mpl, slice),
                );
            }
            245 => {
                _glp_mpl_error(
                    mpl,
                    b"%s has dimension %d, not %d\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    name,
                    dim,
                    _glp_mpl_slice_dimen(mpl, slice),
                );
            }
            _ => {
                (close != close
                    || {
                        glp_assert_(
                            b"close != close\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                            276 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
            }
        }
    }
    _glp_mpl_get_token(mpl);
    return slice;
}
pub unsafe extern "C" fn _glp_mpl_select_set(
    mut mpl: *mut MPL,
    mut name: *mut libc::c_char,
) -> *mut SET {
    let mut set: *mut SET = 0 as *mut SET;
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    (!name.is_null()
        || {
            glp_assert_(
                b"name != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                295 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    node = _glp_avl_find_node((*mpl).tree, name as *const libc::c_void);
    if node.is_null() || _glp_avl_get_node_type(node) != 122 as libc::c_int {
        _glp_mpl_error(
            mpl,
            b"%s not a set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
        );
    }
    set = _glp_avl_get_node_link(node) as *mut SET;
    if !((*set).assign).is_null() || !((*set).gadget).is_null() {
        _glp_mpl_error(
            mpl,
            b"%s needs no data\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
    }
    (*set).data = 1 as libc::c_int;
    return set;
}
pub unsafe extern "C" fn _glp_mpl_simple_format(
    mut mpl: *mut MPL,
    mut set: *mut SET,
    mut memb: *mut MEMBER,
    mut slice: *mut SLICE,
) {
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    let mut temp: *mut SLICE = 0 as *mut SLICE;
    let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut with: *mut SYMBOL = 0 as *mut SYMBOL;
    (!set.is_null()
        || {
            glp_assert_(
                b"set != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                335 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!memb.is_null()
        || {
            glp_assert_(
                b"memb != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                336 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!slice.is_null()
        || {
            glp_assert_(
                b"slice != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                337 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*set).dimen == _glp_mpl_slice_dimen(mpl, slice)
        || {
            glp_assert_(
                b"set->dimen == slice_dimen(mpl, slice)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                338 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*(*memb).value.set).dim == (*set).dimen
        || {
            glp_assert_(
                b"memb->value.set->dim == set->dimen\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                339 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if _glp_mpl_slice_arity(mpl, slice) > 0 as libc::c_int {
        (_glp_mpl_is_symbol(mpl) != 0
            || {
                glp_assert_(
                    b"is_symbol(mpl)\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                    340 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
    }
    tuple = _glp_mpl_create_tuple(mpl);
    temp = slice;
    while !temp.is_null() {
        if ((*temp).sym).is_null() {
            if _glp_mpl_is_symbol(mpl) == 0 {
                let mut lack: libc::c_int = _glp_mpl_slice_arity(mpl, temp);
                (!with.is_null()
                    || {
                        glp_assert_(
                            b"with != NULL\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                            349 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if lack == 1 as libc::c_int {
                    _glp_mpl_error(
                        mpl,
                        b"one item missing in data group beginning with %s\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        _glp_mpl_format_symbol(mpl, with),
                    );
                } else {
                    _glp_mpl_error(
                        mpl,
                        b"%d items missing in data group beginning with %s\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        lack,
                        _glp_mpl_format_symbol(mpl, with),
                    );
                }
            }
            sym = _glp_mpl_read_symbol(mpl);
            if with.is_null() {
                with = sym;
            }
        } else {
            sym = _glp_mpl_copy_symbol(mpl, (*temp).sym);
        }
        tuple = _glp_mpl_expand_tuple(mpl, tuple, sym);
        if !((*temp).next).is_null() && (*mpl).token == 239 as libc::c_int {
            _glp_mpl_get_token(mpl);
        }
        temp = (*temp).next;
    }
    _glp_mpl_check_then_add(mpl, (*memb).value.set, tuple);
}
pub unsafe extern "C" fn _glp_mpl_matrix_format(
    mut mpl: *mut MPL,
    mut set: *mut SET,
    mut memb: *mut MEMBER,
    mut slice: *mut SLICE,
    mut tr: libc::c_int,
) {
    let mut list: *mut SLICE = 0 as *mut SLICE;
    let mut col: *mut SLICE = 0 as *mut SLICE;
    let mut temp: *mut SLICE = 0 as *mut SLICE;
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    let mut row: *mut SYMBOL = 0 as *mut SYMBOL;
    (!set.is_null()
        || {
            glp_assert_(
                b"set != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                408 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!memb.is_null()
        || {
            glp_assert_(
                b"memb != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                409 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!slice.is_null()
        || {
            glp_assert_(
                b"slice != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                410 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*set).dimen == _glp_mpl_slice_dimen(mpl, slice)
        || {
            glp_assert_(
                b"set->dimen == slice_dimen(mpl, slice)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                411 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*(*memb).value.set).dim == (*set).dimen
        || {
            glp_assert_(
                b"memb->value.set->dim == set->dimen\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (_glp_mpl_slice_arity(mpl, slice) == 2 as libc::c_int
        || {
            glp_assert_(
                b"slice_arity(mpl, slice) == 2\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                413 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    list = _glp_mpl_create_slice(mpl);
    while (*mpl).token != 242 as libc::c_int {
        if _glp_mpl_is_symbol(mpl) == 0 {
            _glp_mpl_error(
                mpl,
                b"number, symbol, or := missing where expected\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        list = _glp_mpl_expand_slice(mpl, list, _glp_mpl_read_symbol(mpl));
    }
    _glp_mpl_get_token(mpl);
    while _glp_mpl_is_symbol(mpl) != 0 {
        row = _glp_mpl_read_symbol(mpl);
        let mut current_block_35: u64;
        col = list;
        while !col.is_null() {
            let mut which: libc::c_int = 0 as libc::c_int;
            if _glp_mpl_is_literal(
                mpl,
                b"+\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                current_block_35 = 224731115979188411;
            } else if _glp_mpl_is_literal(
                mpl,
                b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                _glp_mpl_get_token(mpl);
                current_block_35 = 5399440093318478209;
            } else {
                let mut lack: libc::c_int = _glp_mpl_slice_dimen(mpl, col);
                if lack == 1 as libc::c_int {
                    _glp_mpl_error(
                        mpl,
                        b"one item missing in data group beginning with %s\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        _glp_mpl_format_symbol(mpl, row),
                    );
                } else {
                    _glp_mpl_error(
                        mpl,
                        b"%d items missing in data group beginning with %s\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        lack,
                        _glp_mpl_format_symbol(mpl, row),
                    );
                }
                current_block_35 = 224731115979188411;
            }
            match current_block_35 {
                224731115979188411 => {
                    tuple = _glp_mpl_create_tuple(mpl);
                    temp = slice;
                    while !temp.is_null() {
                        if ((*temp).sym).is_null() {
                            which += 1;
                            match which {
                                1 => {
                                    tuple = _glp_mpl_expand_tuple(
                                        mpl,
                                        tuple,
                                        _glp_mpl_copy_symbol(
                                            mpl,
                                            if tr != 0 { (*col).sym } else { row },
                                        ),
                                    );
                                }
                                2 => {
                                    tuple = _glp_mpl_expand_tuple(
                                        mpl,
                                        tuple,
                                        _glp_mpl_copy_symbol(
                                            mpl,
                                            if tr != 0 { row } else { (*col).sym },
                                        ),
                                    );
                                }
                                _ => {
                                    (which != which
                                        || {
                                            glp_assert_(
                                                b"which != which\0" as *const u8 as *const libc::c_char,
                                                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                                                465 as libc::c_int,
                                            );
                                            1 as libc::c_int != 0
                                        }) as libc::c_int;
                                }
                            }
                        } else {
                            tuple = _glp_mpl_expand_tuple(
                                mpl,
                                tuple,
                                _glp_mpl_copy_symbol(mpl, (*temp).sym),
                            );
                        }
                        temp = (*temp).next;
                    }
                    (which == 2 as libc::c_int
                        || {
                            glp_assert_(
                                b"which == 2\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                                474 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    _glp_mpl_check_then_add(mpl, (*memb).value.set, tuple);
                    _glp_mpl_get_token(mpl);
                }
                _ => {}
            }
            col = (*col).next;
        }
        _glp_mpl_delete_symbol(mpl, row);
    }
    _glp_mpl_delete_slice(mpl, list);
}
pub unsafe extern "C" fn _glp_mpl_set_data(mut mpl: *mut MPL) {
    let mut set: *mut SET = 0 as *mut SET;
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    let mut slice: *mut SLICE = 0 as *mut SLICE;
    let mut tr: libc::c_int = 0 as libc::c_int;
    (_glp_mpl_is_literal(
        mpl,
        b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
        || {
            glp_assert_(
                b"is_literal(mpl, \"set\")\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                511 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_mpl_get_token(mpl);
    if _glp_mpl_is_symbol(mpl) == 0 {
        _glp_mpl_error(
            mpl,
            b"set name missing where expected\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    set = _glp_mpl_select_set(mpl, (*mpl).image);
    _glp_mpl_get_token(mpl);
    tuple = _glp_mpl_create_tuple(mpl);
    if (*mpl).token == 246 as libc::c_int {
        if (*set).dim == 0 as libc::c_int {
            _glp_mpl_error(
                mpl,
                b"%s cannot be subscripted\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*set).name,
            );
        }
        _glp_mpl_get_token(mpl);
        loop {
            if _glp_mpl_is_symbol(mpl) == 0 {
                _glp_mpl_error(
                    mpl,
                    b"number or symbol missing where expected\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            tuple = _glp_mpl_expand_tuple(mpl, tuple, _glp_mpl_read_symbol(mpl));
            if (*mpl).token == 239 as libc::c_int {
                _glp_mpl_get_token(mpl);
            } else {
                if (*mpl).token == 247 as libc::c_int {
                    break;
                }
                _glp_mpl_error(
                    mpl,
                    b"syntax error in subscript list\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        if (*set).dim != _glp_mpl_tuple_dimen(mpl, tuple) {
            _glp_mpl_error(
                mpl,
                b"%s must have %d subscript%s rather than %d\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*set).name,
                (*set).dim,
                if (*set).dim == 1 as libc::c_int {
                    b"\0" as *const u8 as *const libc::c_char
                } else {
                    b"s\0" as *const u8 as *const libc::c_char
                },
                _glp_mpl_tuple_dimen(mpl, tuple),
            );
        }
        _glp_mpl_get_token(mpl);
    } else if (*set).dim != 0 as libc::c_int {
        _glp_mpl_error(
            mpl,
            b"%s must be subscripted\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*set).name,
        );
    }
    if !(_glp_mpl_find_member(mpl, (*set).array, tuple)).is_null() {
        _glp_mpl_error(
            mpl,
            b"%s%s already defined\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*set).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
        );
    }
    memb = _glp_mpl_add_member(mpl, (*set).array, tuple);
    (*memb).value.set = _glp_mpl_create_elemset(mpl, (*set).dimen);
    slice = _glp_mpl_fake_slice(mpl, (*set).dimen);
    let mut current_block_55: u64;
    's_139: loop {
        if (*mpl).token == 239 as libc::c_int {
            _glp_mpl_get_token(mpl);
        }
        if (*mpl).token == 242 as libc::c_int {
            _glp_mpl_get_token(mpl);
        } else {
            if (*mpl).token == 244 as libc::c_int {
                let mut is_tr: libc::c_int = 0;
                _glp_mpl_get_token(mpl);
                is_tr = _glp_mpl_is_literal(
                    mpl,
                    b"tr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                _glp_mpl_unget_token(mpl);
                if is_tr != 0 {
                    current_block_55 = 12381812505308290051;
                } else {
                    _glp_mpl_delete_slice(mpl, slice);
                    slice = _glp_mpl_read_slice(mpl, (*set).name, (*set).dimen);
                    tr = 0 as libc::c_int;
                    if _glp_mpl_slice_arity(mpl, slice) == 0 as libc::c_int {
                        _glp_mpl_simple_format(mpl, set, memb, slice);
                    }
                    continue;
                }
            } else if _glp_mpl_is_symbol(mpl) != 0 {
                _glp_mpl_simple_format(mpl, set, memb, slice);
                continue;
            } else if (*mpl).token == 240 as libc::c_int {
                if _glp_mpl_slice_arity(mpl, slice) != 2 as libc::c_int {
                    current_block_55 = 16463562280028132836;
                } else {
                    current_block_55 = 790185930182612747;
                }
            } else if (*mpl).token == 244 as libc::c_int {
                current_block_55 = 12381812505308290051;
            } else if (*mpl).token == 241 as libc::c_int {
                _glp_mpl_get_token(mpl);
                break;
            } else {
                _glp_mpl_error(
                    mpl,
                    b"syntax error in set data block\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                continue;
            }
            match current_block_55 {
                12381812505308290051 => {
                    _glp_mpl_get_token(mpl);
                    if _glp_mpl_is_literal(
                        mpl,
                        b"tr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) == 0
                    {
                        current_block_55 = 16563882426818670160;
                    } else {
                        current_block_55 = 2989495919056355252;
                    }
                    loop {
                        match current_block_55 {
                            16563882426818670160 => {
                                _glp_mpl_error(
                                    mpl,
                                    b"transpose indicator (tr) incomplete\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                current_block_55 = 2989495919056355252;
                            }
                            _ => {
                                if _glp_mpl_slice_arity(mpl, slice) != 2 as libc::c_int {
                                    break;
                                }
                                _glp_mpl_get_token(mpl);
                                if (*mpl).token != 245 as libc::c_int {
                                    current_block_55 = 16563882426818670160;
                                    continue;
                                }
                                _glp_mpl_get_token(mpl);
                                if (*mpl).token == 240 as libc::c_int {
                                    _glp_mpl_get_token(mpl);
                                }
                                tr = 1 as libc::c_int;
                                _glp_mpl_matrix_format(mpl, set, memb, slice, tr);
                                continue 's_139;
                            }
                        }
                    }
                    current_block_55 = 16463562280028132836;
                }
                _ => {}
            }
            match current_block_55 {
                16463562280028132836 => {
                    _glp_mpl_error(
                        mpl,
                        b"slice currently used must specify 2 asterisks, not %d\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        _glp_mpl_slice_arity(mpl, slice),
                    );
                }
                _ => {}
            }
            _glp_mpl_get_token(mpl);
            _glp_mpl_matrix_format(mpl, set, memb, slice, tr);
        }
    }
    _glp_mpl_delete_slice(mpl, slice);
}
pub unsafe extern "C" fn _glp_mpl_select_parameter(
    mut mpl: *mut MPL,
    mut name: *mut libc::c_char,
) -> *mut PARAMETER {
    let mut par: *mut PARAMETER = 0 as *mut PARAMETER;
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    (!name.is_null()
        || {
            glp_assert_(
                b"name != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                641 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    node = _glp_avl_find_node((*mpl).tree, name as *const libc::c_void);
    if node.is_null() || _glp_avl_get_node_type(node) != 120 as libc::c_int {
        _glp_mpl_error(
            mpl,
            b"%s not a parameter\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
    }
    par = _glp_avl_get_node_link(node) as *mut PARAMETER;
    if !((*par).assign).is_null() {
        _glp_mpl_error(
            mpl,
            b"%s needs no data\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
    }
    if (*par).data != 0 {
        _glp_mpl_error(
            mpl,
            b"%s already provided with data\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
    }
    (*par).data = 1 as libc::c_int;
    return par;
}
pub unsafe extern "C" fn _glp_mpl_set_default(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut altval: *mut SYMBOL,
) {
    (!par.is_null()
        || {
            glp_assert_(
                b"par != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                664 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (!altval.is_null()
        || {
            glp_assert_(
                b"altval != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                665 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !((*par).option).is_null() {
        _glp_mpl_error(
            mpl,
            b"default value for %s already specified in model section\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*par).name,
        );
    }
    (((*par).defval).is_null()
        || {
            glp_assert_(
                b"par->defval == NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                669 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (*par).defval = altval;
}
pub unsafe extern "C" fn _glp_mpl_read_value(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut tuple: *mut TUPLE,
) -> *mut MEMBER {
    let mut memb: *mut MEMBER = 0 as *mut MEMBER;
    (!par.is_null()
        || {
            glp_assert_(
                b"par != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                687 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (_glp_mpl_is_symbol(mpl) != 0
        || {
            glp_assert_(
                b"is_symbol(mpl)\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                688 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    if !(_glp_mpl_find_member(mpl, (*par).array, tuple)).is_null() {
        _glp_mpl_error(
            mpl,
            b"%s%s already defined\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*par).name,
            _glp_mpl_format_tuple(mpl, '[' as i32, tuple),
        );
    }
    memb = _glp_mpl_add_member(mpl, (*par).array, tuple);
    match (*par).type_0 {
        118 | 113 | 101 => {
            if _glp_mpl_is_number(mpl) == 0 {
                _glp_mpl_error(
                    mpl,
                    b"%s requires numeric data\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*par).name,
                );
            }
            (*memb).value.num = _glp_mpl_read_number(mpl);
        }
        124 => {
            (*memb).value.sym = _glp_mpl_read_symbol(mpl);
        }
        _ => {
            (par != par
                || {
                    glp_assert_(
                        b"par != par\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                        708 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
        }
    }
    return memb;
}
pub unsafe extern "C" fn _glp_mpl_plain_format(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut slice: *mut SLICE,
) {
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    let mut temp: *mut SLICE = 0 as *mut SLICE;
    let mut sym: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut with: *mut SYMBOL = 0 as *mut SYMBOL;
    (!par.is_null()
        || {
            glp_assert_(
                b"par != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                737 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*par).dim == _glp_mpl_slice_dimen(mpl, slice)
        || {
            glp_assert_(
                b"par->dim == slice_dimen(mpl, slice)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                738 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (_glp_mpl_is_symbol(mpl) != 0
        || {
            glp_assert_(
                b"is_symbol(mpl)\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                739 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    tuple = _glp_mpl_create_tuple(mpl);
    temp = slice;
    while !temp.is_null() {
        if ((*temp).sym).is_null() {
            if _glp_mpl_is_symbol(mpl) == 0 {
                let mut lack: libc::c_int = _glp_mpl_slice_arity(mpl, temp)
                    + 1 as libc::c_int;
                (!with.is_null()
                    || {
                        glp_assert_(
                            b"with != NULL\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                            747 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (lack > 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"lack > 1\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                            748 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                _glp_mpl_error(
                    mpl,
                    b"%d items missing in data group beginning with %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    lack,
                    _glp_mpl_format_symbol(mpl, with),
                );
            }
            sym = _glp_mpl_read_symbol(mpl);
            if with.is_null() {
                with = sym;
            }
        } else {
            sym = _glp_mpl_copy_symbol(mpl, (*temp).sym);
        }
        tuple = _glp_mpl_expand_tuple(mpl, tuple, sym);
        if (*mpl).token == 239 as libc::c_int {
            _glp_mpl_get_token(mpl);
        }
        temp = (*temp).next;
    }
    if _glp_mpl_is_symbol(mpl) == 0 {
        (!with.is_null()
            || {
                glp_assert_(
                    b"with != NULL\0" as *const u8 as *const libc::c_char,
                    b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                    766 as libc::c_int,
                );
                1 as libc::c_int != 0
            }) as libc::c_int;
        _glp_mpl_error(
            mpl,
            b"one item missing in data group beginning with %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            _glp_mpl_format_symbol(mpl, with),
        );
    }
    _glp_mpl_read_value(mpl, par, tuple);
}
pub unsafe extern "C" fn _glp_mpl_tabular_format(
    mut mpl: *mut MPL,
    mut par: *mut PARAMETER,
    mut slice: *mut SLICE,
    mut tr: libc::c_int,
) {
    let mut list: *mut SLICE = 0 as *mut SLICE;
    let mut col: *mut SLICE = 0 as *mut SLICE;
    let mut temp: *mut SLICE = 0 as *mut SLICE;
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    let mut row: *mut SYMBOL = 0 as *mut SYMBOL;
    (!par.is_null()
        || {
            glp_assert_(
                b"par != NULL\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                806 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    ((*par).dim == _glp_mpl_slice_dimen(mpl, slice)
        || {
            glp_assert_(
                b"par->dim == slice_dimen(mpl, slice)\0" as *const u8
                    as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                807 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    (_glp_mpl_slice_arity(mpl, slice) == 2 as libc::c_int
        || {
            glp_assert_(
                b"slice_arity(mpl, slice) == 2\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                808 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    list = _glp_mpl_create_slice(mpl);
    while (*mpl).token != 242 as libc::c_int {
        if _glp_mpl_is_symbol(mpl) == 0 {
            _glp_mpl_error(
                mpl,
                b"number, symbol, or := missing where expected\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        list = _glp_mpl_expand_slice(mpl, list, _glp_mpl_read_symbol(mpl));
    }
    _glp_mpl_get_token(mpl);
    while _glp_mpl_is_symbol(mpl) != 0 {
        row = _glp_mpl_read_symbol(mpl);
        col = list;
        while !col.is_null() {
            let mut which: libc::c_int = 0 as libc::c_int;
            if _glp_mpl_is_literal(
                mpl,
                b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                _glp_mpl_get_token(mpl);
            } else {
                tuple = _glp_mpl_create_tuple(mpl);
                temp = slice;
                while !temp.is_null() {
                    if ((*temp).sym).is_null() {
                        which += 1;
                        match which {
                            1 => {
                                tuple = _glp_mpl_expand_tuple(
                                    mpl,
                                    tuple,
                                    _glp_mpl_copy_symbol(
                                        mpl,
                                        if tr != 0 { (*col).sym } else { row },
                                    ),
                                );
                            }
                            2 => {
                                tuple = _glp_mpl_expand_tuple(
                                    mpl,
                                    tuple,
                                    _glp_mpl_copy_symbol(
                                        mpl,
                                        if tr != 0 { row } else { (*col).sym },
                                    ),
                                );
                            }
                            _ => {
                                (which != which
                                    || {
                                        glp_assert_(
                                            b"which != which\0" as *const u8 as *const libc::c_char,
                                            b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                                            849 as libc::c_int,
                                        );
                                        1 as libc::c_int != 0
                                    }) as libc::c_int;
                            }
                        }
                    } else {
                        tuple = _glp_mpl_expand_tuple(
                            mpl,
                            tuple,
                            _glp_mpl_copy_symbol(mpl, (*temp).sym),
                        );
                    }
                    temp = (*temp).next;
                }
                (which == 2 as libc::c_int
                    || {
                        glp_assert_(
                            b"which == 2\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                            858 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                if _glp_mpl_is_symbol(mpl) == 0 {
                    let mut lack: libc::c_int = _glp_mpl_slice_dimen(mpl, col);
                    if lack == 1 as libc::c_int {
                        _glp_mpl_error(
                            mpl,
                            b"one item missing in data group beginning with %s\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            _glp_mpl_format_symbol(mpl, row),
                        );
                    } else {
                        _glp_mpl_error(
                            mpl,
                            b"%d items missing in data group beginning with %s\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            lack,
                            _glp_mpl_format_symbol(mpl, row),
                        );
                    }
                }
                _glp_mpl_read_value(mpl, par, tuple);
            }
            col = (*col).next;
        }
        _glp_mpl_delete_symbol(mpl, row);
    }
    _glp_mpl_delete_slice(mpl, list);
}
pub unsafe extern "C" fn _glp_mpl_tabbing_format(
    mut mpl: *mut MPL,
    mut altval: *mut SYMBOL,
) {
    let mut set: *mut SET = 0 as *mut SET;
    let mut par: *mut PARAMETER = 0 as *mut PARAMETER;
    let mut list: *mut SLICE = 0 as *mut SLICE;
    let mut col: *mut SLICE = 0 as *mut SLICE;
    let mut tuple: *mut TUPLE = 0 as *mut TUPLE;
    let mut next_token: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dim: libc::c_int = 0 as libc::c_int;
    let mut last_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if _glp_mpl_is_symbol(mpl) != 0 {
        _glp_mpl_get_token(mpl);
        next_token = (*mpl).token;
        _glp_mpl_unget_token(mpl);
        if next_token == 240 as libc::c_int {
            set = _glp_mpl_select_set(mpl, (*mpl).image);
            if (*set).dim != 0 as libc::c_int {
                _glp_mpl_error(
                    mpl,
                    b"%s must be a simple set\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*set).name,
                );
            }
            if !((*(*set).array).head).is_null() {
                _glp_mpl_error(
                    mpl,
                    b"%s already defined\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*set).name,
                );
            }
            let ref mut fresh1 = (*_glp_mpl_add_member(
                mpl,
                (*set).array,
                0 as *mut TUPLE,
            ))
                .value
                .set;
            *fresh1 = _glp_mpl_create_elemset(mpl, (*set).dimen);
            last_name = (*set).name;
            dim = (*set).dimen;
            _glp_mpl_get_token(mpl);
            ((*mpl).token == 240 as libc::c_int
                || {
                    glp_assert_(
                        b"mpl->token == T_COLON\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                        934 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            _glp_mpl_get_token(mpl);
        }
    }
    list = _glp_mpl_create_slice(mpl);
    while (*mpl).token != 242 as libc::c_int {
        if _glp_mpl_is_symbol(mpl) == 0 {
            _glp_mpl_error(
                mpl,
                b"parameter name or := missing where expected\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        par = _glp_mpl_select_parameter(mpl, (*mpl).image);
        if (*par).dim == 0 as libc::c_int {
            _glp_mpl_error(
                mpl,
                b"%s not a subscripted parameter\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*mpl).image,
            );
        }
        if dim != 0 as libc::c_int && (*par).dim != dim {
            (!last_name.is_null()
                || {
                    glp_assert_(
                        b"last_name != NULL\0" as *const u8 as *const libc::c_char,
                        b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                        952 as libc::c_int,
                    );
                    1 as libc::c_int != 0
                }) as libc::c_int;
            _glp_mpl_error(
                mpl,
                b"%s has dimension %d while %s has dimension %d\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                last_name,
                dim,
                (*par).name,
                (*par).dim,
            );
        }
        if !altval.is_null() {
            _glp_mpl_set_default(mpl, par, _glp_mpl_copy_symbol(mpl, altval));
        }
        list = _glp_mpl_expand_slice(mpl, list, par as *mut SYMBOL);
        last_name = (*par).name;
        dim = (*par).dim;
        _glp_mpl_get_token(mpl);
        if (*mpl).token == 239 as libc::c_int {
            _glp_mpl_get_token(mpl);
        }
    }
    if _glp_mpl_slice_dimen(mpl, list) == 0 as libc::c_int {
        _glp_mpl_error(
            mpl,
            b"at least one parameter name required\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    _glp_mpl_get_token(mpl);
    if (*mpl).token == 239 as libc::c_int {
        _glp_mpl_get_token(mpl);
    }
    while _glp_mpl_is_symbol(mpl) != 0 {
        tuple = _glp_mpl_create_tuple(mpl);
        j = 1 as libc::c_int;
        while j <= dim {
            if _glp_mpl_is_symbol(mpl) == 0 {
                let mut lack: libc::c_int = _glp_mpl_slice_dimen(mpl, list) + dim - j
                    + 1 as libc::c_int;
                (!tuple.is_null()
                    || {
                        glp_assert_(
                            b"tuple != NULL\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                            979 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                (lack > 1 as libc::c_int
                    || {
                        glp_assert_(
                            b"lack > 1\0" as *const u8 as *const libc::c_char,
                            b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                            980 as libc::c_int,
                        );
                        1 as libc::c_int != 0
                    }) as libc::c_int;
                _glp_mpl_error(
                    mpl,
                    b"%d items missing in data group beginning with %s\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    lack,
                    _glp_mpl_format_symbol(mpl, (*tuple).sym),
                );
            }
            tuple = _glp_mpl_expand_tuple(mpl, tuple, _glp_mpl_read_symbol(mpl));
            if j < dim && (*mpl).token == 239 as libc::c_int {
                _glp_mpl_get_token(mpl);
            }
            j += 1;
            j;
        }
        if !set.is_null() {
            _glp_mpl_check_then_add(
                mpl,
                (*(*(*set).array).head).value.set,
                _glp_mpl_copy_tuple(mpl, tuple),
            );
        }
        if (*mpl).token == 239 as libc::c_int {
            _glp_mpl_get_token(mpl);
        }
        col = list;
        while !col.is_null() {
            if _glp_mpl_is_literal(
                mpl,
                b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) != 0
            {
                _glp_mpl_get_token(mpl);
            } else {
                if _glp_mpl_is_symbol(mpl) == 0 {
                    let mut lack_0: libc::c_int = _glp_mpl_slice_dimen(mpl, col);
                    (!tuple.is_null()
                        || {
                            glp_assert_(
                                b"tuple != NULL\0" as *const u8 as *const libc::c_char,
                                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                                1007 as libc::c_int,
                            );
                            1 as libc::c_int != 0
                        }) as libc::c_int;
                    if lack_0 == 1 as libc::c_int {
                        _glp_mpl_error(
                            mpl,
                            b"one item missing in data group beginning with %s\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            _glp_mpl_format_symbol(mpl, (*tuple).sym),
                        );
                    } else {
                        _glp_mpl_error(
                            mpl,
                            b"%d items missing in data group beginning with %s\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            lack_0,
                            _glp_mpl_format_symbol(mpl, (*tuple).sym),
                        );
                    }
                }
                _glp_mpl_read_value(
                    mpl,
                    (*col).sym as *mut PARAMETER,
                    _glp_mpl_copy_tuple(mpl, tuple),
                );
                if !((*col).next).is_null() && (*mpl).token == 239 as libc::c_int {
                    _glp_mpl_get_token(mpl);
                }
            }
            col = (*col).next;
        }
        _glp_mpl_delete_tuple(mpl, tuple);
        if (*mpl).token == 239 as libc::c_int {
            _glp_mpl_get_token(mpl);
            if _glp_mpl_is_symbol(mpl) == 0 {
                _glp_mpl_unget_token(mpl);
            }
        }
    }
    col = list;
    while !col.is_null() {
        (*col).sym = 0 as *mut SYMBOL;
        col = (*col).next;
    }
    _glp_mpl_delete_slice(mpl, list);
}
pub unsafe extern "C" fn _glp_mpl_parameter_data(mut mpl: *mut MPL) {
    let mut par: *mut PARAMETER = 0 as *mut PARAMETER;
    let mut altval: *mut SYMBOL = 0 as *mut SYMBOL;
    let mut slice: *mut SLICE = 0 as *mut SLICE;
    let mut tr: libc::c_int = 0 as libc::c_int;
    (_glp_mpl_is_literal(
        mpl,
        b"param\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
        || {
            glp_assert_(
                b"is_literal(mpl, \"param\")\0" as *const u8 as *const libc::c_char,
                b"mpl/mpl2.c\0" as *const u8 as *const libc::c_char,
                1062 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    _glp_mpl_get_token(mpl);
    if _glp_mpl_is_literal(
        mpl,
        b"default\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0
    {
        _glp_mpl_get_token(mpl);
        if _glp_mpl_is_symbol(mpl) == 0 {
            _glp_mpl_error(
                mpl,
                b"default value missing where expected\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        altval = _glp_mpl_read_symbol(mpl);
        if (*mpl).token != 240 as libc::c_int {
            _glp_mpl_error(
                mpl,
                b"colon missing where expected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    if (*mpl).token == 240 as libc::c_int {
        _glp_mpl_get_token(mpl);
        if (*mpl).token == 239 as libc::c_int {
            _glp_mpl_get_token(mpl);
        }
        _glp_mpl_tabbing_format(mpl, altval);
        if !altval.is_null() {
            _glp_mpl_delete_symbol(mpl, altval);
        }
        if (*mpl).token != 241 as libc::c_int {
            _glp_mpl_error(
                mpl,
                b"symbol, number, or semicolon missing where expected\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        _glp_mpl_get_token(mpl);
    } else {
        if _glp_mpl_is_symbol(mpl) == 0 {
            _glp_mpl_error(
                mpl,
                b"parameter name missing where expected\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        par = _glp_mpl_select_parameter(mpl, (*mpl).image);
        _glp_mpl_get_token(mpl);
        if _glp_mpl_is_literal(
            mpl,
            b"default\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            _glp_mpl_get_token(mpl);
            if _glp_mpl_is_symbol(mpl) == 0 {
                _glp_mpl_error(
                    mpl,
                    b"default value missing where expected\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            altval = _glp_mpl_read_symbol(mpl);
            _glp_mpl_set_default(mpl, par, altval);
        }
        slice = _glp_mpl_fake_slice(mpl, (*par).dim);
        let mut current_block_54: u64;
        's_134: loop {
            if (*mpl).token == 239 as libc::c_int {
                _glp_mpl_get_token(mpl);
            }
            if (*mpl).token == 242 as libc::c_int {
                _glp_mpl_get_token(mpl);
            } else if (*mpl).token == 246 as libc::c_int {
                _glp_mpl_delete_slice(mpl, slice);
                slice = _glp_mpl_read_slice(mpl, (*par).name, (*par).dim);
                tr = 0 as libc::c_int;
            } else if _glp_mpl_is_symbol(mpl) != 0 {
                _glp_mpl_plain_format(mpl, par, slice);
            } else {
                if (*mpl).token == 240 as libc::c_int {
                    if (*par).dim == 0 as libc::c_int {
                        current_block_54 = 8589577301855675103;
                    } else {
                        current_block_54 = 4090602189656566074;
                    }
                } else if (*mpl).token == 244 as libc::c_int {
                    _glp_mpl_get_token(mpl);
                    if _glp_mpl_is_literal(
                        mpl,
                        b"tr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ) == 0
                    {
                        current_block_54 = 543885847952364195;
                    } else {
                        current_block_54 = 1622411330066726685;
                    }
                    loop {
                        match current_block_54 {
                            543885847952364195 => {
                                _glp_mpl_error(
                                    mpl,
                                    b"transpose indicator (tr) incomplete\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                current_block_54 = 1622411330066726685;
                            }
                            _ => {
                                if (*par).dim == 0 as libc::c_int {
                                    current_block_54 = 8589577301855675103;
                                    break;
                                }
                                if _glp_mpl_slice_arity(mpl, slice) != 2 as libc::c_int {
                                    current_block_54 = 2117155769546350064;
                                    break;
                                }
                                _glp_mpl_get_token(mpl);
                                if (*mpl).token != 245 as libc::c_int {
                                    current_block_54 = 543885847952364195;
                                    continue;
                                }
                                _glp_mpl_get_token(mpl);
                                if (*mpl).token == 240 as libc::c_int {
                                    _glp_mpl_get_token(mpl);
                                }
                                tr = 1 as libc::c_int;
                                _glp_mpl_tabular_format(mpl, par, slice, tr);
                                continue 's_134;
                            }
                        }
                    }
                } else if (*mpl).token == 241 as libc::c_int {
                    _glp_mpl_get_token(mpl);
                    break;
                } else {
                    _glp_mpl_error(
                        mpl,
                        b"syntax error in parameter data block\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    continue;
                }
                match current_block_54 {
                    8589577301855675103 => {
                        _glp_mpl_error(
                            mpl,
                            b"%s not a subscripted parameter\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            (*par).name,
                        );
                        current_block_54 = 4090602189656566074;
                    }
                    _ => {}
                }
                match current_block_54 {
                    4090602189656566074 => {
                        if _glp_mpl_slice_arity(mpl, slice) != 2 as libc::c_int {
                            current_block_54 = 2117155769546350064;
                        } else {
                            current_block_54 = 11743904203796629665;
                        }
                    }
                    _ => {}
                }
                match current_block_54 {
                    2117155769546350064 => {
                        _glp_mpl_error(
                            mpl,
                            b"slice currently used must specify 2 asterisks, not %d\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            _glp_mpl_slice_arity(mpl, slice),
                        );
                    }
                    _ => {}
                }
                _glp_mpl_get_token(mpl);
                _glp_mpl_tabular_format(mpl, par, slice, tr);
            }
        }
        _glp_mpl_delete_slice(mpl, slice);
    };
}
pub unsafe extern "C" fn _glp_mpl_data_section(mut mpl: *mut MPL) {
    while !((*mpl).token == 201 as libc::c_int
        || _glp_mpl_is_literal(
            mpl,
            b"end\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0)
    {
        if _glp_mpl_is_literal(
            mpl,
            b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            _glp_mpl_set_data(mpl);
        } else if _glp_mpl_is_literal(
            mpl,
            b"param\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            _glp_mpl_parameter_data(mpl);
        } else {
            _glp_mpl_error(
                mpl,
                b"syntax error in data section\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
}

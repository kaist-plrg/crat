use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Get_Float(n: libc::c_double, start_word: WamWord) -> Bool;
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Get_Structure(
        func: libc::c_int,
        arity: libc::c_int,
        start_word: WamWord,
    ) -> Bool;
    fn Pl_Put_X_Variable() -> WamWord;
    fn Pl_Put_Y_Variable(y_adr: *mut WamWord) -> WamWord;
    fn Pl_Put_Unsafe_Value(start_word: WamWord) -> WamWord;
    fn Pl_Put_Atom(atom: libc::c_int) -> WamWord;
    fn Pl_Put_Integer(n: PlLong) -> WamWord;
    fn Pl_Put_Float(n: libc::c_double) -> WamWord;
    fn Pl_Put_List() -> WamWord;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Void(n: libc::c_int);
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Unify_Local_Value(start_word: WamWord) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    fn Pl_Unify_Nil() -> Bool;
    fn Pl_Unify_List() -> Bool;
    fn Pl_Unify_Structure(func: libc::c_int, arity: libc::c_int) -> Bool;
    fn Pl_Allocate(n: libc::c_int);
    fn Pl_Deallocate();
    fn Pl_Get_Current_Choice() -> WamWord;
    fn Pl_Cut(b_word: WamWord);
    fn Pl_Soft_Cut(b_word: WamWord);
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Create_Pred(
        func: libc::c_int,
        arity: libc::c_int,
        pl_file: libc::c_int,
        pl_line: libc::c_int,
        prop: libc::c_int,
        codep: *mut PlLong,
    ) -> *mut PredInf;
    fn Pl_Lookup_Pred(func: libc::c_int, arity: libc::c_int) -> *mut PredInf;
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Realloc_Check(
        ptr: *mut libc::c_char,
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Float(start_word: WamWord) -> libc::c_double;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Atom(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Callable_Check(
        start_word: WamWord,
        func: *mut libc::c_int,
        arity: *mut libc::c_int,
    ) -> *mut WamWord;
    fn Pl_Math_Load_Value(start_word: WamWord, word_adr: *mut WamWord);
    fn Pl_Detect_If_Aux_Name(func: libc::c_int) -> *mut libc::c_char;
    fn Pl_Pred_Without_Aux(
        func: libc::c_int,
        arity: libc::c_int,
        arity1: *mut libc::c_int,
    ) -> libc::c_int;
    fn Pl_Set_Bip_Name_2(func_word: WamWord, arity_word: WamWord);
    fn Pl_Unknown_Pred_Error(func: libc::c_int, arity: libc::c_int);
    fn Pl_Update_Dynamic_Pred(
        func: libc::c_int,
        arity: libc::c_int,
        what_to_do: libc::c_int,
        pl_file_for_multi: libc::c_int,
    ) -> *mut PredInf;
    fn Pl_Scan_Dynamic_Pred(
        owner_func: libc::c_int,
        owner_arity: libc::c_int,
        dyn_0: *mut DynPInf,
        first_arg_word: WamWord,
        alt_fct: ScanFct,
        alt_fct_type: libc::c_int,
        alt_info_size: libc::c_int,
        alt_info: *mut WamWord,
    ) -> *mut DynCInf;
    fn Pl_Copy_Clause_To_Heap(
        clause: *mut DynCInf,
        head_word: *mut WamWord,
        body_word: *mut WamWord,
    );
    fn Pl_Call_Info_Bip_Name_1(call_info_word: WamWord);
    fn X1_2462635F656D756C6174655F636F6E74__a0();
    fn X1_2463616C6C5F696E7465726E616C5F776974685F637574__a3();
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamCont = CodePtr;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct AtomProp {
    #[bitfield(name = "length", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "op_mask", ty = "libc::c_uint", bits = "16..=19")]
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "20..=21")]
    #[bitfield(name = "needs_quote", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "needs_scan", ty = "libc::c_uint", bits = "23..=23")]
    pub length_op_mask_type_0_needs_quote_needs_scan: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AtomInf {
    pub name: *mut libc::c_char,
    pub hash: libc::c_uint,
    pub prop: AtomProp,
    pub info: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PredInf {
    pub f_n: PlLong,
    pub pl_file: libc::c_int,
    pub pl_line: libc::c_int,
    pub prop: libc::c_int,
    pub codep: *mut PlLong,
    pub dyn_0: *mut PlLong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union BCWord {
    pub t1: C2RustUnnamed_0,
    pub t2: C2RustUnnamed,
    pub word: libc::c_uint,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed {
    #[bitfield(name = "code_op", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "i24", ty = "libc::c_uint", bits = "8..=31")]
    pub code_op_i24: [u8; 4],
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    #[bitfield(name = "code_op", ty = "libc::c_uint", bits = "0..=7")]
    #[bitfield(name = "i8_0", ty = "libc::c_uint", bits = "8..=15")]
    #[bitfield(name = "i16_0", ty = "libc::c_uint", bits = "16..=31")]
    pub code_op_i8_0_i16_0: [u8; 4],
}
pub const SOFT_CUT_X: C2RustUnnamed_1 = 54;
pub const CUT_X: C2RustUnnamed_1 = 52;
pub const GET_CURRENT_CHOICE_X: C2RustUnnamed_1 = 50;
pub const FAIL: C2RustUnnamed_1 = 49;
pub const PROCEED: C2RustUnnamed_1 = 48;
pub const EXECUTE: C2RustUnnamed_1 = 46;
pub const CALL: C2RustUnnamed_1 = 44;
pub const DEALLOCATE: C2RustUnnamed_1 = 43;
pub const ALLOCATE: C2RustUnnamed_1 = 42;
pub const UNIFY_STRUCTURE: C2RustUnnamed_1 = 41;
pub const UNIFY_LIST: C2RustUnnamed_1 = 40;
pub const UNIFY_NIL: C2RustUnnamed_1 = 39;
pub const UNIFY_INTEGER: C2RustUnnamed_1 = 37;
pub const UNIFY_ATOM: C2RustUnnamed_1 = 35;
pub const UNIFY_X_LOCAL_VALUE: C2RustUnnamed_1 = 33;
pub const UNIFY_X_VALUE: C2RustUnnamed_1 = 31;
pub const UNIFY_VOID: C2RustUnnamed_1 = 30;
pub const UNIFY_X_VARIABLE: C2RustUnnamed_1 = 28;
pub const MATH_LOAD_X_VALUE: C2RustUnnamed_1 = 26;
pub const PUT_STRUCTURE: C2RustUnnamed_1 = 25;
pub const PUT_LIST: C2RustUnnamed_1 = 24;
pub const PUT_NIL: C2RustUnnamed_1 = 23;
pub const PUT_FLOAT: C2RustUnnamed_1 = 22;
pub const PUT_INTEGER: C2RustUnnamed_1 = 20;
pub const PUT_ATOM: C2RustUnnamed_1 = 18;
pub const PUT_Y_UNSAFE_VALUE: C2RustUnnamed_1 = 17;
pub const PUT_X_VALUE: C2RustUnnamed_1 = 15;
pub const PUT_VOID: C2RustUnnamed_1 = 14;
pub const PUT_X_VARIABLE: C2RustUnnamed_1 = 12;
pub const GET_STRUCTURE: C2RustUnnamed_1 = 11;
pub const GET_LIST: C2RustUnnamed_1 = 10;
pub const GET_NIL: C2RustUnnamed_1 = 9;
pub const GET_FLOAT: C2RustUnnamed_1 = 8;
pub const GET_INTEGER: C2RustUnnamed_1 = 6;
pub const GET_ATOM: C2RustUnnamed_1 = 4;
pub const GET_X_VALUE: C2RustUnnamed_1 = 2;
pub const GET_X_VARIABLE: C2RustUnnamed_1 = 0;
pub type ScanFct = Option::<unsafe extern "C" fn() -> PlLong>;
pub type DynStamp = PlULong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynpinf {
    pub seq_chain: D2ChHdr,
    pub var_ind_chain: D2ChHdr,
    pub atm_htbl: *mut libc::c_char,
    pub int_htbl: *mut libc::c_char,
    pub lst_ind_chain: D2ChHdr,
    pub stc_htbl: *mut libc::c_char,
    pub arity: libc::c_int,
    pub count_a: libc::c_int,
    pub count_z: libc::c_int,
    pub first_erased_cl: DynCInfP,
    pub next_dyn_with_erase: DynPInfP,
}
pub type DynPInfP = *mut dynpinf;
pub type DynCInfP = *mut dyncinf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyncinf {
    pub seq_chain: D2ChCell,
    pub ind_chain: D2ChCell,
    pub dyn_0: DynPInfP,
    pub p_ind_hdr: *mut D2ChHdr,
    pub p_ind_htbl: *mut *mut libc::c_char,
    pub cl_no: libc::c_int,
    pub pl_file: libc::c_int,
    pub erase_stamp: DynStamp,
    pub next_erased_cl: DynCInfP,
    pub byte_code: *mut libc::c_uint,
    pub term_size: libc::c_int,
    pub term_word: WamWord,
    pub head_word: WamWord,
    pub body_word: WamWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct D2ChHdr {
    pub first: DynCInfP,
    pub last: DynCInfP,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct D2ChCell {
    pub next: DynCInfP,
    pub prev: DynCInfP,
}
pub type DynCInf = dyncinf;
pub type DynPInf = dynpinf;
pub const SOFT_CUT_Y: C2RustUnnamed_1 = 55;
pub const CUT_Y: C2RustUnnamed_1 = 53;
pub const GET_CURRENT_CHOICE_Y: C2RustUnnamed_1 = 51;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C64To32 {
    pub d: libc::c_double,
    pub p: *mut libc::c_int,
    pub l: PlLong,
    pub u: [libc::c_uint; 2],
}
pub const EXECUTE_NATIVE: C2RustUnnamed_1 = 47;
pub const CALL_NATIVE: C2RustUnnamed_1 = 45;
pub const UNIFY_INTEGER_BIG: C2RustUnnamed_1 = 38;
pub const UNIFY_ATOM_BIG: C2RustUnnamed_1 = 36;
pub const UNIFY_Y_LOCAL_VALUE: C2RustUnnamed_1 = 34;
pub const UNIFY_Y_VALUE: C2RustUnnamed_1 = 32;
pub const UNIFY_Y_VARIABLE: C2RustUnnamed_1 = 29;
pub const MATH_LOAD_Y_VALUE: C2RustUnnamed_1 = 27;
pub const PUT_INTEGER_BIG: C2RustUnnamed_1 = 21;
pub const PUT_ATOM_BIG: C2RustUnnamed_1 = 19;
pub const PUT_Y_VALUE: C2RustUnnamed_1 = 16;
pub const PUT_Y_VARIABLE: C2RustUnnamed_1 = 13;
pub const GET_INTEGER_BIG: C2RustUnnamed_1 = 7;
pub const GET_ATOM_BIG: C2RustUnnamed_1 = 5;
pub const GET_Y_VALUE: C2RustUnnamed_1 = 3;
pub const GET_Y_VARIABLE: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const libc::c_void,
    mut __base: *const libc::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut libc::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const libc::c_void = 0 as *const libc::c_void;
    let mut __comparison: libc::c_int = 0;
    __l = 0 as libc::c_int as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as libc::c_int as libc::c_ulong);
        __p = (__base as *const libc::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *mut libc::c_void;
        __comparison = (Some(__compar.unwrap())).unwrap()(__key, __p);
        if __comparison < 0 as libc::c_int {
            __u = __idx;
        } else if __comparison > 0 as libc::c_int {
            __l = __idx.wrapping_add(1 as libc::c_int as libc::c_ulong);
        } else {
            return __p as *mut libc::c_void
        }
    }
    return 0 as *mut libc::c_void;
}
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_byte_len: libc::c_int = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
static mut op_tbl: [BCWord; 100] = [BCWord {
    t1: C2RustUnnamed_0 {
        code_op_i8_0_i16_0: [0; 4],
    },
}; 100];
static mut nb_op: libc::c_int = 0;
static mut bc: *mut BCWord = 0 as *const BCWord as *mut BCWord;
static mut bc_sp: *mut BCWord = 0 as *const BCWord as *mut BCWord;
static mut bc_nb_block: libc::c_int = 0;
static mut atom_dynamic: libc::c_int = 0;
static mut atom_public: libc::c_int = 0;
static mut atom_multifile: libc::c_int = 0;
static mut atom_built_in: libc::c_int = 0;
static mut atom_built_in_fd: libc::c_int = 0;
static mut atom_fail: libc::c_int = 0;
static mut caller_func: libc::c_int = 0;
static mut caller_arity: libc::c_int = 0;
static mut glob_func: libc::c_int = 0;
static mut glob_dyn: *mut DynPInf = 0 as *const DynPInf as *mut DynPInf;
static mut debug_call: Bool = 0;
pub static mut pl_debug_call_code: WamCont = None;
unsafe extern "C" fn Find_Inst_Code_Op(mut inst: libc::c_int) -> libc::c_int {
    let mut p: *mut BCWord = 0 as *mut BCWord;
    let mut w: BCWord = BCWord {
        t1: C2RustUnnamed_0 {
            code_op_i8_0_i16_0: [0; 4],
        },
    };
    (w.t2).set_i24(inst as libc::c_uint);
    p = bsearch(
        &mut w as *mut BCWord as *const libc::c_void,
        op_tbl.as_mut_ptr() as *const libc::c_void,
        nb_op as size_t,
        ::std::mem::size_of::<BCWord>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut BCWord, *mut BCWord) -> libc::c_int>,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
            >,
        >(
            Some(
                Compar_Inst_Code_Op
                    as unsafe extern "C" fn(*mut BCWord, *mut BCWord) -> libc::c_int,
            ),
        ),
    ) as *mut BCWord;
    if p.is_null() {
        Pl_Fatal_Error(
            b"bc_supp: Unknown WAM instruction: %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*pl_atom_tbl.offset(inst as isize)).name,
        );
    }
    return ((*p).t1).code_op() as libc::c_int;
}
unsafe extern "C" fn Compar_Inst_Code_Op(
    mut p1: *mut BCWord,
    mut p2: *mut BCWord,
) -> libc::c_int {
    return ((*p1).t2).i24() as libc::c_int - ((*p2).t2).i24() as libc::c_int;
}
pub unsafe extern "C" fn Pl_BC_Start_Pred_8(
    mut func_word: WamWord,
    mut arity_word: WamWord,
    mut pl_file_word: WamWord,
    mut pl_line_word: WamWord,
    mut sta_dyn_word: WamWord,
    mut pub_priv_word: WamWord,
    mut mono_multi_word: WamWord,
    mut us_blp_bfd_word: WamWord,
) {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut pl_file: libc::c_int = 0;
    let mut pl_line: libc::c_int = 0;
    let mut prop: libc::c_int = 0 as libc::c_int;
    let mut atom: libc::c_int = 0;
    let mut multi: libc::c_int = 0 as libc::c_int;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    func = Pl_Rd_Atom_Check(func_word);
    arity = Pl_Rd_Integer_Check(arity_word) as libc::c_int;
    pl_file = Pl_Rd_Atom_Check(pl_file_word);
    pl_line = Pl_Rd_Integer_Check(pl_line_word) as libc::c_int;
    if Pl_Rd_Atom_Check(sta_dyn_word) == atom_dynamic {
        prop = 2 as libc::c_int | 4 as libc::c_int;
    } else if Pl_Rd_Atom_Check(pub_priv_word) == atom_public {
        prop = 4 as libc::c_int;
    }
    if Pl_Rd_Atom_Check(mono_multi_word) == atom_multifile {
        prop |= 64 as libc::c_int;
        multi = 1 as libc::c_int;
    }
    atom = Pl_Rd_Atom_Check(us_blp_bfd_word);
    if atom == atom_built_in {
        prop |= 8 as libc::c_int;
    } else if atom == atom_built_in_fd {
        prop |= 16 as libc::c_int;
    }
    pred = Pl_Update_Dynamic_Pred(
        func,
        arity,
        0 as libc::c_int,
        if multi != 0 { pl_file } else { -(1 as libc::c_int) },
    );
    if pred.is_null() {
        pred = Pl_Create_Pred(func, arity, pl_file, pl_line, prop, 0 as *mut PlLong);
    } else if multi != 0 {
        (*pred).prop |= prop;
    } else {
        (*pred).pl_file = pl_file;
        (*pred).pl_line = pl_line;
        (*pred).prop = prop;
    }
    caller_func = Pl_Pred_Without_Aux(func, arity, &mut caller_arity);
}
pub unsafe extern "C" fn Pl_BC_Start_Emit_0() {
    bc_sp = bc;
}
pub unsafe extern "C" fn Pl_BC_Stop_Emit_0() {
    let mut i: libc::c_int = 0;
    pl_byte_len = bc_sp.offset_from(bc) as libc::c_long as libc::c_int;
    pl_byte_code = Pl_Malloc_Check(
        (pl_byte_len as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<BCWord>() as libc::c_ulong)
            as libc::c_uint,
        b"bc_supp.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        487 as libc::c_int,
    ) as *mut libc::c_uint;
    i = 0 as libc::c_int;
    while i < pl_byte_len {
        *pl_byte_code.offset(i as isize) = (*bc.offset(i as isize)).word;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn Pl_BC_Emit_Inst_1(mut inst_word: WamWord) {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut op: libc::c_int = 0;
    let mut size_bc: libc::c_int = 0;
    let mut w: BCWord = BCWord {
        t1: C2RustUnnamed_0 {
            code_op_i8_0_i16_0: [0; 4],
        },
    };
    let mut w1: libc::c_uint = 0;
    let mut w2: libc::c_uint = 0;
    let mut w3: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut l: PlLong = 0;
    let mut nb_word: libc::c_int = 0;
    let mut cv: C64To32 = C64To32 { d: 0. };
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    arg_adr = Pl_Rd_Callable_Check(inst_word, &mut func, &mut arity);
    op = Find_Inst_Code_Op(func);
    size_bc = bc_sp.offset_from(bc) as libc::c_long as libc::c_int;
    if size_bc + 3 as libc::c_int >= bc_nb_block * 1024 as libc::c_int {
        bc_nb_block += 1;
        bc_nb_block;
        bc = Pl_Realloc_Check(
            bc as *mut libc::c_char,
            ((bc_nb_block * 1024 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<BCWord>() as libc::c_ulong)
                as libc::c_uint,
            b"bc_supp.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            548 as libc::c_int,
        ) as *mut BCWord;
        bc_sp = bc.offset(size_bc as isize);
    }
    w.word = 0 as libc::c_int as libc::c_uint;
    nb_word = 1 as libc::c_int;
    match op {
        0 | 2 | 12 | 15 | 16 | 26 => {
            let fresh0 = arg_adr;
            arg_adr = arg_adr.offset(1);
            (w.t1).set_i16_0(BC_Arg_X_Or_Y(*fresh0, &mut op) as libc::c_uint);
            (w.t1).set_i8_0(Pl_Rd_Integer(*arg_adr) as libc::c_uint);
        }
        4 | 18 => {
            let fresh1 = arg_adr;
            arg_adr = arg_adr.offset(1);
            w1 = Pl_Rd_Atom(*fresh1) as libc::c_uint;
            if (w1 as PlULong)
                < ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong
            {
                (w.t1).set_i16_0(w1);
            } else {
                op += 1;
                op;
                nb_word = 2 as libc::c_int;
            }
            (w.t1).set_i8_0(Pl_Rd_Integer(*arg_adr) as libc::c_uint);
        }
        6 | 20 => {
            let fresh2 = arg_adr;
            arg_adr = arg_adr.offset(1);
            l = Pl_Rd_Integer(*fresh2);
            if (l as PlULong)
                < ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong
            {
                (w.t1).set_i16_0(l as libc::c_uint);
            } else {
                op += 1;
                op;
                cv.l = l;
                w1 = cv.u[0 as libc::c_int as usize];
                w2 = cv.u[1 as libc::c_int as usize];
                nb_word = 3 as libc::c_int;
            }
            (w.t1).set_i8_0(Pl_Rd_Integer(*arg_adr) as libc::c_uint);
        }
        8 | 22 => {
            nb_word = 3 as libc::c_int;
            let fresh3 = arg_adr;
            arg_adr = arg_adr.offset(1);
            cv.d = Pl_Rd_Float(*fresh3);
            (w.t1).set_i8_0(Pl_Rd_Integer(*arg_adr) as libc::c_uint);
            w1 = cv.u[0 as libc::c_int as usize];
            w2 = cv.u[1 as libc::c_int as usize];
        }
        9 | 10 | 23 | 24 => {
            (w.t1).set_i8_0(Pl_Rd_Integer(*arg_adr) as libc::c_uint);
        }
        11 | 25 => {
            nb_word = 2 as libc::c_int;
            let fresh4 = arg_adr;
            arg_adr = arg_adr.offset(1);
            w1 = BC_Arg_Func_Arity(*fresh4, &mut arity) as libc::c_uint;
            (w.t1).set_i16_0(arity as libc::c_uint);
            (w.t1).set_i8_0(Pl_Rd_Integer(*arg_adr) as libc::c_uint);
        }
        14 => {
            (w.t1).set_i8_0(Pl_Rd_Integer(*arg_adr) as libc::c_uint);
        }
        28 | 31 | 33 | 50 | 52 | 54 => {
            (w.t2).set_i24(BC_Arg_X_Or_Y(*arg_adr, &mut op) as libc::c_uint);
        }
        35 => {
            w1 = Pl_Rd_Atom(*arg_adr) as libc::c_uint;
            if (w1 as PlULong)
                < ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_ulong
            {
                (w.t2).set_i24(w1);
            } else {
                op += 1;
                op;
                nb_word = 2 as libc::c_int;
            }
        }
        37 => {
            let fresh5 = arg_adr;
            arg_adr = arg_adr.offset(1);
            l = Pl_Rd_Integer(*fresh5);
            if (l as PlULong)
                < ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_ulong
            {
                (w.t2).set_i24(l as libc::c_uint);
            } else {
                op += 1;
                op;
                cv.l = l;
                w1 = cv.u[0 as libc::c_int as usize];
                w2 = cv.u[1 as libc::c_int as usize];
                nb_word = 3 as libc::c_int;
            }
        }
        41 => {
            let fresh6 = arg_adr;
            arg_adr = arg_adr.offset(1);
            w1 = BC_Arg_Func_Arity(*fresh6, &mut arity as *mut libc::c_int)
                as libc::c_uint;
            (w.t2).set_i24(arity as libc::c_uint);
            nb_word = 2 as libc::c_int;
        }
        30 | 42 => {
            (w.t2).set_i24(Pl_Rd_Integer(*arg_adr) as libc::c_uint);
        }
        44 | 46 => {
            let fresh7 = arg_adr;
            arg_adr = arg_adr.offset(1);
            func = BC_Arg_Func_Arity(*fresh7, &mut arity);
            w1 = func as libc::c_uint;
            (w.t2).set_i24(arity as libc::c_uint);
            pred = Pl_Lookup_Pred(func, arity);
            if !pred.is_null() && (*pred).prop & 1 as libc::c_int != 0 {
                op += 1;
                op;
                nb_word = 4 as libc::c_int;
                cv.p = (*pred).codep as *mut libc::c_int;
                w2 = cv.u[0 as libc::c_int as usize];
                w3 = cv.u[1 as libc::c_int as usize];
            } else {
                nb_word = 3 as libc::c_int;
                w2 = ((caller_arity as PlULong)
                    << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                    | caller_func as libc::c_ulong) as libc::c_uint;
            }
        }
        _ => {}
    }
    (w.t1).set_code_op(op as libc::c_uint);
    let fresh8 = bc_sp;
    bc_sp = bc_sp.offset(1);
    *fresh8 = w;
    if nb_word >= 2 as libc::c_int {
        (*bc_sp).word = w1;
        bc_sp = bc_sp.offset(1);
        bc_sp;
        if nb_word >= 3 as libc::c_int {
            (*bc_sp).word = w2;
            bc_sp = bc_sp.offset(1);
            bc_sp;
            if nb_word >= 4 as libc::c_int {
                (*bc_sp).word = w3;
                bc_sp = bc_sp.offset(1);
                bc_sp;
            }
        }
    }
}
pub unsafe extern "C" fn Pl_BC_Emit_Inst_Execute_Native(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut codep: *mut PlLong,
) {
    let mut w: BCWord = BCWord {
        t1: C2RustUnnamed_0 {
            code_op_i8_0_i16_0: [0; 4],
        },
    };
    let mut w1: libc::c_uint = 0;
    let mut w2: libc::c_uint = 0;
    let mut w3: libc::c_uint = 0;
    let mut nb_word: libc::c_int = 0;
    let mut cv: C64To32 = C64To32 { d: 0. };
    w1 = func as libc::c_uint;
    (w.t2).set_i24(arity as libc::c_uint);
    nb_word = 4 as libc::c_int;
    cv.p = codep as *mut libc::c_int;
    w2 = cv.u[0 as libc::c_int as usize];
    w3 = cv.u[1 as libc::c_int as usize];
    (w.t1).set_code_op(EXECUTE_NATIVE as libc::c_int as libc::c_uint);
    let fresh9 = bc_sp;
    bc_sp = bc_sp.offset(1);
    *fresh9 = w;
    if nb_word >= 2 as libc::c_int {
        (*bc_sp).word = w1;
        bc_sp = bc_sp.offset(1);
        bc_sp;
        if nb_word >= 3 as libc::c_int {
            (*bc_sp).word = w2;
            bc_sp = bc_sp.offset(1);
            bc_sp;
            if nb_word >= 4 as libc::c_int {
                (*bc_sp).word = w3;
                bc_sp = bc_sp.offset(1);
                bc_sp;
            }
        }
    }
}
unsafe extern "C" fn BC_Arg_X_Or_Y(
    mut arg_word: WamWord,
    mut op: *mut libc::c_int,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = arg_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord;
    if *adr.offset(0 as libc::c_int as isize) as PlULong
        & ((1 as libc::c_int as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        != 'x' as i32 as libc::c_uchar as libc::c_int as libc::c_ulong
    {
        *op += 1;
        *op;
    }
    return Pl_Rd_Integer(*adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize))
        as libc::c_int;
}
unsafe extern "C" fn BC_Arg_Func_Arity(
    mut arg_word: WamWord,
    mut arity: *mut libc::c_int,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut stc_adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = arg_word;
    loop {
        deref_last_word = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word) {
            break;
        }
    }
    stc_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
        as *mut WamWord;
    let mut deref_last_word_0: WamWord = 0;
    word = *stc_adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize);
    loop {
        deref_last_word_0 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_0) {
            break;
        }
    }
    *arity = (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int;
    let mut deref_last_word_1: WamWord = 0;
    word = *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
    loop {
        deref_last_word_1 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_1) {
            break;
        }
    }
    return (word as PlULong >> 3 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn Pl_BC_Call_Terminal_Pred_3(
    mut pred_word: WamWord,
    mut call_info_word: WamWord,
    mut first_call_word: WamWord,
) -> WamCont {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut i: libc::c_int = 0;
    arg_adr = Pl_Rd_Callable_Check(pred_word, &mut func, &mut arity);
    debug_call = (call_info_word
        & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_long
        != 0 as libc::c_int as libc::c_long) as libc::c_int;
    if pl_debug_call_code.is_some() && debug_call != 0
        && first_call_word & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_long
            != 0
    {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = pred_word;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = call_info_word;
        return pl_debug_call_code;
    }
    pred = Pl_Lookup_Pred(func, arity);
    if pred.is_null() {
        if func != atom_fail || arity != 0 as libc::c_int {
            Pl_Call_Info_Bip_Name_1(call_info_word);
            Pl_Unknown_Pred_Error(func, arity);
        }
        return *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord
            as *mut CodePtr);
    }
    i = 0 as libc::c_int;
    while i < arity {
        let fresh10 = arg_adr;
        arg_adr = arg_adr.offset(1);
        *pl_reg_bank.offset(i as isize) = *fresh10;
        i += 1;
        i;
    }
    if (*pred).prop & 1 as libc::c_int != 0 {
        return ::std::mem::transmute::<*mut PlLong, WamCont>((*pred).codep);
    }
    return Pl_BC_Emulate_Pred(func, (*pred).dyn_0 as *mut DynPInf);
}
pub unsafe extern "C" fn Pl_BC_Emulate_Pred(
    mut func: libc::c_int,
    mut dyn_0: *mut DynPInf,
) -> WamCont {
    let mut clause: *mut DynCInf = 0 as *mut DynCInf;
    let mut codep: WamCont = None;
    let mut arity: libc::c_int = 0;
    while !dyn_0.is_null() {
        arity = (*dyn_0).arity;
        *pl_reg_bank.offset(arity as isize) = Pl_Get_Current_Choice();
        *pl_reg_bank.offset((arity + 1 as libc::c_int) as isize) = debug_call as WamWord;
        clause = Pl_Scan_Dynamic_Pred(
            func,
            arity,
            dyn_0,
            *pl_reg_bank.offset(0 as libc::c_int as isize),
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut DynCInf, *mut WamWord) -> WamCont>,
                Option::<unsafe extern "C" fn() -> PlLong>,
            >(
                Some(
                    BC_Emulate_Pred_Alt
                        as unsafe extern "C" fn(*mut DynCInf, *mut WamWord) -> WamCont,
                ),
            ),
            1 as libc::c_int,
            arity + 2 as libc::c_int,
            &mut *pl_reg_bank.offset(0 as libc::c_int as isize),
        );
        if clause.is_null() {
            break;
        }
        codep = BC_Emulate_Clause(clause);
        if codep.is_some() {
            return codep;
        }
        func = glob_func;
        dyn_0 = glob_dyn;
    }
    return *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord
        as *mut CodePtr);
}
unsafe extern "C" fn BC_Emulate_Pred_Alt(
    mut clause: *mut DynCInf,
    mut w: *mut WamWord,
) -> WamCont {
    let mut dyn_0: *mut DynPInf = 0 as *mut DynPInf;
    let mut arity: libc::c_int = 0;
    let mut codep: WamCont = None;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    dyn_0 = (*clause).dyn_0;
    arity = (*dyn_0).arity;
    adr = &mut *pl_reg_bank.offset(0 as libc::c_int as isize) as *mut WamWord;
    loop {
        let fresh11 = w;
        w = w.offset(1);
        let fresh12 = adr;
        adr = adr.offset(1);
        *fresh12 = *fresh11;
        arity -= 1;
        if !(arity >= 0 as libc::c_int) {
            break;
        }
    }
    debug_call = *w as Bool;
    codep = BC_Emulate_Clause(clause);
    return if codep.is_some() { codep } else { Pl_BC_Emulate_Pred(glob_func, glob_dyn) };
}
unsafe extern "C" fn BC_Emulate_Clause(mut clause: *mut DynCInf) -> WamCont {
    let mut current_block: u64;
    let mut head_word: WamWord = 0;
    let mut body_word: WamWord = 0;
    let mut arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut bc_0: *mut BCWord = 0 as *mut BCWord;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    bc_0 = (*clause).byte_code as *mut BCWord;
    if !bc_0.is_null() {
        return BC_Emulate_Byte_Code(bc_0);
    }
    Pl_Copy_Clause_To_Heap(clause, &mut head_word, &mut body_word);
    arg_adr = Pl_Rd_Callable_Check(head_word, &mut func, &mut arity);
    i = 0 as libc::c_int;
    loop {
        if !(i < arity) {
            current_block = 10886091980245723256;
            break;
        }
        let fresh13 = arg_adr;
        arg_adr = arg_adr.offset(1);
        if Pl_Unify(*pl_reg_bank.offset(i as isize), *fresh13) == 0 {
            current_block = 10008731211105882818;
            break;
        }
        i += 1;
        i;
    }
    match current_block {
        10008731211105882818 => {
            return *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord
                as *mut CodePtr);
        }
        _ => {
            *pl_reg_bank
                .offset(2 as libc::c_int as isize) = *pl_reg_bank.offset(arity as isize);
            *pl_reg_bank.offset(0 as libc::c_int as isize) = body_word;
            *pl_reg_bank
                .offset(
                    1 as libc::c_int as isize,
                ) = ((((arity as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                | func as libc::c_ulong) << 1 as libc::c_int
                | debug_call as libc::c_ulong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong) as WamWord;
            return ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_2463616C6C5F696E7465726E616C5F776974685F637574__a3),
                ),
            );
        }
    };
}
unsafe extern "C" fn BC_Emulate_Byte_Code(mut bc_0: *mut BCWord) -> WamCont {
    let mut current_block: u64;
    let mut w: BCWord = BCWord {
        t1: C2RustUnnamed_0 {
            code_op_i8_0_i16_0: [0; 4],
        },
    };
    let mut x0: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w1: libc::c_int = 0;
    let mut l: PlLong = 0;
    let mut codep: WamCont = None;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut cv: C64To32 = C64To32 { d: 0. };
    loop {
        let fresh14 = bc_0;
        bc_0 = bc_0.offset(1);
        w = *fresh14;
        match (w.t1).code_op() as libc::c_int {
            0 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                x = (w.t1).i16_0() as libc::c_int;
                *pl_reg_bank.offset(x as isize) = *pl_reg_bank.offset(x0 as isize);
            }
            1 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                y = (w.t1).i16_0() as libc::c_int;
                *(&mut *(*(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                    .offset((-(4 as libc::c_int) - y) as isize)
                    as *mut WamWord) = *pl_reg_bank.offset(x0 as isize);
            }
            2 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                x = (w.t1).i16_0() as libc::c_int;
                if Pl_Unify(
                    *pl_reg_bank.offset(x as isize),
                    *pl_reg_bank.offset(x0 as isize),
                ) == 0
                {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            3 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                y = (w.t1).i16_0() as libc::c_int;
                if Pl_Unify(
                    *(&mut *(*(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                        .offset((-(4 as libc::c_int) - y) as isize) as *mut WamWord),
                    *pl_reg_bank.offset(x0 as isize),
                ) == 0
                {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            4 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                if Pl_Get_Atom(
                    (w.t1).i16_0() as libc::c_int,
                    *pl_reg_bank.offset(x0 as isize),
                ) == 0
                {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            5 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                w1 = (*bc_0).word as libc::c_int;
                bc_0 = bc_0.offset(1);
                bc_0;
                if Pl_Get_Atom(w1, *pl_reg_bank.offset(x0 as isize)) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            6 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                if Pl_Get_Integer(
                    (w.t1).i16_0() as PlLong,
                    *pl_reg_bank.offset(x0 as isize),
                ) == 0
                {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            7 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                cv.u[0 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                cv.u[1 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                l = cv.l;
                if Pl_Get_Integer(l, *pl_reg_bank.offset(x0 as isize)) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            8 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                cv.u[0 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                cv.u[1 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                if Pl_Get_Float(cv.d, *pl_reg_bank.offset(x0 as isize)) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            9 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                if Pl_Get_Nil(*pl_reg_bank.offset(x0 as isize)) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            10 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                if Pl_Get_List(*pl_reg_bank.offset(x0 as isize)) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            11 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                arity = (w.t1).i16_0() as libc::c_int;
                func = (*bc_0).word as libc::c_int;
                bc_0 = bc_0.offset(1);
                bc_0;
                if Pl_Get_Structure(func, arity, *pl_reg_bank.offset(x0 as isize)) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            12 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                x = (w.t1).i16_0() as libc::c_int;
                let ref mut fresh15 = *pl_reg_bank.offset(x0 as isize);
                *fresh15 = Pl_Put_X_Variable();
                *pl_reg_bank.offset(x as isize) = *fresh15;
            }
            13 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                y = (w.t1).i16_0() as libc::c_int;
                *pl_reg_bank
                    .offset(
                        x0 as isize,
                    ) = Pl_Put_Y_Variable(
                    &mut *(*(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                        .offset((-(4 as libc::c_int) - y) as isize) as *mut WamWord,
                );
            }
            14 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                *pl_reg_bank.offset(x0 as isize) = Pl_Put_X_Variable();
            }
            15 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                x = (w.t1).i16_0() as libc::c_int;
                *pl_reg_bank.offset(x0 as isize) = *pl_reg_bank.offset(x as isize);
            }
            16 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                y = (w.t1).i16_0() as libc::c_int;
                *pl_reg_bank
                    .offset(
                        x0 as isize,
                    ) = *(&mut *(*(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                    .offset((-(4 as libc::c_int) - y) as isize) as *mut WamWord);
            }
            17 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                y = (w.t1).i16_0() as libc::c_int;
                *pl_reg_bank
                    .offset(
                        x0 as isize,
                    ) = Pl_Put_Unsafe_Value(
                    *(&mut *(*(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                        .offset((-(4 as libc::c_int) - y) as isize) as *mut WamWord),
                );
            }
            18 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                *pl_reg_bank
                    .offset(x0 as isize) = Pl_Put_Atom((w.t1).i16_0() as libc::c_int);
            }
            19 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                w1 = (*bc_0).word as libc::c_int;
                bc_0 = bc_0.offset(1);
                bc_0;
                *pl_reg_bank.offset(x0 as isize) = Pl_Put_Atom(w1);
            }
            20 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                *pl_reg_bank
                    .offset(x0 as isize) = Pl_Put_Integer((w.t1).i16_0() as PlLong);
            }
            21 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                cv.u[0 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                cv.u[1 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                l = cv.l;
                *pl_reg_bank.offset(x0 as isize) = Pl_Put_Integer(l);
            }
            22 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                cv.u[0 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                cv.u[1 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                *pl_reg_bank.offset(x0 as isize) = Pl_Put_Float(cv.d);
            }
            23 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                *pl_reg_bank
                    .offset(
                        x0 as isize,
                    ) = (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
            }
            24 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                *pl_reg_bank.offset(x0 as isize) = Pl_Put_List();
            }
            25 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                arity = (w.t1).i16_0() as libc::c_int;
                func = (*bc_0).word as libc::c_int;
                bc_0 = bc_0.offset(1);
                bc_0;
                *pl_reg_bank.offset(x0 as isize) = Pl_Put_Structure(func, arity);
            }
            26 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                x = (w.t1).i16_0() as libc::c_int;
                Pl_Math_Load_Value(
                    *pl_reg_bank.offset(x as isize),
                    &mut *pl_reg_bank.offset(x0 as isize),
                );
            }
            27 => {
                x0 = (w.t1).i8_0() as libc::c_int;
                y = (w.t1).i16_0() as libc::c_int;
                Pl_Math_Load_Value(
                    *(&mut *(*(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                        .offset((-(4 as libc::c_int) - y) as isize) as *mut WamWord),
                    &mut *pl_reg_bank.offset(x0 as isize),
                );
            }
            28 => {
                x = (w.t2).i24() as libc::c_int;
                *pl_reg_bank.offset(x as isize) = Pl_Unify_Variable();
            }
            29 => {
                y = (w.t2).i24() as libc::c_int;
                *(&mut *(*(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                    .offset((-(4 as libc::c_int) - y) as isize)
                    as *mut WamWord) = Pl_Unify_Variable();
            }
            30 => {
                Pl_Unify_Void((w.t2).i24() as libc::c_int);
            }
            31 => {
                x = (w.t2).i24() as libc::c_int;
                if Pl_Unify_Value(*pl_reg_bank.offset(x as isize)) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            32 => {
                y = (w.t2).i24() as libc::c_int;
                if Pl_Unify_Value(
                    *(&mut *(*(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                        .offset((-(4 as libc::c_int) - y) as isize) as *mut WamWord),
                ) == 0
                {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            33 => {
                x = (w.t2).i24() as libc::c_int;
                if Pl_Unify_Local_Value(*pl_reg_bank.offset(x as isize)) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            34 => {
                y = (w.t2).i24() as libc::c_int;
                if Pl_Unify_Local_Value(
                    *(&mut *(*(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                        .offset((-(4 as libc::c_int) - y) as isize) as *mut WamWord),
                ) == 0
                {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            35 => {
                if Pl_Unify_Atom((w.t2).i24() as libc::c_int) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            36 => {
                w1 = (*bc_0).word as libc::c_int;
                bc_0 = bc_0.offset(1);
                bc_0;
                if Pl_Unify_Atom(w1) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            37 => {
                if Pl_Unify_Integer((w.t2).i24() as PlLong) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            38 => {
                cv.u[0 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                cv.u[1 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                l = cv.l;
                if Pl_Unify_Integer(l) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            39 => {
                if Pl_Unify_Nil() == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            40 => {
                if Pl_Unify_List() == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            41 => {
                arity = (w.t2).i24() as libc::c_int;
                func = (*bc_0).word as libc::c_int;
                bc_0 = bc_0.offset(1);
                bc_0;
                if Pl_Unify_Structure(func, arity) == 0 {
                    current_block = 2868733186671755069;
                    break;
                }
            }
            42 => {
                Pl_Allocate((w.t2).i24() as libc::c_int);
            }
            43 => {
                Pl_Deallocate();
            }
            44 => {
                *pl_reg_bank
                    .offset(
                        (256 as libc::c_int + 6 as libc::c_int) as isize,
                    ) = bc_0.offset(2 as libc::c_int as isize) as WamWord
                    | debug_call as libc::c_long;
                let ref mut fresh16 = *(pl_reg_bank as *mut WamCont)
                    .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
                *fresh16 = ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    WamCont,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(X1_2462635F656D756C6174655F636F6E74__a0),
                    ),
                );
                current_block = 8052684671881261617;
                break;
            }
            46 => {
                current_block = 8052684671881261617;
                break;
            }
            45 => {
                arity = (w.t2).i24() as libc::c_int;
                func = (*bc_0).word as libc::c_int;
                bc_0 = bc_0.offset(1);
                bc_0;
                cv.u[0 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                cv.u[1 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                codep = ::std::mem::transmute::<*mut libc::c_int, WamCont>(cv.p);
                *pl_reg_bank
                    .offset(
                        (256 as libc::c_int + 6 as libc::c_int) as isize,
                    ) = bc_0 as WamWord | debug_call as libc::c_long;
                let ref mut fresh17 = *(pl_reg_bank as *mut WamCont)
                    .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
                *fresh17 = ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    WamCont,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(X1_2462635F656D756C6174655F636F6E74__a0),
                    ),
                );
                if pl_debug_call_code.is_some() && debug_call != 0 {
                    Prep_Debug_Call(func, arity, 0 as libc::c_int, 0 as libc::c_int);
                    return pl_debug_call_code;
                }
                return codep;
            }
            47 => {
                arity = (w.t2).i24() as libc::c_int;
                func = (*bc_0).word as libc::c_int;
                bc_0 = bc_0.offset(1);
                bc_0;
                cv.u[0 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                cv.u[1 as libc::c_int as usize] = (*bc_0).word;
                bc_0 = bc_0.offset(1);
                bc_0;
                codep = ::std::mem::transmute::<*mut libc::c_int, WamCont>(cv.p);
                if pl_debug_call_code.is_some() && debug_call != 0 {
                    Prep_Debug_Call(func, arity, 0 as libc::c_int, 0 as libc::c_int);
                    return pl_debug_call_code;
                }
                return codep;
            }
            48 => {
                return *(pl_reg_bank as *mut WamCont)
                    .offset((256 as libc::c_int + 1 as libc::c_int) as isize);
            }
            49 => {
                if pl_debug_call_code.is_some() && debug_call != 0 {
                    Prep_Debug_Call(
                        atom_fail,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    return pl_debug_call_code;
                }
                current_block = 2868733186671755069;
                break;
            }
            50 => {
                x = (w.t2).i24() as libc::c_int;
                *pl_reg_bank.offset(x as isize) = Pl_Get_Current_Choice();
            }
            51 => {
                y = (w.t2).i24() as libc::c_int;
                *(&mut *(*(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                    .offset((-(4 as libc::c_int) - y) as isize)
                    as *mut WamWord) = Pl_Get_Current_Choice();
            }
            52 => {
                x = (w.t2).i24() as libc::c_int;
                Pl_Cut(*pl_reg_bank.offset(x as isize));
            }
            53 => {
                y = (w.t2).i24() as libc::c_int;
                Pl_Cut(
                    *(&mut *(*(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                        .offset((-(4 as libc::c_int) - y) as isize) as *mut WamWord),
                );
            }
            54 => {
                x = (w.t2).i24() as libc::c_int;
                Pl_Soft_Cut(*pl_reg_bank.offset(x as isize));
            }
            55 => {
                y = (w.t2).i24() as libc::c_int;
                Pl_Soft_Cut(
                    *(&mut *(*(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 2 as libc::c_int) as isize))
                        .offset((-(4 as libc::c_int) - y) as isize) as *mut WamWord),
                );
            }
            _ => {
                current_block = 2868733186671755069;
                break;
            }
        }
    }
    match current_block {
        8052684671881261617 => {
            arity = (w.t2).i24() as libc::c_int;
            func = (*bc_0).word as libc::c_int;
            bc_0 = bc_0.offset(1);
            bc_0;
            if pl_debug_call_code.is_some() && debug_call != 0
                && (Pl_Detect_If_Aux_Name(func)).is_null()
            {
                w1 = (*bc_0).word as libc::c_int;
                caller_func = (w1 as PlULong
                    & ((1 as libc::c_int as PlULong)
                        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
                caller_arity = (w1 as PlULong
                    >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                    as libc::c_int;
                Prep_Debug_Call(func, arity, caller_func, caller_arity);
                return pl_debug_call_code;
            }
            pred = Pl_Lookup_Pred(func, arity);
            if pred.is_null() {
                w1 = (*bc_0).word as libc::c_int;
                caller_func = (w1 as PlULong
                    & ((1 as libc::c_int as PlULong)
                        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
                caller_arity = (w1 as PlULong
                    >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                    as libc::c_int;
                Pl_Set_Bip_Name_2(
                    (((caller_func as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord,
                    ((caller_arity as PlULong) << 3 as libc::c_int
                        | 0x7 as libc::c_int as PlULong) as WamWord,
                );
                Pl_Unknown_Pred_Error(func, arity);
            } else {
                glob_func = func;
                glob_dyn = (*pred).dyn_0 as *mut DynPInf;
                return None;
            }
        }
        _ => {}
    }
    return *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord
        as *mut CodePtr);
}
pub unsafe extern "C" fn Pl_BC_Emulate_Cont_0() -> WamCont {
    let mut codep: WamCont = None;
    let mut bc_0: *mut BCWord = 0 as *mut BCWord;
    debug_call = (*pl_reg_bank.offset((256 as libc::c_int + 6 as libc::c_int) as isize)
        & 1 as libc::c_int as libc::c_long) as Bool;
    bc_0 = ((*pl_reg_bank.offset((256 as libc::c_int + 6 as libc::c_int) as isize)
        >> 1 as libc::c_int) << 1 as libc::c_int) as *mut BCWord;
    codep = BC_Emulate_Byte_Code(bc_0);
    return if codep.is_some() { codep } else { Pl_BC_Emulate_Pred(glob_func, glob_dyn) };
}
unsafe extern "C" fn Prep_Debug_Call(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut caller_func_0: libc::c_int,
    mut caller_arity_0: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut word: WamWord = 0;
    if arity == 0 as libc::c_int {
        *pl_reg_bank
            .offset(
                0 as libc::c_int as isize,
            ) = (((func as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    } else {
        word = (H as PlLong as libc::c_ulong).wrapping_add(0x2 as libc::c_int as PlULong)
            as WamWord;
        let fresh18 = H;
        H = H.offset(1);
        *fresh18 = ((arity as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
            | func as libc::c_ulong) as WamWord;
        i = 0 as libc::c_int;
        while i < arity {
            let fresh19 = H;
            H = H.offset(1);
            *fresh19 = *pl_reg_bank.offset(i as isize);
            i += 1;
            i;
        }
        *pl_reg_bank.offset(0 as libc::c_int as isize) = word;
    }
    *pl_reg_bank
        .offset(
            1 as libc::c_int as isize,
        ) = ((((caller_arity_0 as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong)
        | caller_func_0 as libc::c_ulong) << 1 as libc::c_int
        | debug_call as libc::c_ulong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
}

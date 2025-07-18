use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Call_Prolog(codep: CodePtr) -> libc::c_int;
    fn Pl_Call_Prolog_Next_Sol(query_b: *mut WamWord) -> libc::c_int;
    fn Pl_Keep_Rest_For_Prolog(query_b: *mut WamWord);
    fn Pl_Execute_A_Continuation(codep: CodePtr);
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_max_atom: PlULong;
    static mut pl_atom_void: libc::c_int;
    static mut pl_atom_false: libc::c_int;
    static mut pl_atom_true: libc::c_int;
    static mut pl_atom_end_of_file: libc::c_int;
    fn Pl_Lookup_Pred(func: libc::c_int, arity: libc::c_int) -> *mut PredInf;
    fn Pl_Strdup_Check(
        str: *mut libc::c_char,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Delete_Choice_Point(arity: libc::c_int);
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Unify_Occurs_Check(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Blt_Var(x: WamWord) -> Bool;
    fn Pl_Blt_Non_Var(x: WamWord) -> Bool;
    fn Pl_Blt_Atom(x: WamWord) -> Bool;
    fn Pl_Blt_Integer(x: WamWord) -> Bool;
    fn Pl_Blt_Float(x: WamWord) -> Bool;
    fn Pl_Blt_Number(x: WamWord) -> Bool;
    fn Pl_Blt_Atomic(x: WamWord) -> Bool;
    fn Pl_Blt_Compound(x: WamWord) -> Bool;
    fn Pl_Blt_Callable(x: WamWord) -> Bool;
    fn Pl_Blt_Fd_Var(x: WamWord) -> Bool;
    fn Pl_Blt_Non_Fd_Var(x: WamWord) -> Bool;
    fn Pl_Blt_Generic_Var(x: WamWord) -> Bool;
    fn Pl_Blt_Non_Generic_Var(x: WamWord) -> Bool;
    fn Pl_Blt_List(x: WamWord) -> Bool;
    fn Pl_Blt_Partial_List(x: WamWord) -> Bool;
    fn Pl_Blt_List_Or_Partial_List(x: WamWord) -> Bool;
    fn Pl_Blt_Term_Eq(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Term_Neq(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Term_Lt(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Term_Lte(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Term_Gt(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Term_Gte(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Compare(cmp_word: WamWord, x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Arg(
        arg_no_word: WamWord,
        term_word: WamWord,
        sub_term_word: WamWord,
    ) -> Bool;
    fn Pl_Blt_Functor(
        term_word: WamWord,
        functor_word: WamWord,
        arity_word: WamWord,
    ) -> Bool;
    fn Pl_Blt_Univ(term_word: WamWord, list_word: WamWord) -> Bool;
    fn Pl_Math_Load_Value(start_word: WamWord, word_adr: *mut WamWord);
    fn Pl_Blt_Neq(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Lt(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Lte(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Gt(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Blt_Gte(x: WamWord, y: WamWord) -> Bool;
    static mut pl_flag_syntax_error: *mut FlagInf;
    fn Pl_Syntax_Error(flag_value: libc::c_int);
    fn Pl_Set_Last_Syntax_Error(
        file_name: *mut libc::c_char,
        err_line: libc::c_int,
        err_col: libc::c_int,
        err_msg: *mut libc::c_char,
    );
    fn Pl_Get_Current_Bip(arity: *mut libc::c_int) -> libc::c_int;
    fn Pl_Blt_Eq(x: WamWord, y: WamWord) -> Bool;
    fn X1_2463616C6C5F696E7465726E616C__a2();
    fn X1_247468726F775F696E7465726E616C__a2();
    fn X1_24706C5F71756572795F7265636F7665725F616C74__a0();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
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
pub type PlTerm = WamWord;
pub type PlBool = libc::c_uint;
pub const PL_TRUE: PlBool = 1;
pub const PL_FALSE: PlBool = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PlFIOArg {
    pub is_var: Bool,
    pub unify: Bool,
    pub value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub l: PlLong,
    pub s: *mut libc::c_char,
    pub d: libc::c_double,
}
pub type FlagInf = flag_inf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flag_inf {
    pub atom_name: libc::c_int,
    pub modifiable: Bool,
    pub type_0: FlagType,
    pub value: PlLong,
    pub fct_get: FlagFctGet,
    pub fct_chk: FlagFctChk,
    pub fct_set: FlagFctSet,
}
pub type FlagFctSet = Option::<unsafe extern "C" fn(FlagInfP, WamWord) -> Bool>;
pub type FlagInfP = *mut flag_inf;
pub type FlagFctChk = Option::<unsafe extern "C" fn(FlagInfP, WamWord, WamWord) -> Bool>;
pub type FlagFctGet = Option::<unsafe extern "C" fn(FlagInfP) -> WamWord>;
pub type FlagType = libc::c_uint;
pub const PF_TYPE_ANY: FlagType = 7;
pub const PF_TYPE_QUOTES: FlagType = 6;
pub const PF_TYPE_ERR: FlagType = 5;
pub const PF_TYPE_ON_OFF: FlagType = 4;
pub const PF_TYPE_BOOL: FlagType = 3;
pub const PF_TYPE_ROUND: FlagType = 2;
pub const PF_TYPE_ATOM: FlagType = 1;
pub const PF_TYPE_INTEGER: FlagType = 0;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_foreign_bkt_counter: libc::c_int = 0;
pub static mut pl_foreign_bkt_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_foreign_long: [PlLong; 256] = [0; 256];
pub static mut pl_foreign_double: [libc::c_double; 256] = [0.; 256];
pub static mut pl_base_fl: *mut PlLong = unsafe { pl_foreign_long.as_ptr() as *mut _ };
pub static mut pl_base_fd: *mut libc::c_double = unsafe {
    pl_foreign_double.as_ptr() as *mut _
};
static mut fio_arg_array: [PlFIOArg; 256] = [PlFIOArg {
    is_var: 0,
    unify: 0,
    value: C2RustUnnamed { l: 0 },
}; 256];
static mut query_stack: [*mut WamWord; 128] = [0 as *const WamWord as *mut WamWord; 128];
static mut query_stack_top: *mut *mut WamWord = unsafe {
    query_stack.as_ptr() as *mut _
};
static mut goal_H: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut pl_query_top_b: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut pl_query_exception: WamWord = 0;
pub unsafe extern "C" fn Pl_Foreign_Create_Choice(
    mut codep_alt: CodePtr,
    mut arity: libc::c_int,
    mut choice_size: libc::c_int,
) {
    *pl_reg_bank.offset(arity as isize) = -(1 as libc::c_int) as WamWord;
    Pl_Create_Choice_Point(codep_alt, arity + 1 as libc::c_int + choice_size);
}
pub unsafe extern "C" fn Pl_Foreign_Update_Choice(
    mut codep_alt: CodePtr,
    mut arity: libc::c_int,
    mut choice_size: libc::c_int,
) {
    pl_foreign_bkt_counter = (*(&mut *B.offset((-(9 as libc::c_int) - arity) as isize)
        as *mut WamWord) + 1 as libc::c_int as libc::c_long) as libc::c_int;
    *(&mut *B.offset((-(9 as libc::c_int) - arity) as isize)
        as *mut WamWord) = pl_foreign_bkt_counter as WamWord;
    pl_foreign_bkt_buffer = &mut *B
        .offset((-(9 as libc::c_int) - (arity + choice_size)) as isize) as *mut WamWord
        as *mut libc::c_char;
    if pl_foreign_bkt_counter > 0 as libc::c_int {
        Pl_Update_Choice_Point(codep_alt, arity);
    }
}
pub unsafe extern "C" fn Pl_Foreign_Jump_Ret(mut codep: CodePtr) -> CodePtr {
    return codep;
}
pub unsafe extern "C" fn Pl_Foreign_Rd_IO_Arg(
    mut arg_long: libc::c_int,
    mut start_word: WamWord,
    mut rd_fct: Option::<unsafe extern "C" fn() -> PlLong>,
    mut fio_arg_index: libc::c_int,
) -> *mut PlFIOArg {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut fa: *mut PlFIOArg = fio_arg_array
        .as_mut_ptr()
        .offset(fio_arg_index as isize);
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    (*fa)
        .unify = (tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong)
        as libc::c_int;
    (*fa).is_var = (*fa).unify;
    if rd_fct.is_none() {
        (*fa).value.l = word;
    } else if (*fa).is_var == 0 {
        if arg_long != 0 {
            (*fa)
                .value
                .l = ::std::mem::transmute::<
                _,
                fn(_) -> PlLong,
            >((Some(rd_fct.unwrap())).unwrap())(word);
            if arg_long == 2 as libc::c_int {
                (*fa)
                    .value
                    .s = Pl_Strdup_Check(
                    (*fa).value.s,
                    b"foreign_supp.c\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    196 as libc::c_int,
                );
            }
        } else {
            (*fa)
                .value
                .d = ::std::mem::transmute::<
                _,
                fn(_) -> libc::c_double,
            >(
                (Some(
                    (::std::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> PlLong>,
                        Option::<unsafe extern "C" fn() -> libc::c_double>,
                    >(rd_fct))
                        .unwrap(),
                ))
                    .unwrap(),
            )(word);
        }
    }
    return fa;
}
pub unsafe extern "C" fn Pl_Foreign_Un_IO_Arg(
    mut arg_long: libc::c_int,
    mut un_fct: Option::<unsafe extern "C" fn() -> Bool>,
    mut fa: *mut PlFIOArg,
    mut start_word: WamWord,
) -> Bool {
    if (*fa).unify == 0 {
        return 1 as libc::c_int;
    }
    if arg_long != 0 {
        return ::std::mem::transmute::<
            _,
            fn(_, _) -> Bool,
        >((Some(un_fct.unwrap())).unwrap())((*fa).value.l, start_word);
    }
    return ::std::mem::transmute::<
        _,
        fn(_, _) -> Bool,
    >((Some(un_fct.unwrap())).unwrap())((*fa).value.d, start_word);
}
pub unsafe extern "C" fn Pl_Emit_Syntax_Error(
    mut file_name: *mut libc::c_char,
    mut err_line: libc::c_int,
    mut err_col: libc::c_int,
    mut err_msg: *mut libc::c_char,
) {
    Pl_Set_Last_Syntax_Error(file_name, err_line, err_col, err_msg);
    Pl_Syntax_Error((*pl_flag_syntax_error).value as libc::c_int);
}
unsafe extern "C" fn Prepare_Call(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg_adr: *mut WamWord,
) -> CodePtr {
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut w: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    let mut bip_func: libc::c_int = 0;
    let mut bip_arity: libc::c_int = 0;
    pred = Pl_Lookup_Pred(func, arity);
    if pred.is_null() || (*pred).prop & 1 as libc::c_int == 0
        || (*pred).prop & 32 as libc::c_int != 0
    {
        if arity == 0 as libc::c_int {
            *pl_reg_bank
                .offset(
                    0 as libc::c_int as isize,
                ) = (((func as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
        } else {
            w = goal_H;
            *pl_reg_bank
                .offset(
                    0 as libc::c_int as isize,
                ) = (w as PlLong as libc::c_ulong)
                .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
            let fresh0 = w;
            w = w.offset(1);
            *fresh0 = ((arity as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                | func as libc::c_ulong) as WamWord;
            i = 0 as libc::c_int;
            while i < arity {
                let fresh1 = arg_adr;
                arg_adr = arg_adr.offset(1);
                let fresh2 = w;
                w = w.offset(1);
                *fresh2 = *fresh1;
                i += 1;
                i;
            }
        }
        bip_func = Pl_Get_Current_Bip(&mut bip_arity);
        *pl_reg_bank
            .offset(
                1 as libc::c_int as isize,
            ) = ((((bip_arity as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
            | bip_func as libc::c_ulong) << 1 as libc::c_int
            | 0 as libc::c_int as libc::c_ulong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord;
        return ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463616C6C5F696E7465726E616C__a2),
            ),
        );
    }
    i = 0 as libc::c_int;
    while i < arity {
        let fresh3 = arg_adr;
        arg_adr = arg_adr.offset(1);
        *pl_reg_bank.offset(i as isize) = *fresh3;
        i += 1;
        i;
    }
    return ::std::mem::transmute::<*mut PlLong, CodePtr>((*pred).codep);
}
pub unsafe extern "C" fn Pl_Exec_Continuation(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg_adr: *mut WamWord,
) {
    Pl_Execute_A_Continuation(Prepare_Call(func, arity, arg_adr));
}
pub unsafe extern "C" fn Pl_Throw(mut ball_word: WamWord) {
    let mut bip_func: libc::c_int = 0;
    let mut bip_arity: libc::c_int = 0;
    bip_func = Pl_Get_Current_Bip(&mut bip_arity);
    *pl_reg_bank.offset(0 as libc::c_int as isize) = ball_word;
    *pl_reg_bank
        .offset(
            1 as libc::c_int as isize,
        ) = ((((bip_arity as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong)
        | bip_func as libc::c_ulong) << 1 as libc::c_int
        | 0 as libc::c_int as libc::c_ulong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_247468726F775F696E7465726E616C__a2),
        ),
    );
}
pub unsafe extern "C" fn Pl_Query_Begin(mut recoverable: Bool) {
    if query_stack_top.offset_from(query_stack.as_mut_ptr()) as libc::c_long
        >= 128 as libc::c_int as libc::c_long
    {
        Pl_Fatal_Error(
            b"too many nested Pl_Query_Start() (max: %d)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            128 as libc::c_int,
        );
    }
    if recoverable != 0 {
        Pl_Create_Choice_Point(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_24706C5F71756572795F7265636F7665725F616C74__a0),
            ),
            0 as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn Pl_Query_Call(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg_adr: *mut WamWord,
) -> libc::c_int {
    pl_query_top_b = B;
    let fresh4 = query_stack_top;
    query_stack_top = query_stack_top.offset(1);
    *fresh4 = pl_query_top_b;
    pl_query_exception = pl_atom_void as WamWord;
    return Pl_Call_Prolog(Prepare_Call(func, arity, arg_adr));
}
pub unsafe extern "C" fn Pl_Query_Start(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arg_adr: *mut WamWord,
    mut recoverable: Bool,
) -> libc::c_int {
    Pl_Query_Begin(recoverable);
    return Pl_Query_Call(func, arity, arg_adr);
}
pub unsafe extern "C" fn Pl_Query_Recover_Alt_0() {
    Pl_Delete_Choice_Point(0 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Query_Next_Solution() -> libc::c_int {
    if query_stack_top == query_stack.as_mut_ptr() {
        Pl_Fatal_Error(
            b"Pl_Query_Next_Solution() but no query remaining\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    pl_query_exception = pl_atom_void as WamWord;
    return Pl_Call_Prolog_Next_Sol(pl_query_top_b);
}
pub unsafe extern "C" fn Pl_Query_End(mut op: libc::c_int) {
    let mut query_b: *mut WamWord = 0 as *mut WamWord;
    let mut prev_b: *mut WamWord = 0 as *mut WamWord;
    let mut b: *mut WamWord = 0 as *mut WamWord;
    let mut recoverable: Bool = 0;
    if query_stack_top == query_stack.as_mut_ptr() {
        Pl_Fatal_Error(
            b"Pl_Query_End() but no query remaining\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    query_stack_top = query_stack_top.offset(-1);
    query_b = *query_stack_top;
    pl_query_top_b = *query_stack_top.offset(-(1 as libc::c_int) as isize);
    recoverable = (*(&mut *query_b.offset(-(1 as libc::c_int) as isize) as *mut WamWord
        as *mut CodePtr)
        == Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F71756572795F7265636F7665725F616C74__a0),
        )) as libc::c_int;
    prev_b = *(&mut *query_b.offset(-(5 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    match op {
        0 => {
            B = query_b;
            let ref mut fresh5 = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
            *fresh5 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
            if recoverable == 0 {
                Pl_Fatal_Error(
                    b"Pl_Query_End(PL_RECOVER) but unrecoverable query\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            Pl_Delete_Choice_Point(0 as libc::c_int);
        }
        1 => {
            B = (if recoverable != 0 { prev_b } else { query_b });
            let ref mut fresh6 = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
            *fresh6 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
        }
        _ => {
            if recoverable != 0 {
                if B == query_b {
                    B = prev_b;
                    let ref mut fresh7 = *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
                    *fresh7 = *(&mut *B.offset(-(6 as libc::c_int) as isize)
                        as *mut WamWord as *mut *mut WamWord);
                } else {
                    b = B;
                    while b > query_b {
                        if *(&mut *b.offset(-(5 as libc::c_int) as isize) as *mut WamWord
                            as *mut *mut WamWord) == query_b
                        {
                            let ref mut fresh8 = *(&mut *b
                                .offset(-(5 as libc::c_int) as isize) as *mut WamWord
                                as *mut *mut WamWord);
                            *fresh8 = prev_b;
                        }
                        b = *(&mut *b.offset(-(5 as libc::c_int) as isize)
                            as *mut WamWord as *mut *mut WamWord);
                    }
                }
            }
            Pl_Keep_Rest_For_Prolog(query_b);
        }
    };
}
pub unsafe extern "C" fn Pl_Get_Exception() -> WamWord {
    return pl_query_exception;
}
pub unsafe extern "C" fn Pl_No_More_Choice() {
    B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
    let ref mut fresh9 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
    *fresh9 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
        as *mut *mut WamWord);
}
pub unsafe extern "C" fn Pl_Type_Of_Term(mut start_word: WamWord) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    return tag_mask as libc::c_int;
}
pub unsafe extern "C" fn Pl_Atom_Name(mut atom: libc::c_int) -> *mut libc::c_char {
    return (*pl_atom_tbl.offset(atom as isize)).name;
}
pub unsafe extern "C" fn Pl_Atom_Length(mut atom: libc::c_int) -> libc::c_int {
    return ((*pl_atom_tbl.offset(atom as isize)).prop).length() as libc::c_int;
}
pub unsafe extern "C" fn Pl_Atom_Needs_Quote(mut atom: libc::c_int) -> Bool {
    return ((*pl_atom_tbl.offset(atom as isize)).prop).needs_quote() as Bool;
}
pub unsafe extern "C" fn Pl_Atom_Needs_Scan(mut atom: libc::c_int) -> Bool {
    return ((*pl_atom_tbl.offset(atom as isize)).prop).needs_scan() as Bool;
}
pub unsafe extern "C" fn Pl_Is_Valid_Atom(mut atom: libc::c_int) -> Bool {
    return ((atom as PlULong) < pl_max_atom
        && !((*pl_atom_tbl.offset(atom as isize)).name).is_null()) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Atom_Char(mut c: libc::c_char) -> libc::c_int {
    return c as libc::c_uchar as libc::c_int;
}
pub unsafe extern "C" fn Pl_Atom_Nil() -> libc::c_int {
    return 256 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Atom_False() -> libc::c_int {
    return pl_atom_false;
}
pub unsafe extern "C" fn Pl_Atom_True() -> libc::c_int {
    return pl_atom_true;
}
pub unsafe extern "C" fn Pl_Atom_End_Of_File() -> libc::c_int {
    return pl_atom_end_of_file;
}
pub unsafe extern "C" fn Pl_Unif(mut term1: PlTerm, mut term2: PlTerm) -> PlBool {
    return Pl_Unify(term1, term2) as PlBool;
}
pub unsafe extern "C" fn Pl_Unif_With_Occurs_Check(
    mut term1: PlTerm,
    mut term2: PlTerm,
) -> PlBool {
    return Pl_Unify_Occurs_Check(term1, term2) as PlBool;
}
pub unsafe extern "C" fn Pl_Builtin_Var(mut term: WamWord) -> Bool {
    return Pl_Blt_Var(term);
}
pub unsafe extern "C" fn Pl_Builtin_Non_Var(mut term: WamWord) -> Bool {
    return Pl_Blt_Non_Var(term);
}
pub unsafe extern "C" fn Pl_Builtin_Atom(mut term: WamWord) -> Bool {
    return Pl_Blt_Atom(term);
}
pub unsafe extern "C" fn Pl_Builtin_Integer(mut term: WamWord) -> Bool {
    return Pl_Blt_Integer(term);
}
pub unsafe extern "C" fn Pl_Builtin_Float(mut term: WamWord) -> Bool {
    return Pl_Blt_Float(term);
}
pub unsafe extern "C" fn Pl_Builtin_Number(mut term: WamWord) -> Bool {
    return Pl_Blt_Number(term);
}
pub unsafe extern "C" fn Pl_Builtin_Atomic(mut term: WamWord) -> Bool {
    return Pl_Blt_Atomic(term);
}
pub unsafe extern "C" fn Pl_Builtin_Compound(mut term: WamWord) -> Bool {
    return Pl_Blt_Compound(term);
}
pub unsafe extern "C" fn Pl_Builtin_Callable(mut term: WamWord) -> Bool {
    return Pl_Blt_Callable(term);
}
pub unsafe extern "C" fn Pl_Builtin_Fd_Var(mut term: WamWord) -> Bool {
    return Pl_Blt_Fd_Var(term);
}
pub unsafe extern "C" fn Pl_Builtin_Non_Fd_Var(mut term: WamWord) -> Bool {
    return Pl_Blt_Non_Fd_Var(term);
}
pub unsafe extern "C" fn Pl_Builtin_Generic_Var(mut term: WamWord) -> Bool {
    return Pl_Blt_Generic_Var(term);
}
pub unsafe extern "C" fn Pl_Builtin_Non_Generic_Var(mut term: WamWord) -> Bool {
    return Pl_Blt_Non_Generic_Var(term);
}
pub unsafe extern "C" fn Pl_Builtin_List(mut term: WamWord) -> Bool {
    return Pl_Blt_List(term);
}
pub unsafe extern "C" fn Pl_Builtin_Partial_List(mut term: WamWord) -> Bool {
    return Pl_Blt_Partial_List(term);
}
pub unsafe extern "C" fn Pl_Builtin_List_Or_Partial_List(mut term: WamWord) -> Bool {
    return Pl_Blt_List_Or_Partial_List(term);
}
pub unsafe extern "C" fn Pl_Builtin_Term_Eq(
    mut term1: WamWord,
    mut term2: WamWord,
) -> Bool {
    return Pl_Blt_Term_Eq(term1, term2);
}
pub unsafe extern "C" fn Pl_Builtin_Term_Neq(
    mut term1: WamWord,
    mut term2: WamWord,
) -> Bool {
    return Pl_Blt_Term_Neq(term1, term2);
}
pub unsafe extern "C" fn Pl_Builtin_Term_Lt(
    mut term1: WamWord,
    mut term2: WamWord,
) -> Bool {
    return Pl_Blt_Term_Lt(term1, term2);
}
pub unsafe extern "C" fn Pl_Builtin_Term_Lte(
    mut term1: WamWord,
    mut term2: WamWord,
) -> Bool {
    return Pl_Blt_Term_Lte(term1, term2);
}
pub unsafe extern "C" fn Pl_Builtin_Term_Gt(
    mut term1: WamWord,
    mut term2: WamWord,
) -> Bool {
    return Pl_Blt_Term_Gt(term1, term2);
}
pub unsafe extern "C" fn Pl_Builtin_Term_Gte(
    mut term1: WamWord,
    mut term2: WamWord,
) -> Bool {
    return Pl_Blt_Term_Gte(term1, term2);
}
pub unsafe extern "C" fn Pl_Builtin_Compare(
    mut cmp_word: WamWord,
    mut x: WamWord,
    mut y: WamWord,
) -> Bool {
    return Pl_Blt_Compare(cmp_word, x, y);
}
pub unsafe extern "C" fn Pl_Builtin_Arg(
    mut arg_no_word: WamWord,
    mut term_word: WamWord,
    mut sub_term_word: WamWord,
) -> Bool {
    return Pl_Blt_Arg(arg_no_word, term_word, sub_term_word);
}
pub unsafe extern "C" fn Pl_Builtin_Functor(
    mut term_word: WamWord,
    mut functor_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    return Pl_Blt_Functor(term_word, functor_word, arity_word);
}
pub unsafe extern "C" fn Pl_Builtin_Univ(
    mut term_word: WamWord,
    mut list_word: WamWord,
) -> Bool {
    return Pl_Blt_Univ(term_word, list_word);
}
pub unsafe extern "C" fn Pl_Builtin_Eq(mut expr1: WamWord, mut expr2: WamWord) -> Bool {
    return Pl_Blt_Eq(expr1, expr2);
}
pub unsafe extern "C" fn Pl_Builtin_Neq(mut expr1: WamWord, mut expr2: WamWord) -> Bool {
    return Pl_Blt_Neq(expr1, expr2);
}
pub unsafe extern "C" fn Pl_Builtin_Lt(mut expr1: WamWord, mut expr2: WamWord) -> Bool {
    return Pl_Blt_Lt(expr1, expr2);
}
pub unsafe extern "C" fn Pl_Builtin_Lte(mut expr1: WamWord, mut expr2: WamWord) -> Bool {
    return Pl_Blt_Lte(expr1, expr2);
}
pub unsafe extern "C" fn Pl_Builtin_Gt(mut expr1: WamWord, mut expr2: WamWord) -> Bool {
    return Pl_Blt_Gt(expr1, expr2);
}
pub unsafe extern "C" fn Pl_Builtin_Gte(mut expr1: WamWord, mut expr2: WamWord) -> Bool {
    return Pl_Blt_Gte(expr1, expr2);
}
pub unsafe extern "C" fn Pl_Math_Evaluate(mut expr: WamWord, mut result: *mut WamWord) {
    Pl_Math_Load_Value(expr, result);
}

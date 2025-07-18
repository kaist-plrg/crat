use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn Pl_Hash_Alloc_Table(
        tbl_size: libc::c_int,
        elem_size: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Hash_Free_Table(tbl: *mut libc::c_char);
    fn Pl_Hash_Insert(
        tbl: *mut libc::c_char,
        elem: *mut libc::c_char,
        replace: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Hash_Find(tbl: *mut libc::c_char, key: PlLong) -> *mut libc::c_char;
    fn Pl_Hash_Delete(tbl: *mut libc::c_char, key: PlLong) -> *mut libc::c_char;
    static mut pl_stk_tbl: [InfStack; 0];
    static mut pl_atom_void: libc::c_int;
    fn Pl_Create_Pred(
        func: libc::c_int,
        arity: libc::c_int,
        pl_file: libc::c_int,
        pl_line: libc::c_int,
        prop: libc::c_int,
        codep: *mut PlLong,
    ) -> *mut PredInf;
    fn Pl_Lookup_Pred(func: libc::c_int, arity: libc::c_int) -> *mut PredInf;
    fn Pl_Delete_Pred(func: libc::c_int, arity: libc::c_int);
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Extend_Table_If_Needed(hash_tbl: *mut *mut libc::c_char);
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Rd_Callable_Check(
        start_word: WamWord,
        func: *mut libc::c_int,
        arity: *mut libc::c_int,
    ) -> *mut WamWord;
    fn Pl_Term_Size(start_word: WamWord) -> libc::c_int;
    fn Pl_Copy_Term(dst_adr: *mut WamWord, src_adr: *mut WamWord);
    fn Pl_Copy_Contiguous_Term(dst_adr: *mut WamWord, src_adr: *mut WamWord);
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_stm_stdin: libc::c_int;
    static mut pl_permission_operation_modify: libc::c_int;
    static mut pl_permission_type_static_procedure: libc::c_int;
    fn Pl_Get_Current_Bip(arity: *mut libc::c_int) -> libc::c_int;
    fn Pl_Err_Permission(atom_oper: libc::c_int, atom_perm: libc::c_int, term: WamWord);
    static mut pl_atom_user_input: libc::c_int;
    fn X1_247363616E5F64796E5F746573745F616C74__a0();
    fn X1_247363616E5F64796E5F6A756D705F616C74__a0();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfStack {
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub env_var_name: *mut libc::c_char,
    pub p_def_size: *mut PlLong,
    pub default_size: libc::c_int,
    pub size: libc::c_int,
    pub stack: *mut WamWord,
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct StmProp {
    #[bitfield(name = "mode", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "input", ty = "libc::c_uint", bits = "2..=2")]
    #[bitfield(name = "output", ty = "libc::c_uint", bits = "3..=3")]
    #[bitfield(name = "text", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "reposition", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "eof_action", ty = "libc::c_uint", bits = "6..=7")]
    #[bitfield(name = "buffering", ty = "libc::c_uint", bits = "8..=9")]
    #[bitfield(name = "special_close", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "other", ty = "libc::c_uint", bits = "11..=15")]
    pub mode_input_output_text_reposition_eof_action_buffering_special_close_other: [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PbStk {
    pub buff: [libc::c_int; 8],
    pub ptr: *mut libc::c_int,
    pub nb_elems: libc::c_int,
}
pub type StmFct = Option::<unsafe extern "C" fn() -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stm_lst {
    pub stm: libc::c_int,
    pub next: PStmLst,
}
pub type PStmLst = *mut stm_lst;
pub type StmLst = stm_lst;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stm_inf {
    pub atom_file_name: libc::c_int,
    pub file: PlLong,
    pub prop: StmProp,
    pub mirror: *mut StmLst,
    pub mirror_of: *mut StmLst,
    pub fct_getc: StmFct,
    pub fct_putc: StmFct,
    pub fct_flush: StmFct,
    pub fct_close: StmFct,
    pub fct_tell: StmFct,
    pub fct_seek: StmFct,
    pub fct_clearerr: StmFct,
    pub eof_reached: Bool,
    pub pb_char: PbStk,
    pub char_count: PlLong,
    pub line_count: PlLong,
    pub line_pos: PlLong,
    pub pb_line_pos: PbStk,
}
pub type StmInf = stm_inf;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DSwtInf {
    pub key: PlLong,
    pub ind_chain: D2ChHdr,
}
pub type DynPInf = dynpinf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DynScan {
    pub alt_fct: ScanFct,
    pub alt_size_info: libc::c_int,
    pub owner_func: libc::c_int,
    pub owner_arity: libc::c_int,
    pub dyn_0: *mut DynPInf,
    pub stop_cl_no: libc::c_int,
    pub erase_stamp: DynStamp,
    pub xxx_is_seq_chain: Bool,
    pub xxx_ind_chain: *mut DynCInf,
    pub var_ind_chain: *mut DynCInf,
    pub clause: *mut DynCInf,
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut erase_stamp: DynStamp = 1 as libc::c_int as DynStamp;
static mut first_dyn_with_erase: *mut DynPInf = 0 as *const DynPInf as *mut DynPInf;
static mut size_of_erased: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn Pl_Add_Dynamic_Clause(
    mut head_word: WamWord,
    mut body_word: WamWord,
    mut asserta: Bool,
    mut check_perm: Bool,
    mut pl_file: libc::c_int,
) -> *mut DynCInf {
    let mut word: WamWord = 0;
    let mut first_arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut index_no: libc::c_int = 0;
    let mut key: PlLong = 0 as libc::c_int as PlLong;
    let mut clause: *mut DynCInf = 0 as *mut DynCInf;
    let mut dyn_0: *mut DynPInf = 0 as *mut DynPInf;
    let mut p_ind_htbl: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut p_ind_hdr: *mut D2ChHdr = 0 as *mut D2ChHdr;
    let mut swt_info: DSwtInf = DSwtInf {
        key: 0,
        ind_chain: D2ChHdr {
            first: 0 as *mut dyncinf,
            last: 0 as *mut dyncinf,
        },
    };
    let mut swt: *mut DSwtInf = 0 as *mut DSwtInf;
    let mut size: libc::c_int = 0;
    let mut lst_h_b: WamWord = 0;
    first_arg_adr = Pl_Rd_Callable_Check(head_word, &mut func, &mut arity);
    pred = Pl_Lookup_Pred(func, arity);
    if pred.is_null() {
        pred = Pl_Create_Pred(
            func,
            arity,
            pl_atom_user_input,
            (**pl_stm_tbl.offset(pl_stm_stdin as isize)).line_count as libc::c_int,
            2 as libc::c_int | 4 as libc::c_int,
            0 as *mut PlLong,
        );
    } else if check_perm != 0 && (*pred).prop & 2 as libc::c_int == 0 {
        word = Pl_Put_Structure(
            '/' as i32 as libc::c_uchar as libc::c_int,
            2 as libc::c_int,
        );
        Pl_Unify_Atom(func);
        Pl_Unify_Integer(arity as PlLong);
        Pl_Err_Permission(
            pl_permission_operation_modify,
            pl_permission_type_static_procedure,
            word,
        );
    }
    if pl_file == pl_atom_void {
        pl_file = -(1 as libc::c_int);
    }
    dyn_0 = (*pred).dyn_0 as *mut DynPInf;
    if dyn_0.is_null() {
        dyn_0 = Alloc_Init_Dyn_Info(pred, arity);
    }
    index_no = if (*dyn_0).arity != 0 {
        Index_From_First_Arg(*first_arg_adr, &mut key)
    } else {
        0 as libc::c_int
    };
    lst_h_b = (H as PlLong as libc::c_ulong).wrapping_add(0x1 as libc::c_int as PlULong)
        as WamWord;
    *H.offset(0 as libc::c_int as isize) = head_word;
    *H.offset(1 as libc::c_int as isize) = body_word;
    size = Pl_Term_Size(lst_h_b);
    clause = Pl_Malloc_Check(
        (::std::mem::size_of::<DynCInf>() as libc::c_ulong)
            .wrapping_sub(
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong),
            )
            .wrapping_add(
                (size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong),
            ) as libc::c_uint,
        b"dynam_supp.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        296 as libc::c_int,
    ) as *mut DynCInf;
    Add_To_2Chain(&mut (*dyn_0).seq_chain, clause, 1 as libc::c_int, asserta);
    (*clause).dyn_0 = dyn_0;
    (*clause)
        .cl_no = if asserta != 0 {
        let fresh0 = (*dyn_0).count_a;
        (*dyn_0).count_a = (*dyn_0).count_a - 1;
        fresh0
    } else {
        let fresh1 = (*dyn_0).count_z;
        (*dyn_0).count_z = (*dyn_0).count_z + 1;
        fresh1
    };
    (*clause).pl_file = pl_file;
    (*clause).erase_stamp = -(1 as libc::c_int) as DynStamp;
    (*clause).next_erased_cl = 0 as DynCInfP;
    (*clause).term_size = size;
    Pl_Copy_Term(&mut (*clause).term_word, &mut lst_h_b);
    (*clause).byte_code = pl_byte_code;
    pl_byte_code = 0 as *mut libc::c_uint;
    match index_no {
        0 => {
            (*clause).ind_chain.next = 0 as DynCInfP;
            (*clause).ind_chain.prev = 0 as DynCInfP;
            p_ind_hdr = 0 as *mut D2ChHdr;
            p_ind_htbl = 0 as *mut *mut libc::c_char;
        }
        1 => {
            p_ind_hdr = &mut (*dyn_0).var_ind_chain;
            p_ind_htbl = 0 as *mut *mut libc::c_char;
        }
        4 => {
            p_ind_hdr = &mut (*dyn_0).lst_ind_chain;
            p_ind_htbl = 0 as *mut *mut libc::c_char;
        }
        2 => {
            p_ind_htbl = &mut (*dyn_0).atm_htbl;
        }
        3 => {
            p_ind_htbl = &mut (*dyn_0).int_htbl;
        }
        5 => {
            p_ind_htbl = &mut (*dyn_0).stc_htbl;
        }
        _ => {}
    }
    (*clause).p_ind_htbl = p_ind_htbl;
    if !p_ind_htbl.is_null() {
        if (*p_ind_htbl).is_null() {
            *p_ind_htbl = Pl_Hash_Alloc_Table(
                32 as libc::c_int,
                ::std::mem::size_of::<DSwtInf>() as libc::c_ulong as libc::c_int,
            );
        }
        swt_info.key = key;
        swt_info.ind_chain.last = 0 as DynCInfP;
        swt_info.ind_chain.first = swt_info.ind_chain.last;
        Pl_Extend_Table_If_Needed(p_ind_htbl);
        swt = Pl_Hash_Insert(
            *p_ind_htbl,
            &mut swt_info as *mut DSwtInf as *mut libc::c_char,
            0 as libc::c_int,
        ) as *mut DSwtInf;
        p_ind_hdr = &mut (*swt).ind_chain;
    }
    (*clause).p_ind_hdr = p_ind_hdr;
    if !p_ind_hdr.is_null() {
        Add_To_2Chain(p_ind_hdr, clause, 0 as libc::c_int, asserta);
    }
    return clause;
}
unsafe extern "C" fn Alloc_Init_Dyn_Info(
    mut pred: *mut PredInf,
    mut arity: libc::c_int,
) -> *mut DynPInf {
    let mut dyn_0: *mut DynPInf = 0 as *mut DynPInf;
    dyn_0 = Pl_Malloc_Check(
        ::std::mem::size_of::<DynPInf>() as libc::c_ulong as libc::c_uint,
        b"dynam_supp.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        383 as libc::c_int,
    ) as *mut DynPInf;
    (*dyn_0).seq_chain.last = 0 as DynCInfP;
    (*dyn_0).seq_chain.first = (*dyn_0).seq_chain.last;
    (*dyn_0).var_ind_chain.last = 0 as DynCInfP;
    (*dyn_0).var_ind_chain.first = (*dyn_0).var_ind_chain.last;
    (*dyn_0).lst_ind_chain.last = 0 as DynCInfP;
    (*dyn_0).lst_ind_chain.first = (*dyn_0).lst_ind_chain.last;
    (*dyn_0).stc_htbl = 0 as *mut libc::c_char;
    (*dyn_0).int_htbl = (*dyn_0).stc_htbl;
    (*dyn_0).atm_htbl = (*dyn_0).int_htbl;
    (*dyn_0).arity = arity;
    (*dyn_0).count_a = -(1 as libc::c_int);
    (*dyn_0).count_z = 0 as libc::c_int;
    (*dyn_0).first_erased_cl = 0 as DynCInfP;
    (*dyn_0).next_dyn_with_erase = 0 as DynPInfP;
    (*pred).dyn_0 = dyn_0 as *mut PlLong;
    return dyn_0;
}
unsafe extern "C" fn Index_From_First_Arg(
    mut first_arg_word: WamWord,
    mut key: *mut PlLong,
) -> libc::c_int {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut index_no: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = first_arg_word;
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
    match tag_mask {
        0 | 5 => {
            index_no = 1 as libc::c_int;
        }
        7 => {
            index_no = 3 as libc::c_int;
            *key = word << 0 as libc::c_int >> 3 as libc::c_int;
        }
        3 => {
            index_no = 2 as libc::c_int;
            *key = (word as PlULong >> 3 as libc::c_int) as PlLong;
        }
        4 => {
            index_no = 0 as libc::c_int;
        }
        1 => {
            index_no = 4 as libc::c_int;
        }
        _ => {
            index_no = 5 as libc::c_int;
            *key = *((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord)
                .offset(0 as libc::c_int as isize);
        }
    }
    return index_no;
}
unsafe extern "C" fn Add_To_2Chain(
    mut hdr: *mut D2ChHdr,
    mut clause: *mut DynCInf,
    mut in_seq_chain: Bool,
    mut asserta: Bool,
) {
    let mut cell: *mut D2ChCell = if in_seq_chain != 0 {
        &mut (*clause).seq_chain
    } else {
        &mut (*clause).ind_chain
    };
    if ((*hdr).first).is_null() {
        (*hdr).last = clause;
        (*hdr).first = (*hdr).last;
        (*cell).prev = 0 as DynCInfP;
        (*cell).next = (*cell).prev;
        return;
    }
    if asserta != 0 {
        (*cell).next = (*hdr).first;
        (*cell).prev = 0 as DynCInfP;
        (*hdr).first = clause;
        if in_seq_chain != 0 {
            (*(*cell).next).seq_chain.prev = clause;
        } else {
            (*(*cell).next).ind_chain.prev = clause;
        }
    } else {
        if in_seq_chain != 0 {
            (*(*hdr).last).seq_chain.next = clause;
        } else {
            (*(*hdr).last).ind_chain.next = clause;
        }
        (*cell).next = 0 as DynCInfP;
        (*cell).prev = (*hdr).last;
        (*hdr).last = clause;
    };
}
unsafe extern "C" fn Remove_From_2Chain(
    mut hdr: *mut D2ChHdr,
    mut clause: *mut DynCInf,
    mut in_seq_chain: Bool,
) {
    let mut cell: *mut D2ChCell = if in_seq_chain != 0 {
        &mut (*clause).seq_chain
    } else {
        &mut (*clause).ind_chain
    };
    let mut prev: *mut DynCInf = (*cell).prev;
    let mut next: *mut DynCInf = (*cell).next;
    if prev.is_null() {
        (*hdr).first = next;
    } else if in_seq_chain != 0 {
        (*prev).seq_chain.next = next;
    } else {
        (*prev).ind_chain.next = next;
    }
    if next.is_null() {
        (*hdr).last = prev;
    } else if in_seq_chain != 0 {
        (*next).seq_chain.prev = prev;
    } else {
        (*next).ind_chain.prev = prev;
    };
}
pub unsafe extern "C" fn Pl_Delete_Dynamic_Clause(mut clause: *mut DynCInf) {
    let mut dyn_0: *mut DynPInf = 0 as *mut DynPInf;
    let mut first: Bool = 0;
    dyn_0 = (*clause).dyn_0;
    first = ((*dyn_0).first_erased_cl == 0 as *mut libc::c_void as DynCInfP)
        as libc::c_int;
    (*clause).erase_stamp = erase_stamp;
    (*clause).next_erased_cl = (*dyn_0).first_erased_cl;
    (*dyn_0).first_erased_cl = clause;
    if first != 0 {
        (*dyn_0).next_dyn_with_erase = first_dyn_with_erase;
        first_dyn_with_erase = dyn_0;
    }
    size_of_erased += (*clause).term_size;
    Clean_Erased_Clauses();
}
unsafe extern "C" fn Erase_All_Clauses_Of_File(
    mut dyn_0: *mut DynPInf,
    mut pl_file: libc::c_int,
) {
    let mut clause: *mut DynCInf = 0 as *mut DynCInf;
    if dyn_0.is_null() {
        return;
    }
    clause = (*dyn_0).seq_chain.first;
    while !clause.is_null() {
        if (*clause).erase_stamp == -(1 as libc::c_int) as DynStamp
            && (*clause).pl_file == pl_file
        {
            Pl_Delete_Dynamic_Clause(clause);
        }
        clause = (*clause).seq_chain.next;
    }
}
unsafe extern "C" fn Erase_All(mut dyn_0: *mut DynPInf) {
    let mut first: Bool = 0;
    let mut clause: *mut DynCInf = 0 as *mut DynCInf;
    if dyn_0.is_null() {
        return;
    }
    first = ((*dyn_0).first_erased_cl == 0 as *mut libc::c_void as DynCInfP)
        as libc::c_int;
    (*dyn_0).first_erased_cl = 2 as libc::c_int as *mut DynCInf;
    if first != 0 {
        (*dyn_0).next_dyn_with_erase = first_dyn_with_erase;
        first_dyn_with_erase = dyn_0;
    }
    clause = (*dyn_0).seq_chain.first;
    while !clause.is_null() {
        if (*clause).erase_stamp == -(1 as libc::c_int) as DynStamp {
            size_of_erased += (*clause).term_size;
        }
        clause = (*clause).seq_chain.next;
    }
    Clean_Erased_Clauses();
}
unsafe extern "C" fn Clean_Erased_Clauses() {
    let mut b: *mut WamWord = 0 as *mut WamWord;
    let mut base: *mut WamWord = 0 as *mut WamWord;
    let mut scan: *mut DynScan = 0 as *mut DynScan;
    let mut dyn_0: *mut DynPInf = 0 as *mut DynPInf;
    let mut dyn1: *mut DynPInf = 0 as *mut DynPInf;
    let mut prev: *mut *mut DynPInf = 0 as *mut *mut DynPInf;
    let mut clause: *mut DynCInf = 0 as *mut DynCInf;
    let mut clause1: *mut DynCInf = 0 as *mut DynCInf;
    if size_of_erased as libc::c_ulong
        <= ((512 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong)
    {
        return;
    }
    base = (*pl_stk_tbl.as_mut_ptr().offset(3 as libc::c_int as isize)).stack;
    b = B;
    while b > base {
        scan = Get_Scan_Choice_Point(b);
        if !scan.is_null() {
            dyn_0 = (*scan).dyn_0;
            if !((*dyn_0).first_erased_cl).is_null() {
                (*dyn_0)
                    .first_erased_cl = ((*dyn_0).first_erased_cl as PlULong
                    | 1 as libc::c_int as libc::c_ulong) as *mut DynCInf;
            }
        }
        b = *(&mut *b.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    }
    prev = &mut first_dyn_with_erase;
    dyn_0 = first_dyn_with_erase;
    while !dyn_0.is_null() {
        dyn1 = (*dyn_0).next_dyn_with_erase;
        if (*dyn_0).first_erased_cl as PlLong & 1 as libc::c_int as libc::c_long != 0 {
            (*dyn_0)
                .first_erased_cl = ((*dyn_0).first_erased_cl as PlULong
                & !(1 as libc::c_int) as libc::c_ulong) as *mut DynCInf;
            prev = &mut (*dyn_0).next_dyn_with_erase;
        } else {
            *prev = (*dyn_0).next_dyn_with_erase;
            if (*dyn_0).first_erased_cl == 2 as libc::c_int as *mut DynCInf {
                clause = (*dyn_0).seq_chain.first;
                while !clause.is_null() {
                    clause1 = (*clause).seq_chain.next;
                    size_of_erased -= (*clause).term_size;
                    Free_Clause(clause);
                    clause = clause1;
                }
                if !((*dyn_0).atm_htbl).is_null() {
                    Pl_Hash_Free_Table((*dyn_0).atm_htbl);
                }
                if !((*dyn_0).int_htbl).is_null() {
                    Pl_Hash_Free_Table((*dyn_0).int_htbl);
                }
                if !((*dyn_0).stc_htbl).is_null() {
                    Pl_Hash_Free_Table((*dyn_0).stc_htbl);
                }
                free(dyn_0 as *mut libc::c_void);
            } else {
                clause = (*dyn_0).first_erased_cl;
                while !clause.is_null() {
                    clause1 = (*clause).next_erased_cl;
                    size_of_erased -= (*clause).term_size;
                    Unlink_Clause(clause);
                    Free_Clause(clause);
                    clause = clause1;
                }
                (*dyn_0).first_erased_cl = 0 as DynCInfP;
                (*dyn_0).next_dyn_with_erase = 0 as DynPInfP;
                if ((*dyn_0).seq_chain.first).is_null() {
                    if !((*dyn_0).atm_htbl).is_null() {
                        Pl_Hash_Free_Table((*dyn_0).atm_htbl);
                    }
                    if !((*dyn_0).int_htbl).is_null() {
                        Pl_Hash_Free_Table((*dyn_0).int_htbl);
                    }
                    if !((*dyn_0).stc_htbl).is_null() {
                        Pl_Hash_Free_Table((*dyn_0).stc_htbl);
                    }
                    (*dyn_0).stc_htbl = 0 as *mut libc::c_char;
                    (*dyn_0).int_htbl = (*dyn_0).stc_htbl;
                    (*dyn_0).atm_htbl = (*dyn_0).int_htbl;
                    (*dyn_0).count_a = -(1 as libc::c_int);
                    (*dyn_0).count_z = 0 as libc::c_int;
                }
            }
        }
        dyn_0 = dyn1;
    }
}
unsafe extern "C" fn Unlink_Clause(mut clause: *mut DynCInf) {
    let mut dyn_0: *mut DynPInf = (*clause).dyn_0;
    let mut p_key: *mut PlLong = 0 as *mut PlLong;
    let mut swt_info: DSwtInf = DSwtInf {
        key: 0,
        ind_chain: D2ChHdr {
            first: 0 as *mut dyncinf,
            last: 0 as *mut dyncinf,
        },
    };
    Remove_From_2Chain(&mut (*dyn_0).seq_chain, clause, 1 as libc::c_int);
    if !((*clause).p_ind_hdr).is_null() {
        Remove_From_2Chain((*clause).p_ind_hdr, clause, 0 as libc::c_int);
    }
    if !((*clause).p_ind_htbl).is_null() && ((*clause).ind_chain.prev).is_null()
        && ((*clause).ind_chain.next).is_null()
    {
        p_key = ((*clause).p_ind_hdr as *mut libc::c_char)
            .offset(
                -((&mut swt_info.ind_chain as *mut D2ChHdr as *mut libc::c_char)
                    .offset_from(&mut swt_info.key as *mut PlLong as *mut libc::c_char)
                    as libc::c_long as isize),
            ) as *mut PlLong;
        Pl_Hash_Delete(*(*clause).p_ind_htbl, *p_key);
    }
}
unsafe extern "C" fn Free_Clause(mut clause: *mut DynCInf) {
    if !((*clause).byte_code).is_null() {
        free((*clause).byte_code as *mut libc::c_void);
    }
    free(clause as *mut libc::c_void);
}
pub unsafe extern "C" fn Pl_Update_Dynamic_Pred(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut what_to_do: libc::c_int,
    mut pl_file_for_multi: libc::c_int,
) -> *mut PredInf {
    let mut word: WamWord = 0;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    pred = Pl_Lookup_Pred(func, arity);
    if pred.is_null() {
        return 0 as *mut PredInf;
    }
    if what_to_do & 1 as libc::c_int != 0 && (*pred).prop & 2 as libc::c_int == 0 {
        word = Pl_Put_Structure(
            '/' as i32 as libc::c_uchar as libc::c_int,
            2 as libc::c_int,
        );
        Pl_Unify_Atom(func);
        Pl_Unify_Integer(arity as PlLong);
        Pl_Err_Permission(
            pl_permission_operation_modify,
            pl_permission_type_static_procedure,
            word,
        );
    }
    if pl_file_for_multi >= 0 as libc::c_int && (*pred).prop & 64 as libc::c_int != 0 {
        Erase_All_Clauses_Of_File((*pred).dyn_0 as *mut DynPInf, pl_file_for_multi);
    } else {
        Erase_All((*pred).dyn_0 as *mut DynPInf);
        (*pred).dyn_0 = 0 as *mut PlLong;
    }
    if what_to_do & 2 as libc::c_int != 0 {
        Pl_Delete_Pred(func, arity);
        return 0 as *mut PredInf;
    }
    return pred;
}
unsafe extern "C" fn Get_Scan_Choice_Point(mut b: *mut WamWord) -> *mut DynScan {
    let mut scan: *mut DynScan = 0 as *mut DynScan;
    let mut i: libc::c_int = 0;
    if *(&mut *b.offset(-(1 as libc::c_int) as isize) as *mut WamWord as *mut CodePtr)
        != ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_247363616E5F64796E5F746573745F616C74__a0),
            ),
        )
        && *(&mut *b.offset(-(1 as libc::c_int) as isize) as *mut WamWord
            as *mut CodePtr)
            != ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_247363616E5F64796E5F6A756D705F616C74__a0),
                ),
            )
    {
        return 0 as *mut DynScan;
    }
    i = (::std::mem::size_of::<DynScan>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    scan = &mut *b.offset((-(9 as libc::c_int) - i) as isize) as *mut WamWord
        as *mut DynScan;
    return scan;
}
pub unsafe extern "C" fn Pl_Scan_Dynamic_Pred(
    mut owner_func: libc::c_int,
    mut owner_arity: libc::c_int,
    mut dyn_0: *mut DynPInf,
    mut first_arg_word: WamWord,
    mut alt_fct: ScanFct,
    mut alt_fct_type: libc::c_int,
    mut alt_info_size: libc::c_int,
    mut alt_info: *mut WamWord,
) -> *mut DynCInf {
    let mut index_no: libc::c_int = 0;
    let mut key: PlLong = 0;
    let mut p_ind_htbl: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut swt: *mut DSwtInf = 0 as *mut DSwtInf;
    let mut scan: DynScan = DynScan {
        alt_fct: None,
        alt_size_info: 0,
        owner_func: 0,
        owner_arity: 0,
        dyn_0: 0 as *mut DynPInf,
        stop_cl_no: 0,
        erase_stamp: 0,
        xxx_is_seq_chain: 0,
        xxx_ind_chain: 0 as *mut DynCInf,
        var_ind_chain: 0 as *mut DynCInf,
        clause: 0 as *mut DynCInf,
    };
    let mut clause: *mut DynCInf = 0 as *mut DynCInf;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    let mut scan_alt: CodePtr = None;
    if owner_func < 0 as libc::c_int {
        owner_func = Pl_Get_Current_Bip(&mut owner_arity);
    }
    index_no = if (*dyn_0).arity != 0 {
        Index_From_First_Arg(first_arg_word, &mut key)
    } else {
        0 as libc::c_int
    };
    scan.alt_fct = alt_fct;
    scan.alt_size_info = alt_info_size;
    scan.owner_func = owner_func;
    scan.owner_arity = owner_arity;
    scan.dyn_0 = dyn_0;
    scan.stop_cl_no = (*dyn_0).count_z;
    let fresh2 = erase_stamp;
    erase_stamp = erase_stamp.wrapping_add(1);
    scan.erase_stamp = fresh2;
    match index_no {
        0 | 1 => {
            scan.xxx_is_seq_chain = 1 as libc::c_int;
            scan.xxx_ind_chain = (*dyn_0).seq_chain.first;
            p_ind_htbl = 0 as *mut *mut libc::c_char;
        }
        4 => {
            scan.xxx_is_seq_chain = 0 as libc::c_int;
            scan.xxx_ind_chain = (*dyn_0).lst_ind_chain.first;
            p_ind_htbl = 0 as *mut *mut libc::c_char;
        }
        2 => {
            p_ind_htbl = &mut (*dyn_0).atm_htbl;
        }
        3 => {
            p_ind_htbl = &mut (*dyn_0).int_htbl;
        }
        5 => {
            p_ind_htbl = &mut (*dyn_0).stc_htbl;
        }
        _ => {}
    }
    if !p_ind_htbl.is_null() {
        scan.xxx_is_seq_chain = 0 as libc::c_int;
        if !(*p_ind_htbl).is_null()
            && {
                swt = Pl_Hash_Find(*p_ind_htbl, key) as *mut DSwtInf;
                !swt.is_null()
            }
        {
            scan.xxx_ind_chain = (*swt).ind_chain.first;
        } else {
            scan.xxx_ind_chain = 0 as *mut DynCInf;
        }
    }
    if scan.xxx_is_seq_chain != 0 {
        scan.var_ind_chain = 0 as *mut DynCInf;
    } else {
        scan.var_ind_chain = (*dyn_0).var_ind_chain.first;
    }
    clause = Scan_Dynamic_Pred_Next(&mut scan);
    if clause.is_null() {
        return 0 as *mut DynCInf;
    }
    if !(Scan_Dynamic_Pred_Next(&mut scan)).is_null() {
        i = (::std::mem::size_of::<DynScan>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong)
            .wrapping_add(alt_info_size as libc::c_ulong) as libc::c_int;
        if alt_fct_type == 0 as libc::c_int {
            scan_alt = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_247363616E5F64796E5F746573745F616C74__a0),
                ),
            );
        } else {
            scan_alt = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_247363616E5F64796E5F6A756D705F616C74__a0),
                ),
            );
        }
        Pl_Create_Choice_Point(scan_alt, i);
        adr = (&mut *B.offset((-(9 as libc::c_int) - i) as isize) as *mut WamWord)
            .offset(1 as libc::c_int as isize);
        i = alt_info_size;
        loop {
            let fresh3 = i;
            i = i - 1;
            if !(fresh3 != 0) {
                break;
            }
            let fresh4 = alt_info;
            alt_info = alt_info.offset(1);
            let fresh5 = adr;
            adr = adr.offset(1);
            *fresh5 = *fresh4;
        }
        *(adr as *mut DynScan) = scan;
    }
    return clause;
}
unsafe extern "C" fn Scan_Dynamic_Pred_Next(mut scan: *mut DynScan) -> *mut DynCInf {
    let mut xxx_ind_chain: *mut DynCInf = 0 as *mut DynCInf;
    let mut var_ind_chain: *mut DynCInf = 0 as *mut DynCInf;
    let mut xxx_clause: *mut DynCInf = 0 as *mut DynCInf;
    let mut var_clause: *mut DynCInf = 0 as *mut DynCInf;
    let mut xxx_nb: PlLong = 0;
    let mut var_nb: PlLong = 0;
    let mut clause: *mut DynCInf = 0 as *mut DynCInf;
    (*scan).clause = 0 as *mut DynCInf;
    loop {
        xxx_ind_chain = (*scan).xxx_ind_chain;
        if !xxx_ind_chain.is_null() {
            xxx_clause = xxx_ind_chain;
            xxx_nb = (*xxx_clause).cl_no as PlLong;
        } else {
            xxx_nb = ((1 as libc::c_int as PlLong)
                << 8 as libc::c_int * 8 as libc::c_int - 3 as libc::c_int
                    - 1 as libc::c_int) - 1 as libc::c_int as libc::c_long;
        }
        var_ind_chain = (*scan).var_ind_chain;
        if !var_ind_chain.is_null() {
            var_clause = var_ind_chain;
            var_nb = (*var_clause).cl_no as PlLong;
        } else {
            var_nb = ((1 as libc::c_int as PlLong)
                << 8 as libc::c_int * 8 as libc::c_int - 3 as libc::c_int
                    - 1 as libc::c_int) - 1 as libc::c_int as libc::c_long;
        }
        if xxx_nb <= var_nb {
            if xxx_nb
                == ((1 as libc::c_int as PlLong)
                    << 8 as libc::c_int * 8 as libc::c_int - 3 as libc::c_int
                        - 1 as libc::c_int) - 1 as libc::c_int as libc::c_long
            {
                return 0 as *mut DynCInf;
            }
            clause = xxx_clause;
            if (*scan).xxx_is_seq_chain != 0 {
                (*scan).xxx_ind_chain = (*xxx_ind_chain).seq_chain.next;
            } else {
                (*scan).xxx_ind_chain = (*xxx_ind_chain).ind_chain.next;
            }
        } else {
            clause = var_clause;
            (*scan).var_ind_chain = (*var_ind_chain).ind_chain.next;
        }
        if (*clause).cl_no >= (*scan).stop_cl_no {
            return 0 as *mut DynCInf;
        }
        if !((*clause).erase_stamp <= (*scan).erase_stamp) {
            break;
        }
    }
    (*scan).clause = clause;
    return clause;
}
pub unsafe extern "C" fn Pl_Scan_Dynamic_Pred_Alt_0() -> PlLong {
    let mut alt_info: *mut WamWord = 0 as *mut WamWord;
    let mut scan: *mut DynScan = 0 as *mut DynScan;
    let mut clause: *mut DynCInf = 0 as *mut DynCInf;
    let mut is_last: Bool = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    let mut scan_alt: CodePtr = None;
    scan_alt = *(&mut *B.offset(-(1 as libc::c_int) as isize) as *mut WamWord
        as *mut CodePtr);
    Pl_Update_Choice_Point(scan_alt, 0 as libc::c_int);
    i = (::std::mem::size_of::<DynScan>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    scan = &mut *B.offset((-(9 as libc::c_int) - i) as isize) as *mut WamWord
        as *mut DynScan;
    adr = scan as *mut WamWord;
    alt_info = adr.offset(-((*scan).alt_size_info as isize));
    clause = (*scan).clause;
    is_last = (Scan_Dynamic_Pred_Next(scan) == 0 as *mut libc::c_void as *mut DynCInf)
        as libc::c_int;
    if is_last != 0 {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh6 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh6 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    }
    return ::std::mem::transmute::<
        _,
        fn(_, _, _) -> PlLong,
    >((Some(((*scan).alt_fct).unwrap())).unwrap())(clause, alt_info, is_last);
}
pub unsafe extern "C" fn Pl_Scan_Choice_Point_Pred(
    mut b: *mut WamWord,
    mut arity: *mut libc::c_int,
) -> libc::c_int {
    let mut scan: *mut DynScan = 0 as *mut DynScan;
    scan = Get_Scan_Choice_Point(b);
    if scan.is_null() {
        return -(1 as libc::c_int);
    }
    *arity = (*scan).owner_arity;
    return (*scan).owner_func;
}
pub unsafe extern "C" fn Pl_Copy_Clause_To_Heap(
    mut clause: *mut DynCInf,
    mut head_word: *mut WamWord,
    mut body_word: *mut WamWord,
) {
    Pl_Copy_Contiguous_Term(H, &mut (*clause).term_word);
    *head_word = *H.offset(1 as libc::c_int as isize);
    *body_word = *H.offset(2 as libc::c_int as isize);
    H = H.offset((*clause).term_size as isize);
}

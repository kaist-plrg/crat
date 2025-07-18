use ::libc;
extern "C" {
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    fn Pl_Lookup_Pred(func: libc::c_int, arity: libc::c_int) -> *mut PredInf;
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    fn Pl_Rd_C_Int_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Atom(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Callable_Check(
        start_word: WamWord,
        func: *mut libc::c_int,
        arity: *mut libc::c_int,
    ) -> *mut WamWord;
    fn Pl_Get_Pred_Indicator(
        pred_indic_word: WamWord,
        must_be_ground: Bool,
        arity: *mut libc::c_int,
    ) -> libc::c_int;
    fn Pl_Copy_Clause_To_Heap(
        clause: *mut DynCInf,
        head_word: *mut WamWord,
        body_word: *mut WamWord,
    );
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
    fn Pl_Update_Dynamic_Pred(
        func: libc::c_int,
        arity: libc::c_int,
        what_to_do: libc::c_int,
        pl_file_for_multi: libc::c_int,
    ) -> *mut PredInf;
    fn Pl_Delete_Dynamic_Clause(clause: *mut DynCInf);
    fn Pl_Add_Dynamic_Clause(
        head_word: WamWord,
        body_word: WamWord,
        asserta: Bool,
        check_perm: Bool,
        pl_file: libc::c_int,
    ) -> *mut DynCInf;
    static mut pl_type_callable: libc::c_int;
    static mut pl_permission_operation_access: libc::c_int;
    static mut pl_permission_operation_modify: libc::c_int;
    static mut pl_permission_type_private_procedure: libc::c_int;
    static mut pl_permission_type_static_procedure: libc::c_int;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Permission(atom_oper: libc::c_int, atom_perm: libc::c_int, term: WamWord);
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut last_clause: *mut DynCInf = 0 as *const DynCInf as *mut DynCInf;
pub unsafe extern "C" fn Pl_Assert_5(
    mut head_word: WamWord,
    mut body_word: WamWord,
    mut asserta_word: WamWord,
    mut check_perm_word: WamWord,
    mut pl_file_word: WamWord,
) {
    let mut asserta: Bool = Pl_Rd_Integer(asserta_word) as Bool;
    let mut check_perm: Bool = Pl_Rd_Integer(check_perm_word) as Bool;
    let mut pl_file: libc::c_int = Pl_Rd_Atom(pl_file_word);
    last_clause = Pl_Add_Dynamic_Clause(
        head_word,
        body_word,
        asserta,
        check_perm,
        pl_file,
    );
}
pub unsafe extern "C" fn Pl_Clause_3(
    mut head_word: WamWord,
    mut body_word: WamWord,
    mut for_what_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut first_arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut head_word1: WamWord = 0;
    let mut body_word1: WamWord = 0;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut for_what: libc::c_int = 0;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut dyn_0: *mut DynPInf = 0 as *mut DynPInf;
    let mut clause: *mut DynCInf = 0 as *mut DynCInf;
    let mut w: [WamWord; 2] = [0; 2];
    first_arg_adr = Pl_Rd_Callable_Check(head_word, &mut func, &mut arity);
    let mut deref_last_word: WamWord = 0;
    word = body_word;
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
    body_word = word;
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_callable, body_word);
    }
    for_what = Pl_Rd_Integer_Check(for_what_word) as libc::c_int;
    pred = Pl_Lookup_Pred(func, arity);
    if pred.is_null() {
        return 0 as libc::c_int;
    }
    if for_what == 0 as libc::c_int && (*pred).prop & 4 as libc::c_int == 0
        || for_what == 2 as libc::c_int && (*pred).prop & 1 as libc::c_int != 0
    {
        word = Pl_Put_Structure(
            '/' as i32 as libc::c_uchar as libc::c_int,
            2 as libc::c_int,
        );
        Pl_Unify_Atom(func);
        Pl_Unify_Integer(arity as PlLong);
        Pl_Err_Permission(
            pl_permission_operation_access,
            pl_permission_type_private_procedure,
            word,
        );
    }
    dyn_0 = (*pred).dyn_0 as *mut DynPInf;
    if dyn_0.is_null() {
        return 0 as libc::c_int;
    }
    if arity > 0 as libc::c_int {
        word = *first_arg_adr;
    }
    w[0 as libc::c_int as usize] = head_word;
    w[1 as libc::c_int as usize] = body_word;
    clause = Pl_Scan_Dynamic_Pred(
        -(1 as libc::c_int),
        0 as libc::c_int,
        (*pred).dyn_0 as *mut DynPInf,
        word,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut DynCInf, *mut WamWord) -> Bool>,
            ScanFct,
        >(Some(Clause_Alt as unsafe extern "C" fn(*mut DynCInf, *mut WamWord) -> Bool)),
        0 as libc::c_int,
        2 as libc::c_int,
        w.as_mut_ptr(),
    );
    if clause.is_null() {
        return 0 as libc::c_int;
    }
    Pl_Copy_Clause_To_Heap(clause, &mut head_word1, &mut body_word1);
    last_clause = clause;
    return (Pl_Unify(head_word, head_word1) != 0 && Pl_Unify(body_word, body_word1) != 0)
        as libc::c_int;
}
unsafe extern "C" fn Clause_Alt(mut clause: *mut DynCInf, mut w: *mut WamWord) -> Bool {
    let mut head_word1: WamWord = 0;
    let mut body_word1: WamWord = 0;
    Pl_Copy_Clause_To_Heap(clause, &mut head_word1, &mut body_word1);
    last_clause = clause;
    return (Pl_Unify(head_word1, *w.offset(0 as libc::c_int as isize)) != 0
        && Pl_Unify(body_word1, *w.offset(1 as libc::c_int as isize)) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Retract_2(
    mut head_word: WamWord,
    mut body_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut first_arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut head_word1: WamWord = 0;
    let mut body_word1: WamWord = 0;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut dyn_0: *mut DynPInf = 0 as *mut DynPInf;
    let mut clause: *mut DynCInf = 0 as *mut DynCInf;
    let mut w: [WamWord; 2] = [0; 2];
    first_arg_adr = Pl_Rd_Callable_Check(head_word, &mut func, &mut arity);
    let mut deref_last_word: WamWord = 0;
    word = body_word;
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
    body_word = word;
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_callable, body_word);
    }
    pred = Pl_Lookup_Pred(func, arity);
    if pred.is_null() {
        return 0 as libc::c_int;
    }
    if (*pred).prop & 2 as libc::c_int == 0 {
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
    dyn_0 = (*pred).dyn_0 as *mut DynPInf;
    if dyn_0.is_null() {
        return 0 as libc::c_int;
    }
    if arity > 0 as libc::c_int {
        word = *first_arg_adr;
    }
    w[0 as libc::c_int as usize] = head_word;
    w[1 as libc::c_int as usize] = body_word;
    clause = Pl_Scan_Dynamic_Pred(
        -(1 as libc::c_int),
        0 as libc::c_int,
        (*pred).dyn_0 as *mut DynPInf,
        word,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut DynCInf, *mut WamWord) -> Bool>,
            ScanFct,
        >(Some(Retract_Alt as unsafe extern "C" fn(*mut DynCInf, *mut WamWord) -> Bool)),
        0 as libc::c_int,
        2 as libc::c_int,
        w.as_mut_ptr(),
    );
    if clause.is_null() {
        return 0 as libc::c_int;
    }
    Pl_Copy_Clause_To_Heap(clause, &mut head_word1, &mut body_word1);
    if Pl_Unify(head_word, head_word1) == 0 || Pl_Unify(body_word, body_word1) == 0 {
        return 0 as libc::c_int;
    }
    Pl_Delete_Dynamic_Clause(clause);
    return 1 as libc::c_int;
}
unsafe extern "C" fn Retract_Alt(mut clause: *mut DynCInf, mut w: *mut WamWord) -> Bool {
    let mut head_word1: WamWord = 0;
    let mut body_word1: WamWord = 0;
    Pl_Copy_Clause_To_Heap(clause, &mut head_word1, &mut body_word1);
    if Pl_Unify(head_word1, *w.offset(0 as libc::c_int as isize)) == 0
        || Pl_Unify(body_word1, *w.offset(1 as libc::c_int as isize)) == 0
    {
        return 0 as libc::c_int;
    }
    Pl_Delete_Dynamic_Clause(clause);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Retract_Last_Found_0() {
    Pl_Delete_Dynamic_Clause(last_clause);
}
pub unsafe extern "C" fn Pl_Setarg_Of_Last_Found_2(
    mut arg_no_word: WamWord,
    mut new_value_word: WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut arg_no: libc::c_int = 0;
    arg_no = (Pl_Rd_Integer(arg_no_word) - 1 as libc::c_int as libc::c_long)
        as libc::c_int;
    let mut deref_last_word: WamWord = 0;
    word = (*last_clause).head_word;
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
    let mut deref_last_word_0: WamWord = 0;
    word = new_value_word;
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
    *adr.offset((1 as libc::c_int + arg_no) as isize) = word;
}
pub unsafe extern "C" fn Pl_Retractall_If_Empty_Head_1(mut head_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut ref_adr: [*mut WamWord; 255] = [0 as *mut WamWord; 255];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: Bool = 0;
    arg_adr = Pl_Rd_Callable_Check(head_word, &mut func, &mut arity);
    pred = Pl_Lookup_Pred(func, arity);
    if pred.is_null() {
        return 1 as libc::c_int;
    }
    if (*pred).prop & 2 as libc::c_int == 0 {
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
    ret = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < arity {
        let mut deref_last_word: WamWord = 0;
        word = *arg_adr;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            ret = 0 as libc::c_int;
            break;
        } else {
            adr = word as *mut WamWord;
            ref_adr[i as usize] = adr;
            *adr = ((0 as libc::c_int as PlULong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong) as WamWord;
            arg_adr = arg_adr.offset(1);
            arg_adr;
            i += 1;
            i;
        }
    }
    j = i;
    i = 0 as libc::c_int;
    while i < j {
        adr = ref_adr[i as usize];
        *adr = (adr as PlLong as libc::c_ulong).wrapping_add(0 as libc::c_int as PlULong)
            as WamWord;
        i += 1;
        i;
    }
    if ret != 0 {
        Pl_Update_Dynamic_Pred(func, arity, 1 as libc::c_int, -(1 as libc::c_int));
    }
    return ret;
}
pub unsafe extern "C" fn Pl_Abolish_1(mut pred_indic_word: WamWord) {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    func = Pl_Get_Pred_Indicator(pred_indic_word, 1 as libc::c_int, &mut arity);
    Pl_Update_Dynamic_Pred(func, arity, 3 as libc::c_int, -(1 as libc::c_int));
}
pub unsafe extern "C" fn Pl_Remove_Predicate_2(
    mut name_word: WamWord,
    mut arity_word: WamWord,
) {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    func = Pl_Rd_Atom_Check(name_word);
    arity = Pl_Rd_C_Int_Check(arity_word);
    Pl_Update_Dynamic_Pred(func, arity, 2 as libc::c_int, -(1 as libc::c_int));
}

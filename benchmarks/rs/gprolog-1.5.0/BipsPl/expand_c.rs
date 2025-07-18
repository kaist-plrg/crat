use ::libc;
extern "C" {
    static mut pl_atom_curly_brackets: libc::c_int;
    static mut pl_atom_true: libc::c_int;
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Get_Structure(
        func: libc::c_int,
        arity: libc::c_int,
        start_word: WamWord,
    ) -> Bool;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Globalize_If_In_Local(start_word: WamWord) -> WamWord;
    fn Pl_Blt_Term_Eq(x: WamWord, y: WamWord) -> Bool;
    fn Pl_Rd_Callable_Check(
        start_word: WamWord,
        func: *mut libc::c_int,
        arity: *mut libc::c_int,
    ) -> *mut WamWord;
    fn Pl_Mk_Variable() -> WamWord;
    static mut pl_type_list: libc::c_int;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Instantiation();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut top: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut opt_term_unif: Bool = 0;
static mut atom_clause: libc::c_int = 0;
static mut atom_phrase: libc::c_int = 0;
static mut atom_if: libc::c_int = 0;
static mut atom_soft_if: libc::c_int = 0;
static mut atom_neg: libc::c_int = 0;
static mut dcg_2: WamWord = 0;
pub unsafe extern "C" fn Pl_Dcg_Trans_Rule_2(
    mut rule_word: WamWord,
    mut clause_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut in_word: WamWord = 0;
    let mut out_word: WamWord = 0;
    let mut head_word: WamWord = 0;
    let mut body_word: WamWord = 0;
    let mut end_lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = rule_word;
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
    if tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong
        || *adr.offset(0 as libc::c_int as isize) != dcg_2
    {
        return 0 as libc::c_int;
    }
    top = if B
        >= *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    {
        B
    } else {
        *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    };
    opt_term_unif = 1 as libc::c_int;
    head_word = Dcg_Head(
        *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
        &mut in_word,
        &mut out_word,
        &mut end_lst_adr,
    );
    body_word = Dcg_Body(
        *adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize),
        0 as libc::c_int,
        in_word,
        out_word,
        end_lst_adr,
    );
    Pl_Get_Structure(atom_clause, 2 as libc::c_int, clause_word);
    Pl_Unify_Value(head_word);
    Pl_Unify_Value(body_word);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Dcg_Trans_Body_4(
    mut dcg_body_word: WamWord,
    mut in_word: WamWord,
    mut out_word: WamWord,
    mut body_word: WamWord,
) -> Bool {
    top = if B
        >= *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    {
        B
    } else {
        *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 2 as libc::c_int) as isize)
    };
    opt_term_unif = 1 as libc::c_int;
    in_word = Pl_Globalize_If_In_Local(in_word);
    out_word = Pl_Globalize_If_In_Local(out_word);
    return Pl_Unify(
        body_word,
        Dcg_Body(dcg_body_word, 0 as libc::c_int, in_word, out_word, 0 as *mut WamWord),
    );
}
unsafe extern "C" fn Dcg_Head(
    mut dcg_head_word: WamWord,
    mut in_word: *mut WamWord,
    mut out_word: *mut WamWord,
    mut end_lst_adr: *mut *mut WamWord,
) -> WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut save_H: *mut WamWord = 0 as *mut WamWord;
    let mut p: *mut WamWord = 0 as *mut WamWord;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut first: Bool = 0;
    first = 1 as libc::c_int;
    *end_lst_adr = 0 as *mut WamWord;
    loop {
        adr = Pl_Rd_Callable_Check(dcg_head_word, &mut func, &mut arity);
        if !(first != 0 && arity == 2 as libc::c_int
            && func == ',' as i32 as libc::c_uchar as libc::c_int)
        {
            break;
        }
        first = 0 as libc::c_int;
        let fresh0 = adr;
        adr = adr.offset(1);
        dcg_head_word = *fresh0;
        let mut deref_last_word: WamWord = 0;
        word = *adr;
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
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            Pl_Err_Instantiation();
        }
        if word as libc::c_ulong
            != (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
                Pl_Err_Type(pl_type_list, word);
            }
            *end_lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
        }
    }
    save_H = H;
    p = save_H;
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = (((arity + 2 as libc::c_int) as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong)
        as WamWord;
    loop {
        let fresh2 = arity;
        arity = arity - 1;
        if !(fresh2 != 0) {
            break;
        }
        let fresh3 = adr;
        adr = adr.offset(1);
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = *fresh3;
    }
    adr = p;
    *in_word = (adr as PlLong as libc::c_ulong).wrapping_add(0 as libc::c_int as PlULong)
        as WamWord;
    let fresh5 = p;
    p = p.offset(1);
    *fresh5 = *in_word;
    adr = p;
    *out_word = (adr as PlLong as libc::c_ulong)
        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    let fresh6 = p;
    p = p.offset(1);
    *fresh6 = *out_word;
    H = p;
    return (save_H as PlLong as libc::c_ulong)
        .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
}
unsafe extern "C" fn Dcg_Body(
    mut dcg_body_word: WamWord,
    mut in_alt: Bool,
    mut in_word: WamWord,
    mut out_word: WamWord,
    mut end_lst_adr: *mut WamWord,
) -> WamWord {
    let mut current_block: u64;
    let mut new_out_word: WamWord = 0;
    let mut word: WamWord = 0;
    let mut save_H: *mut WamWord = 0 as *mut WamWord;
    let mut p: *mut WamWord = 0 as *mut WamWord;
    let mut save_top: *mut WamWord = top;
    let mut save_opt_term_unif: Bool = opt_term_unif;
    let mut opt_equal_between_in_out_vars: Bool = 0;
    let mut base: *mut WamWord = 0 as *mut WamWord;
    if !end_lst_adr.is_null() {
        current_block = 13869171905349467259;
    } else if in_alt != 0 {
        top = top.offset(1);
        top;
        current_block = 13869171905349467259;
    } else {
        new_out_word = out_word;
        current_block = 10879442775620481940;
    }
    match current_block {
        13869171905349467259 => {
            new_out_word = Pl_Mk_Variable();
        }
        _ => {}
    }
    base = top;
    opt_equal_between_in_out_vars = (end_lst_adr
        != 0 as *mut libc::c_void as *mut WamWord) as libc::c_int;
    Dcg_Body_On_Stack(
        dcg_body_word,
        opt_equal_between_in_out_vars,
        in_word,
        new_out_word,
    );
    if !end_lst_adr.is_null() {
        Dcg_Term_List_On_Stack(end_lst_adr, out_word, new_out_word);
    } else if in_alt != 0 {
        if Pl_Blt_Term_Eq(in_word, new_out_word) != 0 {
            base = base.offset(-1);
            *base = Dcg_Compound2(
                '=' as i32 as libc::c_uchar as libc::c_int,
                new_out_word,
                out_word,
            );
        } else {
            Pl_Unify(new_out_word, out_word);
        }
    }
    if top == base {
        word = (((pl_atom_true as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    } else {
        top = top.offset(-1);
        word = *top;
        while top > base {
            save_H = H;
            p = save_H;
            let fresh7 = p;
            p = p.offset(1);
            *fresh7 = ((2 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                | ',' as i32 as libc::c_uchar as libc::c_int as libc::c_ulong)
                as WamWord;
            top = top.offset(-1);
            let fresh8 = p;
            p = p.offset(1);
            *fresh8 = *top;
            let fresh9 = p;
            p = p.offset(1);
            *fresh9 = word;
            H = p;
            word = (save_H as PlLong as libc::c_ulong)
                .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
        }
    }
    top = save_top;
    opt_term_unif = save_opt_term_unif;
    return word;
}
unsafe extern "C" fn Dcg_Body_On_Stack(
    mut dcg_body_word: WamWord,
    mut opt_equal_between_in_out_vars: Bool,
    mut in_word: WamWord,
    mut out_word: WamWord,
) {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut w1: WamWord = 0;
    let mut w2: WamWord = 0;
    let mut save_H: *mut WamWord = 0 as *mut WamWord;
    let mut p: *mut WamWord = 0 as *mut WamWord;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = dcg_body_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        adr = word as *mut WamWord;
        func = atom_phrase;
        arity = 1 as libc::c_int;
    } else {
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            opt_equal_between_in_out_vars = 1 as libc::c_int;
            current_block = 15577247102568781450;
        } else {
            if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
                Dcg_Term_List_On_Stack(
                    (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord,
                    in_word,
                    out_word,
                );
                return;
            }
            adr = Pl_Rd_Callable_Check(word, &mut func, &mut arity);
            if arity == 2 as libc::c_int
                && func == ',' as i32 as libc::c_uchar as libc::c_int
            {
                word = Pl_Mk_Variable();
                let fresh11 = adr;
                adr = adr.offset(1);
                Dcg_Body_On_Stack(
                    *fresh11,
                    (2 as libc::c_int != 0 as libc::c_int) as libc::c_int,
                    in_word,
                    word,
                );
                Dcg_Body_On_Stack(*adr, opt_equal_between_in_out_vars, word, out_word);
                return;
            }
            opt_term_unif = 0 as libc::c_int;
            if arity == 2 as libc::c_int && (func == atom_if || func == atom_soft_if) {
                word = Pl_Mk_Variable();
                let fresh12 = adr;
                adr = adr.offset(1);
                w1 = Dcg_Body(
                    *fresh12,
                    0 as libc::c_int,
                    in_word,
                    word,
                    0 as *mut WamWord,
                );
                w2 = Dcg_Body(*adr, 0 as libc::c_int, word, out_word, 0 as *mut WamWord);
                let fresh13 = top;
                top = top.offset(1);
                *fresh13 = Dcg_Compound2(func, w1, w2);
                return;
            }
            if arity == 2 as libc::c_int
                && (func == ';' as i32 as libc::c_uchar as libc::c_int
                    || func == '|' as i32 as libc::c_uchar as libc::c_int)
            {
                let fresh14 = adr;
                adr = adr.offset(1);
                w1 = Dcg_Body(
                    *fresh14,
                    1 as libc::c_int,
                    in_word,
                    out_word,
                    0 as *mut WamWord,
                );
                w2 = Dcg_Body(
                    *adr,
                    1 as libc::c_int,
                    in_word,
                    out_word,
                    0 as *mut WamWord,
                );
                let fresh15 = top;
                top = top.offset(1);
                *fresh15 = Dcg_Compound2(
                    ';' as i32 as libc::c_uchar as libc::c_int,
                    w1,
                    w2,
                );
                return;
            }
            if arity == 1 as libc::c_int && func == atom_neg {
                word = Pl_Mk_Variable();
                w1 = Dcg_Body(*adr, 0 as libc::c_int, in_word, word, 0 as *mut WamWord);
                let fresh16 = top;
                top = top.offset(1);
                *fresh16 = Dcg_Compound1(func, w1);
                current_block = 15577247102568781450;
            } else if arity == 0 as libc::c_int
                && func == '!' as i32 as libc::c_uchar as libc::c_int
            {
                let fresh17 = top;
                top = top.offset(1);
                *fresh17 = dcg_body_word;
                current_block = 15577247102568781450;
            } else if arity == 1 as libc::c_int && func == pl_atom_curly_brackets {
                let fresh18 = top;
                top = top.offset(1);
                *fresh18 = *adr;
                current_block = 15577247102568781450;
            } else {
                current_block = 11822920164855298463;
            }
        }
        match current_block {
            11822920164855298463 => {}
            _ => {
                if opt_equal_between_in_out_vars != 0 {
                    Pl_Unify(in_word, out_word);
                } else {
                    let fresh10 = top;
                    top = top.offset(1);
                    *fresh10 = Dcg_Compound2(
                        '=' as i32 as libc::c_uchar as libc::c_int,
                        in_word,
                        out_word,
                    );
                }
                return;
            }
        }
    }
    save_H = H;
    p = save_H;
    let fresh19 = p;
    p = p.offset(1);
    *fresh19 = (((arity + 2 as libc::c_int) as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong)
        as WamWord;
    loop {
        let fresh20 = arity;
        arity = arity - 1;
        if !(fresh20 != 0) {
            break;
        }
        let fresh21 = adr;
        adr = adr.offset(1);
        let fresh22 = p;
        p = p.offset(1);
        *fresh22 = *fresh21;
    }
    let fresh23 = p;
    p = p.offset(1);
    *fresh23 = in_word;
    let fresh24 = p;
    p = p.offset(1);
    *fresh24 = out_word;
    H = p;
    let fresh25 = top;
    top = top.offset(1);
    *fresh25 = (save_H as PlLong as libc::c_ulong)
        .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
}
unsafe extern "C" fn Dcg_Term_List_On_Stack(
    mut lst_adr: *mut WamWord,
    mut in_word: WamWord,
    mut out_word: WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut save_lst_adr: *mut WamWord = lst_adr;
    let mut save_H: *mut WamWord = 0 as *mut WamWord;
    let mut p: *mut WamWord = 0 as *mut WamWord;
    save_H = H;
    p = save_H;
    loop {
        let fresh26 = p;
        p = p.offset(1);
        *fresh26 = *lst_adr.offset(0 as libc::c_int as isize);
        let mut deref_last_word: WamWord = 0;
        word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
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
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            Pl_Err_Instantiation();
        }
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            break;
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            Pl_Err_Type(
                pl_type_list,
                (save_lst_adr as PlLong as libc::c_ulong)
                    .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord,
            );
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        adr = p.offset(1 as libc::c_int as isize);
        let fresh27 = p;
        p = p.offset(1);
        *fresh27 = (adr as PlLong as libc::c_ulong)
            .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
    }
    let fresh28 = p;
    p = p.offset(1);
    *fresh28 = out_word;
    H = p;
    word = (save_H as PlLong as libc::c_ulong)
        .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
    if opt_term_unif != 0 {
        Pl_Unify(in_word, word);
    } else {
        opt_term_unif = 1 as libc::c_int;
        let fresh29 = top;
        top = top.offset(1);
        *fresh29 = Dcg_Compound2(
            '=' as i32 as libc::c_uchar as libc::c_int,
            in_word,
            word,
        );
    };
}
unsafe extern "C" fn Dcg_Compound1(mut func: libc::c_int, mut w1: WamWord) -> WamWord {
    let mut save_H: *mut WamWord = 0 as *mut WamWord;
    let mut p: *mut WamWord = 0 as *mut WamWord;
    save_H = H;
    p = save_H;
    let fresh30 = p;
    p = p.offset(1);
    *fresh30 = ((1 as libc::c_int as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong)
        as WamWord;
    let fresh31 = p;
    p = p.offset(1);
    *fresh31 = w1;
    H = p;
    return (save_H as PlLong as libc::c_ulong)
        .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
}
unsafe extern "C" fn Dcg_Compound2(
    mut func: libc::c_int,
    mut w1: WamWord,
    mut w2: WamWord,
) -> WamWord {
    let mut save_H: *mut WamWord = 0 as *mut WamWord;
    let mut p: *mut WamWord = 0 as *mut WamWord;
    save_H = H;
    p = save_H;
    let fresh32 = p;
    p = p.offset(1);
    *fresh32 = ((2 as libc::c_int as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong)
        as WamWord;
    let fresh33 = p;
    p = p.offset(1);
    *fresh33 = w1;
    let fresh34 = p;
    p = p.offset(1);
    *fresh34 = w2;
    H = p;
    return (save_H as PlLong as libc::c_ulong)
        .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
}

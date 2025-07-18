use ::libc;
extern "C" {
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Void(n: libc::c_int);
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Delete_Choice_Point(arity: libc::c_int);
    fn Pl_Defeasible_Open();
    fn Pl_Defeasible_Undo();
    fn Pl_Defeasible_Close(undo_before: Bool);
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    fn Pl_List_Length(start_word: WamWord) -> libc::c_int;
    static mut pl_type_integer: libc::c_int;
    static mut pl_type_list: libc::c_int;
    static mut pl_domain_not_less_than_zero: libc::c_int;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn X1_24617070656E645F616C74__a0();
    fn X1_246C656E6774685F616C74__a0();
    fn X1_246D656D6265725F616C74__a0();
    fn X1_24726576657273655F616C74__a0();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamWordP = *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Append_3(
    mut l1_word: WamWord,
    mut l2_word: WamWord,
    mut l3_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut len1: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    let mut len3: libc::c_int = 0;
    let mut result_word: WamWord = 0;
    let mut next_H: *mut WamWord = 0 as *mut WamWord;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = l1_word;
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
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            break;
        }
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let mut deref_last_word_0: WamWord = 0;
        word = l3_word;
        loop {
            deref_last_word_0 = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word_0) {
                break;
            }
        }
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            result_word = (H as PlLong as libc::c_ulong)
                .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
            loop {
                next_H = H.offset(2 as libc::c_int as isize);
                let fresh0 = H;
                H = H.offset(1);
                *fresh0 = *adr.offset(0 as libc::c_int as isize);
                let fresh1 = H;
                H = H.offset(1);
                *fresh1 = (next_H as PlLong as libc::c_ulong)
                    .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
                l1_word = *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
                let mut deref_last_word_1: WamWord = 0;
                word = l1_word;
                loop {
                    deref_last_word_1 = word;
                    tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                        as WamWord;
                    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                        break;
                    }
                    word = *(word as *mut WamWord);
                    if !(word != deref_last_word_1) {
                        break;
                    }
                }
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                if !(tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong) {
                    break;
                }
            }
            next_H = H.offset(-(1 as libc::c_int as isize));
            *next_H = (next_H as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
            Pl_Unify(result_word, l3_word);
            l3_word = *next_H;
            break;
        } else {
            if Pl_Get_List(l3_word) == 0
                || Pl_Unify_Value(*adr.offset(0 as libc::c_int as isize)) == 0
            {
                return 0 as libc::c_int;
            }
            l3_word = Pl_Unify_Variable();
            l1_word = *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
        }
    }
    if word as libc::c_ulong
        == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    {
        return Pl_Unify(l2_word, l3_word);
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        return 0 as libc::c_int;
    }
    len2 = Pl_List_Length(l2_word);
    if len2 >= 0 as libc::c_int
        && {
            len3 = Pl_List_Length(l3_word);
            len3 >= 0 as libc::c_int
        }
    {
        len1 = len3 - len2;
        if len1 < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        loop {
            let fresh2 = len1;
            len1 = len1 - 1;
            if !(fresh2 > 0 as libc::c_int) {
                break;
            }
            let mut deref_last_word_2: WamWord = 0;
            word = l3_word;
            loop {
                deref_last_word_2 = word;
                tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                    as WamWord;
                if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                    break;
                }
                word = *(word as *mut WamWord);
                if !(word != deref_last_word_2) {
                    break;
                }
            }
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            Pl_Get_List(l1_word);
            Pl_Unify_Value(*adr.offset(0 as libc::c_int as isize));
            l1_word = Pl_Unify_Variable();
            l3_word = *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
        }
        Pl_Get_Nil(l1_word);
        return Pl_Unify(l2_word, l3_word);
    }
    let mut deref_last_word_3: WamWord = 0;
    word = l3_word;
    loop {
        deref_last_word_3 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_3) {
            break;
        }
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong
    {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = l1_word;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = l2_word;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = l3_word;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_24617070656E645F616C74__a0),
                ),
            ),
            3 as libc::c_int,
        );
    }
    Pl_Get_Nil(l1_word);
    return Pl_Unify(l2_word, l3_word);
}
pub unsafe extern "C" fn Pl_Append_Alt_0() -> Bool {
    let mut x_word: WamWord = 0;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    Pl_Delete_Choice_Point(3 as libc::c_int);
    Pl_Get_List(*pl_reg_bank.offset(2 as libc::c_int as isize));
    x_word = Pl_Unify_Variable();
    *pl_reg_bank.offset(2 as libc::c_int as isize) = Pl_Unify_Variable();
    Pl_Get_List(*pl_reg_bank.offset(0 as libc::c_int as isize));
    Pl_Unify_Value(x_word);
    *pl_reg_bank.offset(0 as libc::c_int as isize) = Pl_Unify_Variable();
    let mut deref_last_word: WamWord = 0;
    word = *pl_reg_bank.offset(2 as libc::c_int as isize);
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong
    {
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_24617070656E645F616C74__a0),
                ),
            ),
            3 as libc::c_int,
        );
    }
    Pl_Get_Nil(*pl_reg_bank.offset(0 as libc::c_int as isize));
    return Pl_Unify(
        *pl_reg_bank.offset(1 as libc::c_int as isize),
        *pl_reg_bank.offset(2 as libc::c_int as isize),
    );
}
pub unsafe extern "C" fn Pl_Member_2() -> Bool {
    if Pl_Get_List(*pl_reg_bank.offset(1 as libc::c_int as isize)) == 0 {
        return 0 as libc::c_int;
    }
    *pl_reg_bank.offset(1 as libc::c_int as isize) = Pl_Unify_Variable();
    *pl_reg_bank.offset(2 as libc::c_int as isize) = Pl_Unify_Variable();
    return Pl_Member_3();
}
unsafe extern "C" fn Pl_Member_3() -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut ok: Bool = 0;
    loop {
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_246D656D6265725F616C74__a0),
                ),
            ),
            3 as libc::c_int,
        );
        ok = Pl_Unify(
            *pl_reg_bank.offset(0 as libc::c_int as isize),
            *pl_reg_bank.offset(1 as libc::c_int as isize),
        );
        let mut deref_last_word: WamWord = 0;
        word = *pl_reg_bank.offset(2 as libc::c_int as isize);
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
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
            && tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong
        {
            B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
            let ref mut fresh3 = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
            *fresh3 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
            return ok;
        }
        if ok != 0 {
            *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
                as *mut WamWord) = word;
            return ok;
        }
        Pl_Delete_Choice_Point(3 as libc::c_int);
        Pl_Get_List(*pl_reg_bank.offset(2 as libc::c_int as isize));
        *pl_reg_bank.offset(1 as libc::c_int as isize) = Pl_Unify_Variable();
        *pl_reg_bank.offset(2 as libc::c_int as isize) = Pl_Unify_Variable();
    };
}
pub unsafe extern "C" fn Pl_Member_Alt_0() -> Bool {
    Pl_Delete_Choice_Point(3 as libc::c_int);
    Pl_Get_List(*pl_reg_bank.offset(2 as libc::c_int as isize));
    *pl_reg_bank.offset(1 as libc::c_int as isize) = Pl_Unify_Variable();
    *pl_reg_bank.offset(2 as libc::c_int as isize) = Pl_Unify_Variable();
    return Pl_Member_3();
}
pub unsafe extern "C" fn Pl_Memberchk_2(
    mut elem_word: WamWord,
    mut list_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0;
    Pl_Defeasible_Open();
    loop {
        if Pl_Get_List(list_word) == 0 {
            ret = 0 as libc::c_int;
            break;
        } else if Pl_Unify_Value(elem_word) != 0 {
            Pl_Unify_Void(1 as libc::c_int);
            ret = 1 as libc::c_int;
            break;
        } else {
            Pl_Defeasible_Undo();
            list_word = Pl_Unify_Variable();
        }
    }
    Pl_Defeasible_Close(ret);
    return ret;
}
pub unsafe extern "C" fn Pl_Length_2(
    mut list_word: WamWord,
    mut n_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: PlLong = 0;
    let mut len: PlLong = 0 as libc::c_int as PlLong;
    let mut deref_last_word: WamWord = 0;
    word = n_word;
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
    n_word = word;
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        n = word << 0 as libc::c_int >> 3 as libc::c_int;
        if n < 0 as libc::c_int as libc::c_long {
            Pl_Err_Domain(pl_domain_not_less_than_zero, word);
        }
    } else {
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_integer, word);
        }
        n = -(1 as libc::c_int) as PlLong;
    }
    loop {
        let mut deref_last_word_0: WamWord = 0;
        word = list_word;
        loop {
            deref_last_word_0 = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word_0) {
                break;
            }
        }
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            return (n == len || Pl_Get_Integer(len, n_word) != 0) as libc::c_int;
        }
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            if n < 0 as libc::c_int as libc::c_long {
                break;
            }
            if n == len {
                return Pl_Get_Nil(word);
            }
            Pl_Get_List(word);
            Pl_Unify_Void(1 as libc::c_int);
            list_word = Pl_Unify_Variable();
            len += 1;
            len;
        } else {
            if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
                Pl_Err_Type(pl_type_list, word);
            }
            len += 1;
            len;
            if (n as PlULong) < len as PlULong {
                return 0 as libc::c_int;
            }
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            list_word = *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
        }
    }
    *pl_reg_bank.offset(0 as libc::c_int as isize) = list_word;
    *pl_reg_bank.offset(1 as libc::c_int as isize) = n_word;
    *pl_reg_bank
        .offset(
            2 as libc::c_int as isize,
        ) = ((len as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
    Pl_Create_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_246C656E6774685F616C74__a0),
            ),
        ),
        3 as libc::c_int,
    );
    Pl_Get_Nil(list_word);
    return Pl_Get_Integer(len, n_word);
}
pub unsafe extern "C" fn Pl_Length_Alt_0() -> Bool {
    let mut list_word: WamWord = 0;
    let mut n_word: WamWord = 0;
    let mut len: libc::c_int = 0;
    Pl_Delete_Choice_Point(3 as libc::c_int);
    list_word = *pl_reg_bank.offset(0 as libc::c_int as isize);
    n_word = *pl_reg_bank.offset(1 as libc::c_int as isize);
    len = ((*pl_reg_bank.offset(2 as libc::c_int as isize) << 0 as libc::c_int
        >> 3 as libc::c_int) + 1 as libc::c_int as libc::c_long) as libc::c_int;
    Pl_Get_List(list_word);
    Pl_Unify_Void(1 as libc::c_int);
    list_word = Pl_Unify_Variable();
    *pl_reg_bank.offset(0 as libc::c_int as isize) = list_word;
    *pl_reg_bank
        .offset(
            2 as libc::c_int as isize,
        ) = ((len as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
    Pl_Create_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_246C656E6774685F616C74__a0),
            ),
        ),
        3 as libc::c_int,
    );
    Pl_Get_Nil(list_word);
    return Pl_Get_Integer(len as PlLong, n_word);
}
pub unsafe extern "C" fn Pl_Nth0_3(
    mut n_word: WamWord,
    mut list_word: WamWord,
    mut res_word: WamWord,
    mut base: libc::c_int,
) -> Bool {
    let mut elem_word: WamWord = 0;
    let mut n: PlLong = Pl_Rd_Integer(n_word) - base as libc::c_long;
    if n < 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    loop {
        if Pl_Get_List(list_word) == 0 {
            return 0 as libc::c_int;
        }
        elem_word = Pl_Unify_Variable();
        list_word = Pl_Unify_Variable();
        if n == 0 as libc::c_int as libc::c_long {
            return Pl_Unify(elem_word, res_word);
        }
        n -= 1;
        n;
    };
}
pub unsafe extern "C" fn Pl_Reverse_2(
    mut l1_word: WamWord,
    mut l2_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut len1: libc::c_int = 0 as libc::c_int;
    let mut len2: libc::c_int = 0;
    let mut x_word: WamWord = 0;
    let mut result_word: WamWord = (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
        as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = l1_word;
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
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            break;
        }
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        word = (H as PlLong as libc::c_ulong).wrapping_add(0x1 as libc::c_int as PlULong)
            as WamWord;
        let fresh4 = H;
        H = H.offset(1);
        *fresh4 = *adr.offset(0 as libc::c_int as isize);
        let fresh5 = H;
        H = H.offset(1);
        *fresh5 = result_word;
        result_word = word;
        len1 += 1;
        len1;
        l1_word = *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    if word as libc::c_ulong
        == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    {
        return Pl_Unify(result_word, l2_word);
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        return 0 as libc::c_int;
    }
    len2 = Pl_List_Length(l2_word);
    if len2 >= 0 as libc::c_int {
        if len2 < len1 {
            return 0 as libc::c_int;
        }
        while len1 < len2 {
            Pl_Get_List(l1_word);
            x_word = Pl_Unify_Variable();
            l1_word = Pl_Unify_Variable();
            word = (H as PlLong as libc::c_ulong)
                .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
            let fresh6 = H;
            H = H.offset(1);
            *fresh6 = x_word;
            let fresh7 = H;
            H = H.offset(1);
            *fresh7 = result_word;
            result_word = word;
            len1 += 1;
            len1;
        }
    } else {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = l1_word;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = l2_word;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = result_word;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_24726576657273655F616C74__a0),
                ),
            ),
            3 as libc::c_int,
        );
    }
    Pl_Get_Nil(l1_word);
    return Pl_Unify(result_word, l2_word);
}
pub unsafe extern "C" fn Pl_Reverse_Alt_0() -> Bool {
    let mut x_word: WamWord = 0;
    let mut result_word: WamWord = 0;
    Pl_Delete_Choice_Point(3 as libc::c_int);
    Pl_Get_List(*pl_reg_bank.offset(0 as libc::c_int as isize));
    x_word = Pl_Unify_Variable();
    *pl_reg_bank.offset(0 as libc::c_int as isize) = Pl_Unify_Variable();
    result_word = (H as PlLong as libc::c_ulong)
        .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
    let fresh8 = H;
    H = H.offset(1);
    *fresh8 = x_word;
    let fresh9 = H;
    H = H.offset(1);
    *fresh9 = *pl_reg_bank.offset(2 as libc::c_int as isize);
    *pl_reg_bank.offset(2 as libc::c_int as isize) = result_word;
    Pl_Create_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_24726576657273655F616C74__a0),
            ),
        ),
        3 as libc::c_int,
    );
    Pl_Get_Nil(*pl_reg_bank.offset(0 as libc::c_int as isize));
    return Pl_Unify(result_word, *pl_reg_bank.offset(1 as libc::c_int as isize));
}

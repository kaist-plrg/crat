use ::libc;
extern "C" {
    static mut pl_stk_tbl: [InfStack; 0];
    fn Pl_M_Random_Integer(n: libc::c_int) -> libc::c_int;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Delete_Choice_Point(arity: libc::c_int);
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    static mut pl_type_list: libc::c_int;
    static mut pl_type_fd_variable: libc::c_int;
    static mut pl_sys_var: [PlLong; 0];
    fn Pl_Err_Instantiation();
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Range_Ith_Elem(range: *mut Range, n: libc::c_int) -> libc::c_int;
    fn Pl_Range_Next_After(range: *mut Range, n: libc::c_int) -> libc::c_int;
    fn Pl_Fd_Prolog_To_Value(arg_word: WamWord) -> libc::c_int;
    fn Pl_Fd_New_Variable_Interval(min: libc::c_int, max: libc::c_int) -> *mut WamWord;
    fn Pl_Fd_Display_Extra_Cstr(fdv_adr: *mut WamWord);
    fn Pl_Fd_In_Interval(
        fdv_adr: *mut WamWord,
        min: libc::c_int,
        max: libc::c_int,
    ) -> Bool;
    fn Pl_Fd_Assign_Value_Fast(fdv_adr: *mut WamWord, n: libc::c_int) -> Bool;
    fn Pl_Fd_Remove_Value(fdv_adr: *mut WamWord, n: libc::c_int) -> Bool;
    fn X1_24696E646F6D61696E5F616C74__a0();
    fn X1_2465787472615F637374725F616C74__a0();
    fn pl_fd_domain_r(x_word: WamWord, r_word: WamWord) -> Bool;
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
pub type VecWord = PlULong;
pub type Vector = *mut VecWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub extra_cstr: Bool,
    pub min: libc::c_int,
    pub max: libc::c_int,
    pub vec: Vector,
}
pub type CmpFct = Option::<unsafe extern "C" fn() -> Bool>;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub unsafe extern "C" fn Pl_Fd_Domain_2(
    mut list_word: WamWord,
    mut r_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = list_word;
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
        || tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
    {
        return pl_fd_domain_r(word, r_word);
    }
    save_list_word = list_word;
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
            Pl_Err_Type(pl_type_list, save_list_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        if pl_fd_domain_r(*lst_adr.offset(0 as libc::c_int as isize), r_word) == 0 {
            return 0 as libc::c_int;
        }
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Domain_Interval(
    mut x_word: WamWord,
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut v: PlLong = 0;
    let mut deref_last_word: WamWord = 0;
    word = x_word;
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
        if min > max {
            return 0 as libc::c_int;
        }
        if min == max {
            return Pl_Get_Integer(min as PlLong, x_word);
        }
        adr = word as *mut WamWord;
        fdv_adr = Pl_Fd_New_Variable_Interval(min, max);
        if adr
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || adr
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize) && adr < B
        {
            let fresh0 = TR;
            TR = TR.offset(1);
            *fresh0 = (adr as PlULong | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *adr = (fdv_adr as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        return 1 as libc::c_int;
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        v = word << 0 as libc::c_int >> 3 as libc::c_int;
        return (v >= min as libc::c_long && v <= max as libc::c_long) as libc::c_int;
    }
    if tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    return Pl_Fd_In_Interval(
        (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
        min,
        max,
    );
}
pub unsafe extern "C" fn Pl_Fd_Domain_Var_3(
    mut x_word: WamWord,
    mut l_word: WamWord,
    mut u_word: WamWord,
) -> Bool {
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    min = Pl_Fd_Prolog_To_Value(l_word);
    if min < 0 as libc::c_int {
        min = 0 as libc::c_int;
    }
    max = Pl_Fd_Prolog_To_Value(u_word);
    return Pl_Fd_Domain_Interval(x_word, min, max);
}
pub unsafe extern "C" fn Pl_Fd_Domain_3(
    mut list_word: WamWord,
    mut l_word: WamWord,
    mut u_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    min = Pl_Fd_Prolog_To_Value(l_word);
    if min < 0 as libc::c_int {
        min = 0 as libc::c_int;
    }
    max = Pl_Fd_Prolog_To_Value(u_word);
    let mut deref_last_word: WamWord = 0;
    word = list_word;
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
        || tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
    {
        return Pl_Fd_Domain_Interval(word, min, max);
    }
    save_list_word = list_word;
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
            Pl_Err_Type(pl_type_list, save_list_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        if Pl_Fd_Domain_Interval(*lst_adr.offset(0 as libc::c_int as isize), min, max)
            == 0
        {
            return 0 as libc::c_int;
        }
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Select_Value(
    mut fdv_adr: *mut WamWord,
    mut value_method: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    match value_method {
        0 | 5 => {
            return (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .min;
        }
        1 | 6 => {
            return (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .max;
        }
        4 | 3 => {
            n = (*fdv_adr.offset((4 as libc::c_int + 1 as libc::c_int) as isize)
                / 2 as libc::c_int as libc::c_long) as libc::c_int;
            return Pl_Range_Ith_Elem(
                fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range,
                n,
            );
        }
        2 => {
            n = *fdv_adr.offset((4 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int;
            n = Pl_M_Random_Integer(n);
            return Pl_Range_Ith_Elem(
                fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range,
                n + 1 as libc::c_int,
            );
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Indomain_2(
    mut x_word: WamWord,
    mut method_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut value_method: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    value_method = Pl_Rd_Integer(method_word) as libc::c_int;
    let mut deref_last_word: WamWord = 0;
    word = x_word;
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
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    fdv_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
        as *mut WamWord;
    loop {
        if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
            return 1 as libc::c_int;
        }
        value = Select_Value(fdv_adr, value_method);
        *pl_reg_bank
            .offset(
                0 as libc::c_int as isize,
            ) = fdv_adr as WamWord
            | (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .extra_cstr as libc::c_long;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = value_method as WamWord;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = value as WamWord;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_24696E646F6D61696E5F616C74__a0),
                ),
            ),
            3 as libc::c_int,
        );
        if !(value_method == 4 as libc::c_int) {
            break;
        }
        if Pl_Fd_In_Interval(fdv_adr, 0 as libc::c_int, value) == 0 {
            return 0 as libc::c_int;
        }
        tag_mask = (*fdv_adr as libc::c_ulong & 0x7 as libc::c_int as PlULong)
            as WamWord;
    }
    return Pl_Fd_Assign_Value_Fast(fdv_adr, value);
}
pub unsafe extern "C" fn Pl_Indomain_Alt_0() -> Bool {
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut extra_cstr: libc::c_int = 0;
    let mut value_method: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    Pl_Delete_Choice_Point(3 as libc::c_int);
    let ref mut fresh1 = *pl_sys_var.as_mut_ptr().offset(4 as libc::c_int as isize);
    *fresh1 += 1;
    *fresh1;
    fdv_adr = (*pl_reg_bank.offset(0 as libc::c_int as isize)
        & !(1 as libc::c_int) as libc::c_long) as *mut WamWord;
    extra_cstr = (*pl_reg_bank.offset(0 as libc::c_int as isize)
        & 1 as libc::c_int as libc::c_long) as libc::c_int;
    value_method = *pl_reg_bank.offset(1 as libc::c_int as isize) as libc::c_int;
    value = *pl_reg_bank.offset(2 as libc::c_int as isize) as libc::c_int;
    if value_method == 5 as libc::c_int {
        value_method = 5 as libc::c_int + 1 as libc::c_int;
    } else if value_method == 5 as libc::c_int + 1 as libc::c_int {
        value_method = 5 as libc::c_int;
    } else if value_method == 4 as libc::c_int {
        if Pl_Fd_In_Interval(
            fdv_adr,
            value + 1 as libc::c_int,
            (((1 as libc::c_int as PlLong)
                << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
                - 1 as libc::c_int as libc::c_long) as libc::c_int,
        ) == 0
        {
            return 0 as libc::c_int;
        }
        return Pl_Indomain_2(
            *fdv_adr,
            ((value_method as PlULong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong) as WamWord,
        );
    }
    if Pl_Fd_Remove_Value(fdv_adr, value) == 0 {
        if extra_cstr != 0 {
            Pl_Fd_Display_Extra_Cstr(fdv_adr);
        }
        return 0 as libc::c_int;
    }
    if *fdv_adr as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        if extra_cstr != 0 {
            Pl_Create_Choice_Point(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    CodePtr,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(X1_2465787472615F637374725F616C74__a0),
                    ),
                ),
                1 as libc::c_int,
            );
        }
        return 1 as libc::c_int;
    }
    value = Select_Value(fdv_adr, value_method);
    *pl_reg_bank.offset(1 as libc::c_int as isize) = value_method as WamWord;
    *pl_reg_bank.offset(2 as libc::c_int as isize) = value as WamWord;
    Pl_Create_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_24696E646F6D61696E5F616C74__a0),
            ),
        ),
        3 as libc::c_int,
    );
    return Pl_Fd_Assign_Value_Fast(fdv_adr, value);
}
pub unsafe extern "C" fn Pl_Extra_Cstr_Alt_0() -> Bool {
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    fdv_adr = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord) as *mut WamWord;
    Pl_Delete_Choice_Point(0 as libc::c_int);
    Pl_Fd_Display_Extra_Cstr(fdv_adr);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Sel_Array_From_List_2(
    mut list_word: WamWord,
    mut sel_array_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut array: *mut WamWord = 0 as *mut WamWord;
    let mut save_array: *mut WamWord = 0 as *mut WamWord;
    array = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    save_list_word = list_word;
    save_array = array;
    array = array.offset(1);
    array;
    loop {
        let mut deref_last_word: WamWord = 0;
        word = list_word;
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
            Pl_Err_Type(pl_type_list, save_list_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let mut deref_last_word_0: WamWord = 0;
        word = *lst_adr.offset(0 as libc::c_int as isize);
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
            Pl_Err_Instantiation();
        }
        if tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
            && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        {
            Pl_Err_Type(pl_type_fd_variable, word);
        }
        if tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
            fdv_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            let fresh2 = array;
            array = array.offset(1);
            *fresh2 = fdv_adr as WamWord;
            n += 1;
            n;
        }
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    *save_array = n as WamWord;
    let ref mut fresh3 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh3 = array;
    return Pl_Get_Integer(
        save_array
            .offset_from(
                (*pl_stk_tbl.as_mut_ptr().offset(1 as libc::c_int as isize)).stack,
            ) as libc::c_long,
        sel_array_word,
    );
}
pub unsafe extern "C" fn Pl_Fd_Sel_Array_Pick_Var_4(
    mut sel_array_word: WamWord,
    mut method_word: WamWord,
    mut reorder_word: WamWord,
    mut fdv_word: WamWord,
) -> Bool {
    let mut current_block: u64;
    let mut array: *mut *mut WamWord = 0 as *mut *mut WamWord;
    let mut p: *mut *mut WamWord = 0 as *mut *mut WamWord;
    let mut end: *mut *mut WamWord = 0 as *mut *mut WamWord;
    let mut cmp_meth: CmpFct = None;
    let mut n: PlLong = 0;
    let mut i: libc::c_int = 0;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut res_elem: *mut *mut WamWord = 0 as *mut *mut WamWord;
    let mut reorder: Bool = 0;
    let mut q: *mut *mut WamWord = 0 as *mut *mut WamWord;
    let mut nb_ground: libc::c_int = 0 as libc::c_int;
    array = ((*pl_stk_tbl.as_mut_ptr().offset(1 as libc::c_int as isize)).stack)
        .offset(Pl_Rd_Integer_Check(sel_array_word) as isize) as *mut *mut WamWord;
    n = *array.offset(0 as libc::c_int as isize) as PlLong;
    if n == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    array = array.offset(1);
    array;
    end = array.offset(n as isize);
    reorder = Pl_Rd_Integer_Check(reorder_word) as Bool;
    match Pl_Rd_Integer_Check(method_word) {
        1 => {
            cmp_meth = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool>,
                CmpFct,
            >(
                Some(
                    Cmp_First_Fail
                        as unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool,
                ),
            );
            current_block = 4775909272756257391;
        }
        2 => {
            cmp_meth = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool>,
                CmpFct,
            >(
                Some(
                    Cmp_Most_Constrained
                        as unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool,
                ),
            );
            current_block = 4775909272756257391;
        }
        3 => {
            cmp_meth = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool>,
                CmpFct,
            >(
                Some(
                    Cmp_Smallest
                        as unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool,
                ),
            );
            current_block = 4775909272756257391;
        }
        4 => {
            cmp_meth = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool>,
                CmpFct,
            >(
                Some(
                    Cmp_Largest
                        as unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool,
                ),
            );
            current_block = 4775909272756257391;
        }
        5 => {
            cmp_meth = ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool>,
                CmpFct,
            >(
                Some(
                    Cmp_Max_Regret
                        as unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool,
                ),
            );
            current_block = 4775909272756257391;
        }
        6 => {
            loop {
                i = Pl_M_Random_Integer(n as libc::c_int);
                end = end.offset(-1);
                end;
                n -= 1;
                n;
                fdv_adr = *array.offset(i as isize);
                let ref mut fresh4 = *array.offset(i as isize);
                *fresh4 = *end;
                *end = fdv_adr;
                if !(*fdv_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
                    & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong)
                {
                    let fresh5 = TR;
                    TR = TR.offset(1);
                    *fresh5 = *array.offset(-(1 as libc::c_int as isize)) as WamWord;
                    let fresh6 = TR;
                    TR = TR.offset(1);
                    *fresh6 = (array.offset(-(1 as libc::c_int as isize)) as PlULong
                        | 1 as libc::c_int as libc::c_ulong) as WamWord;
                    let ref mut fresh7 = *array.offset(-(1 as libc::c_int) as isize);
                    *fresh7 = n as *mut WamWord;
                    break;
                } else if n == 0 as libc::c_int as libc::c_long {
                    return 0 as libc::c_int
                }
            }
            current_block = 14193507707493092922;
        }
        _ => {
            current_block = 4775909272756257391;
        }
    }
    match current_block {
        4775909272756257391 => {
            p = array;
            while p < end {
                fdv_adr = *p;
                if !(*fdv_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
                    & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong)
                {
                    if res_elem.is_null() {
                        res_elem = p;
                    } else if ::std::mem::transmute::<
                        _,
                        fn(_, _) -> Bool,
                    >((Some(cmp_meth.unwrap())).unwrap())(*res_elem, fdv_adr) != 0
                    {
                        if reorder != 0 {
                            *p = *res_elem;
                            *res_elem = fdv_adr;
                        } else {
                            res_elem = p;
                        }
                    }
                } else {
                    nb_ground += 1;
                    nb_ground;
                }
                p = p.offset(1);
                p;
            }
            if res_elem.is_null() {
                return 0 as libc::c_int;
            }
            if n > 50 as libc::c_int as libc::c_long
                && nb_ground as libc::c_long >= n / 2 as libc::c_int as libc::c_long
            {
                n = n - nb_ground as libc::c_long;
                let mut s: *mut PlLong = array.offset(-(1 as libc::c_int as isize))
                    as *mut PlLong;
                let mut d: *mut PlLong = TR as *mut PlLong;
                let mut counter: libc::c_int = (n + 1 as libc::c_int as libc::c_long)
                    as libc::c_int;
                loop {
                    let fresh8 = s;
                    s = s.offset(1);
                    let fresh9 = d;
                    d = d.offset(1);
                    *fresh9 = *fresh8;
                    counter -= 1;
                    if !(counter != 0) {
                        break;
                    }
                }
                TR = TR.offset((n + 1 as libc::c_int as libc::c_long) as isize);
                let fresh10 = TR;
                TR = TR.offset(1);
                *fresh10 = n + 1 as libc::c_int as libc::c_long;
                let fresh11 = TR;
                TR = TR.offset(1);
                *fresh11 = (array.offset(-(1 as libc::c_int as isize)) as PlULong
                    | 2 as libc::c_int as libc::c_ulong) as WamWord;
                let ref mut fresh12 = *array.offset(-(1 as libc::c_int) as isize);
                *fresh12 = n as *mut WamWord;
                q = array;
                p = q;
                while n != 0 {
                    fdv_adr = *p;
                    if !(*fdv_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
                        & 0x7 as libc::c_int as PlULong
                        == 7 as libc::c_int as libc::c_ulong)
                    {
                        let fresh13 = q;
                        q = q.offset(1);
                        *fresh13 = *p;
                        n -= 1;
                        n;
                    }
                    p = p.offset(1);
                    p;
                }
            }
            fdv_adr = *res_elem;
        }
        _ => {}
    }
    return Pl_Unify(
        (fdv_adr as PlLong as libc::c_ulong).wrapping_add(0 as libc::c_int as PlULong)
            as WamWord,
        fdv_word,
    );
}
unsafe extern "C" fn Cmp_First_Fail(
    mut last_fdv_adr: *mut WamWord,
    mut new_fdv_adr: *mut WamWord,
) -> Bool {
    return (*new_fdv_adr.offset((4 as libc::c_int + 1 as libc::c_int) as isize)
        < *last_fdv_adr.offset((4 as libc::c_int + 1 as libc::c_int) as isize))
        as libc::c_int;
}
unsafe extern "C" fn Cmp_Most_Constrained(
    mut last_fdv_adr: *mut WamWord,
    mut new_fdv_adr: *mut WamWord,
) -> Bool {
    let mut l_nb: libc::c_int = *last_fdv_adr
        .offset((4 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
    let mut n_nb: libc::c_int = *new_fdv_adr
        .offset((4 as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int;
    return (n_nb < l_nb
        || n_nb == l_nb
            && *new_fdv_adr
                .offset(
                    (4 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                        ),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                > *last_fdv_adr
                    .offset(
                        (4 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                            ),
                                    ),
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    )) as libc::c_int;
}
unsafe extern "C" fn Cmp_Smallest(
    mut last_fdv_adr: *mut WamWord,
    mut new_fdv_adr: *mut WamWord,
) -> Bool {
    let mut l_min: libc::c_int = (*(last_fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .min;
    let mut n_min: libc::c_int = (*(new_fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .min;
    return (n_min < l_min
        || n_min == l_min
            && *new_fdv_adr
                .offset(
                    (4 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                        ),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                > *last_fdv_adr
                    .offset(
                        (4 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                            ),
                                    ),
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    )) as libc::c_int;
}
unsafe extern "C" fn Cmp_Largest(
    mut last_fdv_adr: *mut WamWord,
    mut new_fdv_adr: *mut WamWord,
) -> Bool {
    let mut l_max: libc::c_int = (*(last_fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .max;
    let mut n_max: libc::c_int = (*(new_fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .max;
    return (n_max > l_max
        || n_max == l_max
            && *new_fdv_adr
                .offset(
                    (4 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                        ),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                > *last_fdv_adr
                    .offset(
                        (4 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                            ),
                                    ),
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    )) as libc::c_int;
}
unsafe extern "C" fn Cmp_Max_Regret(
    mut last_fdv_adr: *mut WamWord,
    mut new_fdv_adr: *mut WamWord,
) -> Bool {
    let mut l_diff: libc::c_int = 0;
    let mut n_diff: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    min = (*(last_fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .min;
    l_diff = Pl_Range_Next_After(
        last_fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range,
        min,
    ) - min;
    min = (*(new_fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .min;
    n_diff = Pl_Range_Next_After(
        new_fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range,
        min,
    ) - min;
    return (n_diff > l_diff
        || n_diff == l_diff
            && *new_fdv_adr
                .offset(
                    (4 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                        ),
                                ),
                        )
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                )
                > *last_fdv_adr
                    .offset(
                        (4 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (2 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(
                                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                                            .wrapping_div(
                                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                            ),
                                    ),
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    )) as libc::c_int;
}

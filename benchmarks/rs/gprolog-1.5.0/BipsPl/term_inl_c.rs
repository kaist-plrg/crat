use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Get_Structure(
        func: libc::c_int,
        arity: libc::c_int,
        start_word: WamWord,
    ) -> Bool;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Void(n: libc::c_int);
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Defeasible_Open();
    fn Pl_Defeasible_Close(undo_before: Bool);
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    static mut pl_stk_tbl: [InfStack; 0];
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Unify_Occurs_Check(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    static mut pl_type_atom: libc::c_int;
    static mut pl_type_atomic: libc::c_int;
    static mut pl_type_list: libc::c_int;
    static mut pl_domain_non_empty_list: libc::c_int;
    static mut pl_domain_order: libc::c_int;
    fn Pl_Rd_Boolean_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Un_Positive_Check(value: PlLong, start_word: WamWord) -> Bool;
    static mut pl_glob_dico_var: [PlLong; 0];
    fn Pl_Copy_Term(dst_adr: *mut WamWord, src_adr: *mut WamWord);
    fn Pl_Rd_Positive_Check(start_word: WamWord) -> PlLong;
    fn Pl_Treat_Vars_Of_Term(
        start_word: WamWord,
        generic_var: Bool,
        fct: Option::<unsafe extern "C" fn() -> Bool>,
    ) -> Bool;
    fn Pl_Err_Instantiation();
    fn Pl_Check_For_Un_List(start_word: WamWord);
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Term_Compare(start_u_word: WamWord, start_v_word: WamWord) -> PlLong;
    static mut pl_representation_max_arity: libc::c_int;
    fn Pl_Unset_C_Bip_Name();
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn Pl_Un_Atom_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Set_C_Bip_Name(func_str: *mut libc::c_char, arity: libc::c_int);
    fn Pl_Un_Integer_Check(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Err_Representation(atom_flag: libc::c_int);
    static mut pl_representation_too_many_variables: libc::c_int;
    fn Pl_Term_Size(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Compound_Check(
        start_word: WamWord,
        func: *mut libc::c_int,
        arity: *mut libc::c_int,
    ) -> *mut WamWord;
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
pub struct InfStack {
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub env_var_name: *mut libc::c_char,
    pub p_def_size: *mut PlLong,
    pub default_size: libc::c_int,
    pub size: libc::c_int,
    pub stack: *mut WamWord,
}
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut var_ptr: *mut PlLong = 0 as *const PlLong as *mut PlLong;
static mut base_var_ptr: *mut PlLong = 0 as *const PlLong as *mut PlLong;
pub unsafe extern "C" fn Pl_Blt_Term_Eq(mut x: WamWord, mut y: WamWord) -> Bool {
    return (Pl_Term_Compare(x, y) == 0 as libc::c_int as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Term_Neq(mut x: WamWord, mut y: WamWord) -> Bool {
    return (Pl_Term_Compare(x, y) != 0 as libc::c_int as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Term_Lt(mut x: WamWord, mut y: WamWord) -> Bool {
    return (Pl_Term_Compare(x, y) < 0 as libc::c_int as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Term_Lte(mut x: WamWord, mut y: WamWord) -> Bool {
    return (Pl_Term_Compare(x, y) <= 0 as libc::c_int as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Term_Gt(mut x: WamWord, mut y: WamWord) -> Bool {
    return (Pl_Term_Compare(x, y) > 0 as libc::c_int as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Term_Gte(mut x: WamWord, mut y: WamWord) -> Bool {
    return (Pl_Term_Compare(x, y) >= 0 as libc::c_int as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Compare(
    mut cmp_word: WamWord,
    mut x: WamWord,
    mut y: WamWord,
) -> Bool {
    let mut cmp: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"compare\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int,
    );
    cmp = Pl_Term_Compare(x, y) as libc::c_int;
    c = (if cmp < 0 as libc::c_int {
        '<' as i32
    } else if cmp == 0 as libc::c_int {
        '=' as i32
    } else {
        '>' as i32
    }) as libc::c_char;
    res = Pl_Un_Atom_Check(c as libc::c_uchar as libc::c_int, cmp_word);
    if res == 0 {
        let mut word: WamWord = 0;
        let mut tag_mask: WamWord = 0;
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut deref_last_word: WamWord = 0;
        word = cmp_word;
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
        s = (*pl_atom_tbl.offset((word as PlULong >> 3 as libc::c_int) as isize)).name;
        if *s.offset(0 as libc::c_int as isize) as libc::c_int != '<' as i32
            && *s.offset(0 as libc::c_int as isize) as libc::c_int != '=' as i32
            && *s.offset(0 as libc::c_int as isize) as libc::c_int != '>' as i32
            || *s.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
        {
            Pl_Err_Domain(pl_domain_order, cmp_word);
        }
    }
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_Arg(
    mut arg_no_word: WamWord,
    mut term_word: WamWord,
    mut sub_term_word: WamWord,
) -> Bool {
    let mut arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut arg_no: PlLong = 0;
    Pl_Set_C_Bip_Name(
        b"arg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int,
    );
    arg_no = Pl_Rd_Positive_Check(arg_no_word) - 1 as libc::c_int as libc::c_long;
    arg_adr = Pl_Rd_Compound_Check(term_word, &mut func, &mut arity);
    Pl_Unset_C_Bip_Name();
    return ((arg_no as libc::c_ulong) < arity as libc::c_ulong
        && Pl_Unify(sub_term_word, *arg_adr.offset(arg_no as isize)) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Functor(
    mut term_word: WamWord,
    mut functor_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut tag_functor: WamWord = 0;
    let mut arity: PlLong = 0;
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"functor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int,
    );
    let mut deref_last_word: WamWord = 0;
    word = term_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
            res = (Pl_Un_Atom_Check(
                '.' as i32 as libc::c_uchar as libc::c_int,
                functor_word,
            ) != 0 && Pl_Un_Integer_Check(2 as libc::c_int as PlLong, arity_word) != 0)
                as libc::c_int;
        } else if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            res = (Pl_Un_Atom_Check(
                (*adr.offset(0 as libc::c_int as isize) as PlULong
                    & ((1 as libc::c_int as PlULong)
                        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int,
                functor_word,
            ) != 0
                && Pl_Un_Integer_Check(
                    (*adr.offset(0 as libc::c_int as isize) as PlULong
                        >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as PlLong,
                    arity_word,
                ) != 0) as libc::c_int;
        } else {
            res = (Pl_Unify(word, functor_word) != 0
                && Pl_Un_Integer_Check(0 as libc::c_int as PlLong, arity_word) != 0)
                as libc::c_int;
        }
    } else {
        let mut deref_last_word_0: WamWord = 0;
        word = functor_word;
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
        if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
            && tag_mask as libc::c_ulong != 0x4 as libc::c_int as PlULong
        {
            Pl_Err_Type(pl_type_atomic, functor_word);
        }
        tag_functor = tag_mask;
        functor_word = word;
        arity = Pl_Rd_Positive_Check(arity_word);
        if arity > (256 as libc::c_int - 1 as libc::c_int) as libc::c_long {
            Pl_Err_Representation(pl_representation_max_arity);
        }
        if tag_functor as libc::c_ulong == 0x3 as libc::c_int as PlULong
            && functor_word as PlULong >> 3 as libc::c_int
                == '.' as i32 as libc::c_uchar as libc::c_int as libc::c_ulong
            && arity == 2 as libc::c_int as libc::c_long
        {
            res = if Pl_Get_List(term_word) != 0 {
                Pl_Unify_Void(2 as libc::c_int);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        } else if tag_functor as libc::c_ulong == 0x3 as libc::c_int as PlULong
            && arity > 0 as libc::c_int as libc::c_long
        {
            res = if Pl_Get_Structure(
                (functor_word as PlULong >> 3 as libc::c_int) as libc::c_int,
                arity as libc::c_int,
                term_word,
            ) != 0
            {
                Pl_Unify_Void(arity as libc::c_int);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        } else {
            if arity != 0 as libc::c_int as libc::c_long {
                Pl_Err_Type(pl_type_atom, functor_word);
            }
            res = Pl_Unify(functor_word, term_word);
        }
    }
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_Univ(
    mut term_word: WamWord,
    mut list_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut car_word: WamWord = 0;
    let mut lst_length: libc::c_int = 0;
    let mut arg1_adr: *mut WamWord = 0 as *mut WamWord;
    let mut term_adr: *mut WamWord = 0 as *mut WamWord;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut stc_adr: *mut WamWord = 0 as *mut WamWord;
    let mut functor_word: WamWord = 0;
    let mut functor_tag: WamWord = 0;
    let mut functor: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    Pl_Set_C_Bip_Name(
        b"=..\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    let mut deref_last_word: WamWord = 0;
    word = term_word;
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
        term_adr = word as *mut WamWord;
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
            Pl_Err_Domain(pl_domain_non_empty_list, list_word);
        }
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_list, list_word);
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let mut deref_last_word_1: WamWord = 0;
        functor_word = *lst_adr.offset(0 as libc::c_int as isize);
        loop {
            deref_last_word_1 = functor_word;
            functor_tag = (functor_word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if functor_tag as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            functor_word = *(functor_word as *mut WamWord);
            if !(functor_word != deref_last_word_1) {
                break;
            }
        }
        if functor_tag as libc::c_ulong == 0 as libc::c_int as PlULong {
            Pl_Err_Instantiation();
        }
        let mut deref_last_word_2: WamWord = 0;
        word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
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
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            if functor_tag as libc::c_ulong != 0x3 as libc::c_int as PlULong
                && functor_tag as libc::c_ulong != 0x7 as libc::c_int as PlULong
                && functor_tag as libc::c_ulong != 0x4 as libc::c_int as PlULong
            {
                Pl_Err_Type(pl_type_atomic, functor_word);
            }
            term_word = functor_word;
        } else {
            if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
                Pl_Err_Instantiation();
            }
            if functor_tag as libc::c_ulong != 0x3 as libc::c_int as PlULong {
                Pl_Err_Type(pl_type_atom, functor_word);
            }
            if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
                Pl_Err_Type(pl_type_list, list_word);
            }
            functor = (functor_word as PlULong >> 3 as libc::c_int) as libc::c_int;
            stc_adr = H;
            H = H.offset(1);
            H;
            arity = 0 as libc::c_int;
            loop {
                arity += 1;
                arity;
                lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                let mut deref_last_word_3: WamWord = 0;
                word = *lst_adr.offset(0 as libc::c_int as isize);
                loop {
                    deref_last_word_3 = word;
                    tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                        as WamWord;
                    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                        break;
                    }
                    word = *(word as *mut WamWord);
                    if !(word != deref_last_word_3) {
                        break;
                    }
                }
                if tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
                    word = ((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                }
                let fresh1 = H;
                H = H.offset(1);
                *fresh1 = word;
                let mut deref_last_word_4: WamWord = 0;
                word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
                loop {
                    deref_last_word_4 = word;
                    tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                        as WamWord;
                    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                        break;
                    }
                    word = *(word as *mut WamWord);
                    if !(word != deref_last_word_4) {
                        break;
                    }
                }
                if word as libc::c_ulong
                    == (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                        as libc::c_ulong)
                        .wrapping_add(0x3 as libc::c_int as PlULong)
                {
                    break;
                }
                if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
                    Pl_Err_Instantiation();
                }
                if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
                    Pl_Err_Type(pl_type_list, list_word);
                }
            }
            if arity > 256 as libc::c_int - 1 as libc::c_int {
                Pl_Err_Representation(pl_representation_max_arity);
            }
            if functor == '.' as i32 as libc::c_uchar as libc::c_int
                && arity == 2 as libc::c_int
            {
                term_word = (stc_adr.offset(1 as libc::c_int as isize) as PlLong
                    as libc::c_ulong)
                    .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
            } else {
                *stc_adr = ((arity as PlULong)
                    << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                    | functor as libc::c_ulong) as WamWord;
                term_word = (stc_adr as PlLong as libc::c_ulong)
                    .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
            }
        }
        if term_adr
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || term_adr
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                && term_adr < B
        {
            let fresh2 = TR;
            TR = TR.offset(1);
            *fresh2 = (term_adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                as WamWord;
        }
        *term_adr = term_word;
        Pl_Unset_C_Bip_Name();
        return 1 as libc::c_int;
    } else {
        if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            car_word = ((('.' as i32 as libc::c_uchar as libc::c_int as PlLong)
                << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
            lst_length = 1 as libc::c_int + 2 as libc::c_int;
            arg1_adr = &mut *adr.offset(0 as libc::c_int as isize) as *mut WamWord;
        } else if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            car_word = ((((*adr.offset(0 as libc::c_int as isize) as PlULong
                & ((1 as libc::c_int as PlULong)
                    << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as PlLong)
                << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
            lst_length = (1 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    *adr.offset(0 as libc::c_int as isize) as PlULong
                        >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong),
                ) as libc::c_int;
            arg1_adr = &mut *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
                as *mut WamWord;
        } else if tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            car_word = (adr as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
            lst_length = 1 as libc::c_int + 0 as libc::c_int;
        } else {
            car_word = word;
            lst_length = 1 as libc::c_int + 0 as libc::c_int;
        }
        Pl_Check_For_Un_List(list_word);
        Pl_Unset_C_Bip_Name();
        loop {
            if Pl_Get_List(list_word) == 0 || Pl_Unify_Value(car_word) == 0 {
                return 0 as libc::c_int;
            }
            list_word = Pl_Unify_Variable();
            lst_length -= 1;
            if lst_length == 0 as libc::c_int {
                break;
            }
            let fresh0 = arg1_adr;
            arg1_adr = arg1_adr.offset(1);
            car_word = *fresh0;
        }
        return Pl_Get_Nil(list_word);
    };
}
pub unsafe extern "C" fn Pl_Copy_Term_2(
    mut u_word: WamWord,
    mut v_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut size: libc::c_int = 0;
    static mut fix_bug: WamWord = 0;
    size = Pl_Term_Size(u_word);
    fix_bug = u_word;
    Pl_Copy_Term(H, &mut fix_bug);
    word = *H;
    H = H.offset(size as isize);
    return Pl_Unify(word, v_word);
}
pub unsafe extern "C" fn Pl_Setarg_4(
    mut arg_no_word: WamWord,
    mut term_word: WamWord,
    mut new_value_word: WamWord,
    mut undo_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut undo: libc::c_int = 0;
    let mut arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut arg_no: libc::c_int = 0;
    arg_adr = Pl_Rd_Compound_Check(term_word, &mut func, &mut arity);
    arg_no = (Pl_Rd_Positive_Check(arg_no_word) - 1 as libc::c_int as libc::c_long)
        as libc::c_int;
    undo = Pl_Rd_Boolean_Check(undo_word);
    let mut deref_last_word: WamWord = 0;
    word = new_value_word;
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
    if undo == 0 && tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_atomic, word);
    }
    if arg_no as libc::c_uint >= arity as libc::c_uint {
        return 0 as libc::c_int;
    }
    if undo != 0 {
        if arg_adr.offset(arg_no as isize)
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || arg_adr.offset(arg_no as isize)
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                && arg_adr.offset(arg_no as isize) < B
        {
            let fresh3 = TR;
            TR = TR.offset(1);
            *fresh3 = *arg_adr.offset(arg_no as isize);
            let fresh4 = TR;
            TR = TR.offset(1);
            *fresh4 = (arg_adr.offset(arg_no as isize) as PlULong
                | 1 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *arg_adr.offset(arg_no as isize) = word;
    } else {
        *arg_adr.offset(arg_no as isize) = word;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Term_Ref_2(
    mut term_word: WamWord,
    mut ref_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut word1: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut ref_0: PlLong = 0;
    adr = 0 as *mut WamWord;
    word = term_word;
    loop {
        word1 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        adr = word as *mut WamWord;
        word = *adr;
        if !(word != word1) {
            break;
        }
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        ref_0 = Pl_Rd_Positive_Check(ref_word);
        adr = ((*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).stack)
            .offset(ref_0 as isize);
        return Pl_Unify(word, *adr);
    }
    if adr < (*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).stack
        || adr > H
    {
        adr = H;
        let fresh5 = H;
        H = H.offset(1);
        *fresh5 = word;
    }
    ref_0 = adr
        .offset_from((*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).stack)
        as libc::c_long;
    return Pl_Un_Positive_Check(ref_0, ref_word);
}
pub unsafe extern "C" fn Pl_Term_Variables_3(
    mut start_word: WamWord,
    mut list_word: WamWord,
    mut tail_word: WamWord,
) -> Bool {
    let mut p: *mut PlLong = 0 as *mut PlLong;
    if tail_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        Pl_Check_For_Un_List(list_word);
    }
    var_ptr = pl_glob_dico_var.as_mut_ptr();
    Pl_Treat_Vars_Of_Term(
        start_word,
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WamWord) -> Bool>,
            Option::<unsafe extern "C" fn() -> Bool>,
        >(Some(Collect_Variable as unsafe extern "C" fn(*mut WamWord) -> Bool)),
    );
    p = pl_glob_dico_var.as_mut_ptr();
    while p < var_ptr {
        if Pl_Get_List(list_word) == 0 || Pl_Unify_Value(*p) == 0 {
            return 0 as libc::c_int;
        }
        list_word = Pl_Unify_Variable();
        p = p.offset(1);
        p;
    }
    if tail_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        return Pl_Get_Nil(list_word);
    }
    return Pl_Unify(list_word, tail_word);
}
pub unsafe extern "C" fn Pl_Term_Variables_2(
    mut start_word: WamWord,
    mut list_word: WamWord,
) -> Bool {
    return Pl_Term_Variables_3(
        start_word,
        list_word,
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
    );
}
unsafe extern "C" fn Collect_Variable(mut adr: *mut WamWord) -> Bool {
    let mut p: *mut PlLong = 0 as *mut PlLong;
    p = pl_glob_dico_var.as_mut_ptr();
    while p < var_ptr {
        if *p == adr as PlLong {
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    if var_ptr.offset_from(pl_glob_dico_var.as_mut_ptr()) as libc::c_long
        >= 32768 as libc::c_int as libc::c_long
    {
        Pl_Err_Representation(pl_representation_too_many_variables);
    }
    let fresh6 = var_ptr;
    var_ptr = var_ptr.offset(1);
    *fresh6 = adr as PlLong;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Subsumes_Term_2(
    mut general_word: WamWord,
    mut specific_word: WamWord,
) -> Bool {
    let mut ret: Bool = 0 as libc::c_int;
    Pl_Defeasible_Open();
    var_ptr = pl_glob_dico_var.as_mut_ptr();
    base_var_ptr = var_ptr;
    Pl_Treat_Vars_Of_Term(
        specific_word,
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WamWord) -> Bool>,
            Option::<unsafe extern "C" fn() -> Bool>,
        >(Some(Collect_Variable as unsafe extern "C" fn(*mut WamWord) -> Bool)),
    );
    ret = (Pl_Unify_Occurs_Check(general_word, specific_word) != 0
        && Pl_Treat_Vars_Of_Term(
            specific_word,
            1 as libc::c_int,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut WamWord, WamWord) -> Bool>,
                Option::<unsafe extern "C" fn() -> Bool>,
            >(
                Some(
                    Check_Variable as unsafe extern "C" fn(*mut WamWord, WamWord) -> Bool,
                ),
            ),
        ) != 0 && base_var_ptr == var_ptr) as libc::c_int;
    Pl_Defeasible_Close(0 as libc::c_int);
    return ret;
}
unsafe extern "C" fn Check_Variable(
    mut adr: *mut WamWord,
    mut var_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr1: *mut WamWord = 0 as *mut WamWord;
    let mut p: *mut PlLong = 0 as *mut PlLong;
    if var_word as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 5 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    p = pl_glob_dico_var.as_mut_ptr();
    while p < base_var_ptr {
        if *p == adr as PlLong {
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    if base_var_ptr >= var_ptr {
        return 0 as libc::c_int;
    }
    let mut deref_last_word: WamWord = 0;
    word = *base_var_ptr;
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
    if word as libc::c_ulong & 0x7 as libc::c_int as PlULong
        != 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    adr1 = word as *mut WamWord;
    if adr1 != adr {
        return 0 as libc::c_int;
    }
    *base_var_ptr = adr1 as PlLong;
    base_var_ptr = base_var_ptr.offset(1);
    base_var_ptr;
    return 1 as libc::c_int;
}

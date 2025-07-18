use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    static mut pl_stk_tbl: [InfStack; 0];
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    fn Pl_Mk_Proper_List(n: libc::c_int, arg: *mut WamWord) -> WamWord;
    static mut pl_glob_dico_var: [PlLong; 0];
    fn Pl_Term_Compare(start_u_word: WamWord, start_v_word: WamWord) -> PlLong;
    fn Pl_Treat_Vars_Of_Term(
        start_word: WamWord,
        generic_var: Bool,
        fct: Option::<unsafe extern "C" fn() -> Bool>,
    ) -> Bool;
    fn Pl_Term_Size(start_word: WamWord) -> libc::c_int;
    fn Pl_Copy_Term(dst_adr: *mut WamWord, src_adr: *mut WamWord);
    fn Pl_Copy_Contiguous_Term(dst_adr: *mut WamWord, src_adr: *mut WamWord);
    static mut pl_representation_too_many_variables: libc::c_int;
    fn Pl_Err_Representation(atom_flag: libc::c_int);
    fn X1_2467726F75705F736F6C7574696F6E735F616C74__a0();
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
pub struct onesol {
    pub prev: OneSolP,
    pub sol_no: libc::c_int,
    pub term_size: libc::c_int,
    pub term_word: WamWord,
}
pub type OneSolP = *mut onesol;
pub type OneSol = onesol;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut exist_2: WamWord = 0;
static mut new_gen_word: WamWord = 0;
static mut bound_var_ptr: *mut PlLong = 0 as *const PlLong as *mut PlLong;
static mut free_var_base: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut dummy: OneSol = {
    let mut init = onesol {
        prev: 0 as *const onesol as OneSolP,
        sol_no: 0 as libc::c_int,
        term_size: 0 as libc::c_int,
        term_word: 0,
    };
    init
};
static mut sol: *mut OneSol = unsafe { &dummy as *const OneSol as *mut OneSol };
static mut key_var_ptr: *mut PlLong = 0 as *const PlLong as *mut PlLong;
static mut save_key_var_ptr: *mut PlLong = 0 as *const PlLong as *mut PlLong;
static mut next_key_var_ptr: *mut PlLong = 0 as *const PlLong as *mut PlLong;
pub unsafe extern "C" fn Pl_Free_Variables_4(
    mut templ_word: WamWord,
    mut gen_word: WamWord,
    mut gen1_word: WamWord,
    mut key_word: WamWord,
) -> Bool {
    let mut gl_key_word: WamWord = 0;
    let mut save_H: *mut WamWord = 0 as *mut WamWord;
    let mut arg: *mut WamWord = 0 as *mut WamWord;
    let mut nb_free_var: libc::c_int = 0 as libc::c_int;
    bound_var_ptr = pl_glob_dico_var.as_mut_ptr();
    Pl_Treat_Vars_Of_Term(
        templ_word,
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WamWord) -> Bool>,
            Option::<unsafe extern "C" fn() -> Bool>,
        >(Some(Bound_Var as unsafe extern "C" fn(*mut WamWord) -> Bool)),
    );
    new_gen_word = Existential_Variables(gen_word);
    let fresh0 = H;
    H = H.offset(1);
    save_H = fresh0;
    free_var_base = H;
    arg = free_var_base;
    Pl_Treat_Vars_Of_Term(
        new_gen_word,
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WamWord) -> Bool>,
            Option::<unsafe extern "C" fn() -> Bool>,
        >(Some(Free_Var as unsafe extern "C" fn(*mut WamWord) -> Bool)),
    );
    nb_free_var = H.offset_from(arg) as libc::c_long as libc::c_int;
    if nb_free_var == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if nb_free_var <= 256 as libc::c_int - 1 as libc::c_int {
        *save_H = ((nb_free_var as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
            | '.' as i32 as libc::c_uchar as libc::c_int as libc::c_ulong) as WamWord;
        gl_key_word = (save_H as PlLong as libc::c_ulong)
            .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
    } else {
        H = free_var_base;
        gl_key_word = Pl_Mk_Proper_List(nb_free_var, arg);
    }
    Pl_Unify(new_gen_word, gen1_word);
    return Pl_Unify(gl_key_word, key_word);
}
pub unsafe extern "C" fn Pl_Recover_Generator_1(mut gen1_word: WamWord) {
    Pl_Unify(new_gen_word, gen1_word);
}
unsafe extern "C" fn Bound_Var(mut adr: *mut WamWord) -> Bool {
    let mut p: *mut PlLong = 0 as *mut PlLong;
    p = pl_glob_dico_var.as_mut_ptr();
    while p < bound_var_ptr {
        if *p == adr as PlLong {
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    if bound_var_ptr.offset_from(pl_glob_dico_var.as_mut_ptr()) as libc::c_long
        >= 32768 as libc::c_int as libc::c_long
    {
        Pl_Err_Representation(pl_representation_too_many_variables);
    }
    let fresh1 = bound_var_ptr;
    bound_var_ptr = bound_var_ptr.offset(1);
    *fresh1 = adr as PlLong;
    return 1 as libc::c_int;
}
unsafe extern "C" fn Existential_Variables(mut start_word: WamWord) -> WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
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
    if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        if *adr.offset(0 as libc::c_int as isize) == exist_2 {
            Pl_Treat_Vars_Of_Term(
                *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                1 as libc::c_int,
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut WamWord) -> Bool>,
                    Option::<unsafe extern "C" fn() -> Bool>,
                >(Some(Bound_Var as unsafe extern "C" fn(*mut WamWord) -> Bool)),
            );
            word = Existential_Variables(
                *adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize),
            );
        }
    }
    return word;
}
unsafe extern "C" fn Free_Var(mut adr: *mut WamWord) -> Bool {
    let mut p: *mut PlLong = 0 as *mut PlLong;
    let mut word: WamWord = 0;
    p = pl_glob_dico_var.as_mut_ptr();
    while p < bound_var_ptr {
        if *p == adr as PlLong {
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    word = (adr as PlLong as libc::c_ulong).wrapping_add(0 as libc::c_int as PlULong)
        as WamWord;
    p = free_var_base;
    while p < H {
        if *p == word {
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    let fresh2 = H;
    H = H.offset(1);
    *fresh2 = word;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Stop_Mark_1(mut stop_word: WamWord) {
    Pl_Get_Integer((*sol).sol_no as PlLong, stop_word);
}
pub unsafe extern "C" fn Pl_Store_Solution_1(mut term_word: WamWord) {
    let mut s: *mut OneSol = 0 as *mut OneSol;
    let mut size: libc::c_int = 0;
    static mut fix_bug: WamWord = 0;
    size = Pl_Term_Size(term_word);
    s = Pl_Malloc_Check(
        (::std::mem::size_of::<OneSol>() as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<WamWord>() as libc::c_ulong)
            .wrapping_add(
                (size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong),
            ) as libc::c_uint,
        b"all_solut_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        373 as libc::c_int,
    ) as *mut OneSol;
    (*s).prev = sol;
    (*s).sol_no = (*sol).sol_no + 1 as libc::c_int;
    (*s).term_size = size;
    fix_bug = term_word;
    Pl_Copy_Term(&mut (*s).term_word, &mut fix_bug);
    sol = s;
}
pub unsafe extern "C" fn Pl_Recover_Solutions_4(
    mut stop_word: WamWord,
    mut handle_key_word: WamWord,
    mut list_word: WamWord,
    mut tail_word: WamWord,
) -> Bool {
    let mut stop: libc::c_int = 0;
    let mut nb_sol: libc::c_int = 0;
    let mut p: *mut WamWord = 0 as *mut WamWord;
    let mut q: *mut WamWord = 0 as *mut WamWord;
    let mut s: *mut OneSol = 0 as *mut OneSol;
    let mut handle_key: Bool = 0;
    stop = Pl_Rd_Integer(stop_word) as libc::c_int;
    nb_sol = (*sol).sol_no - stop;
    if nb_sol == 0 as libc::c_int {
        return Pl_Unify(list_word, tail_word);
    }
    handle_key = Pl_Rd_Integer(handle_key_word) as Bool;
    key_var_ptr = pl_glob_dico_var.as_mut_ptr();
    H = H.offset((2 as libc::c_int * nb_sol) as isize);
    if H
        > ((*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).stack)
            .offset(
                (*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).size
                    as isize,
            )
    {
        H = ((*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).stack)
            .offset(
                (*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).size
                    as isize,
            );
    }
    q = H;
    p = q;
    loop {
        let fresh3 = nb_sol;
        nb_sol = nb_sol - 1;
        if !(fresh3 != 0) {
            break;
        }
        p = p.offset(-1);
        p;
        *p = (p.offset(1 as libc::c_int as isize) as PlLong as libc::c_ulong)
            .wrapping_add(0x1 as libc::c_int as PlULong) as WamWord;
        p = p.offset(-1);
        *p = (H as PlLong as libc::c_ulong).wrapping_add(0 as libc::c_int as PlULong)
            as WamWord;
        Pl_Copy_Contiguous_Term(H, &mut (*sol).term_word);
        if handle_key != 0 {
            Handle_Key_Variables(*H);
        }
        H = H.offset((*sol).term_size as isize);
        s = sol;
        sol = (*sol).prev;
        free(s as *mut libc::c_void);
    }
    *q.offset(-(1 as libc::c_int) as isize) = tail_word;
    return Pl_Unify(
        (p as PlLong as libc::c_ulong).wrapping_add(0x1 as libc::c_int as PlULong)
            as WamWord,
        list_word,
    );
}
unsafe extern "C" fn Handle_Key_Variables(mut start_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    save_key_var_ptr = key_var_ptr;
    next_key_var_ptr = pl_glob_dico_var.as_mut_ptr();
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
    adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord;
    Pl_Treat_Vars_Of_Term(
        *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
        1 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut WamWord) -> Bool>,
            Option::<unsafe extern "C" fn() -> Bool>,
        >(Some(Link_Key_Var as unsafe extern "C" fn(*mut WamWord) -> Bool)),
    );
}
unsafe extern "C" fn Link_Key_Var(mut adr: *mut WamWord) -> Bool {
    let mut p: *mut PlLong = 0 as *mut PlLong;
    p = pl_glob_dico_var.as_mut_ptr();
    while p < key_var_ptr {
        if *p == adr as PlLong {
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    if next_key_var_ptr < save_key_var_ptr {
        *adr = *(*next_key_var_ptr as *mut WamWord);
        next_key_var_ptr = next_key_var_ptr.offset(1);
        next_key_var_ptr;
        return 1 as libc::c_int;
    }
    if key_var_ptr.offset_from(pl_glob_dico_var.as_mut_ptr()) as libc::c_long
        >= 32768 as libc::c_int as libc::c_long
    {
        Pl_Err_Representation(pl_representation_too_many_variables);
    }
    let fresh4 = key_var_ptr;
    key_var_ptr = key_var_ptr.offset(1);
    *fresh4 = adr as PlLong;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Group_Solutions_3(
    mut all_sol_word: WamWord,
    mut gl_key_word: WamWord,
    mut sol_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut key_word: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = all_sol_word;
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
    if word as libc::c_ulong
        == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong)
    {
        return 0 as libc::c_int;
    }
    word = Group(all_sol_word, gl_key_word, &mut key_word);
    if word as libc::c_ulong
        != (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = word;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = gl_key_word;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = sol_word;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_2467726F75705F736F6C7574696F6E735F616C74__a0),
                ),
            ),
            3 as libc::c_int,
        );
    }
    Pl_Unify(key_word, gl_key_word);
    return Pl_Unify(sol_word, all_sol_word);
}
pub unsafe extern "C" fn Pl_Group_Solutions_Alt_0() -> Bool {
    let mut all_sol_word: WamWord = 0;
    let mut gl_key_word: WamWord = 0;
    let mut sol_word: WamWord = 0;
    let mut word: WamWord = 0;
    let mut key_word: WamWord = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2467726F75705F736F6C7574696F6E735F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    all_sol_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    gl_key_word = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    sol_word = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord);
    word = Group(all_sol_word, gl_key_word, &mut key_word);
    if word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh5 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh5 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    } else {
        *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
            as *mut WamWord) = word;
    }
    Pl_Unify(key_word, gl_key_word);
    return Pl_Unify(sol_word, all_sol_word);
}
unsafe extern "C" fn Group(
    mut all_sol_word: WamWord,
    mut gl_key_word: WamWord,
    mut key_adr: *mut WamWord,
) -> WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut prev_lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut key_word: WamWord = 0;
    let mut key_word1: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = all_sol_word;
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
    lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
        as *mut WamWord;
    let mut deref_last_word_0: WamWord = 0;
    word = *lst_adr.offset(0 as libc::c_int as isize);
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
    adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord;
    key_word = *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
    *key_adr = key_word;
    loop {
        *lst_adr
            .offset(
                0 as libc::c_int as isize,
            ) = *adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize);
        prev_lst_adr = lst_adr;
        let mut deref_last_word_1: WamWord = 0;
        word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
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
        if word as libc::c_ulong
            == (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            return (0 as libc::c_int as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        }
        prev_lst_adr = lst_adr;
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let mut deref_last_word_2: WamWord = 0;
        word = *lst_adr.offset(0 as libc::c_int as isize);
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
        key_word1 = *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
        if Pl_Term_Compare(key_word, key_word1) != 0 as libc::c_int as libc::c_long {
            break;
        }
    }
    all_sol_word = *prev_lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    *prev_lst_adr
        .offset(
            (0 as libc::c_int + 1 as libc::c_int) as isize,
        ) = (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    return all_sol_word;
}

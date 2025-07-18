use ::libc;
extern "C" {
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Err_Resource(atom_resource: libc::c_int);
    static mut pl_type_fd_variable: libc::c_int;
    static mut pl_type_fd_bool_evaluable: libc::c_int;
    static mut pl_resource_too_big_fd_constraint: libc::c_int;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Fd_Prolog_To_Value(arg_word: WamWord) -> libc::c_int;
    fn Pl_Fd_New_Variable() -> *mut WamWord;
    fn Pl_Fd_Check_For_Bool_Var(x_word: WamWord) -> Bool;
    static mut pl_full_ac: Bool;
    fn pl_truth_x_in_l_u(x: WamWord, l: WamWord, u: WamWord, b: WamWord) -> Bool;
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
static mut bool_tbl: [WamWord; 21] = [0; 21];
static mut bool_xor: WamWord = 0;
static mut stack: [WamWord; 100000] = [0; 100000];
static mut sp: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut vars_tbl: [WamWord; 100000] = [0; 100000];
static mut vars_sp: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut func_tbl: [Option::<
    unsafe extern "C" fn(*mut WamWord, libc::c_int, *mut WamWord) -> Bool,
>; 23] = [None; 23];
pub unsafe extern "C" fn Pl_Fd_Bool_Meta_3(
    mut le_word: WamWord,
    mut re_word: WamWord,
    mut op_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut exp: *mut WamWord = 0 as *mut WamWord;
    let mut op: libc::c_int = 0;
    static mut h: [WamWord; 3] = [0; 3];
    let mut deref_last_word: WamWord = 0;
    word = op_word;
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
    op = (op_word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int;
    h[0 as libc::c_int as usize] = bool_tbl[op as usize];
    h[1 as libc::c_int as usize] = le_word;
    h[2 as libc::c_int as usize] = re_word;
    sp = stack.as_mut_ptr();
    vars_sp = vars_tbl.as_mut_ptr();
    exp = Simplify(
        1 as libc::c_int,
        (h.as_mut_ptr() as PlLong as libc::c_ulong)
            .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord,
    );
    if Load_Bool_Into_Word(exp, 1 as libc::c_int, 0 as *mut WamWord) == 0 {
        return 0 as libc::c_int;
    }
    loop {
        vars_sp = vars_sp.offset(-1);
        if !(vars_sp >= vars_tbl.as_mut_ptr()) {
            break;
        }
        let fresh0 = vars_sp;
        vars_sp = vars_sp.offset(-1);
        if *fresh0 == 0 as libc::c_int as libc::c_long {
            if Pl_Fd_Check_For_Bool_Var(*vars_sp) == 0 {
                return 0 as libc::c_int;
            }
        } else {
            let mut deref_last_word_0: WamWord = 0;
            word = *vars_sp;
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
                adr = word as *mut WamWord;
                fdv_adr = Pl_Fd_New_Variable();
                if adr
                    < *(pl_reg_bank as *mut WamWordP)
                        .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                    || adr
                        >= *(pl_reg_bank as *mut WamWordP)
                            .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                        && adr < B
                {
                    let fresh1 = TR;
                    TR = TR.offset(1);
                    *fresh1 = (adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                        as WamWord;
                }
                *adr = (fdv_adr as PlLong as libc::c_ulong)
                    .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
            }
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn Simplify(
    mut sign: libc::c_int,
    mut e_word: WamWord,
) -> *mut WamWord {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut f_n: WamWord = 0;
    let mut le_word: WamWord = 0;
    let mut re_word: WamWord = 0;
    let mut op: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut exp: *mut WamWord = 0 as *mut WamWord;
    let mut sp1: *mut WamWord = 0 as *mut WamWord;
    let mut l: WamWord = 0;
    let mut r: WamWord = 0;
    exp = sp;
    if sp.offset_from(stack.as_mut_ptr()) as libc::c_long
        > (100000 as libc::c_int - 5 as libc::c_int) as libc::c_long
    {
        Pl_Err_Resource(pl_resource_too_big_fd_constraint);
    }
    let mut deref_last_word: WamWord = 0;
    word = e_word;
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
        || tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong
    {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        if vars_sp.offset_from(vars_tbl.as_mut_ptr()) as libc::c_long
            == 100000 as libc::c_int as libc::c_long
        {
            Pl_Err_Resource(pl_resource_too_big_fd_constraint);
        }
        let fresh2 = vars_sp;
        vars_sp = vars_sp.offset(1);
        *fresh2 = word;
        let fresh3 = vars_sp;
        vars_sp = vars_sp.offset(1);
        *fresh3 = 0 as libc::c_int as WamWord;
        if sign != 1 as libc::c_int {
            let fresh4 = sp;
            sp = sp.offset(1);
            *fresh4 = 0 as libc::c_int as WamWord;
        }
        let fresh5 = sp;
        sp = sp.offset(1);
        *fresh5 = (adr as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        return exp;
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        n = (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int;
        if n as libc::c_uint > 1 as libc::c_int as libc::c_uint {
            current_block = 9833359234302688035;
        } else {
            let fresh6 = sp;
            sp = sp.offset(1);
            *fresh6 = (21 as libc::c_int
                + (if sign == 1 as libc::c_int { n } else { 1 as libc::c_int - n }))
                as WamWord;
            return exp;
        }
    } else if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        word = Pl_Put_Structure(
            '/' as i32 as libc::c_uchar as libc::c_int,
            2 as libc::c_int,
        );
        Pl_Unify_Value(e_word);
        Pl_Unify_Integer(0 as libc::c_int as PlLong);
        current_block = 9833359234302688035;
    } else {
        current_block = 14359455889292382949;
    }
    loop {
        match current_block {
            9833359234302688035 => {
                Pl_Err_Type(pl_type_fd_bool_evaluable, word);
                current_block = 14359455889292382949;
            }
            _ => {
                if tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong {
                    current_block = 9833359234302688035;
                    continue;
                }
                adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
                f_n = *adr.offset(0 as libc::c_int as isize);
                if bool_xor == f_n {
                    op = 2 as libc::c_int;
                    break;
                } else {
                    op = 0 as libc::c_int;
                    while op < 21 as libc::c_int {
                        if bool_tbl[op as usize] == f_n {
                            break;
                        }
                        op += 1;
                        op;
                    }
                    if !(op == 21 as libc::c_int) {
                        break;
                    }
                    word = Pl_Put_Structure(
                        '/' as i32 as libc::c_uchar as libc::c_int,
                        2 as libc::c_int,
                    );
                    Pl_Unify_Atom(
                        (*adr.offset(0 as libc::c_int as isize) as PlULong
                            & ((1 as libc::c_int as PlULong)
                                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as libc::c_int,
                    );
                    Pl_Unify_Integer(
                        (*adr.offset(0 as libc::c_int as isize) as PlULong
                            >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as PlLong,
                    );
                    current_block = 9833359234302688035;
                }
            }
        }
    }
    le_word = *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
    re_word = *adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize);
    if op == 0 as libc::c_int {
        return Simplify(-sign, le_word);
    }
    if sign != 1 as libc::c_int {
        op = if op % 2 as libc::c_int == 9 as libc::c_int % 2 as libc::c_int {
            op + 1 as libc::c_int
        } else {
            op - 1 as libc::c_int
        };
    }
    if op >= 9 as libc::c_int && op <= 20 as libc::c_int {
        Add_Fd_Variables(le_word);
        Add_Fd_Variables(re_word);
        n = if op == 13 as libc::c_int || op == 19 as libc::c_int {
            op - 2 as libc::c_int
        } else if op == 12 as libc::c_int || op == 18 as libc::c_int {
            op + 2 as libc::c_int
        } else {
            op
        };
        let fresh7 = sp;
        sp = sp.offset(1);
        *fresh7 = n as WamWord;
        let fresh8 = sp;
        sp = sp.offset(1);
        *fresh8 = if n == op { le_word } else { re_word };
        let fresh9 = sp;
        sp = sp.offset(1);
        *fresh9 = if n == op { re_word } else { le_word };
        return exp;
    }
    sp = sp.offset(3 as libc::c_int as isize);
    *exp.offset(0 as libc::c_int as isize) = op as WamWord;
    *exp
        .offset(
            1 as libc::c_int as isize,
        ) = Simplify(1 as libc::c_int, le_word) as WamWord;
    sp1 = sp;
    *exp
        .offset(
            2 as libc::c_int as isize,
        ) = Simplify(1 as libc::c_int, re_word) as WamWord;
    l = *(*exp.offset(1 as libc::c_int as isize) as *mut WamWord);
    r = *(*exp.offset(2 as libc::c_int as isize) as *mut WamWord);
    match op {
        1 => {
            if l == 21 as libc::c_int as libc::c_long {
                sp = exp;
                return Simplify(-(1 as libc::c_int), re_word);
            }
            if l == 22 as libc::c_int as libc::c_long {
                return *exp.offset(2 as libc::c_int as isize) as *mut WamWord;
            }
            if r == 21 as libc::c_int as libc::c_long {
                sp = exp;
                return Simplify(-(1 as libc::c_int), le_word);
            }
            if r == 22 as libc::c_int as libc::c_long {
                sp = sp1;
                return *exp.offset(1 as libc::c_int as isize) as *mut WamWord;
            }
            if l == 0 as libc::c_int as libc::c_long {
                let ref mut fresh10 = *exp.offset(1 as libc::c_int as isize);
                *fresh10 = (*fresh10 as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                    as WamWord as WamWord;
                sp = sp1;
                *exp
                    .offset(
                        2 as libc::c_int as isize,
                    ) = Simplify(-(1 as libc::c_int), re_word) as WamWord;
            } else if r == 0 as libc::c_int as libc::c_long {
                *exp
                    .offset(
                        1 as libc::c_int as isize,
                    ) = Simplify(-(1 as libc::c_int), le_word) as WamWord;
                let ref mut fresh11 = *exp.offset(2 as libc::c_int as isize);
                *fresh11 = (*fresh11 as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                    as WamWord as WamWord;
            }
        }
        2 => {
            if l == 21 as libc::c_int as libc::c_long {
                return *exp.offset(2 as libc::c_int as isize) as *mut WamWord;
            }
            if l == 22 as libc::c_int as libc::c_long {
                sp = exp;
                return Simplify(-(1 as libc::c_int), re_word);
            }
            if r == 21 as libc::c_int as libc::c_long {
                sp = sp1;
                return *exp.offset(1 as libc::c_int as isize) as *mut WamWord;
            }
            if r == 22 as libc::c_int as libc::c_long {
                sp = exp;
                return Simplify(-(1 as libc::c_int), le_word);
            }
            if l == 0 as libc::c_int as libc::c_long {
                *exp.offset(0 as libc::c_int as isize) = 1 as libc::c_int as WamWord;
                let ref mut fresh12 = *exp.offset(1 as libc::c_int as isize);
                *fresh12 = (*fresh12 as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                    as WamWord as WamWord;
            } else if r == 0 as libc::c_int as libc::c_long {
                *exp.offset(0 as libc::c_int as isize) = 1 as libc::c_int as WamWord;
                let ref mut fresh13 = *exp.offset(2 as libc::c_int as isize);
                *fresh13 = (*fresh13 as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                    as WamWord as WamWord;
            } else if l >= 22 as libc::c_int as libc::c_long
                && !(r >= 22 as libc::c_int as libc::c_long)
            {
                *exp.offset(0 as libc::c_int as isize) = 1 as libc::c_int as WamWord;
                sp = sp1;
                *exp
                    .offset(
                        2 as libc::c_int as isize,
                    ) = Simplify(-(1 as libc::c_int), re_word) as WamWord;
            } else if r >= 22 as libc::c_int as libc::c_long
                && !(l >= 22 as libc::c_int as libc::c_long)
            {
                *exp.offset(0 as libc::c_int as isize) = 1 as libc::c_int as WamWord;
                *exp
                    .offset(
                        1 as libc::c_int as isize,
                    ) = Simplify(-(1 as libc::c_int), le_word) as WamWord;
            }
        }
        3 => {
            if l == 21 as libc::c_int as libc::c_long
                || r == 22 as libc::c_int as libc::c_long
            {
                sp = exp;
                let fresh14 = sp;
                sp = sp.offset(1);
                *fresh14 = 22 as libc::c_int as WamWord;
            } else {
                if l == 22 as libc::c_int as libc::c_long {
                    return *exp.offset(2 as libc::c_int as isize) as *mut WamWord;
                }
                if r == 21 as libc::c_int as libc::c_long {
                    sp = exp;
                    return Simplify(-(1 as libc::c_int), le_word);
                }
                if l == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 7 as libc::c_int as WamWord;
                    let ref mut fresh15 = *exp.offset(1 as libc::c_int as isize);
                    *fresh15 = (*fresh15 as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord as WamWord;
                } else if r == 0 as libc::c_int as libc::c_long {
                    *exp
                        .offset(
                            1 as libc::c_int as isize,
                        ) = (*exp.offset(2 as libc::c_int as isize) as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord;
                    *exp
                        .offset(
                            2 as libc::c_int as isize,
                        ) = Simplify(-(1 as libc::c_int), le_word) as WamWord;
                }
            }
        }
        4 => {
            if l == 21 as libc::c_int as libc::c_long
                || r == 22 as libc::c_int as libc::c_long
            {
                sp = exp;
                let fresh16 = sp;
                sp = sp.offset(1);
                *fresh16 = 21 as libc::c_int as WamWord;
            } else {
                if l == 22 as libc::c_int as libc::c_long {
                    sp = exp;
                    return Simplify(-(1 as libc::c_int), re_word);
                }
                if r == 21 as libc::c_int as libc::c_long {
                    sp = sp1;
                    return *exp.offset(1 as libc::c_int as isize) as *mut WamWord;
                }
                if l == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 8 as libc::c_int as WamWord;
                    let ref mut fresh17 = *exp.offset(1 as libc::c_int as isize);
                    *fresh17 = (*fresh17 as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord as WamWord;
                } else if r == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 5 as libc::c_int as WamWord;
                    let ref mut fresh18 = *exp.offset(2 as libc::c_int as isize);
                    *fresh18 = (*fresh18 as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord as WamWord;
                }
            }
        }
        5 => {
            if l == 21 as libc::c_int as libc::c_long
                || r == 21 as libc::c_int as libc::c_long
            {
                sp = exp;
                let fresh19 = sp;
                sp = sp.offset(1);
                *fresh19 = 21 as libc::c_int as WamWord;
            } else {
                if l == 22 as libc::c_int as libc::c_long {
                    return *exp.offset(2 as libc::c_int as isize) as *mut WamWord;
                }
                if r == 22 as libc::c_int as libc::c_long {
                    sp = sp1;
                    return *exp.offset(1 as libc::c_int as isize) as *mut WamWord;
                }
                if l == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 4 as libc::c_int as WamWord;
                    word = *exp.offset(1 as libc::c_int as isize);
                    *exp
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *exp.offset(2 as libc::c_int as isize);
                    *exp
                        .offset(
                            2 as libc::c_int as isize,
                        ) = (word as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord;
                } else if r == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 4 as libc::c_int as WamWord;
                    let ref mut fresh20 = *exp.offset(2 as libc::c_int as isize);
                    *fresh20 = (*fresh20 as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord as WamWord;
                }
            }
        }
        6 => {
            if l == 21 as libc::c_int as libc::c_long
                || r == 21 as libc::c_int as libc::c_long
            {
                sp = exp;
                let fresh21 = sp;
                sp = sp.offset(1);
                *fresh21 = 22 as libc::c_int as WamWord;
            } else {
                if l == 22 as libc::c_int as libc::c_long {
                    sp = exp;
                    return Simplify(-(1 as libc::c_int), re_word);
                }
                if r == 22 as libc::c_int as libc::c_long {
                    sp = exp;
                    return Simplify(-(1 as libc::c_int), le_word);
                }
                if l == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 3 as libc::c_int as WamWord;
                    word = *exp.offset(1 as libc::c_int as isize);
                    *exp
                        .offset(
                            1 as libc::c_int as isize,
                        ) = *exp.offset(2 as libc::c_int as isize);
                    *exp
                        .offset(
                            2 as libc::c_int as isize,
                        ) = (word as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord;
                } else if r == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 3 as libc::c_int as WamWord;
                    let ref mut fresh22 = *exp.offset(2 as libc::c_int as isize);
                    *fresh22 = (*fresh22 as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord as WamWord;
                }
            }
        }
        7 => {
            if l == 22 as libc::c_int as libc::c_long
                || r == 22 as libc::c_int as libc::c_long
            {
                sp = exp;
                let fresh23 = sp;
                sp = sp.offset(1);
                *fresh23 = 22 as libc::c_int as WamWord;
            } else {
                if l == 21 as libc::c_int as libc::c_long {
                    return *exp.offset(2 as libc::c_int as isize) as *mut WamWord;
                }
                if r == 21 as libc::c_int as libc::c_long {
                    sp = sp1;
                    return *exp.offset(1 as libc::c_int as isize) as *mut WamWord;
                }
                if l == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 3 as libc::c_int as WamWord;
                    let ref mut fresh24 = *exp.offset(1 as libc::c_int as isize);
                    *fresh24 = (*fresh24 as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord as WamWord;
                } else if r == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 3 as libc::c_int as WamWord;
                    word = *exp.offset(1 as libc::c_int as isize);
                    *exp
                        .offset(
                            1 as libc::c_int as isize,
                        ) = (*exp.offset(2 as libc::c_int as isize) as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord;
                    *exp.offset(2 as libc::c_int as isize) = word;
                }
            }
        }
        8 => {
            if l == 22 as libc::c_int as libc::c_long
                || r == 22 as libc::c_int as libc::c_long
            {
                sp = exp;
                let fresh25 = sp;
                sp = sp.offset(1);
                *fresh25 = 21 as libc::c_int as WamWord;
            } else {
                if l == 21 as libc::c_int as libc::c_long {
                    sp = exp;
                    return Simplify(-(1 as libc::c_int), re_word);
                }
                if r == 21 as libc::c_int as libc::c_long {
                    sp = exp;
                    return Simplify(-(1 as libc::c_int), le_word);
                }
                if l == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 4 as libc::c_int as WamWord;
                    let ref mut fresh26 = *exp.offset(1 as libc::c_int as isize);
                    *fresh26 = (*fresh26 as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord as WamWord;
                } else if r == 0 as libc::c_int as libc::c_long {
                    *exp.offset(0 as libc::c_int as isize) = 4 as libc::c_int as WamWord;
                    word = *exp.offset(1 as libc::c_int as isize);
                    *exp
                        .offset(
                            1 as libc::c_int as isize,
                        ) = (*exp.offset(2 as libc::c_int as isize) as libc::c_ulong)
                        .wrapping_add(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as WamWord;
                    *exp.offset(2 as libc::c_int as isize) = word;
                }
            }
        }
        _ => {}
    }
    return exp;
}
unsafe extern "C" fn Add_Fd_Variables(mut e_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = e_word;
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
        if vars_sp.offset_from(vars_tbl.as_mut_ptr()) as libc::c_long
            == 100000 as libc::c_int as libc::c_long
        {
            Pl_Err_Resource(pl_resource_too_big_fd_constraint);
        }
        let fresh27 = vars_sp;
        vars_sp = vars_sp.offset(1);
        *fresh27 = word;
        let fresh28 = vars_sp;
        vars_sp = vars_sp.offset(1);
        *fresh28 = 1 as libc::c_int as WamWord;
        return;
    }
    if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        Add_Fd_Variables(*adr.offset(0 as libc::c_int as isize));
        Add_Fd_Variables(*adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize));
    }
    if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        i = (*adr.offset(0 as libc::c_int as isize) as PlULong
            >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
        loop {
            i -= 1;
            Add_Fd_Variables(*adr.offset((1 as libc::c_int + i) as isize));
            if !(i != 0) {
                break;
            }
        }
    }
}
unsafe extern "C" fn Load_Bool_Into_Word(
    mut exp: *mut WamWord,
    mut result: libc::c_int,
    mut load_word: *mut WamWord,
) -> Bool {
    let mut op: PlULong = *exp as PlULong;
    if op >= 15 as libc::c_int as libc::c_ulong
        && op <= 20 as libc::c_int as libc::c_ulong
    {
        pl_full_ac = 1 as libc::c_int;
        op = op
            .wrapping_sub(15 as libc::c_int as libc::c_ulong)
            .wrapping_add(9 as libc::c_int as libc::c_ulong);
    } else {
        pl_full_ac = 0 as libc::c_int;
    }
    return (Some(
        if op <= 22 as libc::c_int as libc::c_ulong {
            *func_tbl.as_mut_ptr().offset(op as isize)
        } else {
            Some(
                Set_Var
                    as unsafe extern "C" fn(
                        *mut WamWord,
                        libc::c_int,
                        *mut WamWord,
                    ) -> Bool,
            )
        }
            .unwrap(),
    ))
        .unwrap()(exp, result, load_word);
}
unsafe extern "C" fn Set_Var(
    mut exp: *mut WamWord,
    mut result: libc::c_int,
    mut load_word: *mut WamWord,
) -> Bool {
    if result == 0 as libc::c_int {
        return Pl_Get_Integer(0 as libc::c_int as PlLong, *exp);
    }
    if result == 1 as libc::c_int {
        return Pl_Get_Integer(1 as libc::c_int as PlLong, *exp);
    }
    *load_word = *exp;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Reified_In(
    mut x_word: WamWord,
    mut l_word: WamWord,
    mut u_word: WamWord,
    mut b_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut b_tag_mask: WamWord = 0;
    let mut x_tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut x: PlLong = 0;
    let mut b: PlLong = -(1 as libc::c_int) as PlLong;
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut x_min: libc::c_int = 0;
    let mut x_max: libc::c_int = 0;
    let mut r: *mut Range = 0 as *mut Range;
    extern "C" {
        #[link_name = "Pl_Fd_Domain_Interval"]
        fn Pl_Fd_Domain_Interval_0(
            x_word_0: WamWord,
            min_0: libc::c_int,
            max_0: libc::c_int,
        ) -> Bool;
    }
    extern "C" {
        #[link_name = "pl_fd_not_domain"]
        fn pl_fd_not_domain_0(
            x_word_0: WamWord,
            l_word_0: WamWord,
            u_word_0: WamWord,
        ) -> Bool;
    }
    min = Pl_Fd_Prolog_To_Value(l_word);
    if min < 0 as libc::c_int {
        min = 0 as libc::c_int;
    }
    max = Pl_Fd_Prolog_To_Value(u_word);
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
    x_word = word;
    x_tag_mask = tag_mask;
    if !(tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong)
    {
        let mut deref_last_word_0: WamWord = 0;
        word = b_word;
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
        b_word = word;
        b_tag_mask = tag_mask;
        if !(tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
            && tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong
            && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong)
        {
            if x_tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
                x = x_word << 0 as libc::c_int >> 3 as libc::c_int;
                b = (x >= min as libc::c_long && x <= max as libc::c_long) as libc::c_int
                    as PlLong;
            } else {
                if b_tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
                    b = b_word << 0 as libc::c_int >> 3 as libc::c_int;
                    if b == 0 as libc::c_int as libc::c_long {
                        return pl_fd_not_domain_0(x_word, l_word, u_word);
                    }
                    return (b == 1 as libc::c_int as libc::c_long
                        && Pl_Fd_Domain_Interval_0(x_word, min, max) != 0)
                        as libc::c_int;
                }
                if x_tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
                    adr = x_word as *mut WamWord;
                    fdv_adr = Pl_Fd_New_Variable();
                    if adr
                        < *(pl_reg_bank as *mut WamWordP)
                            .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                        || adr
                            >= *(pl_reg_bank as *mut WamWordP)
                                .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                            && adr < B
                    {
                        let fresh29 = TR;
                        TR = TR.offset(1);
                        *fresh29 = (adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                            as WamWord;
                    }
                    *adr = (fdv_adr as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                } else {
                    fdv_adr = (x_word as libc::c_ulong
                        & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord;
                }
                r = fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range;
                x_min = (*r).min;
                x_max = (*r).max;
                if x_min >= min && x_max <= max {
                    b = 1 as libc::c_int as PlLong;
                } else if min > max || x_max < min || x_min > max {
                    b = 0 as libc::c_int as PlLong;
                } else {
                    if Pl_Fd_Check_For_Bool_Var(b_word) == 0 {
                        return 0 as libc::c_int;
                    }
                    if pl_truth_x_in_l_u(x_word, l_word, u_word, b_word) == 0 {
                        return 0 as libc::c_int;
                    }
                    return 1 as libc::c_int;
                }
            }
            return Pl_Get_Integer(b, b_word);
        }
    }
    Pl_Err_Type(pl_type_fd_variable, word);
    return 0 as libc::c_int;
}

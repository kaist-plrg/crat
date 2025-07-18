use ::libc;
extern "C" {
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn cosh(_: libc::c_double) -> libc::c_double;
    fn sinh(_: libc::c_double) -> libc::c_double;
    fn tanh(_: libc::c_double) -> libc::c_double;
    fn acosh(_: libc::c_double) -> libc::c_double;
    fn asinh(_: libc::c_double) -> libc::c_double;
    fn atanh(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn rint(_: libc::c_double) -> libc::c_double;
    fn Pl_Hash_Find(tbl: *mut libc::c_char, key: PlLong) -> *mut libc::c_char;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Global_Push_Float(n: libc::c_double);
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Obtain_Float(adr: *mut WamWord) -> libc::c_double;
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    static mut pl_type_evaluable: libc::c_int;
    fn Pl_Err_Instantiation();
    static mut pl_type_integer: libc::c_int;
    static mut pl_evaluation_int_overflow: libc::c_int;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Positive_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_String_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Check_For_Un_Positive(start_word: WamWord);
    fn Pl_Un_Positive_Check(value: PlLong, start_word: WamWord) -> Bool;
    static mut pl_flag_strict_iso: *mut FlagInf;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    static mut pl_type_float: libc::c_int;
    static mut pl_evaluation_undefined: libc::c_int;
    static mut pl_evaluation_float_overflow: libc::c_int;
    fn Pl_Set_C_Bip_Name(func_str: *mut libc::c_char, arity: libc::c_int);
    static mut pl_evaluation_zero_divisor: libc::c_int;
    fn Pl_Err_Evaluation(pl_atom_error: libc::c_int);
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
pub struct ArithInf {
    pub f_n: WamWord,
    pub fct: Option::<unsafe extern "C" fn() -> WamWord>,
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
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut arith_tbl: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut atom_pi: libc::c_int = 0;
static mut atom_e: libc::c_int = 0;
static mut atom_epsilon: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Define_Math_Bip_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) {
    let mut cur_bip_func: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur_bip_arity: libc::c_int = 0;
    cur_bip_func = Pl_Rd_String_Check(func_word);
    cur_bip_arity = Pl_Rd_Integer_Check(arity_word) as libc::c_int;
    Pl_Set_C_Bip_Name(cur_bip_func, cur_bip_arity);
}
unsafe extern "C" fn Check_Double_Errors(mut x: libc::c_double) {
    let mut current_block_1: u64;
    match !x.is_nan() as usize {
        0 => {
            Pl_Err_Evaluation(pl_evaluation_undefined);
            current_block_1 = 10034517057706336272;
        }
        1 => {
            current_block_1 = 10034517057706336272;
        }
        _ => {
            current_block_1 = 12517898123489920830;
        }
    }
    match current_block_1 {
        10034517057706336272 => {
            Pl_Err_Evaluation(pl_evaluation_float_overflow);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn Pl_NaN() -> libc::c_double {
    return ::std::f32::NAN as libc::c_double;
}
unsafe extern "C" fn Double_To_PlLong(mut d: libc::c_double) -> PlLong {
    Check_Double_Errors(d);
    let mut x: PlLong = d as PlLong;
    if x as libc::c_double != d {
        Pl_Err_Evaluation(pl_evaluation_int_overflow);
    }
    return x;
}
pub unsafe extern "C" fn Pl_Math_Load_Value(
    mut start_word: WamWord,
    mut word_adr: *mut WamWord,
) {
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
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x4 as libc::c_int as PlULong
    {
        word = Load_Math_Expression(word);
    }
    *word_adr = word;
}
pub unsafe extern "C" fn Pl_Math_Fast_Load_Value(
    mut start_word: WamWord,
    mut word_adr: *mut WamWord,
) {
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
    *word_adr = word;
}
unsafe extern "C" fn Make_Tagged_Float(mut d: libc::c_double) -> WamWord {
    let mut x: WamWord = 0;
    Check_Double_Errors(d);
    x = (H as PlLong as libc::c_ulong).wrapping_add(0x4 as libc::c_int as PlULong)
        as WamWord;
    Pl_Global_Push_Float(d);
    return x;
}
unsafe extern "C" fn To_Double(mut x: WamWord) -> libc::c_double {
    return if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        (x << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_double
    } else {
        Pl_Obtain_Float(
            (x as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
        )
    };
}
unsafe extern "C" fn Load_Math_Expression(mut exp_0: WamWord) -> WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut arith: *mut ArithInf = 0 as *mut ArithInf;
    let mut atom: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = exp_0;
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
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x4 as libc::c_int as PlULong
    {
        return word;
    }
    if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        let mut deref_last_word_0: WamWord = 0;
        word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
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
            != (((256 as libc::c_int as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong)
        {
            word = Pl_Put_Structure(
                '/' as i32 as libc::c_uchar as libc::c_int,
                2 as libc::c_int,
            );
            Pl_Unify_Atom('.' as i32 as libc::c_uchar as libc::c_int);
            Pl_Unify_Integer(2 as libc::c_int as PlLong);
            Pl_Err_Type(pl_type_evaluable, word);
        }
        let mut deref_last_word_1: WamWord = 0;
        word = *lst_adr.offset(0 as libc::c_int as isize);
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
        if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
            Pl_Err_Instantiation();
        }
        if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_integer, word);
        }
        return word;
    }
    if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        arith = Pl_Hash_Find(arith_tbl, *adr.offset(0 as libc::c_int as isize))
            as *mut ArithInf;
        if arith.is_null() {
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
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int,
            );
            Pl_Unify_Integer(
                (*adr.offset(0 as libc::c_int as isize) as PlULong
                    >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as PlLong,
            );
            Pl_Err_Type(pl_type_evaluable, word);
        }
        if *adr.offset(0 as libc::c_int as isize) as PlULong
            >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
            == 1 as libc::c_int as libc::c_ulong
        {
            return ::std::mem::transmute::<
                _,
                fn(_) -> WamWord,
            >(
                (Some(((*arith).fct).unwrap())).unwrap(),
            )(
                Load_Math_Expression(
                    *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                ),
            );
        }
        return ::std::mem::transmute::<
            _,
            fn(_, _) -> WamWord,
        >(
            (Some(((*arith).fct).unwrap())).unwrap(),
        )(
            Load_Math_Expression(
                *adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
            ),
            Load_Math_Expression(
                *adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize),
            ),
        );
    }
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
        if atom == atom_pi {
            return Pl_Fct_PI();
        }
        if atom == atom_e {
            return Pl_Fct_E();
        }
        if atom == atom_epsilon {
            return Pl_Fct_Epsilon();
        }
        word = Pl_Put_Structure(
            '/' as i32 as libc::c_uchar as libc::c_int,
            2 as libc::c_int,
        );
        Pl_Unify_Value(exp_0);
        Pl_Unify_Integer(0 as libc::c_int as PlLong);
    }
    Pl_Err_Type(pl_type_evaluable, word);
    return word;
}
pub unsafe extern "C" fn Pl_Arith_Eval_2(
    mut exp_word: WamWord,
    mut x_word: WamWord,
) -> Bool {
    return Pl_Unify(Load_Math_Expression(exp_word), x_word);
}
pub unsafe extern "C" fn Pl_Succ_2(mut x_word: WamWord, mut y_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut x: PlLong = 0;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        return Pl_Un_Positive_Check(
            Pl_Rd_Positive_Check(word) + 1 as libc::c_int as libc::c_long,
            y_word,
        );
    }
    Pl_Check_For_Un_Positive(word);
    x = Pl_Rd_Positive_Check(y_word) - 1 as libc::c_int as libc::c_long;
    return (x >= 0 as libc::c_int as libc::c_long && Pl_Get_Integer(x, word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Neg(mut x: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    return ((-vx as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Inc(mut x: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    return (((vx + 1 as libc::c_int as libc::c_long) as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Dec(mut x: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    return (((vx - 1 as libc::c_int as libc::c_long) as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Add(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    return (((vx + vy) as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Sub(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    return (((vx - vy) as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Mul(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    return (((vx * vy) as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Div(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    if vy == 0 as libc::c_int as libc::c_long {
        Pl_Err_Evaluation(pl_evaluation_zero_divisor);
    }
    return (((vx / vy) as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Rem(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    if vy == 0 as libc::c_int as libc::c_long {
        Pl_Err_Evaluation(pl_evaluation_zero_divisor);
    }
    return (((vx % vy) as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Mod(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    let mut m: PlLong = 0;
    if vy == 0 as libc::c_int as libc::c_long {
        Pl_Err_Evaluation(pl_evaluation_zero_divisor);
    }
    m = vx % vy;
    if m != 0 as libc::c_int as libc::c_long && m ^ vy < 0 as libc::c_int as libc::c_long
    {
        m += vy;
    }
    return ((m as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Div2(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    let mut m: PlLong = 0;
    if vy == 0 as libc::c_int as libc::c_long {
        Pl_Err_Evaluation(pl_evaluation_zero_divisor);
    }
    m = vx % vy;
    if m != 0 as libc::c_int as libc::c_long && m ^ vy < 0 as libc::c_int as libc::c_long
    {
        m += vy;
    }
    m = (vx - m) / vy;
    return ((m as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_And(mut x: WamWord, mut y: WamWord) -> WamWord {
    return x & y;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Or(mut x: WamWord, mut y: WamWord) -> WamWord {
    return x | y;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Xor(mut x: WamWord, mut y: WamWord) -> WamWord {
    return ((x ^ y) as libc::c_ulong | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Not(mut x: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    return ((!vx as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Shl(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    return (((vx << vy) as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Shr(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    return (((vx >> vy) as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_LSB(mut x: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    return (((if vx == 0 as libc::c_int as libc::c_long {
        -(1 as libc::c_int)
    } else {
        (vx as libc::c_ulong).trailing_zeros() as i32
    }) as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_MSB(mut x: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    return (((if vx == 0 as libc::c_int as libc::c_long {
        -(1 as libc::c_int)
    } else {
        8 as libc::c_int * 8 as libc::c_int - 1 as libc::c_int
            - (vx as libc::c_ulong).leading_zeros() as i32
    }) as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Popcount(mut x: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    return (((vx as libc::c_ulong).count_ones() as i32 as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Abs(mut x: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    return (if vx < 0 as libc::c_int as libc::c_long {
        (-vx as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong
    } else {
        x as libc::c_ulong
    }) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Sign(mut x: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    return (if vx < 0 as libc::c_int as libc::c_long {
        (-(1 as libc::c_int) as PlULong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong
    } else if vx == 0 as libc::c_int as libc::c_long {
        (0 as libc::c_int as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong
    } else {
        (1 as libc::c_int as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong
    }) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_GCD(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    if vx < 0 as libc::c_int as libc::c_long {
        vx = -vx;
    }
    if vy < 0 as libc::c_int as libc::c_long {
        vy = -vy;
    }
    while vy != 0 as libc::c_int as libc::c_long {
        let mut r: PlLong = vx % vy;
        vx = vy;
        vy = r;
    }
    return ((vx as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Fast_Integer_Pow(
    mut x: WamWord,
    mut y: WamWord,
) -> WamWord {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    if vx != 1 as libc::c_int as libc::c_long
        && vy <= -(1 as libc::c_int) as libc::c_long
    {
        Pl_Err_Type(pl_type_float, x);
    }
    let mut r: libc::c_double = Integ_Pow(vx as libc::c_double, vy as libc::c_double);
    let mut p: PlLong = Double_To_PlLong(r);
    return ((p as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Neg(mut x: WamWord) -> WamWord {
    return if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Fct_Fast_Neg(x)
    } else {
        Make_Tagged_Float(-To_Double(x))
    };
}
pub unsafe extern "C" fn Pl_Fct_Inc(mut x: WamWord) -> WamWord {
    return if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Fct_Fast_Inc(x)
    } else {
        Make_Tagged_Float(To_Double(x) + 1 as libc::c_int as libc::c_double)
    };
}
pub unsafe extern "C" fn Pl_Fct_Dec(mut x: WamWord) -> WamWord {
    return if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Fct_Fast_Dec(x)
    } else {
        Make_Tagged_Float(To_Double(x) - 1 as libc::c_int as libc::c_double)
    };
}
pub unsafe extern "C" fn Pl_Fct_Add(mut x: WamWord, mut y: WamWord) -> WamWord {
    return if (x & y) as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Fct_Fast_Add(x, y)
    } else {
        Make_Tagged_Float(To_Double(x) + To_Double(y))
    };
}
pub unsafe extern "C" fn Pl_Fct_Sub(mut x: WamWord, mut y: WamWord) -> WamWord {
    return if (x & y) as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Fct_Fast_Sub(x, y)
    } else {
        Make_Tagged_Float(To_Double(x) - To_Double(y))
    };
}
pub unsafe extern "C" fn Pl_Fct_Mul(mut x: WamWord, mut y: WamWord) -> WamWord {
    return if (x & y) as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Fct_Fast_Mul(x, y)
    } else {
        Make_Tagged_Float(To_Double(x) * To_Double(y))
    };
}
pub unsafe extern "C" fn Pl_Fct_Div(mut x: WamWord, mut y: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    if y as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, y);
    }
    return Pl_Fct_Fast_Div(x, y);
}
pub unsafe extern "C" fn Pl_Fct_Float_Div(mut x: WamWord, mut y: WamWord) -> WamWord {
    return Make_Tagged_Float(
        if To_Double(y) != 0 as libc::c_int as libc::c_double {
            To_Double(x) / To_Double(y)
        } else {
            Pl_Err_Evaluation(pl_evaluation_zero_divisor);
            0 as libc::c_int as libc::c_double
        },
    );
}
pub unsafe extern "C" fn Pl_Fct_Rem(mut x: WamWord, mut y: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    if y as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, y);
    }
    return Pl_Fct_Fast_Rem(x, y);
}
pub unsafe extern "C" fn Pl_Fct_Mod(mut x: WamWord, mut y: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    if y as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, y);
    }
    return Pl_Fct_Fast_Mod(x, y);
}
pub unsafe extern "C" fn Pl_Fct_Div2(mut x: WamWord, mut y: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    if y as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, y);
    }
    return Pl_Fct_Fast_Div2(x, y);
}
pub unsafe extern "C" fn Pl_Fct_And(mut x: WamWord, mut y: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    if y as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, y);
    }
    return Pl_Fct_Fast_And(x, y);
}
pub unsafe extern "C" fn Pl_Fct_Or(mut x: WamWord, mut y: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    if y as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, y);
    }
    return Pl_Fct_Fast_Or(x, y);
}
pub unsafe extern "C" fn Pl_Fct_Xor(mut x: WamWord, mut y: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    if y as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, y);
    }
    return Pl_Fct_Fast_Xor(x, y);
}
pub unsafe extern "C" fn Pl_Fct_Not(mut x: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    return Pl_Fct_Fast_Not(x);
}
pub unsafe extern "C" fn Pl_Fct_Shl(mut x: WamWord, mut y: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    if y as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, y);
    }
    return Pl_Fct_Fast_Shl(x, y);
}
pub unsafe extern "C" fn Pl_Fct_Shr(mut x: WamWord, mut y: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    if y as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, y);
    }
    return Pl_Fct_Fast_Shr(x, y);
}
pub unsafe extern "C" fn Pl_Fct_LSB(mut x: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    return Pl_Fct_Fast_LSB(x);
}
pub unsafe extern "C" fn Pl_Fct_MSB(mut x: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    return Pl_Fct_Fast_MSB(x);
}
pub unsafe extern "C" fn Pl_Fct_Popcount(mut x: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    return Pl_Fct_Fast_Popcount(x);
}
pub unsafe extern "C" fn Pl_Fct_Abs(mut x: WamWord) -> WamWord {
    return if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Fct_Fast_Abs(x)
    } else {
        Make_Tagged_Float(fabs(To_Double(x)))
    };
}
pub unsafe extern "C" fn Pl_Fct_Sign(mut x: WamWord) -> WamWord {
    return if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Fct_Fast_Sign(x)
    } else {
        Make_Tagged_Float(
            if To_Double(x) < 0.0f64 {
                -1.0f64
            } else if To_Double(x) > 0.0f64 {
                1.0f64
            } else {
                0.0f64
            },
        )
    };
}
pub unsafe extern "C" fn Pl_Fct_GCD(mut x: WamWord, mut y: WamWord) -> WamWord {
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, x);
    }
    if y as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x4 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, y);
    }
    return Pl_Fct_Fast_GCD(x, y);
}
pub unsafe extern "C" fn Pl_Fct_Min(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut dx: libc::c_double = To_Double(x);
    let mut dy: libc::c_double = To_Double(y);
    if dx < dy {
        return x;
    }
    if dx > dy {
        return y;
    }
    return if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        x
    } else {
        y
    };
}
pub unsafe extern "C" fn Pl_Fct_Max(mut x: WamWord, mut y: WamWord) -> WamWord {
    let mut dx: libc::c_double = To_Double(x);
    let mut dy: libc::c_double = To_Double(y);
    if dx > dy {
        return x;
    }
    if dx < dy {
        return y;
    }
    return if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        x
    } else {
        y
    };
}
pub unsafe extern "C" fn Pl_Fct_Integer_Pow(mut x: WamWord, mut y: WamWord) -> WamWord {
    return if (x & y) as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Fct_Fast_Integer_Pow(x, y)
    } else {
        Make_Tagged_Float(Integ_Pow(To_Double(x), To_Double(y)))
    };
}
unsafe extern "C" fn Integ_Pow(
    mut x: libc::c_double,
    mut y: libc::c_double,
) -> libc::c_double {
    if x == 0.0f64 && y < 0 as libc::c_int as libc::c_double {
        return Pl_NaN();
    }
    if x == 1.0f64 {
        return x;
    }
    return pow(x, y);
}
pub unsafe extern "C" fn Pl_Fct_Pow(mut x: WamWord, mut y: WamWord) -> WamWord {
    return Make_Tagged_Float(pow(To_Double(x), To_Double(y)));
}
pub unsafe extern "C" fn Pl_Fct_Sqrt(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(sqrt(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Tan(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(tan(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Atan(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(atan(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Atan2(mut x: WamWord, mut y: WamWord) -> WamWord {
    return Make_Tagged_Float(atan2(To_Double(x), To_Double(y)));
}
pub unsafe extern "C" fn Pl_Fct_Cos(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(cos(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Acos(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(acos(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Sin(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(sin(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Asin(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(asin(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Tanh(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(tanh(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Atanh(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(atanh(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Cosh(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(cosh(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Acosh(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(acosh(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Sinh(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(sinh(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Asinh(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(asinh(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Exp(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(exp(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Log(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(log(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Log10(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(log10(To_Double(x)));
}
pub unsafe extern "C" fn Pl_Fct_Log_Radix(mut b: WamWord, mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(log(To_Double(x)) / log(To_Double(b)));
}
pub unsafe extern "C" fn Pl_Fct_Float(mut x: WamWord) -> WamWord {
    return Make_Tagged_Float(To_Double(x));
}
pub unsafe extern "C" fn Pl_Fct_Ceiling(mut x: WamWord) -> WamWord {
    let mut d: libc::c_double = 0.;
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        if (*pl_flag_strict_iso).value != 0 {
            Pl_Err_Type(pl_type_float, x);
        }
        return x;
    } else {
        d = Pl_Obtain_Float(
            (x as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
        );
    }
    return ((ceil(d) as PlLong as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Floor(mut x: WamWord) -> WamWord {
    let mut d: libc::c_double = 0.;
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        if (*pl_flag_strict_iso).value != 0 {
            Pl_Err_Type(pl_type_float, x);
        }
        return x;
    } else {
        d = Pl_Obtain_Float(
            (x as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
        );
    }
    return ((floor(d) as PlLong as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Round(mut x: WamWord) -> WamWord {
    let mut d: libc::c_double = 0.;
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        if (*pl_flag_strict_iso).value != 0 {
            Pl_Err_Type(pl_type_float, x);
        }
        return x;
    } else {
        d = Pl_Obtain_Float(
            (x as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
        );
    }
    return ((rint(d) as PlLong as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Truncate(mut x: WamWord) -> WamWord {
    let mut d: libc::c_double = 0.;
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        if (*pl_flag_strict_iso).value != 0 {
            Pl_Err_Type(pl_type_float, x);
        }
        return x;
    } else {
        d = Pl_Obtain_Float(
            (x as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
        );
    }
    return ((d as PlLong as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
}
pub unsafe extern "C" fn Pl_Fct_Float_Fract_Part(mut x: WamWord) -> WamWord {
    let mut d: libc::c_double = 0.;
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_float, x);
        return x;
    } else {
        d = Pl_Obtain_Float(
            (x as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
        );
    }
    return Make_Tagged_Float(
        d - (if d > 0 as libc::c_int as libc::c_double { floor(d) } else { ceil(d) }),
    );
}
pub unsafe extern "C" fn Pl_Fct_Float_Integ_Part(mut x: WamWord) -> WamWord {
    let mut d: libc::c_double = 0.;
    if x as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_float, x);
        return x;
    } else {
        d = Pl_Obtain_Float(
            (x as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord,
        );
    }
    return Make_Tagged_Float(
        if d > 0 as libc::c_int as libc::c_double { floor(d) } else { ceil(d) },
    );
}
pub unsafe extern "C" fn Pl_Fct_PI() -> WamWord {
    return Make_Tagged_Float(3.14159265358979323846f64);
}
pub unsafe extern "C" fn Pl_Fct_E() -> WamWord {
    return Make_Tagged_Float(2.7182818284590452354f64);
}
pub unsafe extern "C" fn Pl_Fct_Epsilon() -> WamWord {
    return Make_Tagged_Float(2.2204460492503131e-16f64);
}
pub unsafe extern "C" fn Pl_Fct_Identity(mut x: WamWord) -> WamWord {
    return x;
}
pub unsafe extern "C" fn Pl_Blt_Fast_Eq(mut x: WamWord, mut y: WamWord) -> Bool {
    return (x == y) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Fast_Neq(mut x: WamWord, mut y: WamWord) -> Bool {
    return (x != y) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Fast_Lt(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    return (vx < vy) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Fast_Lte(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    return (vx <= vy) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Fast_Gt(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    return (vx > vy) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Fast_Gte(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut vx: PlLong = x << 0 as libc::c_int >> 3 as libc::c_int;
    let mut vy: PlLong = y << 0 as libc::c_int >> 3 as libc::c_int;
    return (vx >= vy) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Blt_Eq(mut x: WamWord, mut y: WamWord) -> Bool {
    return if (x & y) as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Blt_Fast_Eq(x, y)
    } else {
        (To_Double(x) == To_Double(y)) as libc::c_int
    };
}
pub unsafe extern "C" fn Pl_Blt_Neq(mut x: WamWord, mut y: WamWord) -> Bool {
    return if (x & y) as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Blt_Fast_Neq(x, y)
    } else {
        (To_Double(x) != To_Double(y)) as libc::c_int
    };
}
pub unsafe extern "C" fn Pl_Blt_Lt(mut x: WamWord, mut y: WamWord) -> Bool {
    return if (x & y) as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Blt_Fast_Lt(x, y)
    } else {
        (To_Double(x) < To_Double(y)) as libc::c_int
    };
}
pub unsafe extern "C" fn Pl_Blt_Lte(mut x: WamWord, mut y: WamWord) -> Bool {
    return if (x & y) as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Blt_Fast_Lte(x, y)
    } else {
        (To_Double(x) <= To_Double(y)) as libc::c_int
    };
}
pub unsafe extern "C" fn Pl_Blt_Gt(mut x: WamWord, mut y: WamWord) -> Bool {
    return if (x & y) as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Blt_Fast_Gt(x, y)
    } else {
        (To_Double(x) > To_Double(y)) as libc::c_int
    };
}
pub unsafe extern "C" fn Pl_Blt_Gte(mut x: WamWord, mut y: WamWord) -> Bool {
    return if (x & y) as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x7 as libc::c_int as PlULong
    {
        Pl_Blt_Fast_Gte(x, y)
    } else {
        (To_Double(x) >= To_Double(y)) as libc::c_int
    };
}

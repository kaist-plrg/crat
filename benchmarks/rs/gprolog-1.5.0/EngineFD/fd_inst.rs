use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut pl_stk_tbl: [InfStack; 0];
    static mut pl_fd_variable_to_string: Option::<
        unsafe extern "C" fn() -> *mut libc::c_char,
    >;
    static mut pl_fd_copy_variable: Option::<unsafe extern "C" fn() -> libc::c_int>;
    static mut pl_fd_variable_size: Option::<unsafe extern "C" fn() -> libc::c_int>;
    static mut pl_fd_unify_with_fd_var: Option::<unsafe extern "C" fn() -> Bool>;
    static mut pl_fd_unify_with_integer: Option::<unsafe extern "C" fn() -> Bool>;
    fn Pl_Define_Vector_Size(max_val: libc::c_int);
    fn Pl_Vector_Next_After(vec: Vector, n: libc::c_int) -> libc::c_int;
    fn Pl_Vector_Next_Before(vec: Vector, n: libc::c_int) -> libc::c_int;
    fn Pl_Vector_Empty(vec: Vector);
    fn Pl_Range_Test_Value(range: *mut Range, n: libc::c_int) -> Bool;
    fn Pl_Range_Copy(range: *mut Range, range1: *mut Range);
    fn Pl_Range_Nb_Elem(range: *mut Range) -> libc::c_int;
    fn Pl_Range_Becomes_Sparse(range: *mut Range);
    fn Pl_Range_From_Vector(range: *mut Range);
    fn Pl_Range_Inter(range: *mut Range, range1: *mut Range);
    fn Pl_Range_To_String(range: *mut Range) -> *mut libc::c_char;
    static mut pl_stm_stdout: libc::c_int;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_type_integer: libc::c_int;
    static mut pl_type_list: libc::c_int;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    static mut pl_type_fd_variable: libc::c_int;
    fn Pl_Err_Instantiation();
    fn Pl_Stream_Printf(
        pstm: *mut StmInf,
        format: *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub extra_cstr: Bool,
    pub min: libc::c_int,
    pub max: libc::c_int,
    pub vec: Vector,
}
pub type Vector = *mut VecWord;
pub type VecWord = PlULong;
pub type StmInf = stm_inf;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PbStk {
    pub buff: [libc::c_int; 8],
    pub ptr: *mut libc::c_int,
    pub nb_elems: libc::c_int,
}
pub type StmFct = Option::<unsafe extern "C" fn() -> libc::c_int>;
pub type StmLst = stm_lst;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stm_lst {
    pub stm: libc::c_int,
    pub next: PStmLst,
}
pub type PStmLst = *mut stm_lst;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_vec_size: WamWord = 0;
pub static mut pl_vec_max_integer: WamWord = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
static mut TP: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut dummy_fd_var: [WamWord; 17] = [0; 17];
static mut DATE: PlULong = 0;
static mut optim2_date_never: PlULong = 0 as libc::c_int as PlULong;
static mut optim2_date_always: PlULong = 1 as libc::c_int as PlULong;
pub static mut pl_fd_init_solver: Option::<unsafe extern "C" fn() -> ()> = unsafe {
    ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        Option::<unsafe extern "C" fn() -> ()>,
    >(Some(Pl_Fd_Init_Solver0 as unsafe extern "C" fn() -> ()))
};
pub static mut pl_fd_reset_solver: Option::<unsafe extern "C" fn() -> ()> = unsafe {
    ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        Option::<unsafe extern "C" fn() -> ()>,
    >(Some(Pl_Fd_Reset_Solver0 as unsafe extern "C" fn() -> ()))
};
pub unsafe extern "C" fn Pl_Fd_Init_Solver0() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut max_val: libc::c_int = 0;
    p = getenv(b"VECTORMAX\0" as *const u8 as *const libc::c_char);
    if !p.is_null() && *p as libc::c_int != 0 {
        sscanf(
            p,
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut max_val as *mut libc::c_int,
        );
    } else {
        max_val = 127 as libc::c_int;
    }
    Pl_Define_Vector_Size(max_val);
    Pl_Fd_Reset_Solver0();
    pl_fd_unify_with_integer = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut WamWord, libc::c_int) -> Bool>,
        Option::<unsafe extern "C" fn() -> Bool>,
    >(
        Some(
            Pl_Fd_Unify_With_Integer0
                as unsafe extern "C" fn(*mut WamWord, libc::c_int) -> Bool,
        ),
    );
    pl_fd_unify_with_fd_var = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool>,
        Option::<unsafe extern "C" fn() -> Bool>,
    >(
        Some(
            Pl_Fd_Unify_With_Fd_Var0
                as unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> Bool,
        ),
    );
    pl_fd_variable_size = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut WamWord) -> libc::c_int>,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(Some(Pl_Fd_Variable_Size0 as unsafe extern "C" fn(*mut WamWord) -> libc::c_int));
    pl_fd_copy_variable = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> libc::c_int>,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            Pl_Fd_Copy_Variable0
                as unsafe extern "C" fn(*mut WamWord, *mut WamWord) -> libc::c_int,
        ),
    );
    pl_fd_variable_to_string = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut WamWord) -> *mut libc::c_char>,
        Option::<unsafe extern "C" fn() -> *mut libc::c_char>,
    >(
        Some(
            Pl_Fd_Variable_To_String0
                as unsafe extern "C" fn(*mut WamWord) -> *mut libc::c_char,
        ),
    );
}
pub unsafe extern "C" fn Pl_Fd_Reset_Solver0() {
    *pl_reg_bank
        .offset(
            (256 as libc::c_int + 5 as libc::c_int) as isize,
        ) = 0 as libc::c_int as WamWord;
    DATE = 1 as libc::c_int as PlULong;
    TP = dummy_fd_var.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_Fd_Prolog_To_Fd_Var(
    mut arg_word: WamWord,
    mut pl_var_ok: Bool,
) -> *mut WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut deref_last_word: WamWord = 0;
    word = arg_word;
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
        if pl_var_ok == 0 {
            Pl_Err_Instantiation();
        }
        adr = word as *mut WamWord;
        fdv_adr = Pl_Fd_New_Variable();
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
        return fdv_adr;
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        return Pl_Fd_New_Int_Variable(
            (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int,
        );
    }
    if tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    return (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong) as *mut WamWord;
}
pub unsafe extern "C" fn Pl_Fd_Prolog_To_Range(mut list_word: WamWord) -> *mut Range {
    let mut range: *mut Range = 0 as *mut Range;
    range = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as *mut Range;
    let ref mut fresh1 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh1 = (*fresh1)
        .offset(
            (::std::mem::size_of::<Range>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong) as isize,
        );
    (*range).vec = 0 as Vector;
    Pl_Fd_List_Int_To_Range(range, list_word);
    return range;
}
pub unsafe extern "C" fn Pl_Fd_Prolog_To_Value(mut arg_word: WamWord) -> libc::c_int {
    let mut v: PlLong = Pl_Rd_Integer_Check(arg_word);
    if v
        < -((((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int) as libc::c_long
    {
        v = -((((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int) as PlLong;
    }
    if v
        > (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int as libc::c_long
    {
        v = (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int as PlLong;
    }
    return v as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_List_Int_To_Range(
    mut range: *mut Range,
    mut list_word: WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut val: WamWord = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    save_list_word = list_word;
    (*range).extra_cstr = 0 as libc::c_int;
    if ((*range).vec).is_null() {
        (*range)
            .vec = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 3 as libc::c_int) as isize) as Vector;
        let ref mut fresh2 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
        *fresh2 = (*fresh2).offset(pl_vec_size as isize);
    }
    Pl_Vector_Empty((*range).vec);
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
        val = Pl_Fd_Prolog_To_Value(*lst_adr.offset(0 as libc::c_int as isize))
            as WamWord;
        if val as libc::c_uint > pl_vec_max_integer as libc::c_uint {
            (*range).extra_cstr = 1 as libc::c_int;
        } else {
            let ref mut fresh3 = *((*range).vec)
                .offset((val as VecWord >> 6 as libc::c_int) as isize);
            *fresh3
                |= (1 as libc::c_int as VecWord)
                    << (val as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong));
            n += 1;
            n;
        }
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    if n == 0 as libc::c_int {
        (*range)
            .max = (1 as libc::c_int)
            << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    } else {
        Pl_Range_From_Vector(range);
    };
}
pub unsafe extern "C" fn Pl_Fd_Prolog_To_Array_Int(
    mut list_word: WamWord,
) -> *mut WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut val: WamWord = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
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
        if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong {
            Pl_Err_Type(pl_type_integer, word);
        }
        val = word << 0 as libc::c_int >> 3 as libc::c_int;
        let fresh4 = array;
        array = array.offset(1);
        *fresh4 = val;
        n += 1;
        n;
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    *save_array = n as WamWord;
    let ref mut fresh5 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh5 = array;
    return save_array;
}
pub unsafe extern "C" fn Pl_Fd_Prolog_To_Array_Any(
    mut list_word: WamWord,
) -> *mut WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0 as libc::c_int;
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
        let fresh6 = array;
        array = array.offset(1);
        *fresh6 = *lst_adr.offset(0 as libc::c_int as isize);
        n += 1;
        n;
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    *save_array = n as WamWord;
    let ref mut fresh7 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh7 = array;
    return save_array;
}
pub unsafe extern "C" fn Pl_Fd_Prolog_To_Array_Fdv(
    mut list_word: WamWord,
    mut pl_var_ok: Bool,
) -> *mut WamWord {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut save_list_word: WamWord = 0;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut save_array: *mut WamWord = 0 as *mut WamWord;
    let mut array: *mut WamWord = 0 as *mut WamWord;
    save_list_word = list_word;
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
        if tag_mask as libc::c_ulong != 0x1 as libc::c_int as PlULong {
            break;
        }
        lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        n += 1;
        n;
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    array = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh8 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh8 = (*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize))
        .offset(n as isize)
        .offset(1 as libc::c_int as isize);
    list_word = save_list_word;
    save_array = array;
    array = array.offset(1);
    array;
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
        let fresh9 = array;
        array = array.offset(1);
        *fresh9 = Pl_Fd_Prolog_To_Fd_Var(
            *lst_adr.offset(0 as libc::c_int as isize),
            pl_var_ok,
        ) as WamWord;
        list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
    }
    *save_array = n as WamWord;
    return save_array;
}
pub unsafe extern "C" fn Pl_Fd_Create_C_Frame(
    mut cstr_fct: Option::<unsafe extern "C" fn() -> PlLong>,
    mut AF: *mut WamWord,
    mut fdv_adr: *mut WamWord,
    mut optim2: Bool,
) -> *mut WamWord {
    let mut CF: *mut WamWord = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh10 = *(&mut *CF.offset(0 as libc::c_int as isize) as *mut WamWord
        as *mut *mut WamWord);
    *fresh10 = AF;
    let ref mut fresh11 = *(&mut *CF.offset(1 as libc::c_int as isize) as *mut WamWord
        as *mut *mut PlULong);
    *fresh11 = if optim2 != 0 && !fdv_adr.is_null() {
        &mut *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize) as *mut PlULong
    } else {
        &mut optim2_date_always
    };
    let ref mut fresh12 = *(&mut *CF.offset(2 as libc::c_int as isize) as *mut WamWord
        as *mut Option::<unsafe extern "C" fn() -> PlLong>);
    *fresh12 = cstr_fct;
    if !fdv_adr.is_null()
        && !(*fdv_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
            & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong)
    {
        let ref mut fresh13 = *fdv_adr
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
            );
        *fresh13 += 1;
        *fresh13;
    }
    let ref mut fresh14 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh14 = (*fresh14).offset(3 as libc::c_int as isize);
    return CF;
}
pub unsafe extern "C" fn Pl_Fd_Add_Dependency(
    mut fdv_adr: *mut WamWord,
    mut chain_nb: libc::c_int,
    mut CF: *mut WamWord,
) {
    let mut chain_adr: *mut *mut WamWord = 0 as *mut *mut WamWord;
    if *fdv_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
        & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong
    {
        return;
    }
    if *fdv_adr
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
                ) as isize,
        ) != *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize)
    {
        let mut s: *mut PlLong = fdv_adr
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
                    ) as isize,
            ) as *mut PlLong;
        let mut d: *mut PlLong = TR as *mut PlLong;
        let mut counter: libc::c_int = 8 as libc::c_int;
        loop {
            let fresh15 = s;
            s = s.offset(1);
            let fresh16 = d;
            d = d.offset(1);
            *fresh16 = *fresh15;
            counter -= 1;
            if !(counter != 0) {
                break;
            }
        }
        TR = TR.offset(8 as libc::c_int as isize);
        let fresh17 = TR;
        TR = TR.offset(1);
        *fresh17 = 8 as libc::c_int as WamWord;
        let fresh18 = TR;
        TR = TR.offset(1);
        *fresh18 = (fdv_adr
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
                    ) as isize,
            ) as PlULong | 2 as libc::c_int as libc::c_ulong) as WamWord;
        *fdv_adr
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
                    ) as isize,
            ) = *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    }
    let ref mut fresh19 = *fdv_adr
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
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
        );
    *fresh19 |= ((1 as libc::c_int) << chain_nb) as libc::c_long;
    chain_adr = (&mut *(fdv_adr as *mut *mut WamWord)
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
                .wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut *mut WamWord)
        .offset(chain_nb as isize);
    let ref mut fresh20 = *(&mut *(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize))
        .offset(0 as libc::c_int as isize) as *mut WamWord as *mut *mut WamWord);
    *fresh20 = CF;
    let ref mut fresh21 = *(&mut *(*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize))
        .offset(1 as libc::c_int as isize) as *mut WamWord as *mut *mut WamWord);
    *fresh21 = *chain_adr;
    *chain_adr = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh22 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh22 = (*fresh22).offset(2 as libc::c_int as isize);
}
pub unsafe extern "C" fn Pl_Fd_Add_List_Dependency(
    mut array: *mut WamWord,
    mut chain_nb: libc::c_int,
    mut CF: *mut WamWord,
) {
    let fresh23 = array;
    array = array.offset(1);
    let mut n: libc::c_int = *fresh23 as libc::c_int;
    loop {
        let fresh24 = n;
        n = n - 1;
        if !(fresh24 != 0) {
            break;
        }
        let fresh25 = array;
        array = array.offset(1);
        Pl_Fd_Add_Dependency(*fresh25 as *mut WamWord, chain_nb, CF);
    };
}
pub unsafe extern "C" fn Pl_Fd_New_Variable_Interval(
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> *mut WamWord {
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    fdv_adr = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fdv_adr
        .offset(
            0 as libc::c_int as isize,
        ) = (fdv_adr as PlLong as libc::c_ulong)
        .wrapping_add(0x5 as libc::c_int as PlULong) as WamWord;
    *(fdv_adr as *mut PlULong)
        .offset(1 as libc::c_int as isize) = 1 as libc::c_int as PlULong;
    *fdv_adr.offset(2 as libc::c_int as isize) = 0 as libc::c_int as WamWord;
    let ref mut fresh26 = *(fdv_adr as *mut *mut WamWord)
        .offset(3 as libc::c_int as isize);
    *fresh26 = 0 as *mut WamWord;
    *fdv_adr
        .offset(
            4 as libc::c_int as isize,
        ) = *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fdv_adr
        .offset(
            (4 as libc::c_int + 1 as libc::c_int) as isize,
        ) = (max - min + 1 as libc::c_int) as WamWord;
    (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .extra_cstr = 0 as libc::c_int;
    (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .min = min;
    (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .max = max;
    let ref mut fresh27 = (*(fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .vec;
    *fresh27 = 0 as Vector;
    *fdv_adr
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
                ) as isize,
        ) = *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fdv_adr
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
        ) = 0 as libc::c_int as WamWord;
    *fdv_adr
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
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
        ) = 0 as libc::c_int as WamWord;
    let ref mut fresh28 = *(fdv_adr as *mut *mut WamWord)
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
                .wrapping_add(5 as libc::c_int as libc::c_ulong) as isize,
        );
    *fresh28 = 0 as *mut WamWord;
    let ref mut fresh29 = *(fdv_adr as *mut *mut WamWord)
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
                .wrapping_add(4 as libc::c_int as libc::c_ulong) as isize,
        );
    *fresh29 = *fresh28;
    let ref mut fresh30 = *(fdv_adr as *mut *mut WamWord)
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
                .wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
        );
    *fresh30 = *fresh29;
    let ref mut fresh31 = *(fdv_adr as *mut *mut WamWord)
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
                .wrapping_add(7 as libc::c_int as libc::c_ulong) as isize,
        );
    *fresh31 = 0 as *mut WamWord;
    let ref mut fresh32 = *(fdv_adr as *mut *mut WamWord)
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
                .wrapping_add(6 as libc::c_int as libc::c_ulong) as isize,
        );
    *fresh32 = *fresh31;
    let ref mut fresh33 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh33 = (*fresh33)
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
                .wrapping_add(8 as libc::c_int as libc::c_ulong) as isize,
        );
    return fdv_adr;
}
pub unsafe extern "C" fn Pl_Fd_New_Variable() -> *mut WamWord {
    return Pl_Fd_New_Variable_Interval(
        0 as libc::c_int,
        (((1 as libc::c_int as PlLong)
            << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int as libc::c_long) as libc::c_int,
    );
}
pub unsafe extern "C" fn Pl_Fd_New_Variable_Range(mut r: *mut Range) -> *mut WamWord {
    let mut fdv_adr: *mut WamWord = Pl_Fd_New_Variable();
    Pl_Range_Copy(
        fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range,
        r,
    );
    *fdv_adr
        .offset(
            (4 as libc::c_int + 1 as libc::c_int) as isize,
        ) = Pl_Range_Nb_Elem(r) as WamWord;
    return fdv_adr;
}
pub unsafe extern "C" fn Pl_Fd_New_Int_Variable(mut n: libc::c_int) -> *mut WamWord {
    let mut fdv_adr: *mut WamWord = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fdv_adr
        .offset(
            0 as libc::c_int as isize,
        ) = ((n as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
    *(fdv_adr as *mut PlULong)
        .offset(1 as libc::c_int as isize) = 0 as libc::c_int as PlULong;
    *fdv_adr.offset(2 as libc::c_int as isize) = 0 as libc::c_int as WamWord;
    let ref mut fresh34 = *(fdv_adr as *mut *mut WamWord)
        .offset(3 as libc::c_int as isize);
    *fresh34 = 0 as *mut WamWord;
    *fdv_adr
        .offset(
            4 as libc::c_int as isize,
        ) = *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize);
    *fdv_adr
        .offset(
            (4 as libc::c_int + 1 as libc::c_int) as isize,
        ) = 1 as libc::c_int as WamWord;
    (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .extra_cstr = 0 as libc::c_int;
    (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .min = n;
    (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .max = n;
    let ref mut fresh35 = (*(fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .vec;
    *fresh35 = 0 as Vector;
    let ref mut fresh36 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh36 = (*fresh36)
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
                ) as isize,
        );
    return fdv_adr;
}
pub unsafe extern "C" fn Pl_Fd_Before_Add_Cstr() {
    DATE = DATE.wrapping_add(1);
    DATE;
    if DATE == 0 as libc::c_int as libc::c_ulong {
        DATE = DATE.wrapping_add(1);
        DATE;
    }
    TP = dummy_fd_var.as_mut_ptr();
}
unsafe extern "C" fn Clear_Queue() {
    let mut BP: *mut WamWord = 0 as *mut WamWord;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    if TP == dummy_fd_var.as_mut_ptr() {
        return;
    }
    BP = *(dummy_fd_var.as_mut_ptr() as *mut *mut WamWord)
        .offset(3 as libc::c_int as isize);
    loop {
        fdv_adr = BP;
        *fdv_adr.offset(2 as libc::c_int as isize) = 0 as libc::c_int as WamWord;
        if BP == TP {
            break;
        }
        BP = *(BP as *mut *mut WamWord).offset(3 as libc::c_int as isize);
    }
    TP = dummy_fd_var.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_Fd_Tell_Value(
    mut fdv_adr: *mut WamWord,
    mut n: libc::c_int,
) -> Bool {
    let mut propag: libc::c_int = 0;
    if Pl_Range_Test_Value(
        fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range,
        n,
    ) == 0
    {
        if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .extra_cstr != 0 && n as libc::c_long > pl_vec_max_integer
        {
            Pl_Fd_Display_Extra_Cstr(fdv_adr);
        }
        return 0 as libc::c_int;
    }
    if *fdv_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
        & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong
    {
        return 1 as libc::c_int;
    }
    propag = 0 as libc::c_int;
    if (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord)
        < *(&mut *B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord)
    {
        let fresh37 = TR;
        TR = TR.offset(1);
        *fresh37 = *fdv_adr.offset(0 as libc::c_int as isize);
        let fresh38 = TR;
        TR = TR.offset(1);
        *fresh38 = (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord
            as PlULong | 1 as libc::c_int as libc::c_ulong) as WamWord;
        let fresh39 = TR;
        TR = TR.offset(1);
        *fresh39 = *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize)
            as WamWord;
        let fresh40 = TR;
        TR = TR.offset(1);
        *fresh40 = (&mut *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize)
            as *mut PlULong as PlULong | 1 as libc::c_int as libc::c_ulong) as WamWord;
        if *fdv_adr.offset(4 as libc::c_int as isize)
            != *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize)
        {
            let mut s: *mut PlLong = fdv_adr.offset(4 as libc::c_int as isize)
                as *mut PlLong;
            let mut d: *mut PlLong = TR as *mut PlLong;
            let mut counter: libc::c_int = (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong),
                ) as libc::c_int;
            loop {
                let fresh41 = s;
                s = s.offset(1);
                let fresh42 = d;
                d = d.offset(1);
                *fresh42 = *fresh41;
                counter -= 1;
                if !(counter != 0) {
                    break;
                }
            }
            TR = TR
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (::std::mem::size_of::<Range>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                ),
                        ) as isize,
                );
            let fresh43 = TR;
            TR = TR.offset(1);
            *fresh43 = (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong),
                ) as WamWord;
            let fresh44 = TR;
            TR = TR.offset(1);
            *fresh44 = (fdv_adr.offset(4 as libc::c_int as isize) as PlULong
                | 2 as libc::c_int as libc::c_ulong) as WamWord;
            if !((*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .vec)
                .is_null()
            {
                let mut s_0: *mut PlLong = (*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec as *mut WamWord as *mut PlLong;
                let mut d_0: *mut PlLong = TR as *mut PlLong;
                let mut counter_0: libc::c_int = pl_vec_size as libc::c_int;
                loop {
                    let fresh45 = s_0;
                    s_0 = s_0.offset(1);
                    let fresh46 = d_0;
                    d_0 = d_0.offset(1);
                    *fresh46 = *fresh45;
                    counter_0 -= 1;
                    if !(counter_0 != 0) {
                        break;
                    }
                }
                TR = TR.offset(pl_vec_size as isize);
                let fresh47 = TR;
                TR = TR.offset(1);
                *fresh47 = pl_vec_size;
                let fresh48 = TR;
                TR = TR.offset(1);
                *fresh48 = ((*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec as *mut WamWord as PlULong | 2 as libc::c_int as libc::c_ulong)
                    as WamWord;
            }
            *fdv_adr
                .offset(
                    4 as libc::c_int as isize,
                ) = *pl_reg_bank
                .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
        }
    }
    *fdv_adr
        .offset(
            (4 as libc::c_int + 1 as libc::c_int) as isize,
        ) = 1 as libc::c_int as WamWord;
    propag |= 4 as libc::c_int;
    propag |= 8 as libc::c_int;
    propag |= 16 as libc::c_int;
    if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .min != n
    {
        (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .min = n;
        propag |= 1 as libc::c_int;
    }
    if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .max != n
    {
        (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .max = n;
        propag |= 2 as libc::c_int;
    }
    let ref mut fresh49 = (*(fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .vec;
    *fresh49 = 0 as Vector;
    *fdv_adr
        .offset(
            0 as libc::c_int as isize,
        ) = ((n as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
    *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize) = DATE;
    All_Propagations(fdv_adr, propag);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Tell_Not_Value(
    mut fdv_adr: *mut WamWord,
    mut n: libc::c_int,
) -> Bool {
    let mut r: *mut Range = 0 as *mut Range;
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut propag: libc::c_int = 0;
    loop {
        r = fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range;
        if Pl_Range_Test_Value(r, n) == 0 {
            return 1 as libc::c_int;
        }
        if *fdv_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
            & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong
        {
            if (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .extra_cstr != 0
            {
                Pl_Fd_Display_Extra_Cstr(fdv_adr);
            }
            return 0 as libc::c_int;
        }
        min = (*r).min;
        max = (*r).max;
        if !(((*r).vec).is_null() && n != min && n != max) {
            break;
        }
        if min as libc::c_long > pl_vec_max_integer {
            Pl_Fd_Display_Extra_Cstr(fdv_adr);
            return 0 as libc::c_int;
        }
        if min as libc::c_long == pl_vec_max_integer {
            Pl_Fd_Display_Extra_Cstr(fdv_adr);
            propag = 0 as libc::c_int;
            if (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord)
                < *(&mut *B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
                    as *mut *mut WamWord)
            {
                let fresh50 = TR;
                TR = TR.offset(1);
                *fresh50 = *fdv_adr.offset(0 as libc::c_int as isize);
                let fresh51 = TR;
                TR = TR.offset(1);
                *fresh51 = (&mut *fdv_adr.offset(0 as libc::c_int as isize)
                    as *mut WamWord as PlULong | 1 as libc::c_int as libc::c_ulong)
                    as WamWord;
                let fresh52 = TR;
                TR = TR.offset(1);
                *fresh52 = *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize)
                    as WamWord;
                let fresh53 = TR;
                TR = TR.offset(1);
                *fresh53 = (&mut *(fdv_adr as *mut PlULong)
                    .offset(1 as libc::c_int as isize) as *mut PlULong as PlULong
                    | 1 as libc::c_int as libc::c_ulong) as WamWord;
                if *fdv_adr.offset(4 as libc::c_int as isize)
                    != *pl_reg_bank
                        .offset((256 as libc::c_int + 5 as libc::c_int) as isize)
                {
                    let mut s: *mut PlLong = fdv_adr.offset(4 as libc::c_int as isize)
                        as *mut PlLong;
                    let mut d: *mut PlLong = TR as *mut PlLong;
                    let mut counter: libc::c_int = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (::std::mem::size_of::<Range>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                ),
                        ) as libc::c_int;
                    loop {
                        let fresh54 = s;
                        s = s.offset(1);
                        let fresh55 = d;
                        d = d.offset(1);
                        *fresh55 = *fresh54;
                        counter -= 1;
                        if !(counter != 0) {
                            break;
                        }
                    }
                    TR = TR
                        .offset(
                            (2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                                        .wrapping_div(
                                            ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                        ),
                                ) as isize,
                        );
                    let fresh56 = TR;
                    TR = TR.offset(1);
                    *fresh56 = (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (::std::mem::size_of::<Range>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                ),
                        ) as WamWord;
                    let fresh57 = TR;
                    TR = TR.offset(1);
                    *fresh57 = (fdv_adr.offset(4 as libc::c_int as isize) as PlULong
                        | 2 as libc::c_int as libc::c_ulong) as WamWord;
                    if !((*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec)
                        .is_null()
                    {
                        let mut s_0: *mut PlLong = (*(fdv_adr
                            .offset(4 as libc::c_int as isize)
                            .offset(2 as libc::c_int as isize) as *mut Range))
                            .vec as *mut WamWord as *mut PlLong;
                        let mut d_0: *mut PlLong = TR as *mut PlLong;
                        let mut counter_0: libc::c_int = pl_vec_size as libc::c_int;
                        loop {
                            let fresh58 = s_0;
                            s_0 = s_0.offset(1);
                            let fresh59 = d_0;
                            d_0 = d_0.offset(1);
                            *fresh59 = *fresh58;
                            counter_0 -= 1;
                            if !(counter_0 != 0) {
                                break;
                            }
                        }
                        TR = TR.offset(pl_vec_size as isize);
                        let fresh60 = TR;
                        TR = TR.offset(1);
                        *fresh60 = pl_vec_size;
                        let fresh61 = TR;
                        TR = TR.offset(1);
                        *fresh61 = ((*(fdv_adr
                            .offset(4 as libc::c_int as isize)
                            .offset(2 as libc::c_int as isize) as *mut Range))
                            .vec as *mut WamWord as PlULong
                            | 2 as libc::c_int as libc::c_ulong) as WamWord;
                    }
                    *fdv_adr
                        .offset(
                            4 as libc::c_int as isize,
                        ) = *pl_reg_bank
                        .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
                }
            }
            *fdv_adr
                .offset(
                    (4 as libc::c_int + 1 as libc::c_int) as isize,
                ) = 1 as libc::c_int as WamWord;
            propag |= 4 as libc::c_int;
            propag |= 8 as libc::c_int;
            propag |= 16 as libc::c_int;
            if (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .min != min
            {
                (*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .min = min;
                propag |= 1 as libc::c_int;
            }
            if (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .max != min
            {
                (*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .max = min;
                propag |= 2 as libc::c_int;
            }
            let ref mut fresh62 = (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .vec;
            *fresh62 = 0 as Vector;
            *fdv_adr
                .offset(
                    0 as libc::c_int as isize,
                ) = ((min as PlULong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong) as WamWord;
            *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize) = DATE;
            All_Propagations(fdv_adr, propag);
            return 1 as libc::c_int;
        }
        if *fdv_adr.offset(4 as libc::c_int as isize)
            != *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize)
        {
            let mut s_1: *mut PlLong = fdv_adr.offset(4 as libc::c_int as isize)
                as *mut PlLong;
            let mut d_1: *mut PlLong = TR as *mut PlLong;
            let mut counter_1: libc::c_int = (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong),
                ) as libc::c_int;
            loop {
                let fresh63 = s_1;
                s_1 = s_1.offset(1);
                let fresh64 = d_1;
                d_1 = d_1.offset(1);
                *fresh64 = *fresh63;
                counter_1 -= 1;
                if !(counter_1 != 0) {
                    break;
                }
            }
            TR = TR
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (::std::mem::size_of::<Range>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                ),
                        ) as isize,
                );
            let fresh65 = TR;
            TR = TR.offset(1);
            *fresh65 = (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong),
                ) as WamWord;
            let fresh66 = TR;
            TR = TR.offset(1);
            *fresh66 = (fdv_adr.offset(4 as libc::c_int as isize) as PlULong
                | 2 as libc::c_int as libc::c_ulong) as WamWord;
            if !((*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .vec)
                .is_null()
            {
                let mut s_2: *mut PlLong = (*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec as *mut WamWord as *mut PlLong;
                let mut d_2: *mut PlLong = TR as *mut PlLong;
                let mut counter_2: libc::c_int = pl_vec_size as libc::c_int;
                loop {
                    let fresh67 = s_2;
                    s_2 = s_2.offset(1);
                    let fresh68 = d_2;
                    d_2 = d_2.offset(1);
                    *fresh68 = *fresh67;
                    counter_2 -= 1;
                    if !(counter_2 != 0) {
                        break;
                    }
                }
                TR = TR.offset(pl_vec_size as isize);
                let fresh69 = TR;
                TR = TR.offset(1);
                *fresh69 = pl_vec_size;
                let fresh70 = TR;
                TR = TR.offset(1);
                *fresh70 = ((*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec as *mut WamWord as PlULong | 2 as libc::c_int as libc::c_ulong)
                    as WamWord;
            }
            *fdv_adr
                .offset(
                    4 as libc::c_int as isize,
                ) = *pl_reg_bank
                .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
        }
        Pl_Range_Becomes_Sparse(r);
        *fdv_adr
            .offset(
                (4 as libc::c_int + 1 as libc::c_int) as isize,
            ) = ((*r).max - (*r).min + 1 as libc::c_int) as WamWord;
        if (*r).extra_cstr != 0 {
            propag = 0 as libc::c_int;
            propag |= 2 as libc::c_int;
            propag |= 4 as libc::c_int;
            propag |= 8 as libc::c_int;
            All_Propagations(fdv_adr, propag);
        }
    }
    if *fdv_adr.offset((4 as libc::c_int + 1 as libc::c_int) as isize)
        == 2 as libc::c_int as libc::c_long
    {
        if n == min {
            min = max;
        }
        propag = 0 as libc::c_int;
        if (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord)
            < *(&mut *B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord)
        {
            let fresh71 = TR;
            TR = TR.offset(1);
            *fresh71 = *fdv_adr.offset(0 as libc::c_int as isize);
            let fresh72 = TR;
            TR = TR.offset(1);
            *fresh72 = (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord
                as PlULong | 1 as libc::c_int as libc::c_ulong) as WamWord;
            let fresh73 = TR;
            TR = TR.offset(1);
            *fresh73 = *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize)
                as WamWord;
            let fresh74 = TR;
            TR = TR.offset(1);
            *fresh74 = (&mut *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize)
                as *mut PlULong as PlULong | 1 as libc::c_int as libc::c_ulong)
                as WamWord;
            if *fdv_adr.offset(4 as libc::c_int as isize)
                != *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize)
            {
                let mut s_3: *mut PlLong = fdv_adr.offset(4 as libc::c_int as isize)
                    as *mut PlLong;
                let mut d_3: *mut PlLong = TR as *mut PlLong;
                let mut counter_3: libc::c_int = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                            ),
                    ) as libc::c_int;
                loop {
                    let fresh75 = s_3;
                    s_3 = s_3.offset(1);
                    let fresh76 = d_3;
                    d_3 = d_3.offset(1);
                    *fresh76 = *fresh75;
                    counter_3 -= 1;
                    if !(counter_3 != 0) {
                        break;
                    }
                }
                TR = TR
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (::std::mem::size_of::<Range>() as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                let fresh77 = TR;
                TR = TR.offset(1);
                *fresh77 = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                            ),
                    ) as WamWord;
                let fresh78 = TR;
                TR = TR.offset(1);
                *fresh78 = (fdv_adr.offset(4 as libc::c_int as isize) as PlULong
                    | 2 as libc::c_int as libc::c_ulong) as WamWord;
                if !((*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec)
                    .is_null()
                {
                    let mut s_4: *mut PlLong = (*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec as *mut WamWord as *mut PlLong;
                    let mut d_4: *mut PlLong = TR as *mut PlLong;
                    let mut counter_4: libc::c_int = pl_vec_size as libc::c_int;
                    loop {
                        let fresh79 = s_4;
                        s_4 = s_4.offset(1);
                        let fresh80 = d_4;
                        d_4 = d_4.offset(1);
                        *fresh80 = *fresh79;
                        counter_4 -= 1;
                        if !(counter_4 != 0) {
                            break;
                        }
                    }
                    TR = TR.offset(pl_vec_size as isize);
                    let fresh81 = TR;
                    TR = TR.offset(1);
                    *fresh81 = pl_vec_size;
                    let fresh82 = TR;
                    TR = TR.offset(1);
                    *fresh82 = ((*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec as *mut WamWord as PlULong
                        | 2 as libc::c_int as libc::c_ulong) as WamWord;
                }
                *fdv_adr
                    .offset(
                        4 as libc::c_int as isize,
                    ) = *pl_reg_bank
                    .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
            }
        }
        *fdv_adr
            .offset(
                (4 as libc::c_int + 1 as libc::c_int) as isize,
            ) = 1 as libc::c_int as WamWord;
        propag |= 4 as libc::c_int;
        propag |= 8 as libc::c_int;
        propag |= 16 as libc::c_int;
        if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .min != min
        {
            (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .min = min;
            propag |= 1 as libc::c_int;
        }
        if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .max != min
        {
            (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .max = min;
            propag |= 2 as libc::c_int;
        }
        let ref mut fresh83 = (*(fdv_adr
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *mut Range))
            .vec;
        *fresh83 = 0 as Vector;
        *fdv_adr
            .offset(
                0 as libc::c_int as isize,
            ) = ((min as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
            as WamWord;
        *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize) = DATE;
    } else {
        if *fdv_adr.offset(4 as libc::c_int as isize)
            != *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize)
        {
            let mut s_5: *mut PlLong = fdv_adr.offset(4 as libc::c_int as isize)
                as *mut PlLong;
            let mut d_5: *mut PlLong = TR as *mut PlLong;
            let mut counter_5: libc::c_int = (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong),
                ) as libc::c_int;
            loop {
                let fresh84 = s_5;
                s_5 = s_5.offset(1);
                let fresh85 = d_5;
                d_5 = d_5.offset(1);
                *fresh85 = *fresh84;
                counter_5 -= 1;
                if !(counter_5 != 0) {
                    break;
                }
            }
            TR = TR
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (::std::mem::size_of::<Range>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                ),
                        ) as isize,
                );
            let fresh86 = TR;
            TR = TR.offset(1);
            *fresh86 = (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong),
                ) as WamWord;
            let fresh87 = TR;
            TR = TR.offset(1);
            *fresh87 = (fdv_adr.offset(4 as libc::c_int as isize) as PlULong
                | 2 as libc::c_int as libc::c_ulong) as WamWord;
            if !((*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .vec)
                .is_null()
            {
                let mut s_6: *mut PlLong = (*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec as *mut WamWord as *mut PlLong;
                let mut d_6: *mut PlLong = TR as *mut PlLong;
                let mut counter_6: libc::c_int = pl_vec_size as libc::c_int;
                loop {
                    let fresh88 = s_6;
                    s_6 = s_6.offset(1);
                    let fresh89 = d_6;
                    d_6 = d_6.offset(1);
                    *fresh89 = *fresh88;
                    counter_6 -= 1;
                    if !(counter_6 != 0) {
                        break;
                    }
                }
                TR = TR.offset(pl_vec_size as isize);
                let fresh90 = TR;
                TR = TR.offset(1);
                *fresh90 = pl_vec_size;
                let fresh91 = TR;
                TR = TR.offset(1);
                *fresh91 = ((*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec as *mut WamWord as PlULong | 2 as libc::c_int as libc::c_ulong)
                    as WamWord;
            }
            *fdv_adr
                .offset(
                    4 as libc::c_int as isize,
                ) = *pl_reg_bank
                .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
        }
        propag = 0 as libc::c_int;
        propag |= 8 as libc::c_int;
        if !((*r).vec).is_null() {
            let ref mut fresh92 = *((*r).vec)
                .offset((n as VecWord >> 6 as libc::c_int) as isize);
            *fresh92
                &= !((1 as libc::c_int as VecWord)
                    << (n as libc::c_ulong
                        & ((1 as libc::c_int as VecWord) << 6 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)));
        }
        let ref mut fresh93 = *fdv_adr
            .offset((4 as libc::c_int + 1 as libc::c_int) as isize);
        *fresh93 -= 1;
        *fresh93;
        if n == min {
            propag |= 1 as libc::c_int;
            propag |= 4 as libc::c_int;
            (*r)
                .min = if ((*r).vec).is_null() {
                n + 1 as libc::c_int
            } else {
                Pl_Vector_Next_After((*r).vec, n)
            };
        } else if n == max {
            propag |= 2 as libc::c_int;
            propag |= 4 as libc::c_int;
            (*r)
                .max = if ((*r).vec).is_null() {
                n - 1 as libc::c_int
            } else {
                Pl_Vector_Next_Before((*r).vec, n)
            };
        }
    }
    All_Propagations(fdv_adr, propag);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Tell_Int_Range(
    mut fdv_adr: *mut WamWord,
    mut range: *mut Range,
) -> Bool {
    let mut n: libc::c_int = (*(fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .min;
    if Pl_Range_Test_Value(range, n) == 0 {
        if n as libc::c_long > pl_vec_max_integer && (*range).extra_cstr != 0 {
            Pl_Fd_Display_Extra_Cstr(fdv_adr);
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Tell_Interv_Interv(
    mut fdv_adr: *mut WamWord,
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> Bool {
    let mut nb_elem: libc::c_int = 0;
    let mut propag: libc::c_int = 0;
    let mut min1: libc::c_int = 0;
    let mut max1: libc::c_int = 0;
    min1 = (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .min;
    max1 = (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .max;
    min = if min >= min1 { min } else { min1 };
    max = if max <= max1 { max } else { max1 };
    if min > max {
        return 0 as libc::c_int;
    }
    if min == max {
        propag = 0 as libc::c_int;
        if (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord)
            < *(&mut *B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord)
        {
            let fresh94 = TR;
            TR = TR.offset(1);
            *fresh94 = *fdv_adr.offset(0 as libc::c_int as isize);
            let fresh95 = TR;
            TR = TR.offset(1);
            *fresh95 = (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord
                as PlULong | 1 as libc::c_int as libc::c_ulong) as WamWord;
            let fresh96 = TR;
            TR = TR.offset(1);
            *fresh96 = *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize)
                as WamWord;
            let fresh97 = TR;
            TR = TR.offset(1);
            *fresh97 = (&mut *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize)
                as *mut PlULong as PlULong | 1 as libc::c_int as libc::c_ulong)
                as WamWord;
            if *fdv_adr.offset(4 as libc::c_int as isize)
                != *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize)
            {
                let mut s: *mut PlLong = fdv_adr.offset(4 as libc::c_int as isize)
                    as *mut PlLong;
                let mut d: *mut PlLong = TR as *mut PlLong;
                let mut counter: libc::c_int = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                            ),
                    ) as libc::c_int;
                loop {
                    let fresh98 = s;
                    s = s.offset(1);
                    let fresh99 = d;
                    d = d.offset(1);
                    *fresh99 = *fresh98;
                    counter -= 1;
                    if !(counter != 0) {
                        break;
                    }
                }
                TR = TR
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (::std::mem::size_of::<Range>() as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                let fresh100 = TR;
                TR = TR.offset(1);
                *fresh100 = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                            ),
                    ) as WamWord;
                let fresh101 = TR;
                TR = TR.offset(1);
                *fresh101 = (fdv_adr.offset(4 as libc::c_int as isize) as PlULong
                    | 2 as libc::c_int as libc::c_ulong) as WamWord;
                if !((*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec)
                    .is_null()
                {
                    let mut s_0: *mut PlLong = (*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec as *mut WamWord as *mut PlLong;
                    let mut d_0: *mut PlLong = TR as *mut PlLong;
                    let mut counter_0: libc::c_int = pl_vec_size as libc::c_int;
                    loop {
                        let fresh102 = s_0;
                        s_0 = s_0.offset(1);
                        let fresh103 = d_0;
                        d_0 = d_0.offset(1);
                        *fresh103 = *fresh102;
                        counter_0 -= 1;
                        if !(counter_0 != 0) {
                            break;
                        }
                    }
                    TR = TR.offset(pl_vec_size as isize);
                    let fresh104 = TR;
                    TR = TR.offset(1);
                    *fresh104 = pl_vec_size;
                    let fresh105 = TR;
                    TR = TR.offset(1);
                    *fresh105 = ((*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec as *mut WamWord as PlULong
                        | 2 as libc::c_int as libc::c_ulong) as WamWord;
                }
                *fdv_adr
                    .offset(
                        4 as libc::c_int as isize,
                    ) = *pl_reg_bank
                    .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
            }
        }
        *fdv_adr
            .offset(
                (4 as libc::c_int + 1 as libc::c_int) as isize,
            ) = 1 as libc::c_int as WamWord;
        propag |= 4 as libc::c_int;
        propag |= 8 as libc::c_int;
        propag |= 16 as libc::c_int;
        if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .min != min
        {
            (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .min = min;
            propag |= 1 as libc::c_int;
        }
        if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .max != min
        {
            (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .max = min;
            propag |= 2 as libc::c_int;
        }
        let ref mut fresh106 = (*(fdv_adr
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *mut Range))
            .vec;
        *fresh106 = 0 as Vector;
        *fdv_adr
            .offset(
                0 as libc::c_int as isize,
            ) = ((min as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
            as WamWord;
        *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize) = DATE;
    } else {
        nb_elem = max - min + 1 as libc::c_int;
        propag = 0 as libc::c_int;
        if *fdv_adr.offset((4 as libc::c_int + 1 as libc::c_int) as isize)
            != nb_elem as libc::c_long
        {
            if *fdv_adr.offset(4 as libc::c_int as isize)
                != *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize)
            {
                let mut s_1: *mut PlLong = fdv_adr.offset(4 as libc::c_int as isize)
                    as *mut PlLong;
                let mut d_1: *mut PlLong = TR as *mut PlLong;
                let mut counter_1: libc::c_int = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                            ),
                    ) as libc::c_int;
                loop {
                    let fresh107 = s_1;
                    s_1 = s_1.offset(1);
                    let fresh108 = d_1;
                    d_1 = d_1.offset(1);
                    *fresh108 = *fresh107;
                    counter_1 -= 1;
                    if !(counter_1 != 0) {
                        break;
                    }
                }
                TR = TR
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (::std::mem::size_of::<Range>() as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                let fresh109 = TR;
                TR = TR.offset(1);
                *fresh109 = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                            ),
                    ) as WamWord;
                let fresh110 = TR;
                TR = TR.offset(1);
                *fresh110 = (fdv_adr.offset(4 as libc::c_int as isize) as PlULong
                    | 2 as libc::c_int as libc::c_ulong) as WamWord;
                if !((*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec)
                    .is_null()
                {
                    let mut s_2: *mut PlLong = (*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec as *mut WamWord as *mut PlLong;
                    let mut d_2: *mut PlLong = TR as *mut PlLong;
                    let mut counter_2: libc::c_int = pl_vec_size as libc::c_int;
                    loop {
                        let fresh111 = s_2;
                        s_2 = s_2.offset(1);
                        let fresh112 = d_2;
                        d_2 = d_2.offset(1);
                        *fresh112 = *fresh111;
                        counter_2 -= 1;
                        if !(counter_2 != 0) {
                            break;
                        }
                    }
                    TR = TR.offset(pl_vec_size as isize);
                    let fresh113 = TR;
                    TR = TR.offset(1);
                    *fresh113 = pl_vec_size;
                    let fresh114 = TR;
                    TR = TR.offset(1);
                    *fresh114 = ((*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec as *mut WamWord as PlULong
                        | 2 as libc::c_int as libc::c_ulong) as WamWord;
                }
                *fdv_adr
                    .offset(
                        4 as libc::c_int as isize,
                    ) = *pl_reg_bank
                    .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
            }
            *fdv_adr
                .offset(
                    (4 as libc::c_int + 1 as libc::c_int) as isize,
                ) = nb_elem as WamWord;
            propag |= 4 as libc::c_int;
            propag |= 8 as libc::c_int;
            if (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .min != min
            {
                (*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .min = min;
                propag |= 1 as libc::c_int;
            }
            if (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .max != max
            {
                (*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .max = max;
                propag |= 2 as libc::c_int;
            }
        }
    }
    if propag != 0 {
        All_Propagations(fdv_adr, propag);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Tell_Range_Range(
    mut fdv_adr: *mut WamWord,
    mut range: *mut Range,
) -> Bool {
    let mut nb_elem: libc::c_int = 0;
    let mut propag: libc::c_int = 0;
    let mut save_CS: *mut WamWord = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    if !((*range).vec).is_null() {
        let ref mut fresh115 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
        *fresh115 = (*range).vec as *mut WamWord;
    }
    let ref mut fresh116 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh116 = (*fresh116).offset(pl_vec_size as isize);
    Pl_Range_Inter(
        range,
        fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range,
    );
    let ref mut fresh117 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh117 = save_CS;
    if (*range).min > (*range).max {
        if (*range).extra_cstr != 0 {
            Pl_Fd_Display_Extra_Cstr(fdv_adr);
        }
        return 0 as libc::c_int;
    }
    if (*range).min == (*range).max {
        if (*range).extra_cstr != 0 {
            Pl_Fd_Display_Extra_Cstr(fdv_adr);
        }
        propag = 0 as libc::c_int;
        if (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord)
            < *(&mut *B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord)
        {
            let fresh118 = TR;
            TR = TR.offset(1);
            *fresh118 = *fdv_adr.offset(0 as libc::c_int as isize);
            let fresh119 = TR;
            TR = TR.offset(1);
            *fresh119 = (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord
                as PlULong | 1 as libc::c_int as libc::c_ulong) as WamWord;
            let fresh120 = TR;
            TR = TR.offset(1);
            *fresh120 = *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize)
                as WamWord;
            let fresh121 = TR;
            TR = TR.offset(1);
            *fresh121 = (&mut *(fdv_adr as *mut PlULong)
                .offset(1 as libc::c_int as isize) as *mut PlULong as PlULong
                | 1 as libc::c_int as libc::c_ulong) as WamWord;
            if *fdv_adr.offset(4 as libc::c_int as isize)
                != *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize)
            {
                let mut s: *mut PlLong = fdv_adr.offset(4 as libc::c_int as isize)
                    as *mut PlLong;
                let mut d: *mut PlLong = TR as *mut PlLong;
                let mut counter: libc::c_int = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                            ),
                    ) as libc::c_int;
                loop {
                    let fresh122 = s;
                    s = s.offset(1);
                    let fresh123 = d;
                    d = d.offset(1);
                    *fresh123 = *fresh122;
                    counter -= 1;
                    if !(counter != 0) {
                        break;
                    }
                }
                TR = TR
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (::std::mem::size_of::<Range>() as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                let fresh124 = TR;
                TR = TR.offset(1);
                *fresh124 = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                            ),
                    ) as WamWord;
                let fresh125 = TR;
                TR = TR.offset(1);
                *fresh125 = (fdv_adr.offset(4 as libc::c_int as isize) as PlULong
                    | 2 as libc::c_int as libc::c_ulong) as WamWord;
                if !((*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec)
                    .is_null()
                {
                    let mut s_0: *mut PlLong = (*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec as *mut WamWord as *mut PlLong;
                    let mut d_0: *mut PlLong = TR as *mut PlLong;
                    let mut counter_0: libc::c_int = pl_vec_size as libc::c_int;
                    loop {
                        let fresh126 = s_0;
                        s_0 = s_0.offset(1);
                        let fresh127 = d_0;
                        d_0 = d_0.offset(1);
                        *fresh127 = *fresh126;
                        counter_0 -= 1;
                        if !(counter_0 != 0) {
                            break;
                        }
                    }
                    TR = TR.offset(pl_vec_size as isize);
                    let fresh128 = TR;
                    TR = TR.offset(1);
                    *fresh128 = pl_vec_size;
                    let fresh129 = TR;
                    TR = TR.offset(1);
                    *fresh129 = ((*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec as *mut WamWord as PlULong
                        | 2 as libc::c_int as libc::c_ulong) as WamWord;
                }
                *fdv_adr
                    .offset(
                        4 as libc::c_int as isize,
                    ) = *pl_reg_bank
                    .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
            }
        }
        *fdv_adr
            .offset(
                (4 as libc::c_int + 1 as libc::c_int) as isize,
            ) = 1 as libc::c_int as WamWord;
        propag |= 4 as libc::c_int;
        propag |= 8 as libc::c_int;
        propag |= 16 as libc::c_int;
        if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .min != (*range).min
        {
            (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .min = (*range).min;
            propag |= 1 as libc::c_int;
        }
        if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .max != (*range).min
        {
            (*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .max = (*range).min;
            propag |= 2 as libc::c_int;
        }
        let ref mut fresh130 = (*(fdv_adr
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *mut Range))
            .vec;
        *fresh130 = 0 as Vector;
        *fdv_adr
            .offset(
                0 as libc::c_int as isize,
            ) = (((*range).min as PlULong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord;
        *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize) = DATE;
    } else {
        nb_elem = Pl_Range_Nb_Elem(range);
        let mut r: *mut Range = fdv_adr
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *mut Range;
        propag = 0 as libc::c_int;
        if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .min != (*range).min
        {
            propag |= 1 as libc::c_int;
            propag |= 4 as libc::c_int;
        }
        if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .max != (*range).max
        {
            propag |= 2 as libc::c_int;
            propag |= 4 as libc::c_int;
        }
        if *fdv_adr.offset((4 as libc::c_int + 1 as libc::c_int) as isize)
            != nb_elem as libc::c_long
        {
            propag |= 8 as libc::c_int;
        }
        if propag != 0 || ((*r).vec).is_null() && !((*range).vec).is_null() {
            if *fdv_adr.offset(4 as libc::c_int as isize)
                != *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize)
            {
                let mut s_1: *mut PlLong = fdv_adr.offset(4 as libc::c_int as isize)
                    as *mut PlLong;
                let mut d_1: *mut PlLong = TR as *mut PlLong;
                let mut counter_1: libc::c_int = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                            ),
                    ) as libc::c_int;
                loop {
                    let fresh131 = s_1;
                    s_1 = s_1.offset(1);
                    let fresh132 = d_1;
                    d_1 = d_1.offset(1);
                    *fresh132 = *fresh131;
                    counter_1 -= 1;
                    if !(counter_1 != 0) {
                        break;
                    }
                }
                TR = TR
                    .offset(
                        (2 as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                (::std::mem::size_of::<Range>() as libc::c_ulong)
                                    .wrapping_div(
                                        ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                let fresh133 = TR;
                TR = TR.offset(1);
                *fresh133 = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (::std::mem::size_of::<Range>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                            ),
                    ) as WamWord;
                let fresh134 = TR;
                TR = TR.offset(1);
                *fresh134 = (fdv_adr.offset(4 as libc::c_int as isize) as PlULong
                    | 2 as libc::c_int as libc::c_ulong) as WamWord;
                if !((*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec)
                    .is_null()
                {
                    let mut s_2: *mut PlLong = (*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec as *mut WamWord as *mut PlLong;
                    let mut d_2: *mut PlLong = TR as *mut PlLong;
                    let mut counter_2: libc::c_int = pl_vec_size as libc::c_int;
                    loop {
                        let fresh135 = s_2;
                        s_2 = s_2.offset(1);
                        let fresh136 = d_2;
                        d_2 = d_2.offset(1);
                        *fresh136 = *fresh135;
                        counter_2 -= 1;
                        if !(counter_2 != 0) {
                            break;
                        }
                    }
                    TR = TR.offset(pl_vec_size as isize);
                    let fresh137 = TR;
                    TR = TR.offset(1);
                    *fresh137 = pl_vec_size;
                    let fresh138 = TR;
                    TR = TR.offset(1);
                    *fresh138 = ((*(fdv_adr
                        .offset(4 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize) as *mut Range))
                        .vec as *mut WamWord as PlULong
                        | 2 as libc::c_int as libc::c_ulong) as WamWord;
                }
                *fdv_adr
                    .offset(
                        4 as libc::c_int as isize,
                    ) = *pl_reg_bank
                    .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
            }
            *fdv_adr
                .offset(
                    (4 as libc::c_int + 1 as libc::c_int) as isize,
                ) = nb_elem as WamWord;
            Pl_Range_Copy(r, range);
        } else if (*r).extra_cstr != (*range).extra_cstr {
            if (&mut (*r).extra_cstr as *mut Bool as *mut WamWord)
                < *(&mut *B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
                    as *mut *mut WamWord)
            {
                let fresh139 = TR;
                TR = TR.offset(1);
                *fresh139 = *(&mut (*r).extra_cstr as *mut Bool as *mut WamWord);
                let fresh140 = TR;
                TR = TR.offset(1);
                *fresh140 = (&mut (*r).extra_cstr as *mut Bool as *mut WamWord as PlULong
                    | 1 as libc::c_int as libc::c_ulong) as WamWord;
            }
            *(&mut (*r).extra_cstr as *mut Bool
                as *mut WamWord) = (*range).extra_cstr as WamWord;
            propag |= 8 as libc::c_int;
        }
    }
    if propag != 0 {
        All_Propagations(fdv_adr, propag);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Tell_Interval(
    mut fdv_adr: *mut WamWord,
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> Bool {
    let mut n: libc::c_int = 0;
    let mut range: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    if *fdv_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
        & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong
    {
        n = (*(fdv_adr
            .offset(4 as libc::c_int as isize)
            .offset(2 as libc::c_int as isize) as *mut Range))
            .min;
        return (n >= min && n <= max) as libc::c_int;
    }
    if !((*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .vec)
        .is_null()
    {
        range.extra_cstr = 0 as libc::c_int;
        range.min = min;
        range.max = max;
        range.vec = 0 as Vector;
        return Pl_Fd_Tell_Range_Range(fdv_adr, &mut range);
    }
    return Pl_Fd_Tell_Interv_Interv(fdv_adr, min, max);
}
pub unsafe extern "C" fn Pl_Fd_Tell_Range(
    mut fdv_adr: *mut WamWord,
    mut range: *mut Range,
) -> Bool {
    if *fdv_adr.offset(0 as libc::c_int as isize) as libc::c_ulong
        & 0x7 as libc::c_int as PlULong == 7 as libc::c_int as libc::c_ulong
    {
        return Pl_Fd_Tell_Int_Range(fdv_adr, range);
    }
    return Pl_Fd_Tell_Range_Range(fdv_adr, range);
}
unsafe extern "C" fn All_Propagations(
    mut fdv_adr: *mut WamWord,
    mut propag: libc::c_int,
) {
    propag = (propag as libc::c_long
        & *fdv_adr
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
                    .wrapping_add(2 as libc::c_int as libc::c_ulong) as isize,
            )) as libc::c_int;
    if propag != 0 {
        if !(*fdv_adr.offset(2 as libc::c_int as isize)
            != 0 as libc::c_int as libc::c_long)
        {
            *fdv_adr.offset(2 as libc::c_int as isize) = propag as WamWord;
            let ref mut fresh141 = *(TP as *mut *mut WamWord)
                .offset(3 as libc::c_int as isize);
            *fresh141 = fdv_adr;
            TP = fdv_adr;
        } else {
            let ref mut fresh142 = *fdv_adr.offset(2 as libc::c_int as isize);
            *fresh142 |= propag as libc::c_long;
        }
    }
}
pub unsafe extern "C" fn Pl_Fd_After_Add_Cstr(mut result_of_tell: Bool) -> Bool {
    let mut current_block: u64;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut propag: WamWord = 0;
    let mut record_adr: *mut WamWord = 0 as *mut WamWord;
    let mut chain_adr: *mut *mut WamWord = 0 as *mut *mut WamWord;
    let mut CF: *mut WamWord = 0 as *mut WamWord;
    let mut BP: *mut WamWord = 0 as *mut WamWord;
    let mut date: PlULong = DATE;
    let mut pdate: *mut PlULong = 0 as *mut PlULong;
    let mut AF: *mut WamWord = 0 as *mut WamWord;
    let mut fct: Option::<unsafe extern "C" fn() -> PlLong> = None;
    if !(result_of_tell == 0) {
        if TP == dummy_fd_var.as_mut_ptr() {
            return 1 as libc::c_int;
        }
        BP = *(dummy_fd_var.as_mut_ptr() as *mut *mut WamWord)
            .offset(3 as libc::c_int as isize);
        's_49: loop {
            fdv_adr = BP;
            propag = *fdv_adr.offset(2 as libc::c_int as isize);
            *fdv_adr
                .offset(
                    2 as libc::c_int as isize,
                ) = ((1 as libc::c_int) << 8 as libc::c_int) as WamWord;
            chain_adr = &mut *(fdv_adr as *mut *mut WamWord)
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
                        .wrapping_add(3 as libc::c_int as libc::c_ulong) as isize,
                ) as *mut *mut WamWord;
            while propag != 0 {
                if propag & 1 as libc::c_int as libc::c_long != 0 {
                    record_adr = *chain_adr;
                    loop {
                        CF = *(&mut *record_adr.offset(0 as libc::c_int as isize)
                            as *mut WamWord as *mut *mut WamWord);
                        pdate = *(&mut *CF.offset(1 as libc::c_int as isize)
                            as *mut WamWord as *mut *mut PlULong);
                        if !(*pdate != 1 as libc::c_int as libc::c_ulong
                            && *pdate != date)
                        {
                            fct = *(&mut *CF.offset(2 as libc::c_int as isize)
                                as *mut WamWord
                                as *mut Option::<unsafe extern "C" fn() -> PlLong>);
                            AF = *(&mut *CF.offset(0 as libc::c_int as isize)
                                as *mut WamWord as *mut *mut WamWord);
                            fct = ::std::mem::transmute::<
                                libc::intptr_t,
                                Option::<unsafe extern "C" fn() -> PlLong>,
                            >(
                                ::std::mem::transmute::<
                                    _,
                                    fn(_) -> PlLong,
                                >((Some(fct.unwrap())).unwrap())(AF) as libc::intptr_t,
                            );
                            if fct.is_none() {
                                current_block = 2008651230485561240;
                            } else if fct
                                != ::std::mem::transmute::<
                                    libc::intptr_t,
                                    Option::<unsafe extern "C" fn() -> PlLong>,
                                >(1 as libc::c_int as libc::intptr_t)
                            {
                                if ::std::mem::transmute::<
                                    _,
                                    fn(_) -> PlLong,
                                >((Some(fct.unwrap())).unwrap())(AF)
                                    == 0 as libc::c_int as libc::c_long
                                {
                                    current_block = 2008651230485561240;
                                } else {
                                    Pl_Fd_Stop_Constraint(CF);
                                    current_block = 10048703153582371463;
                                }
                            } else {
                                current_block = 10048703153582371463;
                            }
                            match current_block {
                                10048703153582371463 => {}
                                _ => {
                                    let ref mut fresh143 = *(dummy_fd_var.as_mut_ptr()
                                        as *mut *mut WamWord)
                                        .offset(3 as libc::c_int as isize);
                                    *fresh143 = BP;
                                    current_block = 9414711627383530184;
                                    break 's_49;
                                }
                            }
                        }
                        record_adr = *(&mut *record_adr.offset(1 as libc::c_int as isize)
                            as *mut WamWord as *mut *mut WamWord);
                        if record_adr.is_null() {
                            break;
                        }
                    }
                }
                propag >>= 1 as libc::c_int;
                chain_adr = chain_adr.offset(1);
                chain_adr;
            }
            let ref mut fresh144 = *fdv_adr.offset(2 as libc::c_int as isize);
            *fresh144
                &= (((1 as libc::c_int) << 8 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_long;
            if !(*fdv_adr.offset(2 as libc::c_int as isize)
                == 0 as libc::c_int as libc::c_long)
            {
                continue;
            }
            if BP == TP {
                current_block = 11459959175219260272;
                break;
            }
            BP = *(BP as *mut *mut WamWord).offset(3 as libc::c_int as isize);
        }
        match current_block {
            9414711627383530184 => {}
            _ => {
                TP = dummy_fd_var.as_mut_ptr();
                return 1 as libc::c_int;
            }
        }
    }
    Clear_Queue();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Fd_Stop_Constraint(mut CF: *mut WamWord) {
    if CF.offset(1 as libc::c_int as isize)
        < *(&mut *B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord)
    {
        let fresh145 = TR;
        TR = TR.offset(1);
        *fresh145 = *CF.offset(1 as libc::c_int as isize);
        let fresh146 = TR;
        TR = TR.offset(1);
        *fresh146 = (CF.offset(1 as libc::c_int as isize) as PlULong
            | 1 as libc::c_int as libc::c_ulong) as WamWord;
    }
    *CF
        .offset(
            1 as libc::c_int as isize,
        ) = &mut optim2_date_never as *mut PlULong as WamWord;
}
pub unsafe extern "C" fn Pl_Fd_In_Interval(
    mut fdv_adr: *mut WamWord,
    mut min: libc::c_int,
    mut max: libc::c_int,
) -> Bool {
    Pl_Fd_Before_Add_Cstr();
    return Pl_Fd_After_Add_Cstr(Pl_Fd_Tell_Interval(fdv_adr, min, max));
}
pub unsafe extern "C" fn Pl_Fd_In_Range(
    mut fdv_adr: *mut WamWord,
    mut range: *mut Range,
) -> Bool {
    Pl_Fd_Before_Add_Cstr();
    return Pl_Fd_After_Add_Cstr(Pl_Fd_Tell_Range(fdv_adr, range));
}
pub unsafe extern "C" fn Pl_Fd_Assign_Value_Fast(
    mut fdv_adr: *mut WamWord,
    mut n: libc::c_int,
) -> Bool {
    let mut propag: libc::c_int = 0;
    Pl_Fd_Before_Add_Cstr();
    propag = 0 as libc::c_int;
    if (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord)
        < *(&mut *B.offset(-(8 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord)
    {
        let fresh147 = TR;
        TR = TR.offset(1);
        *fresh147 = *fdv_adr.offset(0 as libc::c_int as isize);
        let fresh148 = TR;
        TR = TR.offset(1);
        *fresh148 = (&mut *fdv_adr.offset(0 as libc::c_int as isize) as *mut WamWord
            as PlULong | 1 as libc::c_int as libc::c_ulong) as WamWord;
        let fresh149 = TR;
        TR = TR.offset(1);
        *fresh149 = *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize)
            as WamWord;
        let fresh150 = TR;
        TR = TR.offset(1);
        *fresh150 = (&mut *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize)
            as *mut PlULong as PlULong | 1 as libc::c_int as libc::c_ulong) as WamWord;
        if *fdv_adr.offset(4 as libc::c_int as isize)
            != *pl_reg_bank.offset((256 as libc::c_int + 5 as libc::c_int) as isize)
        {
            let mut s: *mut PlLong = fdv_adr.offset(4 as libc::c_int as isize)
                as *mut PlLong;
            let mut d: *mut PlLong = TR as *mut PlLong;
            let mut counter: libc::c_int = (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong),
                ) as libc::c_int;
            loop {
                let fresh151 = s;
                s = s.offset(1);
                let fresh152 = d;
                d = d.offset(1);
                *fresh152 = *fresh151;
                counter -= 1;
                if !(counter != 0) {
                    break;
                }
            }
            TR = TR
                .offset(
                    (2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            (::std::mem::size_of::<Range>() as libc::c_ulong)
                                .wrapping_div(
                                    ::std::mem::size_of::<WamWord>() as libc::c_ulong,
                                ),
                        ) as isize,
                );
            let fresh153 = TR;
            TR = TR.offset(1);
            *fresh153 = (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong),
                ) as WamWord;
            let fresh154 = TR;
            TR = TR.offset(1);
            *fresh154 = (fdv_adr.offset(4 as libc::c_int as isize) as PlULong
                | 2 as libc::c_int as libc::c_ulong) as WamWord;
            if !((*(fdv_adr
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize) as *mut Range))
                .vec)
                .is_null()
            {
                let mut s_0: *mut PlLong = (*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec as *mut WamWord as *mut PlLong;
                let mut d_0: *mut PlLong = TR as *mut PlLong;
                let mut counter_0: libc::c_int = pl_vec_size as libc::c_int;
                loop {
                    let fresh155 = s_0;
                    s_0 = s_0.offset(1);
                    let fresh156 = d_0;
                    d_0 = d_0.offset(1);
                    *fresh156 = *fresh155;
                    counter_0 -= 1;
                    if !(counter_0 != 0) {
                        break;
                    }
                }
                TR = TR.offset(pl_vec_size as isize);
                let fresh157 = TR;
                TR = TR.offset(1);
                *fresh157 = pl_vec_size;
                let fresh158 = TR;
                TR = TR.offset(1);
                *fresh158 = ((*(fdv_adr
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize) as *mut Range))
                    .vec as *mut WamWord as PlULong | 2 as libc::c_int as libc::c_ulong)
                    as WamWord;
            }
            *fdv_adr
                .offset(
                    4 as libc::c_int as isize,
                ) = *pl_reg_bank
                .offset((256 as libc::c_int + 5 as libc::c_int) as isize);
        }
    }
    *fdv_adr
        .offset(
            (4 as libc::c_int + 1 as libc::c_int) as isize,
        ) = 1 as libc::c_int as WamWord;
    propag |= 4 as libc::c_int;
    propag |= 8 as libc::c_int;
    propag |= 16 as libc::c_int;
    if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .min != n
    {
        (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .min = n;
        propag |= 1 as libc::c_int;
    }
    if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .max != n
    {
        (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range))
            .max = n;
        propag |= 2 as libc::c_int;
    }
    let ref mut fresh159 = (*(fdv_adr
        .offset(4 as libc::c_int as isize)
        .offset(2 as libc::c_int as isize) as *mut Range))
        .vec;
    *fresh159 = 0 as Vector;
    *fdv_adr
        .offset(
            0 as libc::c_int as isize,
        ) = ((n as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
    *(fdv_adr as *mut PlULong).offset(1 as libc::c_int as isize) = DATE;
    All_Propagations(fdv_adr, propag);
    return Pl_Fd_After_Add_Cstr(1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Fd_Unify_With_Integer0(
    mut fdv_adr: *mut WamWord,
    mut n: libc::c_int,
) -> Bool {
    Pl_Fd_Before_Add_Cstr();
    return Pl_Fd_After_Add_Cstr(Pl_Fd_Tell_Value(fdv_adr, n));
}
pub unsafe extern "C" fn Pl_Fd_Remove_Value(
    mut fdv_adr: *mut WamWord,
    mut n: libc::c_int,
) -> Bool {
    Pl_Fd_Before_Add_Cstr();
    return Pl_Fd_After_Add_Cstr(Pl_Fd_Tell_Not_Value(fdv_adr, n));
}
pub unsafe extern "C" fn Pl_Fd_Unify_With_Fd_Var0(
    mut fdv_adr1: *mut WamWord,
    mut fdv_adr2: *mut WamWord,
) -> Bool {
    extern "C" {
        #[link_name = "pl_unify_x_y"]
        fn pl_unify_x_y_0(x: WamWord, y: WamWord) -> Bool;
    }
    return pl_unify_x_y_0(
        (fdv_adr1 as PlLong as libc::c_ulong).wrapping_add(0 as libc::c_int as PlULong)
            as WamWord,
        (fdv_adr2 as PlLong as libc::c_ulong).wrapping_add(0 as libc::c_int as PlULong)
            as WamWord,
    );
}
pub unsafe extern "C" fn Pl_Fd_Use_Vector(mut fdv_adr: *mut WamWord) -> Bool {
    let mut range: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    if !((*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .vec)
        .is_null()
    {
        return 1 as libc::c_int;
    }
    Pl_Fd_Before_Add_Cstr();
    let mut save_CS: *mut WamWord = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh160 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh160 = (*fresh160).offset(pl_vec_size as isize);
    range.extra_cstr = 0 as libc::c_int;
    range.min = 0 as libc::c_int;
    range
        .max = (((1 as libc::c_int as PlLong)
        << 32 as libc::c_int - 3 as libc::c_int - 1 as libc::c_int)
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    range.vec = 0 as Vector;
    Pl_Range_Becomes_Sparse(&mut range);
    let ref mut fresh161 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh161 = save_CS;
    return Pl_Fd_After_Add_Cstr(Pl_Fd_Tell_Range_Range(fdv_adr, &mut range));
}
pub unsafe extern "C" fn Pl_Fd_Check_For_Bool_Var(mut x_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut fdv_adr: *mut WamWord = 0 as *mut WamWord;
    let mut range: Range = Range {
        extra_cstr: 0,
        min: 0,
        max: 0,
        vec: 0 as *mut VecWord,
    };
    let mut result_of_tell: Bool = 0;
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
        adr = word as *mut WamWord;
        fdv_adr = Pl_Fd_New_Variable_Interval(0 as libc::c_int, 1 as libc::c_int);
        if adr
            < *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
            || adr
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize) && adr < B
        {
            let fresh162 = TR;
            TR = TR.offset(1);
            *fresh162 = (adr as PlULong | 0 as libc::c_int as libc::c_ulong) as WamWord;
        }
        *adr = (fdv_adr as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        return 1 as libc::c_int;
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        return ((word << 0 as libc::c_int >> 3 as libc::c_int) as PlULong
            <= 1 as libc::c_int as libc::c_ulong) as libc::c_int;
    }
    if tag_mask as libc::c_ulong != 0x5 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_fd_variable, word);
    }
    fdv_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
        as *mut WamWord;
    if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .min > 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .max <= 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if (*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .min == 1 as libc::c_int
    {
        return Pl_Fd_Unify_With_Integer0(fdv_adr, 1 as libc::c_int);
    }
    if Pl_Range_Test_Value(
        fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range,
        1 as libc::c_int,
    ) == 0
    {
        return Pl_Fd_Unify_With_Integer0(fdv_adr, 0 as libc::c_int);
    }
    Pl_Fd_Before_Add_Cstr();
    if !((*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .vec)
        .is_null()
    {
        range.extra_cstr = 0 as libc::c_int;
        range.min = 0 as libc::c_int;
        range.max = 1 as libc::c_int;
        range.vec = 0 as Vector;
        result_of_tell = Pl_Fd_Tell_Range_Range(fdv_adr, &mut range);
    } else {
        result_of_tell = Pl_Fd_Tell_Interv_Interv(
            fdv_adr,
            0 as libc::c_int,
            1 as libc::c_int,
        );
    }
    return Pl_Fd_After_Add_Cstr(result_of_tell);
}
pub unsafe extern "C" fn Pl_Fd_Variable_Size0(mut fdv_adr: *mut WamWord) -> libc::c_int {
    let mut size: libc::c_int = (4 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<Range>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<WamWord>() as libc::c_ulong),
                ),
        )
        .wrapping_add(8 as libc::c_int as libc::c_ulong) as libc::c_int;
    if !((*(fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
        as *mut Range))
        .vec)
        .is_null()
    {
        size = (size as libc::c_long + pl_vec_size) as libc::c_int;
    }
    return size;
}
pub unsafe extern "C" fn Pl_Fd_Copy_Variable0(
    mut dst_adr: *mut WamWord,
    mut fdv_adr: *mut WamWord,
) -> libc::c_int {
    let mut save_CS: *mut WamWord = 0 as *mut WamWord;
    let mut size: libc::c_int = 0;
    save_CS = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    let ref mut fresh163 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh163 = dst_adr;
    Pl_Fd_New_Variable();
    *dst_adr
        .offset(
            (4 as libc::c_int + 1 as libc::c_int) as isize,
        ) = *fdv_adr.offset((4 as libc::c_int + 1 as libc::c_int) as isize);
    Pl_Range_Copy(
        dst_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range,
        fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range,
    );
    size = (*(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize))
        .offset_from(dst_adr) as libc::c_long as libc::c_int;
    let ref mut fresh164 = *(pl_reg_bank as *mut WamWordP)
        .offset((256 as libc::c_int + 3 as libc::c_int) as isize);
    *fresh164 = save_CS;
    return size;
}
pub unsafe extern "C" fn Pl_Fd_Variable_To_String0(
    mut fdv_adr: *mut WamWord,
) -> *mut libc::c_char {
    return Pl_Range_To_String(
        fdv_adr.offset(4 as libc::c_int as isize).offset(2 as libc::c_int as isize)
            as *mut Range,
    );
}
pub unsafe extern "C" fn Pl_Fd_Display_Extra_Cstr(mut fdv_adr: *mut WamWord) {
    Pl_Stream_Printf(
        *pl_stm_tbl.offset(pl_stm_stdout as isize),
        b"Warning: Vector too small - maybe lost solutions (FD Var:_%ld)\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        fdv_adr
            .offset_from(
                (*pl_stk_tbl.as_mut_ptr().offset(1 as libc::c_int as isize)).stack,
            ) as libc::c_long,
    );
}

use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Hash_First(tbl: *mut libc::c_char, scan: *mut HashScan) -> *mut libc::c_char;
    fn Pl_Hash_Next(scan: *mut HashScan) -> *mut libc::c_char;
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_atom_void: libc::c_int;
    static mut pl_pred_tbl: *mut libc::c_char;
    fn Pl_Lookup_Pred(func: libc::c_int, arity: libc::c_int) -> *mut PredInf;
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Atom(start_word: WamWord) -> libc::c_int;
    fn Pl_Un_Integer_Check(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Un_Atom_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Detect_If_Aux_Name(func: libc::c_int) -> *mut libc::c_char;
    fn Pl_Father_Pred_Of_Aux(
        func: libc::c_int,
        father_arity: *mut libc::c_int,
    ) -> libc::c_int;
    fn Pl_Pred_Without_Aux(
        func: libc::c_int,
        arity: libc::c_int,
        arity1: *mut libc::c_int,
    ) -> libc::c_int;
    fn Pl_Make_Aux_Name(
        func: libc::c_int,
        arity: libc::c_int,
        aux_nb: libc::c_int,
    ) -> libc::c_int;
    static mut pl_pi_name_word: WamWord;
    static mut pl_pi_arity_word: WamWord;
    fn Pl_Get_Pred_Indicator(
        pred_indic_word: WamWord,
        must_be_ground: Bool,
        arity: *mut libc::c_int,
    ) -> libc::c_int;
    static mut pl_flag_strict_iso: *mut FlagInf;
    fn X1_2463757272656E745F7072656469636174655F616C74__a0();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashScan {
    pub endt: *mut libc::c_char,
    pub cur_t: *mut libc::c_char,
    pub cur_p: *mut libc::c_char,
}
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamWordP = *mut WamWord;
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
pub type FlagInf = flag_inf;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Current_Predicate_2(
    mut pred_indic_word: WamWord,
    mut which_preds_word: WamWord,
) -> Bool {
    let mut name_word: WamWord = 0;
    let mut arity_word: WamWord = 0;
    let mut scan: HashScan = HashScan {
        endt: 0 as *mut libc::c_char,
        cur_t: 0 as *mut libc::c_char,
        cur_p: 0 as *mut libc::c_char,
    };
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut func1: libc::c_int = 0;
    let mut arity1: libc::c_int = 0;
    let mut which_preds: libc::c_int = 0;
    let mut all: Bool = 0;
    func = Pl_Get_Pred_Indicator(pred_indic_word, 0 as libc::c_int, &mut arity);
    name_word = pl_pi_name_word;
    arity_word = pl_pi_arity_word;
    which_preds = Pl_Rd_Integer(which_preds_word) as libc::c_int;
    if which_preds == 0 as libc::c_int && (*pl_flag_strict_iso).value == 0 {
        which_preds = 1 as libc::c_int;
    }
    if func >= 0 as libc::c_int && arity >= 0 as libc::c_int {
        pred = Pl_Lookup_Pred(func, arity);
        return (!pred.is_null()
            && (which_preds == 2 as libc::c_int
                || *((*pl_atom_tbl.offset(func as isize)).name)
                    .offset(0 as libc::c_int as isize) as libc::c_int != '$' as i32
                    && (which_preds == 1 as libc::c_int
                        || (*pred).prop & 8 as libc::c_int == 0))) as libc::c_int;
    }
    all = (func == -(1 as libc::c_int) && arity == -(1 as libc::c_int)) as libc::c_int;
    pred = Pl_Hash_First(pl_pred_tbl, &mut scan) as *mut PredInf;
    loop {
        if pred.is_null() {
            return 0 as libc::c_int;
        }
        func1 = ((*pred).f_n as PlULong
            & ((1 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        arity1 = ((*pred).f_n as PlULong
            >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
        if (all != 0 || func == func1 || arity == arity1)
            && (which_preds == 2 as libc::c_int
                || *((*pl_atom_tbl.offset(func1 as isize)).name)
                    .offset(0 as libc::c_int as isize) as libc::c_int != '$' as i32
                    && (which_preds == 1 as libc::c_int
                        || (*pred).prop & 8 as libc::c_int == 0))
        {
            break;
        }
        pred = Pl_Hash_Next(&mut scan) as *mut PredInf;
    }
    *pl_reg_bank.offset(0 as libc::c_int as isize) = name_word;
    *pl_reg_bank.offset(1 as libc::c_int as isize) = arity_word;
    *pl_reg_bank.offset(2 as libc::c_int as isize) = which_preds as WamWord;
    *pl_reg_bank.offset(3 as libc::c_int as isize) = scan.endt as WamWord;
    *pl_reg_bank.offset(4 as libc::c_int as isize) = scan.cur_t as WamWord;
    *pl_reg_bank.offset(5 as libc::c_int as isize) = scan.cur_p as WamWord;
    Pl_Create_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F7072656469636174655F616C74__a0),
            ),
        ),
        6 as libc::c_int,
    );
    return (Pl_Get_Atom(
        ((*pred).f_n as PlULong
            & ((1 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int,
        name_word,
    ) != 0
        && Pl_Get_Integer(
            ((*pred).f_n as PlULong
                >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as PlLong,
            arity_word,
        ) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Current_Predicate_Alt_0() -> Bool {
    let mut name_word: WamWord = 0;
    let mut arity_word: WamWord = 0;
    let mut scan: HashScan = HashScan {
        endt: 0 as *mut libc::c_char,
        cur_t: 0 as *mut libc::c_char,
        cur_p: 0 as *mut libc::c_char,
    };
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut which_preds: libc::c_int = 0;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut func1: libc::c_int = 0;
    let mut arity1: libc::c_int = 0;
    let mut all: Bool = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F7072656469636174655F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    name_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    arity_word = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    which_preds = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    scan
        .endt = *(&mut *B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
        as *mut WamWord) as *mut libc::c_char;
    scan
        .cur_t = *(&mut *B.offset((-(9 as libc::c_int) - 4 as libc::c_int) as isize)
        as *mut WamWord) as *mut libc::c_char;
    scan
        .cur_p = *(&mut *B.offset((-(9 as libc::c_int) - 5 as libc::c_int) as isize)
        as *mut WamWord) as *mut libc::c_char;
    func = (if name_word as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0 as libc::c_int as PlULong
    {
        -(1 as libc::c_int) as libc::c_ulong
    } else {
        name_word as PlULong >> 3 as libc::c_int
    }) as libc::c_int;
    arity = (if arity_word as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0 as libc::c_int as PlULong
    {
        -(1 as libc::c_int) as libc::c_long
    } else {
        arity_word << 0 as libc::c_int >> 3 as libc::c_int
    }) as libc::c_int;
    all = (func == -(1 as libc::c_int) && arity == -(1 as libc::c_int)) as libc::c_int;
    loop {
        pred = Pl_Hash_Next(&mut scan) as *mut PredInf;
        if pred.is_null() {
            B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
            let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
            *fresh0 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
            return 0 as libc::c_int;
        }
        func1 = ((*pred).f_n as PlULong
            & ((1 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        arity1 = ((*pred).f_n as PlULong
            >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
        if (all != 0 || func == func1 || arity == arity1)
            && (which_preds == 2 as libc::c_int
                || *((*pl_atom_tbl.offset(func1 as isize)).name)
                    .offset(0 as libc::c_int as isize) as libc::c_int != '$' as i32
                    && (which_preds == 1 as libc::c_int
                        || (*pred).prop & 8 as libc::c_int == 0))
        {
            break;
        }
    }
    *(&mut *B.offset((-(9 as libc::c_int) - 4 as libc::c_int) as isize)
        as *mut WamWord) = scan.cur_t as WamWord;
    *(&mut *B.offset((-(9 as libc::c_int) - 5 as libc::c_int) as isize)
        as *mut WamWord) = scan.cur_p as WamWord;
    return (Pl_Get_Atom(
        ((*pred).f_n as PlULong
            & ((1 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int,
        name_word,
    ) != 0
        && Pl_Get_Integer(
            ((*pred).f_n as PlULong
                >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as PlLong,
            arity_word,
        ) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Static_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 2 as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Dynamic_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 2 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Private_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 4 as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Public_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 4 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Monofile_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 64 as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Multifile_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 64 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_User_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 8 as libc::c_int == 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Built_In_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 8 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Built_In_Fd_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 16 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Control_Construct_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 32 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Native_Code_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && (*pred).prop & 1 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Prolog_File_3(
    mut func_word: WamWord,
    mut arity_word: WamWord,
    mut pl_file_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null() && Pl_Un_Atom_Check((*pred).pl_file, pl_file_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Prop_Prolog_Line_3(
    mut func_word: WamWord,
    mut arity_word: WamWord,
    mut pl_line_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = Pl_Rd_Atom(func_word);
    let mut arity: libc::c_int = Pl_Rd_Integer(arity_word) as libc::c_int;
    let mut pred: *mut PredInf = Pl_Lookup_Pred(func, arity);
    return (!pred.is_null()
        && Pl_Un_Integer_Check((*pred).pl_line as PlLong, pl_line_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Get_Pred_Indicator_3(
    mut pred_indic_word: WamWord,
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    func = Pl_Get_Pred_Indicator(pred_indic_word, 1 as libc::c_int, &mut arity);
    return (Pl_Get_Atom(func, func_word) != 0
        && Pl_Get_Integer(arity as PlLong, arity_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Get_Predicate_File_Info_3(
    mut pred_indic_word: WamWord,
    mut pl_file_word: WamWord,
    mut pl_line_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    func = Pl_Get_Pred_Indicator(pred_indic_word, 1 as libc::c_int, &mut arity);
    pred = Pl_Lookup_Pred(func, arity);
    if pred.is_null() {
        return 0 as libc::c_int;
    }
    if (*pred).pl_file == pl_atom_void || (*pred).pl_line == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return (Pl_Un_Atom_Check((*pred).pl_file, pl_file_word) != 0
        && Pl_Un_Integer_Check((*pred).pl_line as PlLong, pl_line_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Set_Predicate_File_Info_3(
    mut pred_indic_word: WamWord,
    mut pl_file_word: WamWord,
    mut pl_line_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut pl_file: libc::c_int = 0;
    let mut pl_line: libc::c_int = 0;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    func = Pl_Get_Pred_Indicator(pred_indic_word, 1 as libc::c_int, &mut arity);
    pred = Pl_Lookup_Pred(func, arity);
    if pred.is_null() {
        return 0 as libc::c_int;
    }
    pl_file = Pl_Rd_Atom_Check(pl_file_word);
    pl_line = Pl_Rd_Integer_Check(pl_line_word) as libc::c_int;
    if pl_line < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*pred).pl_file = pl_file;
    (*pred).pl_line = pl_line;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Aux_Name_1(mut name_word: WamWord) -> Bool {
    let mut func: libc::c_int = 0;
    func = Pl_Rd_Atom_Check(name_word);
    return (Pl_Detect_If_Aux_Name(func) != 0 as *mut libc::c_void as *mut libc::c_char)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Not_Aux_Name_1(mut name_word: WamWord) -> Bool {
    let mut func: libc::c_int = 0;
    func = Pl_Rd_Atom_Check(name_word);
    return (Pl_Detect_If_Aux_Name(func) == 0 as *mut libc::c_void as *mut libc::c_char)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Father_Of_Aux_Name_3(
    mut name_word: WamWord,
    mut father_name_word: WamWord,
    mut father_arity_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = 0;
    let mut father_func: libc::c_int = 0;
    let mut father_arity: libc::c_int = 0;
    func = Pl_Rd_Atom_Check(name_word);
    father_func = Pl_Father_Pred_Of_Aux(func, &mut father_arity);
    if father_func < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return (Pl_Un_Atom_Check(father_func, father_name_word) != 0
        && Pl_Un_Integer_Check(father_arity as PlLong, father_arity_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Pred_Without_Aux_4(
    mut name_word: WamWord,
    mut arity_word: WamWord,
    mut name1_word: WamWord,
    mut arity1_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut func1: libc::c_int = 0;
    let mut arity1: libc::c_int = 0;
    func = Pl_Rd_Atom_Check(name_word);
    arity = Pl_Rd_Integer_Check(arity_word) as libc::c_int;
    func1 = Pl_Pred_Without_Aux(func, arity, &mut arity1);
    return (Pl_Un_Atom_Check(func1, name1_word) != 0
        && Pl_Un_Integer_Check(arity1 as PlLong, arity1_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Make_Aux_Name_4(
    mut name_word: WamWord,
    mut arity_word: WamWord,
    mut aux_nb_word: WamWord,
    mut aux_name_word: WamWord,
) -> Bool {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut aux_nb: libc::c_int = 0;
    let mut aux_name: libc::c_int = 0;
    func = Pl_Rd_Atom_Check(name_word);
    arity = Pl_Rd_Integer_Check(arity_word) as libc::c_int;
    aux_nb = Pl_Rd_Integer_Check(aux_nb_word) as libc::c_int;
    aux_name = Pl_Make_Aux_Name(func, arity, aux_nb);
    return Pl_Un_Atom_Check(aux_name, aux_name_word);
}

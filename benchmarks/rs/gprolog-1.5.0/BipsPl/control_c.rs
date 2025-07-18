use ::libc;
extern "C" {
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Lookup_Pred(func: libc::c_int, arity: libc::c_int) -> *mut PredInf;
    fn Pl_Exit_With_Value(ret_val: libc::c_int);
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    static mut pl_sys_var: [PlLong; 0];
    fn X1_246265747765656E5F616C74__a0();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamCont = CodePtr;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Halt_If_No_Top_Level_1(
    mut exit_code_word: WamWord,
) -> WamCont {
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut x: libc::c_int = 0;
    x = Pl_Rd_Integer_Check(exit_code_word) as libc::c_int;
    if *pl_sys_var.as_mut_ptr().offset(10 as libc::c_int as isize)
        == 0 as libc::c_int as libc::c_long
    {
        Pl_Exit_With_Value(x);
    }
    pred = Pl_Lookup_Pred(
        Pl_Create_Atom(
            (if x != 0 {
                b"$top_level_abort\0" as *const u8 as *const libc::c_char
            } else {
                b"$top_level_stop\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
        ),
        0 as libc::c_int,
    );
    if pred.is_null() {
        Pl_Exit_With_Value(x);
    }
    return ::std::mem::transmute::<*mut PlLong, WamCont>((*pred).codep);
}
pub unsafe extern "C" fn Pl_Halt_1(mut exit_code_word: WamWord) {
    Pl_Exit_With_Value(Pl_Rd_Integer_Check(exit_code_word) as libc::c_int);
}
pub unsafe extern "C" fn Pl_Between_3(
    mut l_word: WamWord,
    mut u_word: WamWord,
    mut i_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut l: PlLong = 0;
    let mut u: PlLong = 0;
    let mut i: PlLong = 0;
    l = Pl_Rd_Integer_Check(l_word);
    u = Pl_Rd_Integer_Check(u_word);
    let mut deref_last_word: WamWord = 0;
    word = i_word;
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
        i = Pl_Rd_Integer_Check(word);
        return (i >= l && i <= u) as libc::c_int;
    }
    i_word = word;
    if l > u {
        return 0 as libc::c_int;
    }
    if l < u {
        *pl_reg_bank
            .offset(0 as libc::c_int as isize) = l + 1 as libc::c_int as libc::c_long;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = u;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = i_word;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_246265747765656E5F616C74__a0),
                ),
            ),
            3 as libc::c_int,
        );
    }
    return Pl_Get_Integer(l, i_word);
}
pub unsafe extern "C" fn Pl_Between_Alt_0() {
    let mut l: PlLong = 0;
    let mut u: PlLong = 0;
    let mut i_word: WamWord = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_246265747765656E5F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    l = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    u = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    i_word = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord);
    if l == u {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh0 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    } else {
        *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
            as *mut WamWord) = l + 1 as libc::c_int as libc::c_long;
    }
    Pl_Get_Integer(l, i_word);
}

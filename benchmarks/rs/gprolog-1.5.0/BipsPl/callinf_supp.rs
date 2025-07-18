use ::libc;
extern "C" {
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Atom(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_String(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Set_Bip_Name_2(func_word: WamWord, arity_word: WamWord);
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
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
static mut save_call_info: PlLong = 0;
pub unsafe extern "C" fn Pl_Save_Call_Info_3(
    mut func_word: WamWord,
    mut arity_word: WamWord,
    mut debug_call_word: WamWord,
) {
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut debug_call: Bool = 0;
    func = Pl_Rd_Atom(func_word);
    arity = Pl_Rd_Integer(arity_word) as libc::c_int;
    debug_call = (*Pl_Rd_String(debug_call_word) as libc::c_int == 't' as i32)
        as libc::c_int;
    save_call_info = (((arity as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong)
        << 1 as libc::c_int | debug_call as libc::c_ulong) as PlLong;
}
pub unsafe extern "C" fn Pl_Load_Call_Info_Arg_1(mut arg_no_word: WamWord) {
    let mut arg_no: libc::c_int = 0;
    arg_no = Pl_Rd_Integer(arg_no_word) as libc::c_int;
    *pl_reg_bank
        .offset(
            arg_no as isize,
        ) = ((save_call_info as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Call_Info_Bip_Name_1(mut call_info_word: WamWord) {
    let mut call_info: PlLong = 0;
    let mut func: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    call_info = call_info_word << 0 as libc::c_int >> 3 as libc::c_int
        >> 1 as libc::c_int;
    func = (call_info as PlULong
        & ((1 as libc::c_int as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
    arity = (call_info as PlULong
        >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
    Pl_Set_Bip_Name_2(
        (((func as PlLong) << 3 as libc::c_int) as libc::c_ulong)
            .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord,
        ((arity as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
            as WamWord,
    );
}

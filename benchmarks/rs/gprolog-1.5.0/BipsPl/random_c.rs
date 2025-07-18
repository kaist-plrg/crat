use ::libc;
extern "C" {
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Get_Float(n: libc::c_double, start_word: WamWord) -> Bool;
    fn Pl_M_Set_Seed(n: libc::c_int);
    fn Pl_M_Get_Seed() -> libc::c_int;
    fn Pl_M_Random_Integer(n: libc::c_int) -> libc::c_int;
    fn Pl_M_Random_Float(n: libc::c_double) -> libc::c_double;
    fn Pl_Rd_C_Int_Positive_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Number_Check(start_word: WamWord) -> libc::c_double;
    fn Pl_Check_For_Un_Variable(start_word: WamWord);
    fn Pl_Un_Positive_Check(value: PlLong, start_word: WamWord) -> Bool;
}
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub unsafe extern "C" fn Pl_Set_Seed_1(mut seed_word: WamWord) {
    Pl_M_Set_Seed(Pl_Rd_C_Int_Positive_Check(seed_word));
}
pub unsafe extern "C" fn Pl_Get_Seed_1(mut seed_word: WamWord) -> Bool {
    return Pl_Un_Positive_Check(Pl_M_Get_Seed() as PlLong, seed_word);
}
pub unsafe extern "C" fn Pl_Random_1(mut n_word: WamWord) {
    Pl_Check_For_Un_Variable(n_word);
    Pl_Get_Float(Pl_M_Random_Float(1.0f64), n_word);
}
pub unsafe extern "C" fn Pl_Random_3(
    mut l_word: WamWord,
    mut u_word: WamWord,
    mut n_word: WamWord,
) -> Bool {
    let mut l: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut l1: PlLong = 0;
    let mut u1: PlLong = 0;
    let mut i: PlLong = 0;
    let mut d: libc::c_double = 0.;
    l = Pl_Rd_Number_Check(l_word);
    u = Pl_Rd_Number_Check(u_word);
    Pl_Check_For_Un_Variable(n_word);
    if l >= u {
        return 0 as libc::c_int;
    }
    l1 = l as PlLong;
    u1 = u as PlLong;
    if l1 as libc::c_double == l && u1 as libc::c_double == u {
        i = l1 + Pl_M_Random_Integer((u1 - l1) as libc::c_int) as libc::c_long;
        Pl_Get_Integer(i, n_word);
    } else {
        d = l + Pl_M_Random_Float(u - l);
        Pl_Get_Float(d, n_word);
    }
    return 1 as libc::c_int;
}

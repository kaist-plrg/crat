use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_String_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Check_For_Un_Atom(start_word: WamWord);
    fn Pl_Un_String_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    static mut pl_le_prompt: *mut libc::c_char;
    fn Pl_LE_Compl_Add_Word(
        word: *mut libc::c_char,
        word_length: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_LE_Compl_Init_Match(
        prefix: *mut libc::c_char,
        nb_match: *mut libc::c_int,
        max_lg: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_LE_Compl_Find_Match(is_last: *mut libc::c_int) -> *mut libc::c_char;
    fn X1_2466696E645F6C696E656469745F636F6D706C6574696F6E5F616C74__a0();
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_le_hook_start: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_put_char: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_char0: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_emit_beep: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_ins_mode: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_screen_size: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_kbd_is_not_empty: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_backd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_forwd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ_str: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_erase: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_set_line_buffering: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_line_buffering: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_flush: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_confirm_box: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_message_box: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_exit_process: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_initialize: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub unsafe extern "C" fn Pl_Get_Linedit_Prompt_1(mut prompt_word: WamWord) -> Bool {
    return Pl_Un_String_Check(pl_le_prompt, prompt_word);
}
pub unsafe extern "C" fn Pl_Set_Linedit_Prompt_1(mut prompt_word: WamWord) {
    pl_le_prompt = (*pl_atom_tbl.offset(Pl_Rd_Atom_Check(prompt_word) as isize)).name;
}
pub unsafe extern "C" fn Pl_Add_Linedit_Completion_1(mut compl_word: WamWord) -> Bool {
    let mut atom: libc::c_int = 0;
    let mut prop: AtomProp = AtomProp {
        length_op_mask_type_0_needs_quote_needs_scan: [0; 3],
        c2rust_padding: [0; 1],
    };
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    atom = Pl_Rd_Atom_Check(compl_word);
    prop = (*pl_atom_tbl.offset(atom as isize)).prop;
    if prop.length() as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if prop.type_0() as libc::c_int != 0 as libc::c_int {
        p = (*pl_atom_tbl.offset(atom as isize)).name;
        while *p != 0 {
            if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int == 0
                && *p as libc::c_int != '_' as i32
            {
                return 0 as libc::c_int;
            }
            p = p.offset(1);
            p;
        }
    }
    Pl_LE_Compl_Add_Word(
        (*pl_atom_tbl.offset(atom as isize)).name,
        prop.length() as libc::c_int,
    );
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Find_Linedit_Completion_2(
    mut prefix_word: WamWord,
    mut compl_word: WamWord,
) -> Bool {
    let mut prefix: *mut libc::c_char = Pl_Rd_String_Check(prefix_word);
    let mut nb_match: libc::c_int = 0;
    let mut max_lg: libc::c_int = 0;
    let mut is_last: libc::c_int = 0;
    let mut compl: *mut libc::c_char = 0 as *mut libc::c_char;
    Pl_Check_For_Un_Atom(compl_word);
    if (Pl_LE_Compl_Init_Match(prefix, &mut nb_match, &mut max_lg)).is_null() {
        return 0 as libc::c_int;
    }
    compl = Pl_LE_Compl_Find_Match(&mut is_last);
    if is_last == 0 {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = compl_word;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_2466696E645F6C696E656469745F636F6D706C6574696F6E5F616C74__a0),
                ),
            ),
            1 as libc::c_int,
        );
    }
    return Pl_Get_Atom(Pl_Create_Atom(compl), compl_word);
}
pub unsafe extern "C" fn Pl_Find_Linedit_Completion_Alt_0() -> Bool {
    let mut compl_word: WamWord = 0;
    let mut is_last: libc::c_int = 0;
    let mut compl: *mut libc::c_char = 0 as *mut libc::c_char;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2466696E645F6C696E656469745F636F6D706C6574696F6E5F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    compl_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    compl = Pl_LE_Compl_Find_Match(&mut is_last);
    if is_last != 0 {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh0 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    }
    return Pl_Get_Atom(Pl_Create_Atom(compl), compl_word);
}

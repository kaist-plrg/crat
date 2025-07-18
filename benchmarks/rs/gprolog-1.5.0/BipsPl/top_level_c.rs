use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Install_Ctrl_C_Handler(
        handler: Option::<unsafe extern "C" fn(libc::c_int) -> PlLong>,
    );
    fn Pl_Call_Prolog(codep: CodePtr) -> libc::c_int;
    fn Pl_Execute_A_Continuation(codep: CodePtr);
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Lookup_Pred(func: libc::c_int, arity: libc::c_int) -> *mut PredInf;
    fn Pl_Exit_With_Value(ret_val: libc::c_int);
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_stm_top_level_input: libc::c_int;
    static mut pl_stm_top_level_output: libc::c_int;
    static mut pl_sys_var: [PlLong; 0];
    fn Pl_Stream_Get_Key(
        pstm: *mut StmInf,
        echo: Bool,
        catch_ctrl_c: Bool,
    ) -> libc::c_int;
    fn Pl_Stream_Putc(c: libc::c_int, pstm: *mut StmInf);
    fn Pl_Stream_Printf(
        pstm: *mut StmInf,
        format: *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn Pl_Stream_Flush(pstm: *mut StmInf);
    fn X0_abort__a0();
    fn X0_break__a0();
}
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PbStk {
    pub buff: [libc::c_int; 8],
    pub ptr: *mut libc::c_int,
    pub nb_elems: libc::c_int,
}
pub type StmFct = Option::<unsafe extern "C" fn() -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stm_lst {
    pub stm: libc::c_int,
    pub next: PStmLst,
}
pub type PStmLst = *mut stm_lst;
pub type StmLst = stm_lst;
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
pub type StmInf = stm_inf;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut buff_save_machine_regs: [WamWord; 4] = [0; 4];
pub unsafe extern "C" fn Pl_Set_Ctrl_C_Handler_0() {
    Pl_Install_Ctrl_C_Handler(
        Some(Ctrl_C_Manager as unsafe extern "C" fn(libc::c_int) -> PlLong),
    );
}
pub unsafe extern "C" fn Pl_Save_Regs_For_Signal() {
    buff_save_machine_regs[0 as libc::c_int as usize] = pl_reg_bank as WamWord;
    buff_save_machine_regs[1 as libc::c_int as usize] = TR as WamWord;
    buff_save_machine_regs[2 as libc::c_int as usize] = B as WamWord;
    buff_save_machine_regs[3 as libc::c_int as usize] = H as WamWord;
}
unsafe extern "C" fn Ctrl_C_Manager(mut from_callback: libc::c_int) -> PlLong {
    let mut current_block: u64;
    let mut pstm: *mut StmInf = *pl_stm_tbl.offset(pl_stm_top_level_output as isize);
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut c: libc::c_int = 0;
    let mut to_execute: CodePtr = None;
    pl_reg_bank = buff_save_machine_regs[0 as libc::c_int as usize] as WamWordP;
    TR = buff_save_machine_regs[1 as libc::c_int as usize] as WamWordP;
    B = buff_save_machine_regs[2 as libc::c_int as usize] as WamWordP;
    H = buff_save_machine_regs[3 as libc::c_int as usize] as WamWordP;
    loop {
        Pl_Stream_Printf(
            pstm,
            b"\nProlog interruption (h for help) ? \0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        Pl_Stream_Flush(pstm);
        c = Pl_Stream_Get_Key(
            *pl_stm_tbl.offset(pl_stm_top_level_input as isize),
            1 as libc::c_int,
            0 as libc::c_int,
        );
        Pl_Stream_Putc('\n' as i32, pstm);
        match c {
            97 => {
                to_execute = Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X0_abort__a0),
                );
                if from_callback != 0 {
                    return ::std::mem::transmute::<CodePtr, PlLong>(to_execute);
                }
                Pl_Execute_A_Continuation(to_execute);
                break;
            }
            98 => {
                Pl_Call_Prolog(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(X0_break__a0),
                    ),
                );
                continue;
            }
            99 => {
                break;
            }
            101 => {
                Pl_Exit_With_Value(0 as libc::c_int);
                current_block = 2377605650710193812;
            }
            116 => {
                current_block = 2377605650710193812;
            }
            100 => {
                current_block = 546479973304185255;
            }
            _ => {
                current_block = 16005334357292621435;
            }
        }
        match current_block {
            2377605650710193812 => {
                current_block = 546479973304185255;
            }
            _ => {}
        }
        match current_block {
            546479973304185255 => {
                if *pl_sys_var.as_mut_ptr().offset(13 as libc::c_int as isize) != 0 {
                    pred = Pl_Lookup_Pred(
                        Pl_Create_Atom(
                            (if c == 't' as i32 {
                                b"trace\0" as *const u8 as *const libc::c_char
                            } else {
                                b"debug\0" as *const u8 as *const libc::c_char
                            }) as *mut libc::c_char,
                        ),
                        0 as libc::c_int,
                    );
                    if pred.is_null() {
                        Pl_Fatal_Error(
                            b"top_level_c: debug/trace not found\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    Pl_Call_Prolog(
                        ::std::mem::transmute::<*mut PlLong, CodePtr>((*pred).codep),
                    );
                    break;
                }
            }
            _ => {}
        }
        Pl_Stream_Printf(
            pstm,
            b"   a  abort        b  break\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        Pl_Stream_Printf(
            pstm,
            b"   c  continue     e  exit\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if *pl_sys_var.as_mut_ptr().offset(13 as libc::c_int as isize) != 0 {
            Pl_Stream_Printf(
                pstm,
                b"   d  debug        t  trace\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        Pl_Stream_Printf(
            pstm,
            b"  h/? help\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    return 0 as libc::c_int as PlLong;
}

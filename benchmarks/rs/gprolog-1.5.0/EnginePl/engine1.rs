use ::libc;
use std::arch::asm;
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamWordP = *mut WamWord;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_ensure_reserved: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub unsafe extern "C" fn Pl_Call_Compiled(mut codep: CodePtr) {
    let mut reserved_stack_space: [WamWord; 1024] = [0; 1024];
    pl_ensure_reserved = reserved_stack_space.as_mut_ptr();
    asm!("andq $0xfffffffffffffff0,%rsp", options(preserves_flags, att_syntax));
    asm!("addq $8,%rsp", options(preserves_flags, att_syntax));
    ::std::mem::transmute::<_, fn()>((Some(codep.unwrap())).unwrap())();
}

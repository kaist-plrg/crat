use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Lookup_Pred(func: libc::c_int, arity: libc::c_int) -> *mut PredInf;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Callable_Check(
        start_word: WamWord,
        func: *mut libc::c_int,
        arity: *mut libc::c_int,
    ) -> *mut WamWord;
    fn Pl_BC_Emulate_Pred(func: libc::c_int, dyn_0: *mut DynPInf) -> WamCont;
    static mut pl_representation_max_arity: libc::c_int;
    fn Pl_Set_C_Bip_Name(func_str: *mut libc::c_char, arity: libc::c_int);
    fn Pl_Unset_C_Bip_Name();
    fn Pl_Err_Representation(atom_flag: libc::c_int);
    fn X1_2463616C6C5F696E7465726E616C__a2();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamCont = CodePtr;
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
pub type DynStamp = PlULong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynpinf {
    pub seq_chain: D2ChHdr,
    pub var_ind_chain: D2ChHdr,
    pub atm_htbl: *mut libc::c_char,
    pub int_htbl: *mut libc::c_char,
    pub lst_ind_chain: D2ChHdr,
    pub stc_htbl: *mut libc::c_char,
    pub arity: libc::c_int,
    pub count_a: libc::c_int,
    pub count_z: libc::c_int,
    pub first_erased_cl: DynCInfP,
    pub next_dyn_with_erase: DynPInfP,
}
pub type DynPInfP = *mut dynpinf;
pub type DynCInfP = *mut dyncinf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dyncinf {
    pub seq_chain: D2ChCell,
    pub ind_chain: D2ChCell,
    pub dyn_0: DynPInfP,
    pub p_ind_hdr: *mut D2ChHdr,
    pub p_ind_htbl: *mut *mut libc::c_char,
    pub cl_no: libc::c_int,
    pub pl_file: libc::c_int,
    pub erase_stamp: DynStamp,
    pub next_erased_cl: DynCInfP,
    pub byte_code: *mut libc::c_uint,
    pub term_size: libc::c_int,
    pub term_word: WamWord,
    pub head_word: WamWord,
    pub body_word: WamWord,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct D2ChHdr {
    pub first: DynCInfP,
    pub last: DynCInfP,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct D2ChCell {
    pub next: DynCInfP,
    pub prev: DynCInfP,
}
pub type DynPInf = dynpinf;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
static mut atom_call_with_args: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Call_Closure(
    mut atom_bip: libc::c_int,
    mut arity_rest: libc::c_int,
) -> WamCont {
    let mut func: libc::c_int = 0;
    let mut arity_clos: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut w: *mut WamWord = 0 as *mut WamWord;
    let mut i: libc::c_int = 0;
    Pl_Set_C_Bip_Name(
        (*pl_atom_tbl.offset(atom_bip as isize)).name,
        1 as libc::c_int + arity_rest,
    );
    if atom_bip == atom_call_with_args {
        func = Pl_Rd_Atom_Check(*pl_reg_bank.offset(0 as libc::c_int as isize));
        arity_clos = 0 as libc::c_int;
    } else {
        arg_adr = Pl_Rd_Callable_Check(
            *pl_reg_bank.offset(0 as libc::c_int as isize),
            &mut func,
            &mut arity_clos,
        );
    }
    arity = arity_clos + arity_rest;
    if arity > 256 as libc::c_int - 1 as libc::c_int {
        Pl_Err_Representation(pl_representation_max_arity);
    }
    Pl_Unset_C_Bip_Name();
    pred = Pl_Lookup_Pred(func, arity);
    if pred.is_null() || (*pred).prop & 32 as libc::c_int != 0 {
        if arity > 0 as libc::c_int {
            w = H;
            *pl_reg_bank
                .offset(
                    0 as libc::c_int as isize,
                ) = (w as PlLong as libc::c_ulong)
                .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
            let fresh0 = w;
            w = w.offset(1);
            *fresh0 = ((arity as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong)
                | func as libc::c_ulong) as WamWord;
            loop {
                let fresh1 = arity_clos;
                arity_clos = arity_clos - 1;
                if !(fresh1 > 0 as libc::c_int) {
                    break;
                }
                let fresh2 = arg_adr;
                arg_adr = arg_adr.offset(1);
                let fresh3 = w;
                w = w.offset(1);
                *fresh3 = *fresh2;
            }
            i = 1 as libc::c_int;
            while i <= arity_rest {
                let fresh4 = w;
                w = w.offset(1);
                *fresh4 = *pl_reg_bank.offset(i as isize);
                i += 1;
                i;
            }
            H = w;
        }
        *pl_reg_bank
            .offset(
                1 as libc::c_int as isize,
            ) = (((((arity_rest + 1 as libc::c_int) as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong)
            | atom_bip as libc::c_ulong) << 1 as libc::c_int
            | 1 as libc::c_int as libc::c_ulong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord;
        return ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463616C6C5F696E7465726E616C__a2),
            ),
        );
    }
    if arity_clos != 1 as libc::c_int {
        memmove(
            &mut *pl_reg_bank.offset(arity_clos as isize) as *mut WamWord
                as *mut libc::c_void,
            &mut *pl_reg_bank.offset(1 as libc::c_int as isize) as *mut WamWord
                as *const libc::c_void,
            (::std::mem::size_of::<WamWord>() as libc::c_ulong)
                .wrapping_mul(arity_rest as libc::c_ulong),
        );
    }
    w = &mut *pl_reg_bank.offset(0 as libc::c_int as isize) as *mut WamWord;
    loop {
        let fresh5 = arity_clos;
        arity_clos = arity_clos - 1;
        if !(fresh5 > 0 as libc::c_int) {
            break;
        }
        let fresh6 = arg_adr;
        arg_adr = arg_adr.offset(1);
        let fresh7 = w;
        w = w.offset(1);
        *fresh7 = *fresh6;
    }
    if (*pred).prop & 1 as libc::c_int != 0 {
        return ::std::mem::transmute::<*mut PlLong, WamCont>((*pred).codep);
    }
    return Pl_BC_Emulate_Pred(func, (*pred).dyn_0 as *mut DynPInf);
}

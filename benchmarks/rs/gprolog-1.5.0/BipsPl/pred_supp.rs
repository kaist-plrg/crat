use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut pl_glob_buff: [libc::c_char; 0];
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn Pl_BC_Emit_Inst_Execute_Native(
        func: libc::c_int,
        arity: libc::c_int,
        codep: *mut PlLong,
    );
    fn Pl_BC_Stop_Emit_0();
    fn Pl_BC_Start_Emit_0();
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
pub type WamWord = intptr_t;
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
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Detect_If_Aux_Name(
    mut func: libc::c_int,
) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = (*pl_atom_tbl.offset(func as isize)).name;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    if *str as libc::c_int != '$' as i32
        || {
            p = strstr(str, b"_$aux\0" as *const u8 as *const libc::c_char);
            p.is_null()
        }
    {
        return 0 as *mut libc::c_char;
    }
    q = p
        .offset(::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong as isize)
        .offset(-(1 as libc::c_int as isize));
    if *(*__ctype_b_loc()).offset(*q as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return 0 as *mut libc::c_char;
    }
    loop {
        q = q.offset(1);
        if !(*(*__ctype_b_loc()).offset(*q as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0)
        {
            break;
        }
    }
    if *q as libc::c_int != '\0' as i32 {
        return 0 as *mut libc::c_char;
    }
    loop {
        p = p.offset(-1);
        if !(*(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0 && p > str)
        {
            break;
        }
    }
    if *p as libc::c_int != '/' as i32 {
        return 0 as *mut libc::c_char;
    }
    return p;
}
pub unsafe extern "C" fn Pl_Father_Pred_Of_Aux(
    mut func: libc::c_int,
    mut father_arity: *mut libc::c_int,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    p = Pl_Detect_If_Aux_Name(func);
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    l = p.offset_from((*pl_atom_tbl.offset(func as isize)).name) as libc::c_long
        as libc::c_int;
    *father_arity = strtol(
        p.offset(1 as libc::c_int as isize),
        0 as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
    strcpy(
        pl_glob_buff.as_mut_ptr(),
        ((*pl_atom_tbl.offset(func as isize)).name).offset(1 as libc::c_int as isize),
    );
    *pl_glob_buff
        .as_mut_ptr()
        .offset((l - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    return Pl_Create_Allocate_Atom(pl_glob_buff.as_mut_ptr());
}
pub unsafe extern "C" fn Pl_Pred_Without_Aux(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut arity1: *mut libc::c_int,
) -> libc::c_int {
    let mut func1: libc::c_int = 0;
    func1 = Pl_Father_Pred_Of_Aux(func, arity1);
    if func1 < 0 as libc::c_int {
        *arity1 = arity;
        func1 = func;
    }
    return func1;
}
pub unsafe extern "C" fn Pl_Make_Aux_Name(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut aux_nb: libc::c_int,
) -> libc::c_int {
    func = Pl_Pred_Without_Aux(func, arity, &mut arity);
    sprintf(
        pl_glob_buff.as_mut_ptr(),
        b"$%s/%d%s%d\0" as *const u8 as *const libc::c_char,
        (*pl_atom_tbl.offset(func as isize)).name,
        arity,
        b"_$aux\0" as *const u8 as *const libc::c_char,
        aux_nb,
    );
    return Pl_Create_Allocate_Atom(pl_glob_buff.as_mut_ptr());
}
pub unsafe extern "C" fn Pl_Emit_BC_Execute_Wrapper(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut codep: *mut PlLong,
) {
    Pl_BC_Start_Emit_0();
    Pl_BC_Emit_Inst_Execute_Native(func, arity, codep);
    Pl_BC_Stop_Emit_0();
}

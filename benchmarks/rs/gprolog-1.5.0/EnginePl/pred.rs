use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Hash_Alloc_Table(
        tbl_size: libc::c_int,
        elem_size: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Hash_Insert(
        tbl: *mut libc::c_char,
        elem: *mut libc::c_char,
        replace: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Hash_Find(tbl: *mut libc::c_char, key: PlLong) -> *mut libc::c_char;
    fn Pl_Hash_Delete(tbl: *mut libc::c_char, key: PlLong) -> *mut libc::c_char;
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_atom_true: libc::c_int;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    fn Pl_Extend_Table_If_Needed(hash_tbl: *mut *mut libc::c_char);
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
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
pub static mut pl_pred_tbl: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn Pl_Init_Pred() {
    let mut file: libc::c_int = Pl_Create_Atom(
        b"pred.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut prop: libc::c_int = 1 as libc::c_int | 32 as libc::c_int
        | 128 as libc::c_int;
    pl_pred_tbl = Pl_Hash_Alloc_Table(
        4096 as libc::c_int,
        ::std::mem::size_of::<PredInf>() as libc::c_ulong as libc::c_int,
    );
    Pl_Create_Pred(
        ',' as i32 as libc::c_uchar as libc::c_int,
        2 as libc::c_int,
        file,
        110 as libc::c_int,
        prop,
        0 as *mut PlLong,
    );
    Pl_Create_Pred(
        ';' as i32 as libc::c_uchar as libc::c_int,
        2 as libc::c_int,
        file,
        111 as libc::c_int,
        prop,
        0 as *mut PlLong,
    );
    Pl_Create_Pred(
        Pl_Create_Atom(b"->\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        file,
        112 as libc::c_int,
        prop,
        0 as *mut PlLong,
    );
    Pl_Create_Pred(
        Pl_Create_Atom(
            b"*->\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        file,
        113 as libc::c_int,
        prop,
        0 as *mut PlLong,
    );
    Pl_Create_Pred(
        '!' as i32 as libc::c_uchar as libc::c_int,
        0 as libc::c_int,
        file,
        114 as libc::c_int,
        prop,
        0 as *mut PlLong,
    );
    Pl_Create_Pred(
        Pl_Create_Atom(
            b"fail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        0 as libc::c_int,
        file,
        115 as libc::c_int,
        prop,
        0 as *mut PlLong,
    );
    Pl_Create_Pred(
        pl_atom_true,
        0 as libc::c_int,
        file,
        116 as libc::c_int,
        prop,
        0 as *mut PlLong,
    );
    Pl_Create_Pred(
        Pl_Create_Atom(
            b"call\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        1 as libc::c_int,
        file,
        117 as libc::c_int,
        prop,
        0 as *mut PlLong,
    );
    Pl_Create_Pred(
        Pl_Create_Atom(
            b"catch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        3 as libc::c_int,
        file,
        118 as libc::c_int,
        prop,
        0 as *mut PlLong,
    );
    Pl_Create_Pred(
        Pl_Create_Atom(
            b"throw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        1 as libc::c_int,
        file,
        119 as libc::c_int,
        prop,
        0 as *mut PlLong,
    );
}
pub unsafe extern "C" fn Pl_Create_Pred(
    mut func: libc::c_int,
    mut arity: libc::c_int,
    mut pl_file: libc::c_int,
    mut pl_line: libc::c_int,
    mut prop: libc::c_int,
    mut codep: *mut PlLong,
) -> *mut PredInf {
    let mut pred_info: PredInf = PredInf {
        f_n: 0,
        pl_file: 0,
        pl_line: 0,
        prop: 0,
        codep: 0 as *mut PlLong,
        dyn_0: 0 as *mut PlLong,
    };
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut key: PlLong = ((arity as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong)
        as PlLong;
    if prop & (16 as libc::c_int | 32 as libc::c_int) != 0 {
        prop |= 8 as libc::c_int;
    }
    pred_info.f_n = key;
    pred_info.prop = prop;
    pred_info.pl_file = pl_file;
    pred_info.pl_line = pl_line;
    pred_info.codep = codep;
    pred_info.dyn_0 = 0 as *mut PlLong;
    Pl_Extend_Table_If_Needed(&mut pl_pred_tbl);
    pred = Pl_Hash_Insert(
        pl_pred_tbl,
        &mut pred_info as *mut PredInf as *mut libc::c_char,
        0 as libc::c_int,
    ) as *mut PredInf;
    if prop != (*pred).prop {
        Pl_Fatal_Error(
            b"multifile predicate %s/%d not declared consistently\n    in  %s\n    and %s\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*pl_atom_tbl.offset(func as isize)).name,
            arity,
            (*pl_atom_tbl.offset((*pred).pl_file as isize)).name,
            (*pl_atom_tbl.offset(pl_file as isize)).name,
        );
        (*pred).prop = prop;
    }
    (*pred).pl_file = pl_file;
    (*pred).pl_line = pl_line;
    return pred;
}
pub unsafe extern "C" fn Pl_Lookup_Pred(
    mut func: libc::c_int,
    mut arity: libc::c_int,
) -> *mut PredInf {
    let mut key: PlLong = ((arity as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong)
        as PlLong;
    return Pl_Hash_Find(pl_pred_tbl, key) as *mut PredInf;
}
pub unsafe extern "C" fn Pl_Delete_Pred(mut func: libc::c_int, mut arity: libc::c_int) {
    let mut key: PlLong = ((arity as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong) | func as libc::c_ulong)
        as PlLong;
    Pl_Hash_Delete(pl_pred_tbl, key);
}

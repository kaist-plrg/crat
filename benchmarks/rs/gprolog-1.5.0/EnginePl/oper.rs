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
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
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
pub struct OperInf {
    pub a_t: PlLong,
    pub prec: libc::c_int,
    pub left: libc::c_int,
    pub right: libc::c_int,
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_oper_tbl: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn Pl_Init_Oper() {
    pl_oper_tbl = Pl_Hash_Alloc_Table(
        1024 as libc::c_int,
        ::std::mem::size_of::<OperInf>() as libc::c_ulong as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b":-\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        1200 as libc::c_int,
        1200 as libc::c_int - 1 as libc::c_int,
        1200 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"-->\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        1200 as libc::c_int,
        1200 as libc::c_int - 1 as libc::c_int,
        1200 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b":-\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        0 as libc::c_int,
        1200 as libc::c_int,
        0 as libc::c_int,
        1200 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"?-\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        0 as libc::c_int,
        1200 as libc::c_int,
        0 as libc::c_int,
        1200 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"|\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        1105 as libc::c_int,
        1105 as libc::c_int - 1 as libc::c_int,
        1105 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b";\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        1100 as libc::c_int,
        1100 as libc::c_int - 1 as libc::c_int,
        1100 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"->\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        1050 as libc::c_int,
        1050 as libc::c_int - 1 as libc::c_int,
        1050 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"*->\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        1050 as libc::c_int,
        1050 as libc::c_int - 1 as libc::c_int,
        1050 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        1000 as libc::c_int,
        1000 as libc::c_int - 1 as libc::c_int,
        1000 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"\\+\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        0 as libc::c_int,
        900 as libc::c_int,
        0 as libc::c_int,
        900 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"=\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"\\=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"==\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"\\==\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"@<\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"@>\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"@=<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"@>=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"=..\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"is\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"=:=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"=\\=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"<\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"=<\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b">\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b">=\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        600 as libc::c_int,
        600 as libc::c_int - 1 as libc::c_int,
        600 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"+\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        500 as libc::c_int,
        500 as libc::c_int,
        500 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        500 as libc::c_int,
        500 as libc::c_int,
        500 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"/\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        500 as libc::c_int,
        500 as libc::c_int,
        500 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"\\/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        500 as libc::c_int,
        500 as libc::c_int,
        500 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"*\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"//\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"rem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"mod\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"div\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"<<\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b">>\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int,
        400 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"**\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        200 as libc::c_int,
        200 as libc::c_int - 1 as libc::c_int,
        200 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"^\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        200 as libc::c_int,
        200 as libc::c_int - 1 as libc::c_int,
        200 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"+\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        0 as libc::c_int,
        200 as libc::c_int,
        0 as libc::c_int,
        200 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        0 as libc::c_int,
        200 as libc::c_int,
        0 as libc::c_int,
        200 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        0 as libc::c_int,
        200 as libc::c_int,
        0 as libc::c_int,
        200 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#<=>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        750 as libc::c_int,
        750 as libc::c_int - 1 as libc::c_int,
        750 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#\\<=>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        750 as libc::c_int,
        750 as libc::c_int - 1 as libc::c_int,
        750 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#==>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        740 as libc::c_int,
        740 as libc::c_int - 1 as libc::c_int,
        740 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#\\==>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        740 as libc::c_int,
        740 as libc::c_int - 1 as libc::c_int,
        740 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"##\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        730 as libc::c_int,
        730 as libc::c_int - 1 as libc::c_int,
        730 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#\\/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        730 as libc::c_int,
        730 as libc::c_int,
        730 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#\\\\/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        730 as libc::c_int,
        730 as libc::c_int,
        730 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#/\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        720 as libc::c_int,
        720 as libc::c_int,
        720 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#\\/\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        720 as libc::c_int,
        720 as libc::c_int,
        720 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#\\\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        0 as libc::c_int,
        710 as libc::c_int,
        0 as libc::c_int,
        710 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"#=\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#\\=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"#<\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#=<\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(b"#>\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#>=\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#=#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#\\=#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#<#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#=<#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#>#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
    Pl_Create_Oper(
        Pl_Create_Atom(
            b"#>=#\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        2 as libc::c_int,
        700 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
        700 as libc::c_int - 1 as libc::c_int,
    );
}
pub unsafe extern "C" fn Pl_Create_Oper(
    mut atom_op: libc::c_int,
    mut type_0: libc::c_int,
    mut prec: libc::c_int,
    mut left: libc::c_int,
    mut right: libc::c_int,
) -> *mut OperInf {
    let mut oper_info: OperInf = OperInf {
        a_t: 0,
        prec: 0,
        left: 0,
        right: 0,
    };
    let mut oper: *mut OperInf = 0 as *mut OperInf;
    Pl_Extend_Table_If_Needed(&mut pl_oper_tbl);
    oper_info
        .a_t = ((atom_op as PlULong) << 2 as libc::c_int | type_0 as libc::c_ulong)
        as PlLong;
    oper_info.prec = prec;
    oper_info.left = left;
    oper_info.right = right;
    oper = Pl_Hash_Insert(
        pl_oper_tbl,
        &mut oper_info as *mut OperInf as *mut libc::c_char,
        1 as libc::c_int,
    ) as *mut OperInf;
    let ref mut fresh0 = (*pl_atom_tbl.offset(atom_op as isize)).prop;
    (*fresh0)
        .set_op_mask(
            (*fresh0).op_mask() | ((1 as libc::c_int) << type_0) as libc::c_uint,
        );
    return oper;
}
pub unsafe extern "C" fn Pl_Lookup_Oper(
    mut atom_op: libc::c_int,
    mut type_0: libc::c_int,
) -> *mut OperInf {
    if ((*pl_atom_tbl.offset(atom_op as isize)).prop).op_mask() as libc::c_int
        & (1 as libc::c_int) << type_0 == 0
    {
        return 0 as *mut OperInf;
    }
    return Pl_Hash_Find(
        pl_oper_tbl,
        ((atom_op as PlULong) << 2 as libc::c_int | type_0 as libc::c_ulong) as PlLong,
    ) as *mut OperInf;
}
pub unsafe extern "C" fn Pl_Lookup_Oper_Any_Type(
    mut atom_op: libc::c_int,
) -> *mut OperInf {
    let mut op_mask: libc::c_int = ((*pl_atom_tbl.offset(atom_op as isize)).prop)
        .op_mask() as libc::c_int;
    if op_mask & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        return Pl_Hash_Find(
            pl_oper_tbl,
            ((atom_op as PlULong) << 2 as libc::c_int
                | 0 as libc::c_int as libc::c_ulong) as PlLong,
        ) as *mut OperInf;
    }
    if op_mask & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        return Pl_Hash_Find(
            pl_oper_tbl,
            ((atom_op as PlULong) << 2 as libc::c_int
                | 2 as libc::c_int as libc::c_ulong) as PlLong,
        ) as *mut OperInf;
    }
    if op_mask & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        return Pl_Hash_Find(
            pl_oper_tbl,
            ((atom_op as PlULong) << 2 as libc::c_int
                | 1 as libc::c_int as libc::c_ulong) as PlLong,
        ) as *mut OperInf;
    }
    return 0 as *mut OperInf;
}
pub unsafe extern "C" fn Pl_Delete_Oper(
    mut atom_op: libc::c_int,
    mut type_0: libc::c_int,
) -> *mut OperInf {
    let mut key: PlLong = ((atom_op as PlULong) << 2 as libc::c_int
        | type_0 as libc::c_ulong) as PlLong;
    let ref mut fresh1 = (*pl_atom_tbl.offset(atom_op as isize)).prop;
    (*fresh1)
        .set_op_mask(
            (*fresh1).op_mask() & !((1 as libc::c_int) << type_0) as libc::c_uint,
        );
    return Pl_Hash_Delete(pl_oper_tbl, key) as *mut OperInf;
}

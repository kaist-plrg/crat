use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Hash_First(tbl: *mut libc::c_char, scan: *mut HashScan) -> *mut libc::c_char;
    fn Pl_Hash_Next(scan: *mut HashScan) -> *mut libc::c_char;
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_atom_curly_brackets: libc::c_int;
    static mut pl_oper_tbl: *mut libc::c_char;
    fn Pl_Create_Oper(
        atom_op: libc::c_int,
        type_0: libc::c_int,
        prec: libc::c_int,
        left: libc::c_int,
        right: libc::c_int,
    ) -> *mut OperInf;
    fn Pl_Lookup_Oper(atom_op: libc::c_int, type_0: libc::c_int) -> *mut OperInf;
    fn Pl_Delete_Oper(atom_op: libc::c_int, type_0: libc::c_int) -> *mut OperInf;
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    static mut pl_type_atom: libc::c_int;
    static mut pl_domain_operator_priority: libc::c_int;
    static mut pl_domain_operator_specifier: libc::c_int;
    static mut pl_permission_operation_create: libc::c_int;
    static mut pl_permission_operation_modify: libc::c_int;
    static mut pl_permission_type_operator: libc::c_int;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn Pl_Err_Permission(atom_oper: libc::c_int, atom_perm: libc::c_int, term: WamWord);
    fn X1_2463757272656E745F6F705F616C74__a0();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashScan {
    pub endt: *mut libc::c_char,
    pub cur_t: *mut libc::c_char,
    pub cur_p: *mut libc::c_char,
}
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
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut atom_specif_tbl: [libc::c_int; 7] = [0; 7];
pub unsafe extern "C" fn Pl_Op_3(
    mut prec_word: WamWord,
    mut specif_word: WamWord,
    mut oper_word: WamWord,
) {
    let mut atom_op: libc::c_int = 0;
    let mut prec: PlLong = 0;
    let mut atom_specif: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    atom_op = Pl_Rd_Atom_Check(oper_word);
    prec = Pl_Rd_Integer_Check(prec_word);
    if prec < 0 as libc::c_int as libc::c_long
        || prec > 1200 as libc::c_int as libc::c_long
    {
        Pl_Err_Domain(pl_domain_operator_priority, prec_word);
    }
    atom_specif = Pl_Rd_Atom_Check(specif_word);
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int && atom_specif != atom_specif_tbl[i as usize] {
        i += 1;
        i;
    }
    match i {
        0 | 1 => {
            type_0 = 0 as libc::c_int;
            left = 0 as libc::c_int;
            right = (if i == 0 as libc::c_int {
                prec - 1 as libc::c_int as libc::c_long
            } else {
                prec
            }) as libc::c_int;
        }
        2 | 3 => {
            type_0 = 1 as libc::c_int;
            left = (if i == 2 as libc::c_int {
                prec - 1 as libc::c_int as libc::c_long
            } else {
                prec
            }) as libc::c_int;
            right = 0 as libc::c_int;
        }
        4 | 5 | 6 => {
            type_0 = 2 as libc::c_int;
            left = (if i == 4 as libc::c_int || i == 5 as libc::c_int {
                prec - 1 as libc::c_int as libc::c_long
            } else {
                prec
            }) as libc::c_int;
            right = (if i == 4 as libc::c_int || i == 6 as libc::c_int {
                prec - 1 as libc::c_int as libc::c_long
            } else {
                prec
            }) as libc::c_int;
        }
        _ => {
            Pl_Err_Domain(pl_domain_operator_specifier, specif_word);
            right = 0 as libc::c_int;
            left = right;
            type_0 = left;
        }
    }
    if type_0 != 0 as libc::c_int
        && ((*pl_atom_tbl.offset(atom_op as isize)).prop).op_mask() as libc::c_int
            & (1 as libc::c_int)
                << (if type_0 == 1 as libc::c_int {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                }) != 0
        || atom_op == '|' as i32 as libc::c_uchar as libc::c_int
            && (type_0 != 2 as libc::c_int
                || prec > 0 as libc::c_int as libc::c_long
                    && prec <= 1000 as libc::c_int as libc::c_long)
        || (atom_op == 256 as libc::c_int || atom_op == pl_atom_curly_brackets)
    {
        Pl_Err_Permission(
            pl_permission_operation_create,
            pl_permission_type_operator,
            oper_word,
        );
    }
    if atom_op == ',' as i32 as libc::c_uchar as libc::c_int {
        Pl_Err_Permission(
            pl_permission_operation_modify,
            pl_permission_type_operator,
            oper_word,
        );
    }
    if prec > 0 as libc::c_int as libc::c_long {
        Pl_Create_Oper(atom_op, type_0, prec as libc::c_int, left, right);
    } else {
        Pl_Delete_Oper(atom_op, type_0);
    };
}
pub unsafe extern "C" fn Pl_Current_Op_3(
    mut prec_word: WamWord,
    mut specif_word: WamWord,
    mut oper_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut scan: HashScan = HashScan {
        endt: 0 as *mut libc::c_char,
        cur_t: 0 as *mut libc::c_char,
        cur_p: 0 as *mut libc::c_char,
    };
    let mut prec: PlLong = 0;
    let mut atom_specif: libc::c_int = 0;
    let mut oper: *mut OperInf = 0 as *mut OperInf;
    let mut atom: libc::c_int = 0;
    let mut op_mask: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = oper_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_atom, word);
    }
    oper_word = word;
    let mut deref_last_word_0: WamWord = 0;
    word = prec_word;
    loop {
        deref_last_word_0 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_0) {
            break;
        }
    }
    prec = word << 0 as libc::c_int >> 3 as libc::c_int;
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
            || prec < 0 as libc::c_int as libc::c_long
            || prec > 1200 as libc::c_int as libc::c_long)
    {
        Pl_Err_Domain(pl_domain_operator_priority, word);
    }
    prec_word = word;
    let mut deref_last_word_1: WamWord = 0;
    word = specif_word;
    loop {
        deref_last_word_1 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_1) {
            break;
        }
    }
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        atom_specif = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
        i = 0 as libc::c_int;
        while i < 7 as libc::c_int && atom_specif != atom_specif_tbl[i as usize] {
            i += 1;
            i;
        }
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
        && (tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong
            || i >= 7 as libc::c_int)
    {
        Pl_Err_Domain(pl_domain_operator_specifier, specif_word);
    }
    specif_word = word;
    if oper_word as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x3 as libc::c_int as PlULong
    {
        atom = (oper_word as PlULong >> 3 as libc::c_int) as libc::c_int;
        op_mask = ((*pl_atom_tbl.offset(atom as isize)).prop).op_mask() as libc::c_int;
        if op_mask == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i <= 1 as libc::c_int {
            if op_mask & (1 as libc::c_int) << i != 0 {
                break;
            }
            i += 1;
            i;
        }
        op_mask &= !((1 as libc::c_int) << i);
        if op_mask != 0 {
            *pl_reg_bank.offset(0 as libc::c_int as isize) = prec_word;
            *pl_reg_bank.offset(1 as libc::c_int as isize) = specif_word;
            *pl_reg_bank.offset(2 as libc::c_int as isize) = oper_word;
            *pl_reg_bank.offset(3 as libc::c_int as isize) = op_mask as WamWord;
            Pl_Create_Choice_Point(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    CodePtr,
                >(
                    Some(
                        ::std::mem::transmute::<
                            unsafe extern "C" fn() -> (),
                            unsafe extern "C" fn() -> (),
                        >(X1_2463757272656E745F6F705F616C74__a0),
                    ),
                ),
                4 as libc::c_int,
            );
        }
        oper = Pl_Lookup_Oper(atom, i);
    } else {
        oper = Pl_Hash_First(pl_oper_tbl, &mut scan) as *mut OperInf;
        if oper.is_null() {
            return 0 as libc::c_int;
        }
        *pl_reg_bank.offset(0 as libc::c_int as isize) = prec_word;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = specif_word;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = oper_word;
        *pl_reg_bank.offset(3 as libc::c_int as isize) = scan.endt as WamWord;
        *pl_reg_bank.offset(4 as libc::c_int as isize) = scan.cur_t as WamWord;
        *pl_reg_bank.offset(5 as libc::c_int as isize) = scan.cur_p as WamWord;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_2463757272656E745F6F705F616C74__a0),
                ),
            ),
            6 as libc::c_int,
        );
    }
    return (Pl_Get_Integer((*oper).prec as PlLong, prec_word) != 0
        && Pl_Get_Atom(Detect_Oper_Specif(oper), specif_word) != 0
        && Pl_Get_Atom(
            ((*oper).a_t as PlULong >> 2 as libc::c_int) as libc::c_int,
            oper_word,
        ) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Current_Op_Alt_0() -> Bool {
    let mut prec_word: WamWord = 0;
    let mut specif_word: WamWord = 0;
    let mut oper_word: WamWord = 0;
    let mut scan: HashScan = HashScan {
        endt: 0 as *mut libc::c_char,
        cur_t: 0 as *mut libc::c_char,
        cur_p: 0 as *mut libc::c_char,
    };
    let mut oper: *mut OperInf = 0 as *mut OperInf;
    let mut atom: libc::c_int = 0;
    let mut op_mask: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F6F705F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    prec_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    specif_word = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    oper_word = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord);
    if oper_word as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0x3 as libc::c_int as PlULong
    {
        atom = (oper_word as PlULong >> 3 as libc::c_int) as libc::c_int;
        op_mask = *(&mut *B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
            as *mut WamWord) as libc::c_int;
        i = 0 as libc::c_int;
        while i <= 1 as libc::c_int {
            if op_mask & (1 as libc::c_int) << i != 0 {
                break;
            }
            i += 1;
            i;
        }
        oper = Pl_Lookup_Oper(atom, i);
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh0 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    } else {
        scan
            .endt = *(&mut *B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
            as *mut WamWord) as *mut libc::c_char;
        scan
            .cur_t = *(&mut *B.offset((-(9 as libc::c_int) - 4 as libc::c_int) as isize)
            as *mut WamWord) as *mut libc::c_char;
        scan
            .cur_p = *(&mut *B.offset((-(9 as libc::c_int) - 5 as libc::c_int) as isize)
            as *mut WamWord) as *mut libc::c_char;
        oper = Pl_Hash_Next(&mut scan) as *mut OperInf;
        if oper.is_null() {
            B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
            let ref mut fresh1 = *(pl_reg_bank as *mut WamWordP)
                .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
            *fresh1 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
                as *mut *mut WamWord);
            return 0 as libc::c_int;
        }
        *(&mut *B.offset((-(9 as libc::c_int) - 4 as libc::c_int) as isize)
            as *mut WamWord) = scan.cur_t as WamWord;
        *(&mut *B.offset((-(9 as libc::c_int) - 5 as libc::c_int) as isize)
            as *mut WamWord) = scan.cur_p as WamWord;
    }
    return (Pl_Get_Integer((*oper).prec as PlLong, prec_word) != 0
        && Pl_Get_Atom(Detect_Oper_Specif(oper), specif_word) != 0
        && Pl_Get_Atom(
            ((*oper).a_t as PlULong >> 2 as libc::c_int) as libc::c_int,
            oper_word,
        ) != 0) as libc::c_int;
}
unsafe extern "C" fn Detect_Oper_Specif(mut oper: *mut OperInf) -> libc::c_int {
    let mut prec: libc::c_int = (*oper).prec;
    let mut i: libc::c_int = 0;
    match (*oper).a_t as PlULong & 3 as libc::c_int as libc::c_ulong {
        0 => {
            i = if (*oper).right < prec { 0 as libc::c_int } else { 1 as libc::c_int };
        }
        1 => {
            i = if (*oper).left < prec { 2 as libc::c_int } else { 3 as libc::c_int };
        }
        _ => {
            i = if (*oper).left < prec {
                if (*oper).right < prec { 4 as libc::c_int } else { 5 as libc::c_int }
            } else {
                6 as libc::c_int
            };
        }
    }
    return atom_specif_tbl[i as usize];
}

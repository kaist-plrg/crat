use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Realloc_Check(
        ptr: *mut libc::c_char,
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Get_Structure(
        func: libc::c_int,
        arity: libc::c_int,
        start_word: WamWord,
    ) -> Bool;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    static mut pl_domain_g_argument_selector: libc::c_int;
    static mut pl_domain_g_array_index: libc::c_int;
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    static mut pl_type_integer: libc::c_int;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Unset_C_Bip_Name();
    fn Pl_Rd_Callable_Check(
        start_word: WamWord,
        func: *mut libc::c_int,
        arity: *mut libc::c_int,
    ) -> *mut WamWord;
    fn Pl_Copy_Term(dst_adr: *mut WamWord, src_adr: *mut WamWord);
    fn Pl_Term_Size(start_word: WamWord) -> libc::c_int;
    fn Pl_List_Length(start_word: WamWord) -> libc::c_int;
    fn Pl_Set_C_Bip_Name(func_str: *mut libc::c_char, arity: libc::c_int);
    fn Pl_Check_For_Un_Integer(start_word: WamWord);
    fn Pl_Rd_Positive_Check(start_word: WamWord) -> PlLong;
    fn Pl_Copy_Contiguous_Term(dst_adr: *mut WamWord, src_adr: *mut WamWord);
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
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
pub struct GVarElt {
    pub size: libc::c_int,
    pub val: WamWord,
    pub undo: PGUndo,
}
pub type PGUndo = *mut gundo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gundo {
    pub g_elem: *mut GVarElt,
    pub save_size: libc::c_int,
    pub save_val: WamWord,
    pub next: PGUndo,
    pub prev: PGUndo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GTarget {
    pub g_elem: *mut GVarElt,
    pub g_arg: *mut WamWord,
}
pub type GUndo = gundo;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
static mut g_target: GTarget = GTarget {
    g_elem: 0 as *const GVarElt as *mut GVarElt,
    g_arg: 0 as *const WamWord as *mut WamWord,
};
static mut atom_g_array: libc::c_int = 0;
static mut atom_g_array_auto: libc::c_int = 0;
static mut atom_g_array_extend: libc::c_int = 0;
pub unsafe extern "C" fn Pl_Blt_G_Assign(mut x: WamWord, mut y: WamWord) {
    Pl_Set_C_Bip_Name(
        b"g_assign\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    G_Assign(x, y, 0 as libc::c_int, 1 as libc::c_int);
    Pl_Unset_C_Bip_Name();
}
pub unsafe extern "C" fn Pl_Blt_G_Assignb(mut x: WamWord, mut y: WamWord) {
    Pl_Set_C_Bip_Name(
        b"g_assignb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    G_Assign(x, y, 1 as libc::c_int, 1 as libc::c_int);
    Pl_Unset_C_Bip_Name();
}
pub unsafe extern "C" fn Pl_Blt_G_Link(mut x: WamWord, mut y: WamWord) {
    Pl_Set_C_Bip_Name(
        b"g_link\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    G_Assign(x, y, 1 as libc::c_int, 0 as libc::c_int);
    Pl_Unset_C_Bip_Name();
}
pub unsafe extern "C" fn Pl_Blt_G_Read(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"g_read\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    res = G_Read(x, y);
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_G_Array_Size(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"g_array_size\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    res = G_Array_Size(x, y);
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_G_Inc(mut x: WamWord) {
    Pl_Set_C_Bip_Name(
        b"g_inc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    G_Inc_Dec(
        x,
        1 as libc::c_int,
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
    );
    Pl_Unset_C_Bip_Name();
}
pub unsafe extern "C" fn Pl_Blt_G_Inco(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"g_inco\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    res = G_Inc_Dec(
        x,
        1 as libc::c_int,
        y,
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
    );
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_G_Inc_2(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"g_inc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    res = G_Inc_Dec(
        x,
        1 as libc::c_int,
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        y,
    );
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_G_Inc_3(
    mut x: WamWord,
    mut y: WamWord,
    mut z: WamWord,
) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"g_inc\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int,
    );
    res = G_Inc_Dec(x, 1 as libc::c_int, y, z);
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_G_Dec(mut x: WamWord) {
    Pl_Set_C_Bip_Name(
        b"g_dec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    G_Inc_Dec(
        x,
        -(1 as libc::c_int),
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
    );
    Pl_Unset_C_Bip_Name();
}
pub unsafe extern "C" fn Pl_Blt_G_Deco(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"g_deco\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    res = G_Inc_Dec(
        x,
        -(1 as libc::c_int),
        y,
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
    );
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_G_Dec_2(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"g_dec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    res = G_Inc_Dec(
        x,
        -(1 as libc::c_int),
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        y,
    );
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_G_Dec_3(
    mut x: WamWord,
    mut y: WamWord,
    mut z: WamWord,
) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"g_dec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int,
    );
    res = G_Inc_Dec(x, -(1 as libc::c_int), y, z);
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_G_Set_Bit(mut x: WamWord, mut y: WamWord) {
    Pl_Set_C_Bip_Name(
        b"g_set_bit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    G_Set_Bit(x, y);
    Pl_Unset_C_Bip_Name();
}
pub unsafe extern "C" fn Pl_Blt_G_Reset_Bit(mut x: WamWord, mut y: WamWord) {
    Pl_Set_C_Bip_Name(
        b"g_reset_bit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    G_Reset_Bit(x, y);
    Pl_Unset_C_Bip_Name();
}
pub unsafe extern "C" fn Pl_Blt_G_Test_Set_Bit(mut x: WamWord, mut y: WamWord) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"g_test_set_bit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    res = G_Test_Set_Bit(x, y);
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Blt_G_Test_Reset_Bit(
    mut x: WamWord,
    mut y: WamWord,
) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"g_test_reset_bit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    res = G_Test_Reset_Bit(x, y);
    Pl_Unset_C_Bip_Name();
    return res;
}
unsafe extern "C" fn G_Assign(
    mut gvar_word: WamWord,
    mut gval_word: WamWord,
    mut backtrack: Bool,
    mut copy: Bool,
) {
    let mut gt: *mut GTarget = Get_Target_From_Gvar(gvar_word);
    let mut g_elem: *mut GVarElt = (*gt).g_elem;
    let mut g_arg: *mut WamWord = (*gt).g_arg;
    if !g_arg.is_null() {
        if backtrack != 0 {
            Pl_Err_Domain(pl_domain_g_argument_selector, gvar_word);
        }
        G_Assign_Arg(g_elem, g_arg, gval_word);
    } else {
        G_Assign_Element(g_elem, gval_word, backtrack, copy);
    };
}
unsafe extern "C" fn G_Assign_Element(
    mut g_elem: *mut GVarElt,
    mut gval_word: WamWord,
    mut backtrack: Bool,
    mut copy: Bool,
) {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut size: libc::c_int = 0;
    let mut atom: libc::c_int = 0;
    let mut save_size: libc::c_int = 0;
    let mut save_val: WamWord = 0;
    let mut array_op: libc::c_int = 0;
    save_size = (*g_elem).size;
    save_val = (*g_elem).val;
    let mut deref_last_word: WamWord = 0;
    word = gval_word;
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
    if tag_mask as libc::c_ulong != 0x2 as libc::c_int as PlULong {
        current_block = 7274886997594406540;
    } else {
        adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
            as *mut WamWord;
        atom = (*adr.offset(0 as libc::c_int as isize) as PlULong
            & ((1 as libc::c_int as PlULong)
                << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(10 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
        if atom == atom_g_array {
            array_op = 0 as libc::c_int;
            current_block = 2838571290723028321;
        } else if atom == atom_g_array_auto {
            array_op = 1 as libc::c_int;
            current_block = 2838571290723028321;
        } else if atom == atom_g_array_extend {
            array_op = 2 as libc::c_int;
            current_block = 2838571290723028321;
        } else {
            current_block = 7274886997594406540;
        }
        match current_block {
            7274886997594406540 => {}
            _ => {
                G_Assign_Array(g_elem, adr, array_op, backtrack, copy);
                current_block = 4636288331473361484;
            }
        }
    }
    match current_block {
        7274886997594406540 => {
            if backtrack == 0 {
                G_Free_Element(g_elem, 1 as libc::c_int);
            }
            if copy == 0 || tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
                || tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
            {
                if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong
                    && {
                        adr = word as *mut WamWord;
                        adr
                            >= *(pl_reg_bank as *mut WamWordP)
                                .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                    }
                {
                    let mut cur_H: *mut WamWord = H;
                    word = (cur_H as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                    *cur_H = word;
                    H = H.offset(1);
                    H;
                    if adr
                        < *(pl_reg_bank as *mut WamWordP)
                            .offset((256 as libc::c_int + 0 as libc::c_int) as isize)
                        || adr
                            >= *(pl_reg_bank as *mut WamWordP)
                                .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
                            && adr < B
                    {
                        let fresh0 = TR;
                        TR = TR.offset(1);
                        *fresh0 = (adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                            as WamWord;
                    }
                    *adr = word;
                }
                (*g_elem).size = 0 as libc::c_int;
                if tag_mask as libc::c_ulong == 0x5 as libc::c_int as PlULong {
                    word = ((word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
                }
                (*g_elem).val = word;
            } else {
                size = Pl_Term_Size(word);
                adr = Pl_Malloc_Check(
                    (size as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong)
                        as libc::c_uint,
                    b"g_var_inl_c.c\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    604 as libc::c_int,
                ) as *mut WamWord;
                (*g_elem).size = size;
                (*g_elem).val = adr as WamWord;
                Pl_Copy_Term(adr, &mut word);
            }
        }
        _ => {}
    }
    if backtrack != 0 {
        G_Trail_For_Backtrack(g_elem, save_size, save_val);
    }
}
unsafe extern "C" fn G_Assign_Arg(
    mut g_elem: *mut GVarElt,
    mut g_arg: *mut WamWord,
    mut word: WamWord,
) {
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut size: libc::c_int = 0;
    let mut u: *mut GUndo = 0 as *mut GUndo;
    if Pl_Term_Size(*g_arg) == 1 as libc::c_int && Pl_Term_Size(word) == 1 as libc::c_int
    {
        Pl_Copy_Term(g_arg, &mut word);
        u = (*g_elem).undo;
        while !u.is_null() {
            (*u).g_elem = 0 as *mut GVarElt;
            u = (*u).next;
        }
        (*g_elem).undo = 0 as PGUndo;
        return;
    }
    *g_arg = word;
    Pl_Copy_Term(H, (*g_elem).val as *mut WamWord);
    G_Free_Element(g_elem, 1 as libc::c_int);
    size = Pl_Term_Size(*H);
    adr = Pl_Malloc_Check(
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong)
            as libc::c_uint,
        b"g_var_inl_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        651 as libc::c_int,
    ) as *mut WamWord;
    (*g_elem).size = size;
    (*g_elem).val = adr as WamWord;
    Pl_Copy_Contiguous_Term(adr, H);
}
unsafe extern "C" fn G_Assign_Array(
    mut g_elem: *mut GVarElt,
    mut stc_adr: *mut WamWord,
    mut array_op: libc::c_int,
    mut backtrack: Bool,
    mut copy: Bool,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut arity: PlLong = 0;
    let mut same_init_value: Bool = 0;
    let mut init_word: WamWord = 0 as libc::c_int as WamWord;
    let mut lst_word: WamWord = 0 as libc::c_int as WamWord;
    let mut new_size: PlLong = 0;
    let mut size: PlLong = 0;
    let mut p: *mut GVarElt = 0 as *mut GVarElt;
    let mut i: libc::c_int = 0;
    arity = (*stc_adr.offset(0 as libc::c_int as isize) as PlULong
        >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as PlLong;
    let mut deref_last_word: WamWord = 0;
    word = *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
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
    new_size = if tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong {
        Pl_List_Length(word) as libc::c_long
    } else {
        word << 0 as libc::c_int >> 3 as libc::c_int
    };
    if !(new_size > 0 as libc::c_int as libc::c_long
        && (tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
            && arity <= 2 as libc::c_int as libc::c_long
            || tag_mask as libc::c_ulong == 0x1 as libc::c_int as PlULong
                && arity == 1 as libc::c_int as libc::c_long))
    {
        Pl_Err_Domain(
            pl_domain_g_array_index,
            (stc_adr as PlLong as libc::c_ulong)
                .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord,
        );
    }
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        same_init_value = 1 as libc::c_int;
        init_word = (if arity == 1 as libc::c_int as libc::c_long {
            (0 as libc::c_int as PlULong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong
        } else {
            *stc_adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_ulong
        }) as WamWord;
    } else {
        same_init_value = 0 as libc::c_int;
        lst_word = word;
    }
    if array_op == 2 as libc::c_int && (*g_elem).size >= 0 as libc::c_int {
        array_op = 0 as libc::c_int;
    }
    if array_op != 2 as libc::c_int && backtrack == 0 {
        G_Free_Element(g_elem, 1 as libc::c_int);
        (*g_elem).size = 0 as libc::c_int;
    }
    size = -(*g_elem).size as PlLong;
    p = G_Alloc_Array(g_elem, new_size as libc::c_int, backtrack);
    if array_op == 2 as libc::c_int {
        if same_init_value == 0 {
            i = 0 as libc::c_int;
            while (i as libc::c_long) < size {
                Pl_Get_List(lst_word);
                init_word = Pl_Unify_Variable();
                lst_word = Pl_Unify_Variable();
                i += 1;
                i;
            }
        }
        i = size as libc::c_int;
        p = p.offset(size as isize);
    } else {
        i = 0 as libc::c_int;
    }
    while (i as libc::c_long) < new_size {
        if same_init_value == 0 {
            Pl_Get_List(lst_word);
            init_word = Pl_Unify_Variable();
            lst_word = Pl_Unify_Variable();
        }
        (*p).size = 0 as libc::c_int;
        (*p)
            .val = ((0 as libc::c_int as PlULong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord;
        (*p).undo = 0 as PGUndo;
        let fresh1 = p;
        p = p.offset(1);
        G_Assign_Element(fresh1, init_word, 0 as libc::c_int, copy);
        i += 1;
        i;
    }
    if array_op == 1 as libc::c_int {
        if same_init_value == 0 {
            init_word = ((0 as libc::c_int as PlULong) << 3 as libc::c_int
                | 0x7 as libc::c_int as PlULong) as WamWord;
        }
        (*p).size = 0 as libc::c_int;
        (*p)
            .val = ((0 as libc::c_int as PlULong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord;
        (*p).undo = 0 as PGUndo;
        G_Assign_Element(p, init_word, 0 as libc::c_int, copy);
    }
}
unsafe extern "C" fn G_Alloc_Array(
    mut g_elem: *mut GVarElt,
    mut new_size: libc::c_int,
    mut backtrack: Bool,
) -> *mut GVarElt {
    let mut p: *mut GVarElt = 0 as *mut GVarElt;
    let mut p_new_end: *mut GVarElt = 0 as *mut GVarElt;
    let mut src: *mut GVarElt = 0 as *mut GVarElt;
    let mut dst: *mut GVarElt = 0 as *mut GVarElt;
    let mut u: *mut GUndo = 0 as *mut GUndo;
    let mut old_size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    old_size = -(*g_elem).size;
    src = (*g_elem).val as *mut GVarElt;
    if old_size <= 0 as libc::c_int || backtrack != 0 {
        p = Pl_Malloc_Check(
            ((new_size + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<GVarElt>() as libc::c_ulong)
                as libc::c_uint,
            b"g_var_inl_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            784 as libc::c_int,
        ) as *mut GVarElt;
    } else {
        p = Pl_Realloc_Check(
            src as *mut libc::c_char,
            ((new_size + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<GVarElt>() as libc::c_ulong)
                as libc::c_uint,
            b"g_var_inl_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            786 as libc::c_int,
        ) as *mut GVarElt;
    }
    p_new_end = p.offset(new_size as isize);
    if old_size <= 0 as libc::c_int {
        (*p_new_end)
            .size = (-(1 as libc::c_int) as libc::c_uint >> 1 as libc::c_int)
            as libc::c_int;
        (*p_new_end).val = 0 as *mut libc::c_void as WamWord;
        (*p_new_end).undo = 0 as PGUndo;
    } else if backtrack != 0 {
        dst = p;
        i = 0 as libc::c_int;
        while i < old_size {
            let fresh2 = dst;
            dst = dst.offset(1);
            let fresh3 = src;
            src = src.offset(1);
            G_Copy_Element(fresh2, fresh3);
            i += 1;
            i;
        }
        if (*src).size as libc::c_uint
            != -(1 as libc::c_int) as libc::c_uint >> 1 as libc::c_int
        {
            G_Copy_Element(p_new_end, src);
        } else {
            *p_new_end = *src;
        }
    } else {
        dst = p;
        i = 0 as libc::c_int;
        while i < old_size {
            u = (*dst).undo;
            while !u.is_null() {
                (*u).g_elem = dst;
                u = (*u).next;
            }
            dst = dst.offset(1);
            dst;
            i += 1;
            i;
        }
        *p_new_end = *dst;
    }
    (*g_elem).size = -new_size;
    (*g_elem).val = p as WamWord;
    return p;
}
unsafe extern "C" fn Get_Target_From_Gvar(mut gvar_word: WamWord) -> *mut GTarget {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut word1: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut arity: libc::c_int = 0;
    let mut arg_adr: *mut WamWord = 0 as *mut WamWord;
    let mut g_elem: *mut GVarElt = 0 as *mut GVarElt;
    let mut g_end: *mut GVarElt = 0 as *mut GVarElt;
    let mut p: *mut GVarElt = 0 as *mut GVarElt;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut new_size: libc::c_int = 0;
    let mut index: PlLong = 0;
    let mut gt: *mut GTarget = &mut g_target;
    arg_adr = Pl_Rd_Callable_Check(gvar_word, &mut atom, &mut arity);
    if atom == '-' as i32 as libc::c_uchar as libc::c_int && arity == 2 as libc::c_int {
        return Get_Target_From_Selector(arg_adr.offset(-(1 as libc::c_int as isize)));
    }
    g_elem = (*pl_atom_tbl.offset(atom as isize)).info as *mut GVarElt;
    if g_elem.is_null() {
        g_elem = Pl_Malloc_Check(
            ::std::mem::size_of::<GVarElt>() as libc::c_ulong as libc::c_uint,
            b"g_var_inl_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            860 as libc::c_int,
        ) as *mut GVarElt;
        (*g_elem).size = 0 as libc::c_int;
        (*g_elem)
            .val = ((0 as libc::c_int as PlULong) << 3 as libc::c_int
            | 0x7 as libc::c_int as PlULong) as WamWord;
        (*g_elem).undo = 0 as PGUndo;
        let ref mut fresh4 = (*pl_atom_tbl.offset(atom as isize)).info;
        *fresh4 = g_elem as *mut libc::c_void;
    }
    if arity > 0 as libc::c_int && (*g_elem).size >= 0 as libc::c_int {
        current_block = 17312456195979378307;
    } else {
        current_block = 12599329904712511516;
    }
    '_error: loop {
        match current_block {
            17312456195979378307 => {
                Pl_Err_Domain(pl_domain_g_array_index, gvar_word);
                current_block = 12599329904712511516;
            }
            _ => {
                i = 0 as libc::c_int;
                loop {
                    if !(i < arity) {
                        break '_error;
                    }
                    size = (*g_elem).size;
                    word1 = *arg_adr;
                    let mut deref_last_word: WamWord = 0;
                    word = word1;
                    loop {
                        deref_last_word = word;
                        tag_mask = (word as libc::c_ulong
                            & 0x7 as libc::c_int as PlULong) as WamWord;
                        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                            break;
                        }
                        word = *(word as *mut WamWord);
                        if !(word != deref_last_word) {
                            break;
                        }
                    }
                    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong {
                        word = *Get_Int_Addr_From_Gvar(word);
                    }
                    index = word << 0 as libc::c_int >> 3 as libc::c_int;
                    if size >= 0 as libc::c_int
                        || index < 0 as libc::c_int as libc::c_long
                    {
                        current_block = 17312456195979378307;
                        break;
                    }
                    size = -size;
                    if index >= size as libc::c_long {
                        p = (*g_elem).val as *mut GVarElt;
                        g_end = p.offset(size as isize);
                        if (*g_end).size as libc::c_uint
                            == -(1 as libc::c_int) as libc::c_uint >> 1 as libc::c_int
                            || index
                                > ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_long
                        {
                            current_block = 17312456195979378307;
                            break;
                        }
                        new_size = 1 as libc::c_int;
                        while new_size as libc::c_long <= index {
                            new_size <<= 1 as libc::c_int;
                        }
                        p = G_Alloc_Array(g_elem, new_size, 0 as libc::c_int);
                        g_end = p.offset(new_size as isize);
                        p = p.offset(size as isize);
                        j = size;
                        while j < new_size {
                            let fresh5 = p;
                            p = p.offset(1);
                            G_Copy_Element(fresh5, g_end);
                            j += 1;
                            j;
                        }
                    }
                    g_elem = ((*g_elem).val as *mut GVarElt).offset(index as isize);
                    arg_adr = arg_adr.offset(1);
                    arg_adr;
                    i += 1;
                    i;
                }
            }
        }
    }
    (*gt).g_elem = g_elem;
    (*gt).g_arg = 0 as *mut WamWord;
    return gt;
}
unsafe extern "C" fn Get_Target_From_Selector(
    mut stc_adr: *mut WamWord,
) -> *mut GTarget {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut word1: WamWord = 0;
    let mut arg_no: PlLong = 0;
    let mut gt: *mut GTarget = 0 as *mut GTarget;
    gt = Get_Target_From_Gvar(
        *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
    );
    arg_no = Get_Int_From_Word(
        *stc_adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize),
    );
    adr = Get_Term_Addr_From_Target(gt);
    if adr.is_null() {
        current_block = 4081598079686252473;
    } else {
        word1 = *adr;
        let mut deref_last_word: WamWord = 0;
        word = word1;
        loop {
            deref_last_word = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word) {
                break;
            }
        }
        if tag_mask == 2 as libc::c_int as libc::c_long {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            if arg_no < 1 as libc::c_int as libc::c_long
                || arg_no as PlULong
                    > *adr.offset(0 as libc::c_int as isize) as PlULong
                        >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(10 as libc::c_int as libc::c_ulong)
            {
                current_block = 4081598079686252473;
            } else {
                (*gt)
                    .g_arg = &mut *adr
                    .offset(
                        (1 as libc::c_int as libc::c_long + arg_no
                            - 1 as libc::c_int as libc::c_long) as isize,
                    ) as *mut WamWord;
                current_block = 9520865839495247062;
            }
        } else if tag_mask == 1 as libc::c_int as libc::c_long {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            if arg_no < 0 as libc::c_int as libc::c_long {
                current_block = 4081598079686252473;
            } else {
                loop {
                    arg_no -= 1;
                    if !(arg_no != 0) {
                        current_block = 17281240262373992796;
                        break;
                    }
                    let mut deref_last_word_0: WamWord = 0;
                    word = *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
                    loop {
                        deref_last_word_0 = word;
                        tag_mask = (word as libc::c_ulong
                            & 0x7 as libc::c_int as PlULong) as WamWord;
                        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                            break;
                        }
                        word = *(word as *mut WamWord);
                        if !(word != deref_last_word_0) {
                            break;
                        }
                    }
                    if tag_mask != 1 as libc::c_int as libc::c_long {
                        current_block = 4081598079686252473;
                        break;
                    }
                    adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord;
                }
                match current_block {
                    4081598079686252473 => {}
                    _ => {
                        (*gt)
                            .g_arg = &mut *adr.offset(0 as libc::c_int as isize)
                            as *mut WamWord;
                        current_block = 9520865839495247062;
                    }
                }
            }
        } else {
            current_block = 4081598079686252473;
        }
    }
    match current_block {
        4081598079686252473 => {
            Pl_Err_Domain(
                pl_domain_g_argument_selector,
                (stc_adr as PlLong as libc::c_ulong)
                    .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord,
            );
        }
        _ => {}
    }
    return gt;
}
unsafe extern "C" fn Get_Term_Addr_From_Target(mut gt: *mut GTarget) -> *mut WamWord {
    let mut g_elem: *mut GVarElt = (*gt).g_elem;
    if !((*gt).g_arg).is_null() {
        return (*gt).g_arg;
    }
    if (*g_elem).size < 0 as libc::c_int {
        return 0 as *mut WamWord;
    }
    if (*g_elem).size == 0 as libc::c_int {
        return &mut (*g_elem).val as *mut WamWord;
    }
    return (*g_elem).val as *mut WamWord;
}
unsafe extern "C" fn Get_Int_Addr_From_Gvar(mut gvar_word: WamWord) -> *mut WamWord {
    let mut save_g_target: GTarget = g_target;
    let mut gt: *mut GTarget = Get_Target_From_Gvar(gvar_word);
    let mut adr: *mut WamWord = Get_Term_Addr_From_Target(gt);
    g_target = save_g_target;
    if adr.is_null() {
        Pl_Err_Type(
            pl_type_integer,
            (((atom_g_array as PlLong) << 3 as libc::c_int) as libc::c_ulong)
                .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord,
        );
    }
    if *adr as libc::c_ulong & 0x7 as libc::c_int as PlULong
        != 0x7 as libc::c_int as PlULong
    {
        Pl_Err_Type(pl_type_integer, *adr);
    }
    return adr;
}
unsafe extern "C" fn Get_Int_From_Gvar(mut gvar_word: WamWord) -> PlLong {
    return *Get_Int_Addr_From_Gvar(gvar_word);
}
unsafe extern "C" fn Get_Int_From_Word(mut start_word: WamWord) -> PlLong {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = start_word;
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
    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong {
        word = *Get_Int_Addr_From_Gvar(word);
    }
    return word << 0 as libc::c_int >> 3 as libc::c_int;
}
unsafe extern "C" fn G_Free_Element(mut g_elem: *mut GVarElt, mut reinit_undo: Bool) {
    let mut size: libc::c_int = 0;
    let mut p: *mut GVarElt = 0 as *mut GVarElt;
    let mut u: *mut GUndo = 0 as *mut GUndo;
    let mut i: libc::c_int = 0;
    if reinit_undo != 0 {
        u = (*g_elem).undo;
        while !u.is_null() {
            (*u).g_elem = 0 as *mut GVarElt;
            u = (*u).next;
        }
        (*g_elem).undo = 0 as PGUndo;
    }
    size = (*g_elem).size;
    if size == 0 as libc::c_int {
        return;
    }
    if size < 0 as libc::c_int {
        size = -size;
        p = (*g_elem).val as *mut GVarElt;
        i = 0 as libc::c_int;
        while i < size {
            let fresh6 = p;
            p = p.offset(1);
            G_Free_Element(fresh6, reinit_undo);
            i += 1;
            i;
        }
        if (*p).size as libc::c_uint
            != -(1 as libc::c_int) as libc::c_uint >> 1 as libc::c_int
        {
            G_Free_Element(p, reinit_undo);
        }
    }
    free((*g_elem).val as *mut libc::c_char as *mut libc::c_void);
}
unsafe extern "C" fn G_Copy_Element(
    mut dst_g_elem: *mut GVarElt,
    mut src_g_elem: *mut GVarElt,
) {
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut p: *mut GVarElt = 0 as *mut GVarElt;
    let mut size: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    (*dst_g_elem).size = (*src_g_elem).size;
    size = (*dst_g_elem).size;
    (*dst_g_elem).undo = 0 as PGUndo;
    if size == 0 as libc::c_int {
        (*dst_g_elem).val = (*src_g_elem).val;
        return;
    }
    if size < 0 as libc::c_int {
        size = -size;
        p = Pl_Malloc_Check(
            ((size + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<GVarElt>() as libc::c_ulong)
                as libc::c_uint,
            b"g_var_inl_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1130 as libc::c_int,
        ) as *mut GVarElt;
        (*dst_g_elem).val = p as WamWord;
        dst_g_elem = p;
        src_g_elem = (*src_g_elem).val as *mut GVarElt;
        i = 0 as libc::c_int;
        while i < size {
            let fresh7 = dst_g_elem;
            dst_g_elem = dst_g_elem.offset(1);
            let fresh8 = src_g_elem;
            src_g_elem = src_g_elem.offset(1);
            G_Copy_Element(fresh7, fresh8);
            i += 1;
            i;
        }
        if (*src_g_elem).size as libc::c_uint
            == -(1 as libc::c_int) as libc::c_uint >> 1 as libc::c_int
        {
            *dst_g_elem = *src_g_elem;
        } else {
            G_Copy_Element(dst_g_elem, src_g_elem);
        }
        return;
    }
    adr = Pl_Malloc_Check(
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong)
            as libc::c_uint,
        b"g_var_inl_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1150 as libc::c_int,
    ) as *mut WamWord;
    (*dst_g_elem).val = adr as WamWord;
    Pl_Copy_Contiguous_Term(adr, (*src_g_elem).val as *mut WamWord);
}
unsafe extern "C" fn G_Trail_For_Backtrack(
    mut g_elem: *mut GVarElt,
    mut save_size: libc::c_int,
    mut save_val: WamWord,
) {
    let mut arg_frame: [WamWord; 1] = [0; 1];
    let mut u: *mut GUndo = Pl_Malloc_Check(
        ::std::mem::size_of::<GUndo>() as libc::c_ulong as libc::c_uint,
        b"g_var_inl_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1168 as libc::c_int,
    ) as *mut GUndo;
    (*u).g_elem = g_elem;
    (*u).save_size = save_size;
    (*u).save_val = save_val;
    (*u).next = (*g_elem).undo;
    (*u).prev = 0 as PGUndo;
    if !((*u).next).is_null() {
        (*(*u).next).prev = u;
    }
    (*g_elem).undo = u;
    arg_frame[0 as libc::c_int as usize] = u as WamWord;
    let mut s: *mut PlLong = arg_frame.as_mut_ptr() as *mut PlLong;
    let mut d: *mut PlLong = TR as *mut PlLong;
    let mut counter: libc::c_int = 1 as libc::c_int;
    loop {
        let fresh9 = s;
        s = s.offset(1);
        let fresh10 = d;
        d = d.offset(1);
        *fresh10 = *fresh9;
        counter -= 1;
        if !(counter != 0) {
            break;
        }
    }
    TR = TR.offset(1 as libc::c_int as isize);
    let fresh11 = TR;
    TR = TR.offset(1);
    *fresh11 = 1 as libc::c_int as WamWord;
    let fresh12 = TR;
    TR = TR.offset(1);
    *fresh12 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(libc::c_int, *mut WamWord) -> ()>,
        WamWord,
    >(Some(G_Untrail as unsafe extern "C" fn(libc::c_int, *mut WamWord) -> ()));
    let fresh13 = TR;
    TR = TR.offset(1);
    *fresh13 = (0 as libc::c_int as PlULong | 3 as libc::c_int as libc::c_ulong)
        as WamWord;
}
unsafe extern "C" fn G_Untrail(mut n: libc::c_int, mut arg_frame: *mut WamWord) {
    let mut u: *mut GUndo = *arg_frame.offset(0 as libc::c_int as isize) as *mut GUndo;
    let mut g_elem: *mut GVarElt = (*u).g_elem;
    if !g_elem.is_null() {
        G_Free_Element(g_elem, 0 as libc::c_int);
        (*g_elem).size = (*u).save_size;
        (*g_elem).val = (*u).save_val;
    }
    if !((*u).next).is_null() {
        (*(*u).next).prev = (*u).prev;
    }
    if !((*u).prev).is_null() {
        (*(*u).prev).next = (*u).next;
    } else if !g_elem.is_null() {
        (*g_elem).undo = (*u).next;
    }
    free(u as *mut libc::c_void);
}
unsafe extern "C" fn G_Read(mut gvar_word: WamWord, mut gval_word: WamWord) -> Bool {
    let mut gt: *mut GTarget = Get_Target_From_Gvar(gvar_word);
    let mut g_elem: *mut GVarElt = (*gt).g_elem;
    let mut g_arg: *mut WamWord = (*gt).g_arg;
    let mut word: WamWord = 0;
    if !g_arg.is_null() {
        Pl_Copy_Term(H, g_arg);
        word = *H;
        H = H.offset(Pl_Term_Size(word) as isize);
        return Pl_Unify(word, gval_word);
    }
    return G_Read_Element(g_elem, gval_word);
}
unsafe extern "C" fn G_Read_Element(
    mut g_elem: *mut GVarElt,
    mut gval_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut size: libc::c_int = (*g_elem).size;
    let mut p: *mut GVarElt = 0 as *mut GVarElt;
    let mut i: libc::c_int = 0;
    if size == 0 as libc::c_int {
        return Pl_Unify((*g_elem).val, gval_word);
    }
    if size > 0 as libc::c_int {
        Pl_Copy_Contiguous_Term(H, (*g_elem).val as *mut WamWord);
        word = *H;
        H = H.offset(size as isize);
        return Pl_Unify(word, gval_word);
    }
    size = -size;
    p = (*g_elem).val as *mut GVarElt;
    if Pl_Get_Structure(atom_g_array, 1 as libc::c_int, gval_word) == 0 {
        return 0 as libc::c_int;
    }
    gval_word = Pl_Unify_Variable();
    i = 0 as libc::c_int;
    while i < size {
        if Pl_Get_List(gval_word) == 0 {
            return 0 as libc::c_int;
        }
        word = Pl_Unify_Variable();
        gval_word = Pl_Unify_Variable();
        let fresh14 = p;
        p = p.offset(1);
        if G_Read_Element(fresh14, word) == 0 {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return Pl_Get_Nil(gval_word);
}
unsafe extern "C" fn G_Array_Size(
    mut gvar_word: WamWord,
    mut size_word: WamWord,
) -> Bool {
    let mut gt: *mut GTarget = Get_Target_From_Gvar(gvar_word);
    let mut g_elem: *mut GVarElt = (*gt).g_elem;
    let mut g_arg: *mut WamWord = (*gt).g_arg;
    let mut size: libc::c_int = 0;
    Pl_Check_For_Un_Integer(size_word);
    size = (*g_elem).size;
    return (g_arg.is_null() && size < 0 as libc::c_int
        && Pl_Get_Integer(-size as PlLong, size_word) != 0) as libc::c_int;
}
unsafe extern "C" fn G_Inc_Dec(
    mut gvar_word: WamWord,
    mut inc: libc::c_int,
    mut old_word: WamWord,
    mut new_word: WamWord,
) -> Bool {
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut old: PlLong = 0;
    let mut new: PlLong = 0;
    if old_word as libc::c_ulong
        != (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        Pl_Check_For_Un_Integer(old_word);
    }
    if new_word as libc::c_ulong
        != (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        Pl_Check_For_Un_Integer(new_word);
    }
    adr = Get_Int_Addr_From_Gvar(gvar_word);
    old = *adr << 0 as libc::c_int >> 3 as libc::c_int;
    new = old + inc as libc::c_long;
    if old_word as libc::c_ulong
        != (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
        && Pl_Get_Integer(old, old_word) == 0
    {
        return 0 as libc::c_int;
    }
    *adr = ((new as PlULong) << 3 as libc::c_int | 0x7 as libc::c_int as PlULong)
        as WamWord;
    if new_word as libc::c_ulong
        != (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
        && Pl_Get_Integer(new, new_word) == 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn G_Set_Bit(mut gvar_word: WamWord, mut bit_word: WamWord) {
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut bit: libc::c_int = (Pl_Rd_Positive_Check(bit_word)
        % 61 as libc::c_int as libc::c_long) as libc::c_int;
    let mut mask: PlULong = 0;
    adr = Get_Int_Addr_From_Gvar(gvar_word);
    mask = ((1 as libc::c_int) << bit + 3 as libc::c_int) as PlULong;
    *adr = (*adr as libc::c_ulong | mask) as WamWord;
}
unsafe extern "C" fn G_Reset_Bit(mut gvar_word: WamWord, mut bit_word: WamWord) {
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut bit: libc::c_int = (Pl_Rd_Positive_Check(bit_word)
        % 61 as libc::c_int as libc::c_long) as libc::c_int;
    let mut mask: PlULong = 0;
    adr = Get_Int_Addr_From_Gvar(gvar_word);
    mask = ((1 as libc::c_int) << bit + 3 as libc::c_int) as PlULong;
    *adr = (*adr as libc::c_ulong & !mask) as WamWord;
}
unsafe extern "C" fn G_Test_Set_Bit(
    mut gvar_word: WamWord,
    mut bit_word: WamWord,
) -> Bool {
    let mut bit: libc::c_int = (Pl_Rd_Positive_Check(bit_word)
        % 61 as libc::c_int as libc::c_long) as libc::c_int;
    let mut val: PlULong = 0;
    let mut mask: PlULong = 0;
    val = Get_Int_From_Gvar(gvar_word) as PlULong;
    mask = ((1 as libc::c_int) << bit + 3 as libc::c_int) as PlULong;
    return (val & mask != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn G_Test_Reset_Bit(
    mut gvar_word: WamWord,
    mut bit_word: WamWord,
) -> Bool {
    let mut bit: libc::c_int = (Pl_Rd_Positive_Check(bit_word)
        % 61 as libc::c_int as libc::c_long) as libc::c_int;
    let mut mask: PlULong = 0;
    let mut val: PlULong = 0;
    val = Get_Int_From_Gvar(gvar_word) as PlULong;
    mask = ((1 as libc::c_int) << bit + 3 as libc::c_int) as PlULong;
    return (val & mask == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
}

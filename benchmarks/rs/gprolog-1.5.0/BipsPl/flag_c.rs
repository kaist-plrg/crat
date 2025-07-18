use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Hash_Delete_All(tbl: *mut libc::c_char);
    fn Pl_Hash_First(tbl: *mut libc::c_char, scan: *mut HashScan) -> *mut libc::c_char;
    fn Pl_Hash_Next(scan: *mut HashScan) -> *mut libc::c_char;
    fn Pl_Hash_Nb_Elements(tbl: *mut libc::c_char) -> libc::c_int;
    static mut pl_os_argc: libc::c_int;
    static mut pl_os_argv: *mut *mut libc::c_char;
    static mut pl_glob_buff: [libc::c_char; 0];
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_char_conv: [libc::c_char; 0];
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    static mut pl_oper_tbl: *mut libc::c_char;
    fn Pl_Create_Oper(
        atom_op: libc::c_int,
        type_0: libc::c_int,
        prec: libc::c_int,
        left: libc::c_int,
        right: libc::c_int,
    ) -> *mut OperInf;
    fn Pl_M_Absolute_Path_Name(src: *mut libc::c_char) -> *mut libc::c_char;
    fn Pl_Unify(start_u_word: WamWord, start_v_word: WamWord) -> Bool;
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Get_Current_Choice() -> WamWord;
    fn Pl_Cut(b_word: WamWord);
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    fn Pl_Rd_C_Int_Positive_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_String_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Check_For_Un_Atom(start_word: WamWord);
    fn Pl_Check_For_Un_List(start_word: WamWord);
    fn Pl_Un_Integer_Check(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Un_Integer(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Un_Atom_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Un_String_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_Term_Size(start_word: WamWord) -> libc::c_int;
    fn Pl_Copy_Term(dst_adr: *mut WamWord, src_adr: *mut WamWord);
    fn Pl_Copy_Contiguous_Term(dst_adr: *mut WamWord, src_adr: *mut WamWord);
    fn Pl_Os_Error(ret_val: libc::c_int);
    static mut environ: *mut *mut libc::c_char;
    fn X1_24656E7669726F6E5F616C74__a0();
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type FlagInf = flag_inf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flag_inf {
    pub atom_name: libc::c_int,
    pub modifiable: Bool,
    pub type_0: FlagType,
    pub value: PlLong,
    pub fct_get: FlagFctGet,
    pub fct_chk: FlagFctChk,
    pub fct_set: FlagFctSet,
}
pub type FlagFctSet = Option::<unsafe extern "C" fn(FlagInfP, WamWord) -> Bool>;
pub type FlagInfP = *mut flag_inf;
pub type FlagFctChk = Option::<unsafe extern "C" fn(FlagInfP, WamWord, WamWord) -> Bool>;
pub type FlagFctGet = Option::<unsafe extern "C" fn(FlagInfP) -> WamWord>;
pub type FlagType = libc::c_uint;
pub const PF_TYPE_ANY: FlagType = 7;
pub const PF_TYPE_QUOTES: FlagType = 6;
pub const PF_TYPE_ERR: FlagType = 5;
pub const PF_TYPE_ON_OFF: FlagType = 4;
pub const PF_TYPE_BOOL: FlagType = 3;
pub const PF_TYPE_ROUND: FlagType = 2;
pub const PF_TYPE_ATOM: FlagType = 1;
pub const PF_TYPE_INTEGER: FlagType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFOp {
    pub type_0: libc::c_int,
    pub prec: libc::c_int,
    pub left: libc::c_int,
    pub right: libc::c_int,
    pub length: libc::c_int,
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_flag_double_quotes: *mut FlagInf = 0 as *const FlagInf as *mut FlagInf;
pub static mut pl_sys_var: [PlLong; 256] = [0; 256];
pub static mut pl_flag_char_conversion: *mut FlagInf = 0 as *const FlagInf
    as *mut FlagInf;
pub static mut pl_flag_singleton_warning: *mut FlagInf = 0 as *const FlagInf
    as *mut FlagInf;
pub static mut pl_flag_suspicious_warning: *mut FlagInf = 0 as *const FlagInf
    as *mut FlagInf;
pub static mut pl_flag_multifile_warning: *mut FlagInf = 0 as *const FlagInf
    as *mut FlagInf;
pub static mut pl_flag_strict_iso: *mut FlagInf = 0 as *const FlagInf as *mut FlagInf;
pub static mut pl_flag_debug: *mut FlagInf = 0 as *const FlagInf as *mut FlagInf;
pub static mut pl_flag_os_error: *mut FlagInf = 0 as *const FlagInf as *mut FlagInf;
pub static mut pl_flag_syntax_error: *mut FlagInf = 0 as *const FlagInf as *mut FlagInf;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_flag_unknown: *mut FlagInf = 0 as *const FlagInf as *mut FlagInf;
pub static mut pl_flag_back_quotes: *mut FlagInf = 0 as *const FlagInf as *mut FlagInf;
pub unsafe extern "C" fn Pl_Sys_Var_Write_2(mut var_word: WamWord, mut n_word: WamWord) {
    pl_sys_var[Pl_Rd_Integer(var_word) as usize] = Pl_Rd_Integer(n_word);
}
pub unsafe extern "C" fn Pl_Sys_Var_Read_2(
    mut var_word: WamWord,
    mut n_word: WamWord,
) -> Bool {
    return Pl_Get_Integer(pl_sys_var[Pl_Rd_Integer(var_word) as usize], n_word);
}
pub unsafe extern "C" fn Pl_Sys_Var_Inc_1(mut var_word: WamWord) {
    pl_sys_var[Pl_Rd_Integer(var_word) as usize] += 1;
    pl_sys_var[Pl_Rd_Integer(var_word) as usize];
}
pub unsafe extern "C" fn Pl_Sys_Var_Dec_1(mut var_word: WamWord) {
    pl_sys_var[Pl_Rd_Integer(var_word) as usize] -= 1;
    pl_sys_var[Pl_Rd_Integer(var_word) as usize];
}
pub unsafe extern "C" fn Pl_Sys_Var_Set_Bit_2(
    mut var_word: WamWord,
    mut bit_word: WamWord,
) {
    pl_sys_var[Pl_Rd_Integer(var_word) as usize]
        |= ((1 as libc::c_int) << Pl_Rd_Integer(bit_word)) as libc::c_long;
}
pub unsafe extern "C" fn Pl_Sys_Var_Reset_Bit_2(
    mut var_word: WamWord,
    mut bit_word: WamWord,
) {
    pl_sys_var[Pl_Rd_Integer(var_word) as usize]
        &= !((1 as libc::c_int) << Pl_Rd_Integer(bit_word)) as libc::c_long;
}
pub unsafe extern "C" fn Pl_Sys_Var_Get_Bit_3(
    mut var_word: WamWord,
    mut bit_word: WamWord,
    mut value_word: WamWord,
) -> Bool {
    let mut x: libc::c_uint = 0;
    x = (pl_sys_var[Pl_Rd_Integer(var_word) as usize] >> Pl_Rd_Integer(bit_word)
        & 1 as libc::c_int as libc::c_long) as libc::c_uint;
    return Pl_Un_Integer(x as PlLong, value_word);
}
pub unsafe extern "C" fn Pl_Sys_Var_Put_2(
    mut var_word: WamWord,
    mut term_word: WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut sv: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    sv = Pl_Rd_Integer(var_word) as libc::c_int;
    word = pl_sys_var[sv as usize];
    tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        adr = word as *mut WamWord;
        if !adr.is_null() {
            free(adr as *mut libc::c_void);
        }
    }
    let mut deref_last_word: WamWord = 0;
    word = term_word;
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
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        || tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
    {
        pl_sys_var[sv as usize] = word;
        return;
    }
    size = Pl_Term_Size(word);
    adr = Pl_Malloc_Check(
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<WamWord>() as libc::c_ulong)
            as libc::c_uint,
        b"flag_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        443 as libc::c_int,
    ) as *mut WamWord;
    Pl_Copy_Term(adr, &mut word);
    pl_sys_var[sv
        as usize] = (adr as PlLong as libc::c_ulong)
        .wrapping_add(0 as libc::c_int as PlULong) as PlLong;
}
pub unsafe extern "C" fn Pl_Sys_Var_Get_2(
    mut var_word: WamWord,
    mut term_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut size: libc::c_int = 0;
    word = pl_sys_var[Pl_Rd_Integer(var_word) as usize];
    if word as libc::c_ulong & 0x7 as libc::c_int as PlULong
        == 0 as libc::c_int as PlULong
    {
        adr = word as *mut WamWord;
        size = Pl_Term_Size(*adr);
        Pl_Copy_Contiguous_Term(H, adr);
        word = *H;
        H = H.offset(size as isize);
    }
    return Pl_Unify(word, term_word);
}
pub unsafe extern "C" fn Pl_Get_Current_B_1(mut b_word: WamWord) {
    let mut word: WamWord = 0;
    word = Pl_Get_Current_Choice();
    Pl_Unify(word, b_word);
}
pub unsafe extern "C" fn Pl_Set_Current_B_1(mut b_word: WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = b_word;
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
    Pl_Cut(word);
}
pub unsafe extern "C" fn Pl_Write_Pl_State_File(mut file_word: WamWord) -> Bool {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut scan: HashScan = HashScan {
        endt: 0 as *mut libc::c_char,
        cur_t: 0 as *mut libc::c_char,
        cur_p: 0 as *mut libc::c_char,
    };
    let mut oper: *mut OperInf = 0 as *mut OperInf;
    let mut sf_op: SFOp = SFOp {
        type_0: 0,
        prec: 0,
        left: 0,
        right: 0,
        length: 0,
    };
    let mut c: libc::c_int = 0;
    static mut cv: [libc::c_char; 2] = [0; 2];
    file = (*pl_atom_tbl.offset(Pl_Rd_Atom_Check(file_word) as isize)).name;
    file = Pl_M_Absolute_Path_Name(file);
    f = fopen(file, b"wb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    i = Pl_Hash_Nb_Elements(pl_oper_tbl);
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    oper = Pl_Hash_First(pl_oper_tbl, &mut scan) as *mut OperInf;
    while !oper.is_null() {
        sf_op
            .type_0 = ((*oper).a_t as PlULong & 3 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        sf_op.prec = (*oper).prec;
        sf_op.left = (*oper).left;
        sf_op.right = (*oper).right;
        sf_op
            .length = ((*pl_atom_tbl
            .offset(((*oper).a_t as PlULong >> 2 as libc::c_int) as isize))
            .prop)
            .length() as libc::c_int;
        fwrite(
            &mut sf_op as *mut SFOp as *const libc::c_void,
            ::std::mem::size_of::<SFOp>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            f,
        ) != 1 as libc::c_int as libc::c_ulong;
        fwrite(
            (*pl_atom_tbl.offset(((*oper).a_t as PlULong >> 2 as libc::c_int) as isize))
                .name as *const libc::c_void,
            sf_op.length as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            f,
        ) != 1 as libc::c_int as libc::c_ulong;
        oper = Pl_Hash_Next(&mut scan) as *mut OperInf;
    }
    i = (*pl_flag_double_quotes).value as libc::c_int;
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    i = (*pl_flag_back_quotes).value as libc::c_int;
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    i = (*pl_flag_char_conversion).value as libc::c_int;
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    i = (*pl_flag_singleton_warning).value as libc::c_int;
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    i = (*pl_flag_suspicious_warning).value as libc::c_int;
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    i = (*pl_flag_multifile_warning).value as libc::c_int;
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    i = (*pl_flag_strict_iso).value as libc::c_int;
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    i = pl_sys_var[20 as libc::c_int as usize] as libc::c_int;
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    c = 0 as libc::c_int;
    while c < 256 as libc::c_int {
        if *pl_char_conv.as_mut_ptr().offset(c as isize) as libc::c_int != c {
            cv[0 as libc::c_int as usize] = c as libc::c_char;
            cv[1 as libc::c_int
                as usize] = *pl_char_conv.as_mut_ptr().offset(c as isize);
            fwrite(
                &mut cv as *mut [libc::c_char; 2] as *const libc::c_void,
                2 as libc::c_int as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                f,
            ) != 1 as libc::c_int as libc::c_ulong;
        }
        c += 1;
        c;
    }
    cv[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    cv[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    fwrite(
        &mut cv as *mut [libc::c_char; 2] as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    fclose(f);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Read_Pl_State_File(mut file_word: WamWord) -> Bool {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut sf_op: SFOp = SFOp {
        type_0: 0,
        prec: 0,
        left: 0,
        right: 0,
        length: 0,
    };
    let mut c: libc::c_int = 0;
    let mut cv: [libc::c_char; 2] = [0; 2];
    file = (*pl_atom_tbl.offset(Pl_Rd_Atom_Check(file_word) as isize)).name;
    file = Pl_M_Absolute_Path_Name(file);
    f = fopen(file, b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        Pl_Os_Error(-(1 as libc::c_int));
        return 0 as libc::c_int;
    }
    Pl_Hash_Delete_All(pl_oper_tbl);
    fread(
        &mut i as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    loop {
        let fresh0 = i;
        i = i - 1;
        if !(fresh0 != 0) {
            break;
        }
        fread(
            &mut sf_op as *mut SFOp as *mut libc::c_void,
            ::std::mem::size_of::<SFOp>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            f,
        ) != 1 as libc::c_int as libc::c_ulong;
        fread(
            pl_glob_buff.as_mut_ptr() as *mut libc::c_void,
            sf_op.length as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            f,
        ) != 1 as libc::c_int as libc::c_ulong;
        *pl_glob_buff
            .as_mut_ptr()
            .offset(sf_op.length as isize) = '\0' as i32 as libc::c_char;
        Pl_Create_Oper(
            Pl_Create_Allocate_Atom(pl_glob_buff.as_mut_ptr()),
            sf_op.type_0,
            sf_op.prec,
            sf_op.left,
            sf_op.right,
        );
    }
    fread(
        &mut i as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    (*pl_flag_double_quotes).value = i as PlLong;
    fread(
        &mut i as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    (*pl_flag_back_quotes).value = i as PlLong;
    fread(
        &mut i as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    (*pl_flag_char_conversion).value = i as PlLong;
    fread(
        &mut i as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    (*pl_flag_singleton_warning).value = i as PlLong;
    fread(
        &mut i as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    (*pl_flag_suspicious_warning).value = i as PlLong;
    fread(
        &mut i as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    (*pl_flag_multifile_warning).value = i as PlLong;
    fread(
        &mut i as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    (*pl_flag_strict_iso).value = i as PlLong;
    fread(
        &mut i as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    ) != 1 as libc::c_int as libc::c_ulong;
    pl_sys_var[20 as libc::c_int as usize] = i as PlLong;
    loop {
        fread(
            &mut cv as *mut [libc::c_char; 2] as *mut libc::c_void,
            2 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            f,
        ) != 1 as libc::c_int as libc::c_ulong;
        c = cv[0 as libc::c_int as usize] as libc::c_int;
        if c == 0 as libc::c_int
            && cv[1 as libc::c_int as usize] as libc::c_int == 0 as libc::c_int
        {
            break;
        }
        *pl_char_conv.as_mut_ptr().offset(c as isize) = cv[1 as libc::c_int as usize];
    }
    fclose(f);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Argument_Counter_1(mut n_word: WamWord) -> Bool {
    return Pl_Un_Integer_Check(pl_os_argc as PlLong, n_word);
}
pub unsafe extern "C" fn Pl_Argument_Value_2(
    mut i_word: WamWord,
    mut a_word: WamWord,
) -> Bool {
    let mut i: libc::c_int = 0;
    i = Pl_Rd_C_Int_Positive_Check(i_word);
    if i >= pl_os_argc {
        return 0 as libc::c_int;
    }
    return Pl_Un_Atom_Check(Pl_Create_Atom(*pl_os_argv.offset(i as isize)), a_word);
}
pub unsafe extern "C" fn Pl_Argument_List_1(mut list_word: WamWord) -> Bool {
    let mut i: libc::c_int = 0;
    Pl_Check_For_Un_List(list_word);
    i = 1 as libc::c_int;
    while i < pl_os_argc {
        if Pl_Get_List(list_word) == 0
            || Pl_Unify_Atom(Pl_Create_Atom(*pl_os_argv.offset(i as isize))) == 0
        {
            return 0 as libc::c_int;
        }
        list_word = Pl_Unify_Variable();
        i += 1;
        i;
    }
    return Pl_Get_Nil(list_word);
}
pub unsafe extern "C" fn Pl_Environ_2(
    mut var_name_word: WamWord,
    mut value_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut var_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur_env: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut one_env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lg: libc::c_int = 0;
    Pl_Check_For_Un_Atom(value_word);
    let mut deref_last_word: WamWord = 0;
    word = var_name_word;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        var_name = Pl_Rd_String_Check(word);
        value = getenv(var_name);
        return (!value.is_null() && Pl_Un_String_Check(value, value_word) != 0)
            as libc::c_int;
    }
    cur_env = environ;
    let fresh1 = cur_env;
    cur_env = cur_env.offset(1);
    one_env = *fresh1;
    if one_env.is_null() {
        return 0 as libc::c_int;
    }
    if !(*cur_env).is_null() {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = var_name_word;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = value_word;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = cur_env as WamWord;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_24656E7669726F6E5F616C74__a0),
                ),
            ),
            3 as libc::c_int,
        );
    }
    value = strchr(one_env, '=' as i32);
    lg = value.offset_from(one_env) as libc::c_long as libc::c_int;
    var_name = pl_glob_buff.as_mut_ptr();
    strncpy(var_name, one_env, lg as libc::c_ulong);
    *var_name.offset(lg as isize) = '\0' as i32 as libc::c_char;
    value = value.offset(1);
    value;
    return (Pl_Un_String_Check(var_name, var_name_word) != 0
        && Pl_Get_Atom(Pl_Create_Atom(value), value_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Environ_Alt_0() -> Bool {
    let mut var_name_word: WamWord = 0;
    let mut value_word: WamWord = 0;
    let mut var_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cur_env: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut one_env: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lg: libc::c_int = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_24656E7669726F6E5F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    var_name_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    value_word = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    cur_env = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) as *mut *mut libc::c_char;
    let fresh2 = cur_env;
    cur_env = cur_env.offset(1);
    one_env = *fresh2;
    if (*cur_env).is_null() {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh3 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh3 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    } else {
        *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
            as *mut WamWord) = cur_env as WamWord;
    }
    value = strchr(one_env, '=' as i32);
    lg = value.offset_from(one_env) as libc::c_long as libc::c_int;
    var_name = pl_glob_buff.as_mut_ptr();
    strncpy(var_name, one_env, lg as libc::c_ulong);
    *var_name.offset(lg as isize) = '\0' as i32 as libc::c_char;
    value = value.offset(1);
    value;
    return (Pl_Un_String_Check(var_name, var_name_word) != 0
        && Pl_Get_Atom(Pl_Create_Atom(value), value_word) != 0) as libc::c_int;
}

use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    static mut pl_stk_tbl: [InfStack; 0];
    fn Pl_Call_Prolog(codep: CodePtr) -> libc::c_int;
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_char_type: [libc::c_int; 0];
    static mut pl_escape_symbol: [libc::c_char; 0];
    static mut pl_escape_char: [libc::c_char; 0];
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Lookup_Pred(func: libc::c_int, arity: libc::c_int) -> *mut PredInf;
    fn Pl_Lookup_Oper(atom_op: libc::c_int, type_0: libc::c_int) -> *mut OperInf;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn Pl_Obtain_Float(adr: *mut WamWord) -> libc::c_double;
    static mut pl_fd_variable_to_string: Option::<
        unsafe extern "C" fn() -> *mut libc::c_char,
    >;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_stm_output: libc::c_int;
    fn Pl_Find_Stream_From_PStm(pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Stream_Putc(c: libc::c_int, pstm: *mut StmInf);
    fn Pl_Stream_Puts(str: *mut libc::c_char, pstm: *mut StmInf) -> libc::c_int;
    static mut pl_resource_print_object_not_linked: libc::c_int;
    fn Pl_Err_Resource(atom_resource: libc::c_int);
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
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type CodePtr = Option::<unsafe extern "C" fn() -> ()>;
pub type WamWordP = *mut WamWord;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct InfStack {
    pub name: *mut libc::c_char,
    pub desc: *mut libc::c_char,
    pub env_var_name: *mut libc::c_char,
    pub p_def_size: *mut PlLong,
    pub default_size: libc::c_int,
    pub size: libc::c_int,
    pub stack: *mut WamWord,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OperInf {
    pub a_t: PlLong,
    pub prec: libc::c_int,
    pub left: libc::c_int,
    pub right: libc::c_int,
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_writing: libc::c_int = 0;
static mut curly_brackets_1: WamWord = 0;
static mut dollar_var_1: WamWord = 0;
static mut dollar_varname_1: WamWord = 0;
static mut atom_dots: libc::c_int = 0;
static mut pstm_o: *mut StmInf = 0 as *const StmInf as *mut StmInf;
static mut quoted: Bool = 0;
static mut ignore_op: Bool = 0;
static mut number_vars: Bool = 0;
static mut name_vars: Bool = 0;
static mut space_args: Bool = 0;
static mut portrayed: Bool = 0;
static mut name_number_above_H: *mut WamWord = 0 as *const WamWord as *mut WamWord;
static mut last_is_space: Bool = 0;
static mut last_prefix_op: libc::c_int = 0 as libc::c_int;
static mut p_bracket_op_minus: *mut Bool = 0 as *const Bool as *mut Bool;
pub unsafe extern "C" fn Pl_Write_Term(
    mut pstm: *mut StmInf,
    mut depth: libc::c_int,
    mut prec: libc::c_int,
    mut mask: libc::c_int,
    mut above_H: *mut WamWord,
    mut term_word: WamWord,
) {
    pstm_o = pstm;
    quoted = mask & 1 as libc::c_int;
    ignore_op = mask & 2 as libc::c_int;
    number_vars = mask & 4 as libc::c_int;
    name_vars = mask & 8 as libc::c_int;
    space_args = mask & 16 as libc::c_int;
    portrayed = mask & 32 as libc::c_int;
    name_number_above_H = above_H;
    last_is_space = 0 as libc::c_int;
    last_prefix_op = 0 as libc::c_int;
    pl_last_writing = 0 as libc::c_int;
    Show_Term(
        depth,
        prec,
        if prec >= 1200 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int },
        term_word,
    );
}
pub unsafe extern "C" fn Pl_Write(mut term_word: WamWord) {
    let mut pstm: *mut StmInf = *pl_stm_tbl.offset(pl_stm_output as isize);
    Pl_Write_Term(
        pstm,
        -(1 as libc::c_int),
        1200 as libc::c_int,
        4 as libc::c_int | 8 as libc::c_int,
        0 as *mut WamWord,
        term_word,
    );
}
unsafe extern "C" fn Out_Space() {
    if last_is_space == 0 {
        Pl_Stream_Putc(' ' as i32, pstm_o);
        last_is_space = 1 as libc::c_int;
    }
    pl_last_writing = 0 as libc::c_int;
}
unsafe extern "C" fn Out_Char(mut c: libc::c_int) {
    Need_Space(c);
    Pl_Stream_Putc(c, pstm_o);
    last_is_space = 0 as libc::c_int;
}
unsafe extern "C" fn Out_String(mut str: *mut libc::c_char) {
    Need_Space(*str as libc::c_int);
    Pl_Stream_Puts(str, pstm_o);
    last_is_space = 0 as libc::c_int;
}
unsafe extern "C" fn Need_Space(mut c: libc::c_int) {
    let mut c_type: libc::c_int = *pl_char_type.as_mut_ptr().offset(c as isize);
    let mut space: libc::c_int = 0;
    let mut current_block_5: u64;
    match pl_last_writing {
        2 => {
            if c_type == 4 as libc::c_int {
                space = 1 as libc::c_int;
                current_block_5 = 11650488183268122163;
            } else {
                current_block_5 = 3286415154635924360;
            }
        }
        1 => {
            current_block_5 = 3286415154635924360;
        }
        3 => {
            space = (c_type
                & (256 as libc::c_int | 512 as libc::c_int | 1024 as libc::c_int
                    | 128 as libc::c_int) != 0 || c == '[' as i32 || c == '{' as i32)
                as libc::c_int;
            current_block_5 = 11650488183268122163;
        }
        4 => {
            space = (c_type == 4 as libc::c_int) as libc::c_int;
            current_block_5 = 11650488183268122163;
        }
        5 => {
            space = (c_type == 32 as libc::c_int) as libc::c_int;
            current_block_5 = 11650488183268122163;
        }
        _ => {
            space = 0 as libc::c_int;
            current_block_5 = 11650488183268122163;
        }
    }
    match current_block_5 {
        3286415154635924360 => {
            space = (c_type
                & (256 as libc::c_int | 512 as libc::c_int | 1024 as libc::c_int
                    | 128 as libc::c_int) != 0 || c == '.' as i32) as libc::c_int;
        }
        _ => {}
    }
    if space != 0 || c == '(' as i32 && last_prefix_op != 0 as libc::c_int {
        Out_Space();
    } else if c_type == 128 as libc::c_int && last_prefix_op == 2 as libc::c_int {
        *p_bracket_op_minus += 1;
        *p_bracket_op_minus;
        Out_Space();
        Out_Char('(' as i32);
    }
    last_prefix_op = 0 as libc::c_int;
    pl_last_writing = 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Write_A_Full_Stop(mut pstm: *mut StmInf) {
    pstm_o = pstm;
    if pl_last_writing == 2 as libc::c_int || pl_last_writing == 1 as libc::c_int {
        pl_last_writing = 0 as libc::c_int;
    }
    Out_Char('.' as i32);
    Out_Char('\n' as i32);
}
pub unsafe extern "C" fn Pl_Write_A_Char(mut pstm: *mut StmInf, mut c: libc::c_int) {
    pstm_o = pstm;
    Out_Char(c);
}
pub unsafe extern "C" fn Pl_Float_To_String(mut d: libc::c_double) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut buff: [libc::c_char; 32] = [0; 32];
    sprintf(buff.as_mut_ptr(), b"%#.17g\0" as *const u8 as *const libc::c_char, d);
    p = buff.as_mut_ptr();
    while *p as libc::c_int == ' ' as i32 {
        p = p.offset(1);
        p;
    }
    if p != buff.as_mut_ptr() {
        q = buff.as_mut_ptr();
        loop {
            let fresh0 = p;
            p = p.offset(1);
            let fresh1 = q;
            q = q.offset(1);
            *fresh1 = *fresh0;
            if !(*fresh1 != 0) {
                break;
            }
        }
    }
    p = strchr(buff.as_mut_ptr(), '.' as i32);
    if p.is_null() {
        return buff.as_mut_ptr();
    }
    if *p.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        strcat(buff.as_mut_ptr(), b"0\0" as *const u8 as *const libc::c_char);
        return buff.as_mut_ptr();
    }
    e = strchr(buff.as_mut_ptr(), 'e' as i32);
    if e.is_null() {
        e = buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize);
    }
    p = e.offset(-(1 as libc::c_int as isize));
    while *p as libc::c_int == '0' as i32 {
        p = p.offset(-1);
        p;
    }
    q = if *p as libc::c_int == '.' as i32 {
        p.offset(2 as libc::c_int as isize)
    } else {
        p.offset(1 as libc::c_int as isize)
    };
    if q != e {
        loop {
            let fresh2 = e;
            e = e.offset(1);
            let fresh3 = q;
            q = q.offset(1);
            *fresh3 = *fresh2;
            if !(*fresh3 != 0) {
                break;
            }
        }
    }
    return buff.as_mut_ptr();
}
unsafe extern "C" fn Show_Term(
    mut depth: libc::c_int,
    mut prec: libc::c_int,
    mut context: libc::c_int,
    mut term_word: WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    if depth == 0 as libc::c_int {
        Show_Atom(0 as libc::c_int, atom_dots);
        return;
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
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong && Try_Portray(word) != 0
    {
        return;
    }
    match tag_mask {
        0 => {
            adr = word as *mut WamWord;
            if adr
                >= *(pl_reg_bank as *mut WamWordP)
                    .offset((256 as libc::c_int + 7 as libc::c_int) as isize)
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
                    let fresh4 = TR;
                    TR = TR.offset(1);
                    *fresh4 = (adr as PlULong | 0 as libc::c_int as libc::c_ulong)
                        as WamWord;
                }
                *adr = word;
                adr = word as *mut WamWord;
            }
            Show_Global_Var(adr);
        }
        3 => {
            Show_Atom(context, (word as PlULong >> 3 as libc::c_int) as libc::c_int);
        }
        5 => {
            Show_Fd_Variable(
                (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord,
            );
        }
        7 => {
            Show_Integer(word << 0 as libc::c_int >> 3 as libc::c_int);
        }
        4 => {
            Show_Float(
                Pl_Obtain_Float(
                    (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord,
                ),
            );
        }
        1 => {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            if ignore_op != 0 {
                Out_String(
                    b"'.'(\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                Show_Term(
                    depth - 1 as libc::c_int,
                    999 as libc::c_int,
                    0 as libc::c_int,
                    *adr.offset(0 as libc::c_int as isize),
                );
                Out_Char(',' as i32);
                Show_Term(
                    depth - 1 as libc::c_int,
                    999 as libc::c_int,
                    0 as libc::c_int,
                    *adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize),
                );
                Out_Char(')' as i32);
            } else {
                Out_Char('[' as i32);
                Show_List_Arg(depth, adr);
                Out_Char(']' as i32);
            }
        }
        2 => {
            adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            Show_Structure(depth, prec, context, adr);
        }
        _ => {}
    };
}
unsafe extern "C" fn Show_Global_Var(mut adr: *mut WamWord) {
    let mut str: [libc::c_char; 32] = [0; 32];
    sprintf(
        str.as_mut_ptr(),
        b"_%d\0" as *const u8 as *const libc::c_char,
        adr
            .offset_from(
                (*pl_stk_tbl.as_mut_ptr().offset(2 as libc::c_int as isize)).stack,
            ) as libc::c_long as libc::c_int,
    );
    Out_String(str.as_mut_ptr());
    pl_last_writing = 3 as libc::c_int;
}
unsafe extern "C" fn Show_Fd_Variable(mut fdv_adr: *mut WamWord) {
    let mut str: [libc::c_char; 32] = [0; 32];
    sprintf(
        str.as_mut_ptr(),
        b"_#%d(\0" as *const u8 as *const libc::c_char,
        fdv_adr
            .offset_from(
                (*pl_stk_tbl.as_mut_ptr().offset(1 as libc::c_int as isize)).stack,
            ) as libc::c_long as libc::c_int,
    );
    Out_String(str.as_mut_ptr());
    Out_String(
        ::std::mem::transmute::<
            _,
            fn(_) -> *mut libc::c_char,
        >((Some(pl_fd_variable_to_string.unwrap())).unwrap())(fdv_adr),
    );
    Out_Char(')' as i32);
    pl_last_writing = 3 as libc::c_int;
}
unsafe extern "C" fn Show_Atom(mut context: libc::c_int, mut atom: libc::c_int) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str: [libc::c_char; 32] = [0; 32];
    let mut bracket: Bool = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut c_type: libc::c_int = 0;
    let mut prop: AtomProp = AtomProp {
        length_op_mask_type_0_needs_quote_needs_scan: [0; 3],
        c2rust_padding: [0; 1],
    };
    prop = (*pl_atom_tbl.offset(atom as isize)).prop;
    if context != 0 as libc::c_int
        && ((*pl_atom_tbl.offset(atom as isize)).prop).op_mask() as libc::c_int != 0
    {
        Out_Char('(' as i32);
        bracket = 1 as libc::c_int;
    }
    if quoted == 0 || prop.needs_quote() == 0 {
        Out_String((*pl_atom_tbl.offset(atom as isize)).name);
        match prop.type_0() as libc::c_int {
            0 => {
                pl_last_writing = 3 as libc::c_int;
            }
            1 => {
                pl_last_writing = 5 as libc::c_int;
            }
            2 => {
                pl_last_writing = 0 as libc::c_int;
            }
            3 => {
                if prop.length() as libc::c_int == 0 as libc::c_int {
                    pl_last_writing = 0 as libc::c_int;
                } else {
                    c = *((*pl_atom_tbl.offset(atom as isize)).name)
                        .offset(
                            (prop.length() as libc::c_int - 1 as libc::c_int) as isize,
                        ) as libc::c_int;
                    c_type = *pl_char_type.as_mut_ptr().offset(c as isize);
                    if c_type
                        & (256 as libc::c_int | 512 as libc::c_int | 1024 as libc::c_int
                            | 128 as libc::c_int) != 0
                    {
                        pl_last_writing = 3 as libc::c_int;
                    } else if c == '\'' as i32 {
                        pl_last_writing = 4 as libc::c_int;
                    } else if c_type == 32 as libc::c_int {
                        pl_last_writing = 5 as libc::c_int;
                    } else {
                        pl_last_writing = 0 as libc::c_int;
                    }
                }
            }
            _ => {}
        }
    } else {
        Out_Char('\'' as i32);
        if prop.needs_scan() != 0 {
            p = (*pl_atom_tbl.offset(atom as isize)).name;
            while *p != 0 {
                q = strchr(pl_escape_char.as_mut_ptr(), *p as libc::c_int);
                if !q.is_null() {
                    Out_Char('\\' as i32);
                    Out_Char(
                        *pl_escape_symbol
                            .as_mut_ptr()
                            .offset(
                                q.offset_from(pl_escape_char.as_mut_ptr()) as libc::c_long
                                    as isize,
                            ) as libc::c_int,
                    );
                } else if *p as libc::c_int == '\'' as i32
                    || *p as libc::c_int == '\\' as i32
                {
                    Out_Char(*p as libc::c_int);
                    Out_Char(*p as libc::c_int);
                } else if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize)
                    as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
                {
                    sprintf(
                        str.as_mut_ptr(),
                        b"\\x%x\\\0" as *const u8 as *const libc::c_char,
                        *p as libc::c_uchar as libc::c_uint,
                    );
                    Out_String(str.as_mut_ptr());
                } else {
                    Out_Char(*p as libc::c_int);
                }
                p = p.offset(1);
                p;
            }
        } else {
            Out_String((*pl_atom_tbl.offset(atom as isize)).name);
        }
        Out_Char('\'' as i32);
        pl_last_writing = 4 as libc::c_int;
    }
    if bracket != 0 {
        Out_Char(')' as i32);
    }
}
unsafe extern "C" fn Show_Integer(mut x: PlLong) {
    let mut str: [libc::c_char; 32] = [0; 32];
    sprintf(str.as_mut_ptr(), b"%ld\0" as *const u8 as *const libc::c_char, x);
    Show_Number_Str(str.as_mut_ptr());
}
unsafe extern "C" fn Show_Float(mut x: libc::c_double) {
    Show_Number_Str(Pl_Float_To_String(x));
}
unsafe extern "C" fn Show_Number_Str(mut str: *mut libc::c_char) {
    Out_String(str);
    pl_last_writing = if *str as libc::c_int == '0' as i32
        && *str.offset(1 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn Show_List_Arg(mut depth: libc::c_int, mut lst_adr: *mut WamWord) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    loop {
        depth -= 1;
        depth;
        Show_Term(
            depth,
            999 as libc::c_int,
            0 as libc::c_int,
            *lst_adr.offset(0 as libc::c_int as isize),
        );
        if depth == 0 as libc::c_int {
            return;
        }
        let mut deref_last_word: WamWord = 0;
        word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
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
        match tag_mask {
            0 => {
                Out_Char('|' as i32);
                Show_Global_Var(word as *mut WamWord);
                break;
            }
            3 => {
                if word as libc::c_ulong
                    != (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                        as libc::c_ulong)
                        .wrapping_add(0x3 as libc::c_int as PlULong)
                {
                    Out_Char('|' as i32);
                    if Try_Portray(word) != 0 {
                        return;
                    }
                    Show_Atom(
                        0 as libc::c_int,
                        (word as PlULong >> 3 as libc::c_int) as libc::c_int,
                    );
                }
                break;
            }
            5 => {
                Out_Char('|' as i32);
                if Try_Portray(word) != 0 {
                    return;
                }
                Show_Fd_Variable(
                    (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord,
                );
                break;
            }
            7 => {
                Out_Char('|' as i32);
                if Try_Portray(word) != 0 {
                    return;
                }
                Show_Integer(word << 0 as libc::c_int >> 3 as libc::c_int);
                break;
            }
            4 => {
                Out_Char('|' as i32);
                if Try_Portray(word) != 0 {
                    return;
                }
                Show_Float(
                    Pl_Obtain_Float(
                        (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                            as *mut WamWord,
                    ),
                );
                break;
            }
            1 => {
                Out_Char(',' as i32);
                if space_args != 0 {
                    Out_Space();
                }
                lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                    as *mut WamWord;
            }
            2 => {
                Out_Char('|' as i32);
                if Try_Portray(word) != 0 {
                    return;
                }
                Show_Structure(
                    depth,
                    999 as libc::c_int,
                    0 as libc::c_int,
                    (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                        as *mut WamWord,
                );
                break;
            }
            _ => {
                break;
            }
        }
    };
}
unsafe extern "C" fn Is_Valid_Var_Name(mut str: *mut libc::c_char) -> Bool {
    let mut c_type: libc::c_int = 0;
    c_type = *pl_char_type.as_mut_ptr().offset(*str as libc::c_uint as isize);
    if c_type & (256 as libc::c_int | 512 as libc::c_int) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    loop {
        str = str.offset(1);
        if !(*str as libc::c_int != '\0' as i32) {
            break;
        }
        c_type = *pl_char_type.as_mut_ptr().offset(*str as libc::c_uint as isize);
        if c_type
            & (256 as libc::c_int | 512 as libc::c_int | 1024 as libc::c_int
                | 128 as libc::c_int) == 0 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Is_Valid_Var_Name_1(mut name_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut deref_last_word: WamWord = 0;
    word = name_word;
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
    return (tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
        && Is_Valid_Var_Name(
            (*pl_atom_tbl.offset((word as PlULong >> 3 as libc::c_int) as isize)).name,
        ) != 0) as libc::c_int;
}
unsafe extern "C" fn Show_Structure(
    mut depth: libc::c_int,
    mut prec: libc::c_int,
    mut context: libc::c_int,
    mut stc_adr: *mut WamWord,
) {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut adr: *mut WamWord = 0 as *mut WamWord;
    let mut f_n: WamWord = *stc_adr.offset(0 as libc::c_int as isize);
    let mut functor: libc::c_int = (*stc_adr.offset(0 as libc::c_int as isize) as PlULong
        & ((1 as libc::c_int as PlULong)
            << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(10 as libc::c_int as libc::c_ulong))
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as libc::c_int;
    let mut arity: libc::c_int = (*stc_adr.offset(0 as libc::c_int as isize) as PlULong
        >> (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong)) as libc::c_int;
    let mut oper: *mut OperInf = 0 as *mut OperInf;
    let mut nb_args_to_disp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut str: [libc::c_char; 32] = [0; 32];
    let mut bracket: Bool = 0;
    let mut surround_space: Bool = 0;
    depth -= 1;
    depth;
    if name_vars != 0 && f_n == dollar_varname_1 && stc_adr >= name_number_above_H {
        let mut deref_last_word: WamWord = 0;
        word = *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
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
        if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
            let mut save_quoted: libc::c_int = quoted;
            quoted = 0 as libc::c_int;
            Show_Atom(
                0 as libc::c_int,
                (word as PlULong >> 3 as libc::c_int) as libc::c_int,
            );
            quoted = save_quoted;
            return;
        }
    }
    if number_vars != 0 && f_n == dollar_var_1 && stc_adr >= name_number_above_H {
        let mut deref_last_word_0: WamWord = 0;
        word = *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
        loop {
            deref_last_word_0 = word;
            tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                as WamWord;
            if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
                break;
            }
            word = *(word as *mut WamWord);
            if !(word != deref_last_word_0) {
                break;
            }
        }
        if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong
            && {
                n = (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int;
                n >= 0 as libc::c_int
            }
        {
            i = n % 26 as libc::c_int;
            j = n / 26 as libc::c_int;
            Out_Char('A' as i32 + i);
            if j != 0 {
                sprintf(
                    str.as_mut_ptr(),
                    b"%d\0" as *const u8 as *const libc::c_char,
                    j,
                );
                Out_String(str.as_mut_ptr());
            }
            pl_last_writing = 3 as libc::c_int;
            return;
        }
    }
    if !(ignore_op != 0 || arity > 2 as libc::c_int) {
        if f_n == curly_brackets_1 {
            Out_Char('{' as i32);
            if space_args != 0 {
                Out_Space();
            }
            Show_Term(
                depth,
                1200 as libc::c_int,
                0 as libc::c_int,
                *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
            );
            if space_args != 0 {
                Out_Space();
            }
            Out_Char('}' as i32);
            return;
        }
        bracket = 0 as libc::c_int;
        if arity == 1 as libc::c_int
            && {
                oper = Pl_Lookup_Oper(functor, 0 as libc::c_int);
                !oper.is_null()
            }
        {
            let mut oper1: *mut OperInf = 0 as *mut OperInf;
            if (*oper).prec > (*oper).right
                && {
                    oper1 = Pl_Lookup_Oper(functor, 1 as libc::c_int);
                    !oper1.is_null()
                } && (*oper1).left == (*oper1).prec
            {
                oper = oper1;
            } else {
                if (*oper).prec > prec
                    || context == 2 as libc::c_int
                        && ((*oper).prec == (*oper).right && (*oper).prec == prec)
                {
                    Out_Char('(' as i32);
                    bracket = 1 as libc::c_int;
                }
                Show_Atom(0 as libc::c_int, functor);
                last_prefix_op = 1 as libc::c_int;
                if space_args != 0 {
                    Out_Space();
                }
                if strcmp(
                    (*pl_atom_tbl.offset(functor as isize)).name,
                    b"-\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    last_prefix_op = 2 as libc::c_int;
                    p_bracket_op_minus = &mut bracket;
                }
                Show_Term(
                    depth,
                    (*oper).right,
                    1 as libc::c_int,
                    *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                );
                last_prefix_op = 0 as libc::c_int;
                loop {
                    let fresh5 = bracket;
                    bracket = bracket - 1;
                    if !(fresh5 != 0) {
                        break;
                    }
                    Out_Char(')' as i32);
                }
                return;
            }
            current_block = 3524696382762650850;
        } else if arity == 1 as libc::c_int
            && {
                oper = Pl_Lookup_Oper(functor, 1 as libc::c_int);
                !oper.is_null()
            }
        {
            current_block = 3524696382762650850;
        } else {
            if arity == 2 as libc::c_int
                && {
                    oper = Pl_Lookup_Oper(functor, 2 as libc::c_int);
                    !oper.is_null()
                }
            {
                if (*oper).prec > prec
                    || context == 2 as libc::c_int
                        && ((*oper).prec == (*oper).right && (*oper).prec == prec)
                {
                    Out_Char('(' as i32);
                    bracket = 1 as libc::c_int;
                }
                context = if (*oper).left == (*oper).prec {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                };
                Show_Term(
                    depth,
                    (*oper).left,
                    context,
                    *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                );
                if functor == '|' as i32 as libc::c_uchar as libc::c_int
                    && (*oper).prec > 1000 as libc::c_int
                {
                    if space_args != 0 {
                        Out_Space();
                    }
                    Out_Char('|' as i32);
                    if space_args != 0 {
                        Out_Space();
                    }
                } else if functor == ',' as i32 as libc::c_uchar as libc::c_int {
                    Out_Char(',' as i32);
                    if space_args != 0 {
                        Out_Space();
                    }
                } else {
                    surround_space = 0 as libc::c_int;
                    if ((*pl_atom_tbl.offset(functor as isize)).prop).type_0()
                        as libc::c_int == 0 as libc::c_int
                        || ((*pl_atom_tbl.offset(functor as isize)).prop).type_0()
                            as libc::c_int == 3 as libc::c_int || space_args != 0
                    {
                        surround_space = 1 as libc::c_int;
                        Out_Space();
                    }
                    Show_Atom(0 as libc::c_int, functor);
                    if surround_space != 0 {
                        Out_Space();
                    }
                }
                Show_Term(
                    depth,
                    (*oper).right,
                    1 as libc::c_int,
                    *stc_adr.offset((1 as libc::c_int + 1 as libc::c_int) as isize),
                );
                if bracket != 0 {
                    Out_Char(')' as i32);
                }
                return;
            }
            current_block = 8045309554469507602;
        }
        match current_block {
            8045309554469507602 => {}
            _ => {
                if (*oper).prec > prec {
                    Out_Char('(' as i32);
                    bracket = 1 as libc::c_int;
                }
                context = if (*oper).left == (*oper).prec {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                };
                Show_Term(
                    depth,
                    (*oper).left,
                    context,
                    *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize),
                );
                if space_args != 0 {
                    Out_Space();
                }
                Show_Atom(0 as libc::c_int, functor);
                if bracket != 0 {
                    Out_Char(')' as i32);
                }
                return;
            }
        }
    }
    Show_Atom(0 as libc::c_int, functor);
    Out_Char('(' as i32);
    i = if arity < depth + 1 as libc::c_int || depth < 0 as libc::c_int {
        arity
    } else {
        depth + 1 as libc::c_int
    };
    nb_args_to_disp = i;
    adr = &mut *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize)
        as *mut WamWord;
    loop {
        let fresh6 = adr;
        adr = adr.offset(1);
        Show_Term(depth, 999 as libc::c_int, 0 as libc::c_int, *fresh6);
        i -= 1;
        if !(i != 0) {
            break;
        }
        Out_Char(',' as i32);
        if space_args != 0 {
            Out_Space();
        }
    }
    if arity != nb_args_to_disp {
        Out_Char(',' as i32);
        if space_args != 0 {
            Out_Space();
        }
        Show_Atom(0 as libc::c_int, atom_dots);
    }
    Out_Char(')' as i32);
}
unsafe extern "C" fn Try_Portray(mut word: WamWord) -> Bool {
    let mut pred: *mut PredInf = 0 as *mut PredInf;
    let mut print_pstm_o: *mut StmInf = 0 as *mut StmInf;
    let mut print_quoted: Bool = 0;
    let mut print_ignore_op: Bool = 0;
    let mut print_number_vars: Bool = 0;
    let mut print_name_vars: Bool = 0;
    let mut print_space_args: Bool = 0;
    let mut print_portrayed: Bool = 0;
    let mut print_ok: Bool = 0;
    static mut try_portray_code: CodePtr = None;
    if portrayed == 0 {
        return 0 as libc::c_int;
    }
    if try_portray_code.is_none() {
        pred = Pl_Lookup_Pred(
            Pl_Create_Atom(
                b"$try_portray\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int,
        );
        if pred.is_null() || ((*pred).codep).is_null() {
            Pl_Err_Resource(pl_resource_print_object_not_linked);
        }
        try_portray_code = ::std::mem::transmute::<*mut PlLong, CodePtr>((*pred).codep);
    }
    print_pstm_o = pstm_o;
    print_quoted = quoted;
    print_ignore_op = ignore_op;
    print_number_vars = number_vars;
    print_name_vars = name_vars;
    print_space_args = space_args;
    print_portrayed = portrayed;
    *pl_reg_bank.offset(0 as libc::c_int as isize) = word;
    print_ok = Pl_Call_Prolog(try_portray_code);
    pstm_o = print_pstm_o;
    quoted = print_quoted;
    ignore_op = print_ignore_op;
    number_vars = print_number_vars;
    name_vars = print_name_vars;
    space_args = print_space_args;
    portrayed = print_portrayed;
    return print_ok;
}
pub unsafe extern "C" fn Pl_Get_Print_Stm_1(mut stm_word: WamWord) -> Bool {
    let mut stm: libc::c_int = Pl_Find_Stream_From_PStm(pstm_o);
    if stm < 0 as libc::c_int {
        stm = pl_stm_output;
    }
    return Pl_Get_Integer(stm as PlLong, stm_word);
}

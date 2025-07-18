use ::libc;
use ::c2rust_bitfields;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn Pl_Execute_A_Continuation(codep: CodePtr);
    static mut pl_atom_tbl: *mut AtomInf;
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn Pl_M_Sys_Err_String(ret_val: libc::c_int) -> *mut libc::c_char;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Get_Structure(
        func: libc::c_int,
        arity: libc::c_int,
        start_word: WamWord,
    ) -> Bool;
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Check_For_Un_Integer(start_word: WamWord);
    fn Pl_Check_For_Un_Atom(start_word: WamWord);
    fn Pl_Un_Integer_Check(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Un_Integer(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Un_Atom_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Un_String(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_stm_top_level_output: libc::c_int;
    fn Pl_Stream_Printf(
        pstm: *mut StmInf,
        format: *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut pl_flag_unknown: *mut FlagInf;
    static mut pl_flag_os_error: *mut FlagInf;
    fn X1_24706C5F6572725F696E7374616E74696174696F6E__a0();
    fn X1_24706C5F6572725F756E696E7374616E74696174696F6E__a1();
    fn X1_24706C5F6572725F74797065__a2();
    fn X1_24706C5F6572725F646F6D61696E__a2();
    fn X1_24706C5F6572725F6578697374656E6365__a2();
    fn X1_24706C5F6572725F7065726D697373696F6E__a3();
    fn X1_24706C5F6572725F726570726573656E746174696F6E__a1();
    fn X1_24706C5F6572725F6576616C756174696F6E__a1();
    fn X1_24706C5F6572725F7265736F75726365__a1();
    fn X1_24706C5F6572725F73796E746178__a1();
    fn X1_24706C5F6572725F73797374656D__a1();
}
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
pub type PlLong = intptr_t;
pub type PlULong = uintptr_t;
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
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_type_atom: libc::c_int = 0;
pub static mut pl_type_atomic: libc::c_int = 0;
pub static mut pl_type_byte: libc::c_int = 0;
pub static mut pl_type_callable: libc::c_int = 0;
pub static mut pl_type_character: libc::c_int = 0;
pub static mut pl_type_compound: libc::c_int = 0;
pub static mut pl_type_evaluable: libc::c_int = 0;
pub static mut pl_type_float: libc::c_int = 0;
pub static mut pl_type_boolean: libc::c_int = 0;
pub static mut pl_type_in_byte: libc::c_int = 0;
pub static mut pl_type_in_character: libc::c_int = 0;
pub static mut pl_type_integer: libc::c_int = 0;
pub static mut pl_type_list: libc::c_int = 0;
pub static mut pl_type_number: libc::c_int = 0;
pub static mut pl_type_predicate_indicator: libc::c_int = 0;
pub static mut pl_type_variable: libc::c_int = 0;
pub static mut pl_type_pair: libc::c_int = 0;
pub static mut pl_type_fd_variable: libc::c_int = 0;
pub static mut pl_type_fd_evaluable: libc::c_int = 0;
pub static mut pl_type_fd_bool_evaluable: libc::c_int = 0;
pub static mut pl_domain_character_code_list: libc::c_int = 0;
pub static mut pl_domain_close_option: libc::c_int = 0;
pub static mut pl_domain_flag_value: libc::c_int = 0;
pub static mut pl_domain_io_mode: libc::c_int = 0;
pub static mut pl_domain_non_empty_list: libc::c_int = 0;
pub static mut pl_domain_not_less_than_zero: libc::c_int = 0;
pub static mut pl_domain_operator_priority: libc::c_int = 0;
pub static mut pl_domain_operator_specifier: libc::c_int = 0;
pub static mut pl_domain_prolog_flag: libc::c_int = 0;
pub static mut pl_domain_read_option: libc::c_int = 0;
pub static mut pl_domain_source_sink: libc::c_int = 0;
pub static mut pl_domain_stream: libc::c_int = 0;
pub static mut pl_domain_stream_option: libc::c_int = 0;
pub static mut pl_domain_stream_or_alias: libc::c_int = 0;
pub static mut pl_domain_stream_position: libc::c_int = 0;
pub static mut pl_domain_stream_property: libc::c_int = 0;
pub static mut pl_domain_write_option: libc::c_int = 0;
pub static mut pl_domain_order: libc::c_int = 0;
pub static mut pl_domain_term_stream_or_alias: libc::c_int = 0;
pub static mut pl_domain_g_array_index: libc::c_int = 0;
pub static mut pl_domain_g_argument_selector: libc::c_int = 0;
pub static mut pl_domain_stream_seek_method: libc::c_int = 0;
pub static mut pl_domain_format_control_sequence: libc::c_int = 0;
pub static mut pl_domain_os_path: libc::c_int = 0;
pub static mut pl_domain_os_file_permission: libc::c_int = 0;
pub static mut pl_domain_selectable_item: libc::c_int = 0;
pub static mut pl_domain_date_time: libc::c_int = 0;
pub static mut pl_domain_socket_domain: libc::c_int = 0;
pub static mut pl_domain_socket_address: libc::c_int = 0;
pub static mut pl_existence_procedure: libc::c_int = 0;
pub static mut pl_existence_source_sink: libc::c_int = 0;
pub static mut pl_existence_stream: libc::c_int = 0;
pub static mut pl_existence_sr_descriptor: libc::c_int = 0;
pub static mut pl_permission_operation_access: libc::c_int = 0;
pub static mut pl_permission_operation_close: libc::c_int = 0;
pub static mut pl_permission_operation_create: libc::c_int = 0;
pub static mut pl_permission_operation_input: libc::c_int = 0;
pub static mut pl_permission_operation_modify: libc::c_int = 0;
pub static mut pl_permission_operation_open: libc::c_int = 0;
pub static mut pl_permission_operation_output: libc::c_int = 0;
pub static mut pl_permission_type_binary_stream: libc::c_int = 0;
pub static mut pl_permission_type_flag: libc::c_int = 0;
pub static mut pl_permission_type_operator: libc::c_int = 0;
pub static mut pl_permission_type_past_end_of_stream: libc::c_int = 0;
pub static mut pl_permission_type_private_procedure: libc::c_int = 0;
pub static mut pl_permission_type_static_procedure: libc::c_int = 0;
pub static mut pl_permission_type_source_sink: libc::c_int = 0;
pub static mut pl_permission_type_stream: libc::c_int = 0;
pub static mut pl_permission_type_text_stream: libc::c_int = 0;
pub static mut pl_representation_character: libc::c_int = 0;
pub static mut pl_representation_character_code: libc::c_int = 0;
pub static mut pl_representation_in_character_code: libc::c_int = 0;
pub static mut pl_representation_max_arity: libc::c_int = 0;
pub static mut pl_representation_max_integer: libc::c_int = 0;
pub static mut pl_representation_min_integer: libc::c_int = 0;
pub static mut pl_representation_integer_32bits: libc::c_int = 0;
pub static mut pl_representation_too_many_variables: libc::c_int = 0;
pub static mut pl_evaluation_float_overflow: libc::c_int = 0;
pub static mut pl_permission_operation_reposition: libc::c_int = 0;
pub static mut pl_resource_too_big_fd_constraint: libc::c_int = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_evaluation_int_overflow: libc::c_int = 0;
pub static mut pl_evaluation_undefined: libc::c_int = 0;
pub static mut pl_evaluation_underflow: libc::c_int = 0;
pub static mut pl_evaluation_zero_divisor: libc::c_int = 0;
pub static mut pl_resource_print_object_not_linked: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
static mut cur_bip_func: libc::c_int = 0;
static mut cur_bip_arity: libc::c_int = 0;
static mut c_bip_func_str: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut c_bip_arity: libc::c_int = 0;
static mut last_err_file: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut last_err_line: libc::c_int = 0;
static mut last_err_col: libc::c_int = 0;
static mut last_err_msg: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub unsafe extern "C" fn Pl_Set_Bip_Name_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) {
    let mut arity: PlLong = 0;
    Pl_Set_C_Bip_Name(
        b"set_bip_name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    cur_bip_func = Pl_Rd_Atom_Check(func_word);
    arity = Pl_Rd_Integer_Check(arity_word);
    if arity > (256 as libc::c_int - 1 as libc::c_int) as libc::c_long {
        arity = -(1 as libc::c_int) as PlLong;
    }
    cur_bip_arity = arity as libc::c_int;
    Pl_Unset_C_Bip_Name();
    c_bip_func_str = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn Pl_Set_Bip_Name_Untagged_2(
    mut func: libc::c_int,
    mut arity: libc::c_int,
) {
    cur_bip_func = func;
    cur_bip_arity = arity;
}
pub unsafe extern "C" fn Pl_Current_Bip_Name_2(
    mut func_word: WamWord,
    mut arity_word: WamWord,
) -> Bool {
    let mut res: Bool = 0;
    Pl_Set_C_Bip_Name(
        b"current_bip_name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    res = (Pl_Un_Atom_Check(cur_bip_func, func_word) != 0
        && Pl_Un_Integer_Check(cur_bip_arity as PlLong, arity_word) != 0) as libc::c_int;
    Pl_Unset_C_Bip_Name();
    return res;
}
pub unsafe extern "C" fn Pl_Set_C_Bip_Name(
    mut func_str: *mut libc::c_char,
    mut arity: libc::c_int,
) {
    c_bip_func_str = func_str;
    c_bip_arity = arity;
}
pub unsafe extern "C" fn Pl_Unset_C_Bip_Name() {
    c_bip_func_str = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn Pl_Get_Current_Bip(mut arity: *mut libc::c_int) -> libc::c_int {
    Update_Cur_From_C_Bip();
    *arity = cur_bip_arity;
    return cur_bip_func;
}
unsafe extern "C" fn Update_Cur_From_C_Bip() {
    if !c_bip_func_str.is_null() {
        cur_bip_func = Pl_Create_Allocate_Atom(c_bip_func_str);
        cur_bip_arity = c_bip_arity;
    }
}
pub unsafe extern "C" fn Pl_Context_Error_1(mut err_word: WamWord) {
    if cur_bip_arity >= 0 as libc::c_int {
        Pl_Get_Structure(
            '/' as i32 as libc::c_uchar as libc::c_int,
            2 as libc::c_int,
            err_word,
        );
        Pl_Unify_Atom(cur_bip_func);
        Pl_Unify_Integer(cur_bip_arity as PlLong);
    } else {
        Pl_Get_Atom(cur_bip_func, err_word);
    };
}
unsafe extern "C" fn Context_Error_String() -> *mut libc::c_char {
    static mut buff: [libc::c_char; 1024] = [0; 1024];
    if cur_bip_arity < 0 as libc::c_int {
        return (*pl_atom_tbl.offset(cur_bip_func as isize)).name;
    }
    sprintf(
        buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
        b"%s/%d\0" as *const u8 as *const libc::c_char,
        (*pl_atom_tbl.offset(cur_bip_func as isize)).name,
        cur_bip_arity,
    );
    return buff.as_mut_ptr();
}
pub unsafe extern "C" fn Pl_Set_Last_Syntax_Error(
    mut file_name: *mut libc::c_char,
    mut err_line: libc::c_int,
    mut err_col: libc::c_int,
    mut err_msg: *mut libc::c_char,
) {
    last_err_file = file_name;
    last_err_line = err_line;
    last_err_col = err_col;
    last_err_msg = err_msg;
}
pub unsafe extern "C" fn Pl_Syntax_Error_Info_4(
    mut file_name_word: WamWord,
    mut line_word: WamWord,
    mut char_word: WamWord,
    mut msg_word: WamWord,
) -> Bool {
    Pl_Check_For_Un_Atom(file_name_word);
    Pl_Check_For_Un_Integer(line_word);
    Pl_Check_For_Un_Integer(char_word);
    Pl_Check_For_Un_Atom(msg_word);
    return (!last_err_file.is_null() && Pl_Un_String(last_err_file, file_name_word) != 0
        && Pl_Un_Integer(last_err_line as PlLong, line_word) != 0
        && Pl_Un_Integer(last_err_col as PlLong, char_word) != 0
        && Pl_Un_String(last_err_msg, msg_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Syntax_Error(mut flag_value: libc::c_int) {
    let mut str: [libc::c_char; 512] = [0; 512];
    if last_err_file.is_null() || *last_err_file as libc::c_int == '\0' as i32 {
        sprintf(
            str.as_mut_ptr(),
            b"(char:%d) %s\0" as *const u8 as *const libc::c_char,
            last_err_col,
            last_err_msg,
        );
    } else {
        sprintf(
            str.as_mut_ptr(),
            b"%s:%d (char:%d) %s\0" as *const u8 as *const libc::c_char,
            last_err_file,
            last_err_line,
            last_err_col,
            last_err_msg,
        );
    }
    if flag_value == 0 as libc::c_int {
        Pl_Err_Syntax(Pl_Create_Allocate_Atom(str.as_mut_ptr()));
    }
    Update_Cur_From_C_Bip();
    if flag_value == 1 as libc::c_int {
        Pl_Stream_Printf(
            *pl_stm_tbl.offset(pl_stm_top_level_output as isize),
            b"warning: syntax error: %s (from %s)\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            str.as_mut_ptr(),
            Context_Error_String(),
        );
    }
}
pub unsafe extern "C" fn Pl_Unknown_Pred_Error(
    mut func: libc::c_int,
    mut arity: libc::c_int,
) {
    let mut term: WamWord = 0;
    if (*pl_flag_unknown).value == 0 as libc::c_int as libc::c_long {
        term = Pl_Put_Structure(
            '/' as i32 as libc::c_uchar as libc::c_int,
            2 as libc::c_int,
        );
        Pl_Unify_Atom(func);
        Pl_Unify_Integer(arity as PlLong);
        Pl_Err_Existence(pl_existence_procedure, term);
    }
    Update_Cur_From_C_Bip();
    if (*pl_flag_unknown).value == 1 as libc::c_int as libc::c_long {
        Pl_Stream_Printf(
            *pl_stm_tbl.offset(pl_stm_top_level_output as isize),
            b"warning: unknown procedure %s/%d (from %s)\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*pl_atom_tbl.offset(func as isize)).name,
            arity,
            Context_Error_String(),
        );
    }
}
pub unsafe extern "C" fn Pl_Os_Error(mut ret_val: libc::c_int) {
    let mut err_str: *mut libc::c_char = Pl_M_Sys_Err_String(ret_val);
    if (*pl_flag_os_error).value == 0 as libc::c_int as libc::c_long {
        Pl_Err_System(Pl_Create_Allocate_Atom(err_str));
    }
    Update_Cur_From_C_Bip();
    if (*pl_flag_os_error).value == 1 as libc::c_int as libc::c_long {
        Pl_Stream_Printf(
            *pl_stm_tbl.offset(pl_stm_top_level_output as isize),
            b"warning: OS error: %s (from %s)\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            err_str,
            Context_Error_String(),
        );
    }
}
pub unsafe extern "C" fn Pl_Err_Instantiation() {
    Update_Cur_From_C_Bip();
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F696E7374616E74696174696F6E__a0),
        ),
    );
}
pub unsafe extern "C" fn Pl_Err_Uninstantiation(mut term: WamWord) {
    Update_Cur_From_C_Bip();
    *pl_reg_bank.offset(0 as libc::c_int as isize) = term;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F756E696E7374616E74696174696F6E__a1),
        ),
    );
}
pub unsafe extern "C" fn Pl_Err_Type(mut atom_type: libc::c_int, mut term: WamWord) {
    Update_Cur_From_C_Bip();
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = (((atom_type as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    *pl_reg_bank.offset(1 as libc::c_int as isize) = term;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F74797065__a2),
        ),
    );
}
pub unsafe extern "C" fn Pl_Err_Domain(mut atom_domain: libc::c_int, mut term: WamWord) {
    Update_Cur_From_C_Bip();
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = (((atom_domain as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    *pl_reg_bank.offset(1 as libc::c_int as isize) = term;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F646F6D61696E__a2),
        ),
    );
}
pub unsafe extern "C" fn Pl_Err_Existence(
    mut atom_object: libc::c_int,
    mut term: WamWord,
) {
    Update_Cur_From_C_Bip();
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = (((atom_object as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    *pl_reg_bank.offset(1 as libc::c_int as isize) = term;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F6578697374656E6365__a2),
        ),
    );
}
pub unsafe extern "C" fn Pl_Err_Permission(
    mut atom_oper: libc::c_int,
    mut atom_perm: libc::c_int,
    mut term: WamWord,
) {
    Update_Cur_From_C_Bip();
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = (((atom_oper as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    *pl_reg_bank
        .offset(
            1 as libc::c_int as isize,
        ) = (((atom_perm as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    *pl_reg_bank.offset(2 as libc::c_int as isize) = term;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F7065726D697373696F6E__a3),
        ),
    );
}
pub unsafe extern "C" fn Pl_Err_Representation(mut atom_flag: libc::c_int) {
    Update_Cur_From_C_Bip();
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = (((atom_flag as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F726570726573656E746174696F6E__a1),
        ),
    );
}
pub unsafe extern "C" fn Pl_Err_Evaluation(mut pl_atom_error: libc::c_int) {
    Update_Cur_From_C_Bip();
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = (((pl_atom_error as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F6576616C756174696F6E__a1),
        ),
    );
}
pub unsafe extern "C" fn Pl_Err_Resource(mut atom_resource: libc::c_int) {
    Update_Cur_From_C_Bip();
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = (((atom_resource as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F7265736F75726365__a1),
        ),
    );
}
pub unsafe extern "C" fn Pl_Err_Syntax(mut pl_atom_error: libc::c_int) {
    Update_Cur_From_C_Bip();
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = (((pl_atom_error as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F73796E746178__a1),
        ),
    );
}
pub unsafe extern "C" fn Pl_Err_System(mut pl_atom_error: libc::c_int) {
    Update_Cur_From_C_Bip();
    *pl_reg_bank
        .offset(
            0 as libc::c_int as isize,
        ) = (((pl_atom_error as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    Pl_Execute_A_Continuation(
        Some(
            ::std::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(X1_24706C5F6572725F73797374656D__a1),
        ),
    );
}

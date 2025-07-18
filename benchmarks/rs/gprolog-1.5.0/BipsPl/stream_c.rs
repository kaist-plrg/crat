use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn Pl_Hash_First(tbl: *mut libc::c_char, scan: *mut HashScan) -> *mut libc::c_char;
    fn Pl_Hash_Next(scan: *mut HashScan) -> *mut libc::c_char;
    static mut pl_glob_buff: [libc::c_char; 0];
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_atom_false: libc::c_int;
    static mut pl_atom_true: libc::c_int;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Get_Structure(
        func: libc::c_int,
        arity: libc::c_int,
        start_word: WamWord,
    ) -> Bool;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_M_Absolute_Path_Name(src: *mut libc::c_char) -> *mut libc::c_char;
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Chars_Str_Check(start_word: WamWord, str: *mut libc::c_char) -> libc::c_int;
    fn Pl_Rd_Codes_Str_Check(start_word: WamWord, str: *mut libc::c_char) -> libc::c_int;
    fn Pl_Check_For_Un_Integer(start_word: WamWord);
    fn Pl_Un_Integer_Check(value: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Un_Atom_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Un_String_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_Un_Chars_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_Un_Codes_Check(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    fn Pl_List_Length(start_word: WamWord) -> libc::c_int;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_stm_last_used: libc::c_int;
    static mut pl_alias_tbl: *mut libc::c_char;
    static mut pl_last_output_sora: WamWord;
    static mut pl_stm_stdin: libc::c_int;
    static mut pl_stm_stdout: libc::c_int;
    static mut pl_stm_stderr: libc::c_int;
    static mut pl_stm_input: libc::c_int;
    static mut pl_stm_output: libc::c_int;
    static mut pl_stm_top_level_input: libc::c_int;
    static mut pl_stm_top_level_output: libc::c_int;
    static mut pl_stm_debugger_input: libc::c_int;
    static mut pl_stm_debugger_output: libc::c_int;
    static mut pl_atom_top_level_input: libc::c_int;
    static mut pl_atom_top_level_output: libc::c_int;
    static mut pl_atom_debugger_input: libc::c_int;
    static mut pl_atom_debugger_output: libc::c_int;
    static mut pl_atom_read: libc::c_int;
    static mut pl_atom_write: libc::c_int;
    static mut pl_atom_append: libc::c_int;
    static mut pl_atom_reposition: libc::c_int;
    static mut pl_atom_stream_position: libc::c_int;
    static mut pl_atom_text: libc::c_int;
    static mut pl_atom_binary: libc::c_int;
    static mut pl_atom_error: libc::c_int;
    static mut pl_atom_eof_code: libc::c_int;
    static mut pl_atom_reset: libc::c_int;
    static mut pl_atom_none: libc::c_int;
    static mut pl_atom_line: libc::c_int;
    static mut pl_atom_block: libc::c_int;
    static mut pl_atom_not: libc::c_int;
    static mut pl_atom_at: libc::c_int;
    static mut pl_atom_past: libc::c_int;
    static mut pl_atom_bof: libc::c_int;
    static mut pl_atom_current: libc::c_int;
    static mut pl_atom_eof: libc::c_int;
    fn Pl_Add_Stream_For_Stdio_File(
        path: *mut libc::c_char,
        mode: libc::c_int,
        text: Bool,
    ) -> libc::c_int;
    fn Pl_Delete_Stream(stm: libc::c_int);
    fn Pl_Find_Stream_By_Alias(atom_alias: libc::c_int) -> libc::c_int;
    fn Pl_Add_Alias_To_Stream(atom_alias: libc::c_int, stm: libc::c_int) -> Bool;
    fn Pl_Reassign_Alias(atom_alias: libc::c_int, stm: libc::c_int);
    fn Pl_Add_Mirror_To_Stream(stm: libc::c_int, m_stm: libc::c_int);
    fn Pl_Del_Mirror_From_Stream(stm: libc::c_int, m_stm: libc::c_int) -> Bool;
    fn Pl_Set_Stream_Buffering(stm: libc::c_int);
    fn Pl_Get_Stream_Or_Alias(sora_word: WamWord, test_mask: libc::c_int) -> libc::c_int;
    fn Pl_Stdio_Set_Buffering(f: *mut FILE, buffering: libc::c_int);
    fn Pl_PB_Empty_Buffer(pstm: *mut StmInf);
    fn Pl_Stream_Flush(pstm: *mut StmInf);
    fn Pl_Stream_Close(pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Stream_End_Of_Stream(pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Stream_Get_Position(
        pstm: *mut StmInf,
        offset: *mut PlLong,
        char_count: *mut PlLong,
        line_count: *mut PlLong,
        line_pos: *mut PlLong,
    );
    fn Pl_Stream_Set_Position(
        pstm: *mut StmInf,
        whence: libc::c_int,
        offset: PlLong,
        char_count: PlLong,
        line_count: PlLong,
        line_pos: PlLong,
    ) -> libc::c_int;
    fn Pl_Stream_Set_Position_LC(
        pstm: *mut StmInf,
        line_count: PlLong,
        line_pos: PlLong,
    ) -> libc::c_int;
    fn Pl_Add_Str_Stream(
        buff: *mut libc::c_char,
        prop_other: libc::c_int,
    ) -> libc::c_int;
    fn Pl_Delete_Str_Stream(stm: libc::c_int);
    fn Pl_Term_Write_Str_Stream(stm: libc::c_int) -> *mut libc::c_char;
    static mut pl_sys_var: [PlLong; 0];
    static mut pl_permission_type_binary_stream: libc::c_int;
    static mut pl_permission_operation_reposition: libc::c_int;
    static mut pl_permission_operation_open: libc::c_int;
    static mut pl_permission_operation_modify: libc::c_int;
    static mut pl_permission_operation_close: libc::c_int;
    static mut pl_permission_operation_access: libc::c_int;
    static mut pl_existence_source_sink: libc::c_int;
    static mut pl_domain_stream_seek_method: libc::c_int;
    static mut pl_domain_term_stream_or_alias: libc::c_int;
    static mut pl_permission_type_text_stream: libc::c_int;
    static mut pl_permission_type_stream: libc::c_int;
    static mut pl_permission_type_source_sink: libc::c_int;
    static mut pl_domain_stream_position: libc::c_int;
    static mut pl_domain_source_sink: libc::c_int;
    static mut pl_domain_io_mode: libc::c_int;
    fn Pl_Err_Instantiation();
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn Pl_Err_Existence(atom_object: libc::c_int, term: WamWord);
    fn Pl_Err_Permission(atom_oper: libc::c_int, atom_perm: libc::c_int, term: WamWord);
    fn Pl_Err_System(pl_atom_error_0: libc::c_int);
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn X1_2463757272656E745F73747265616D5F616C74__a0();
    fn X1_2463757272656E745F616C6961735F616C74__a0();
    fn X1_2463757272656E745F6D6972726F725F616C74__a0();
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AliasInf {
    pub atom: PlLong,
    pub stm: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StrSInf {
    pub buff: *mut libc::c_char,
    pub ptr: *mut libc::c_char,
    pub buff_alloc_size: Bool,
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
pub static mut pl_le_hook_start: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_put_char: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_char0: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_emit_beep: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_ins_mode: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_screen_size: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_kbd_is_not_empty: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_backd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_forwd: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_displ_str: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_erase: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_set_line_buffering: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_get_line_buffering: Option::<
    unsafe extern "C" fn() -> libc::c_int,
> = None;
pub static mut pl_le_hook_flush: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_confirm_box: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub static mut pl_le_hook_message_box: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_hook_exit_process: Option::<unsafe extern "C" fn() -> ()> = None;
pub static mut pl_le_initialize: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
pub unsafe extern "C" fn Pl_Current_Input_1(mut stm_word: WamWord) -> Bool {
    return Pl_Get_Integer(pl_stm_input as PlLong, stm_word);
}
pub unsafe extern "C" fn Pl_Current_Output_1(mut stm_word: WamWord) -> Bool {
    return Pl_Get_Integer(pl_stm_output as PlLong, stm_word);
}
pub unsafe extern "C" fn Pl_Set_Input_1(mut sora_word: WamWord) {
    pl_stm_input = Pl_Get_Stream_Or_Alias(sora_word, 2 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Set_Output_1(mut sora_word: WamWord) {
    pl_stm_output = Pl_Get_Stream_Or_Alias(sora_word, 3 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Set_Top_Level_Streams_2(
    mut sora_in_word: WamWord,
    mut sora_out_word: WamWord,
) {
    pl_stm_top_level_input = Pl_Get_Stream_Or_Alias(sora_in_word, 2 as libc::c_int);
    pl_stm_top_level_output = Pl_Get_Stream_Or_Alias(sora_out_word, 3 as libc::c_int);
    Pl_Reassign_Alias(pl_atom_top_level_input, pl_stm_top_level_input);
    Pl_Reassign_Alias(pl_atom_top_level_output, pl_stm_top_level_output);
}
pub unsafe extern "C" fn Pl_Set_Debugger_Streams_2(
    mut sora_in_word: WamWord,
    mut sora_out_word: WamWord,
) {
    pl_stm_debugger_input = Pl_Get_Stream_Or_Alias(sora_in_word, 2 as libc::c_int);
    pl_stm_debugger_output = Pl_Get_Stream_Or_Alias(sora_out_word, 3 as libc::c_int);
    Pl_Reassign_Alias(pl_atom_debugger_input, pl_stm_debugger_input);
    Pl_Reassign_Alias(pl_atom_debugger_output, pl_stm_debugger_output);
}
pub unsafe extern "C" fn Pl_Open_3(
    mut source_sink_word: WamWord,
    mut mode_word: WamWord,
    mut stm_word: WamWord,
) {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut mode: libc::c_int = 0 as libc::c_int;
    let mut text: Bool = 0;
    let mut prop: StmProp = StmProp {
        mode_input_output_text_reposition_eof_action_buffering_special_close_other: [0; 2],
        c2rust_padding: [0; 2],
    };
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut atom_file_name: libc::c_int = 0;
    let mut stm: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut mask: libc::c_int = *pl_sys_var
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as libc::c_int;
    let mut reposition: Bool = 0;
    let mut deref_last_word: WamWord = 0;
    word = source_sink_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if tag_mask as libc::c_ulong != 0x3 as libc::c_int as PlULong {
        Pl_Err_Domain(pl_domain_source_sink, source_sink_word);
    }
    atom_file_name = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
    path = (*pl_atom_tbl.offset(atom_file_name as isize)).name;
    path = Pl_M_Absolute_Path_Name(path);
    if path.is_null() {
        Pl_Err_Existence(pl_existence_source_sink, source_sink_word);
    }
    text = mask & 1 as libc::c_int;
    mask >>= 1 as libc::c_int;
    atom = Pl_Rd_Atom_Check(mode_word);
    if atom == pl_atom_read {
        mode = 0 as libc::c_int;
    } else if atom == pl_atom_write {
        mode = 1 as libc::c_int;
    } else if atom == pl_atom_append {
        mode = 2 as libc::c_int;
    } else {
        Pl_Err_Domain(pl_domain_io_mode, mode_word);
    }
    stm = Pl_Add_Stream_For_Stdio_File(path, mode, text);
    if stm < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int
            || *__errno_location() == 20 as libc::c_int
        {
            Pl_Err_Existence(pl_existence_source_sink, source_sink_word);
        } else {
            Pl_Err_Permission(
                pl_permission_operation_open,
                pl_permission_type_source_sink,
                source_sink_word,
            );
        }
    }
    prop = (**pl_stm_tbl.offset(stm as isize)).prop;
    f = (**pl_stm_tbl.offset(stm as isize)).file as *mut FILE;
    if mask & 2 as libc::c_int != 0 as libc::c_int {
        reposition = mask & 1 as libc::c_int;
        if reposition != 0 && prop.reposition() == 0 {
            fclose(f);
            word = Pl_Put_Structure(pl_atom_reposition, 1 as libc::c_int);
            Pl_Unify_Atom(pl_atom_true);
            Pl_Err_Permission(
                pl_permission_operation_open,
                pl_permission_type_source_sink,
                word,
            );
        }
        prop.set_reposition(reposition as libc::c_uint);
    }
    mask >>= 2 as libc::c_int;
    if mask & 4 as libc::c_int != 0 as libc::c_int {
        prop.set_eof_action((mask & 3 as libc::c_int) as libc::c_uint);
    }
    mask >>= 3 as libc::c_int;
    if mask & 4 as libc::c_int != 0 as libc::c_int {
        if prop.buffering() != (mask & 3 as libc::c_int) as libc::c_uint {
            prop.set_buffering((mask & 3 as libc::c_int) as libc::c_uint);
            Pl_Stdio_Set_Buffering(f, prop.buffering() as libc::c_int);
        }
    }
    mask >>= 3 as libc::c_int;
    (**pl_stm_tbl.offset(stm as isize)).atom_file_name = atom_file_name;
    (**pl_stm_tbl.offset(stm as isize)).prop = prop;
    Pl_Get_Integer(stm as PlLong, stm_word);
}
pub unsafe extern "C" fn Pl_Test_Alias_Not_Assigned_1(mut alias_word: WamWord) -> Bool {
    return (Pl_Find_Stream_By_Alias(Pl_Rd_Atom_Check(alias_word)) < 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_From_Alias_To_Stream_2(
    mut alias_word: WamWord,
    mut stm_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    stm = Pl_Find_Stream_By_Alias(Pl_Rd_Atom_Check(alias_word));
    return (stm >= 0 as libc::c_int && Pl_Get_Integer(stm as PlLong, stm_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Add_Stream_Alias_2(
    mut sora_word: WamWord,
    mut alias_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    return Pl_Add_Alias_To_Stream(Pl_Rd_Atom_Check(alias_word), stm);
}
pub unsafe extern "C" fn Pl_Check_Valid_Mirror_1(mut mirror_word: WamWord) {
    Pl_Get_Stream_Or_Alias(mirror_word, 3 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Add_Stream_Mirror_2(
    mut sora_word: WamWord,
    mut mirror_word: WamWord,
) {
    let mut stm: libc::c_int = 0;
    let mut m_stm: libc::c_int = 0;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    m_stm = Pl_Get_Stream_Or_Alias(mirror_word, 3 as libc::c_int);
    Pl_Add_Mirror_To_Stream(stm, m_stm);
}
pub unsafe extern "C" fn Pl_Remove_Stream_Mirror_2(
    mut sora_word: WamWord,
    mut mirror_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut m_stm: libc::c_int = 0;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    m_stm = Pl_Get_Stream_Or_Alias(mirror_word, 1 as libc::c_int);
    return Pl_Del_Mirror_From_Stream(stm, m_stm);
}
pub unsafe extern "C" fn Pl_Set_Stream_Type_2(
    mut sora_word: WamWord,
    mut is_text_word: WamWord,
) {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut text: libc::c_int = 0;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    text = Pl_Rd_Integer_Check(is_text_word) as libc::c_int;
    if text as libc::c_uint == ((*pstm).prop).text() {
        return;
    }
    if (*pstm).char_count != 0 {
        Pl_Err_Permission(
            pl_permission_operation_modify,
            pl_permission_type_stream,
            sora_word,
        );
    }
    ((*pstm).prop).set_text(text as libc::c_uint);
}
pub unsafe extern "C" fn Pl_Set_Stream_Eof_Action_2(
    mut sora_word: WamWord,
    mut action_word: WamWord,
) {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    if ((*pstm).prop).output() != 0 {
        Pl_Err_Permission(
            pl_permission_operation_modify,
            pl_permission_type_stream,
            sora_word,
        );
    }
    ((*pstm).prop).set_eof_action(Pl_Rd_Integer_Check(action_word) as libc::c_uint);
}
pub unsafe extern "C" fn Pl_Set_Stream_Buffering_2(
    mut sora_word: WamWord,
    mut buff_mode_word: WamWord,
) {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    ((*pstm).prop).set_buffering(Pl_Rd_Integer_Check(buff_mode_word) as libc::c_uint);
    Pl_Set_Stream_Buffering(stm);
}
pub unsafe extern "C" fn Pl_Close_1(mut sora_word: WamWord) {
    let mut stm: libc::c_int = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    Pl_Close_Stm(
        stm,
        (*pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
            & 1 as libc::c_int as libc::c_long) as Bool,
    );
}
pub unsafe extern "C" fn Pl_Close_Stm(mut stm: libc::c_int, mut force: Bool) {
    let mut pstm: *mut StmInf = *pl_stm_tbl.offset(stm as isize);
    let mut fd: libc::c_int = 0 as libc::c_int;
    Pl_Stream_Flush(pstm);
    if stm == pl_stm_stdin || stm == pl_stm_stdout || stm == pl_stm_stderr {
        return;
    }
    if stm == pl_stm_top_level_input || stm == pl_stm_top_level_output {
        return;
    }
    if stm == pl_stm_debugger_input || stm == pl_stm_debugger_output {
        return;
    }
    if stm == pl_stm_input {
        pl_stm_input = pl_stm_stdin;
    } else if stm == pl_stm_output {
        pl_stm_output = pl_stm_stdout;
    }
    if ((*pstm).prop).special_close() != 0 {
        Pl_Err_System(
            Pl_Create_Atom(
                b"special stream: needs appropriate close predicate\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            ),
        );
    }
    if (*pstm).fct_close
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_int>,
            StmFct,
        >(Some(fclose as unsafe extern "C" fn(*mut FILE) -> libc::c_int))
    {
        fd = fileno((*pstm).file as *mut FILE);
    }
    if Pl_Stream_Close(pstm) != 0 as libc::c_int {
        if force == 0 as libc::c_int {
            Pl_Err_System(
                Pl_Create_Atom(
                    b"cannot close stream\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
            );
        }
        if fd > 2 as libc::c_int {
            close(fd);
        }
    }
    Pl_Delete_Stream(stm);
}
pub unsafe extern "C" fn Pl_PB_Empty_Buffer_1(mut sora_word: WamWord) {
    let mut stm: libc::c_int = 0;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_input
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 2 as libc::c_int)
    };
    Pl_PB_Empty_Buffer(*pl_stm_tbl.offset(stm as isize));
}
pub unsafe extern "C" fn Pl_Flush_Output_1(mut sora_word: WamWord) {
    let mut stm: libc::c_int = 0;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_output
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 3 as libc::c_int)
    };
    pl_last_output_sora = sora_word;
    Pl_Stream_Flush(*pl_stm_tbl.offset(stm as isize));
}
pub unsafe extern "C" fn Pl_Flush_Output_0() {
    Pl_Flush_Output_1(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
    );
}
pub unsafe extern "C" fn Pl_Current_Stream_1(mut stm_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut stm: libc::c_int = 0 as libc::c_int;
    let mut deref_last_word: WamWord = 0;
    word = stm_word;
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
    if tag_mask as libc::c_ulong == 0x7 as libc::c_int as PlULong {
        stm = (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int;
        return (stm >= 0 as libc::c_int && stm <= pl_stm_last_used
            && !(*pl_stm_tbl.offset(stm as isize)).is_null()) as libc::c_int;
    }
    while stm <= pl_stm_last_used {
        if !(*pl_stm_tbl.offset(stm as isize)).is_null() {
            break;
        }
        stm += 1;
        stm;
    }
    if stm >= pl_stm_last_used {
        if stm > pl_stm_last_used {
            return 0 as libc::c_int;
        }
    } else {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = stm_word;
        *pl_reg_bank
            .offset(1 as libc::c_int as isize) = (stm + 1 as libc::c_int) as WamWord;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_2463757272656E745F73747265616D5F616C74__a0),
                ),
            ),
            2 as libc::c_int,
        );
    }
    return Pl_Get_Integer(stm as PlLong, stm_word);
}
pub unsafe extern "C" fn Pl_Current_Stream_Alt_0() -> Bool {
    let mut stm_word: WamWord = 0;
    let mut stm: libc::c_int = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F73747265616D5F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    stm_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    stm = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    while stm <= pl_stm_last_used {
        if !(*pl_stm_tbl.offset(stm as isize)).is_null() {
            break;
        }
        stm += 1;
        stm;
    }
    if stm >= pl_stm_last_used {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh0 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        if stm > pl_stm_last_used {
            return 0 as libc::c_int;
        }
    } else {
        *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
            as *mut WamWord) = (stm + 1 as libc::c_int) as WamWord;
    }
    return Pl_Get_Integer(stm as PlLong, stm_word);
}
pub unsafe extern "C" fn Pl_Stream_Prop_File_Name_2(
    mut file_name_word: WamWord,
    mut stm_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    stm = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    return Pl_Un_Atom_Check(
        (**pl_stm_tbl.offset(stm as isize)).atom_file_name,
        file_name_word,
    );
}
pub unsafe extern "C" fn Pl_Stream_Prop_Mode_2(
    mut mode_word: WamWord,
    mut stm_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut atom: libc::c_int = 0 as libc::c_int;
    stm = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    match ((**pl_stm_tbl.offset(stm as isize)).prop).mode() as libc::c_int {
        0 => {
            atom = pl_atom_read;
        }
        1 => {
            atom = pl_atom_write;
        }
        2 => {
            atom = pl_atom_append;
        }
        _ => {}
    }
    return Pl_Un_Atom_Check(atom, mode_word);
}
pub unsafe extern "C" fn Pl_Stream_Prop_Input_1(mut stm_word: WamWord) -> Bool {
    let mut stm: libc::c_int = 0;
    stm = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    return ((**pl_stm_tbl.offset(stm as isize)).prop).input() as Bool;
}
pub unsafe extern "C" fn Pl_Stream_Prop_Output_1(mut stm_word: WamWord) -> Bool {
    let mut stm: libc::c_int = 0;
    stm = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    return ((**pl_stm_tbl.offset(stm as isize)).prop).output() as Bool;
}
pub unsafe extern "C" fn Pl_Stream_Prop_Type_2(
    mut type_word: WamWord,
    mut stm_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut atom: libc::c_int = 0;
    stm = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    atom = if ((**pl_stm_tbl.offset(stm as isize)).prop).text() as libc::c_int != 0 {
        pl_atom_text
    } else {
        pl_atom_binary
    };
    return Pl_Un_Atom_Check(atom, type_word);
}
pub unsafe extern "C" fn Pl_Stream_Prop_Reposition_2(
    mut reposition_word: WamWord,
    mut stm_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut atom: libc::c_int = 0;
    stm = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    atom = if ((**pl_stm_tbl.offset(stm as isize)).prop).reposition() as libc::c_int != 0
    {
        pl_atom_true
    } else {
        pl_atom_false
    };
    return Pl_Un_Atom_Check(atom, reposition_word);
}
pub unsafe extern "C" fn Pl_Stream_Prop_Eof_Action_2(
    mut eof_action_word: WamWord,
    mut stm_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut atom: libc::c_int = 0 as libc::c_int;
    stm = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    match ((**pl_stm_tbl.offset(stm as isize)).prop).eof_action() as libc::c_int {
        0 => {
            atom = pl_atom_error;
        }
        1 => {
            atom = pl_atom_eof_code;
        }
        2 => {
            atom = pl_atom_reset;
        }
        _ => {}
    }
    return Pl_Un_Atom_Check(atom, eof_action_word);
}
pub unsafe extern "C" fn Pl_Stream_Prop_Buffering_2(
    mut buffering_word: WamWord,
    mut stm_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut atom: libc::c_int = 0 as libc::c_int;
    stm = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    if (**pl_stm_tbl.offset(stm as isize)).file == stdout as PlLong
        && pl_le_hook_get_line_buffering.is_some()
    {
        if ::std::mem::transmute::<
            _,
            fn() -> libc::c_int,
        >((Some(pl_le_hook_get_line_buffering.unwrap())).unwrap())() != 0
        {
            let ref mut fresh1 = (**pl_stm_tbl.offset(stm as isize)).prop;
            (*fresh1).set_buffering(1 as libc::c_int as libc::c_uint);
        } else {
            let ref mut fresh2 = (**pl_stm_tbl.offset(stm as isize)).prop;
            (*fresh2).set_buffering(0 as libc::c_int as libc::c_uint);
        }
    }
    match ((**pl_stm_tbl.offset(stm as isize)).prop).buffering() as libc::c_int {
        0 => {
            atom = pl_atom_none;
        }
        1 => {
            atom = pl_atom_line;
        }
        2 => {
            atom = pl_atom_block;
        }
        _ => {}
    }
    return Pl_Un_Atom_Check(atom, buffering_word);
}
pub unsafe extern "C" fn Pl_Stream_Prop_End_Of_Stream_2(
    mut end_of_stream_word: WamWord,
    mut stm_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut atom: libc::c_int = 0 as libc::c_int;
    stm = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    match Pl_Stream_End_Of_Stream(*pl_stm_tbl.offset(stm as isize)) {
        0 => {
            atom = pl_atom_not;
        }
        1 => {
            atom = pl_atom_at;
        }
        2 => {
            atom = pl_atom_past;
        }
        _ => {}
    }
    return Pl_Un_Atom_Check(atom, end_of_stream_word);
}
pub unsafe extern "C" fn Pl_At_End_Of_Stream_1(mut sora_word: WamWord) -> Bool {
    let mut stm: libc::c_int = 0;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_input
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 2 as libc::c_int)
    };
    return (Pl_Stream_End_Of_Stream(*pl_stm_tbl.offset(stm as isize))
        != 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn Pl_At_End_Of_Stream_0() -> Bool {
    return Pl_At_End_Of_Stream_1(
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
    );
}
pub unsafe extern "C" fn Pl_Current_Alias_2(
    mut stm_word: WamWord,
    mut alias_word: WamWord,
) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut stm: libc::c_int = 0;
    let mut scan: HashScan = HashScan {
        endt: 0 as *mut libc::c_char,
        cur_t: 0 as *mut libc::c_char,
        cur_p: 0 as *mut libc::c_char,
    };
    let mut alias: *mut AliasInf = 0 as *mut AliasInf;
    let mut save_alias: *mut AliasInf = 0 as *mut AliasInf;
    stm = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    let mut deref_last_word: WamWord = 0;
    word = alias_word;
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
        return (Pl_Find_Stream_By_Alias(Pl_Rd_Atom_Check(word)) == stm) as libc::c_int;
    }
    alias = Pl_Hash_First(pl_alias_tbl, &mut scan) as *mut AliasInf;
    while !alias.is_null() {
        if (*alias).stm == stm {
            break;
        }
        alias = Pl_Hash_Next(&mut scan) as *mut AliasInf;
    }
    if alias.is_null() {
        return 0 as libc::c_int;
    }
    save_alias = alias;
    loop {
        alias = Pl_Hash_Next(&mut scan) as *mut AliasInf;
        if alias.is_null() || (*alias).stm == stm {
            break;
        }
    }
    if !alias.is_null() {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = stm as WamWord;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = alias_word;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = scan.endt as WamWord;
        *pl_reg_bank.offset(3 as libc::c_int as isize) = scan.cur_t as WamWord;
        *pl_reg_bank.offset(4 as libc::c_int as isize) = scan.cur_p as WamWord;
        *pl_reg_bank.offset(5 as libc::c_int as isize) = alias as WamWord;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_2463757272656E745F616C6961735F616C74__a0),
                ),
            ),
            6 as libc::c_int,
        );
    }
    Pl_Get_Atom((*save_alias).atom as libc::c_int, alias_word);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Current_Alias_Alt_0() -> Bool {
    let mut stm: libc::c_int = 0;
    let mut alias_word: WamWord = 0;
    let mut scan: HashScan = HashScan {
        endt: 0 as *mut libc::c_char,
        cur_t: 0 as *mut libc::c_char,
        cur_p: 0 as *mut libc::c_char,
    };
    let mut alias: *mut AliasInf = 0 as *mut AliasInf;
    let mut save_alias: *mut AliasInf = 0 as *mut AliasInf;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F616C6961735F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    stm = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    alias_word = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    scan
        .endt = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) as *mut libc::c_char;
    scan
        .cur_t = *(&mut *B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
        as *mut WamWord) as *mut libc::c_char;
    scan
        .cur_p = *(&mut *B.offset((-(9 as libc::c_int) - 4 as libc::c_int) as isize)
        as *mut WamWord) as *mut libc::c_char;
    alias = *(&mut *B.offset((-(9 as libc::c_int) - 5 as libc::c_int) as isize)
        as *mut WamWord) as *mut AliasInf;
    save_alias = alias;
    loop {
        alias = Pl_Hash_Next(&mut scan) as *mut AliasInf;
        if alias.is_null() || (*alias).stm == stm {
            break;
        }
    }
    if !alias.is_null() {
        *(&mut *B.offset((-(9 as libc::c_int) - 3 as libc::c_int) as isize)
            as *mut WamWord) = scan.cur_t as WamWord;
        *(&mut *B.offset((-(9 as libc::c_int) - 4 as libc::c_int) as isize)
            as *mut WamWord) = scan.cur_p as WamWord;
        *(&mut *B.offset((-(9 as libc::c_int) - 5 as libc::c_int) as isize)
            as *mut WamWord) = alias as WamWord;
    } else {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh3 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh3 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    }
    Pl_Get_Atom((*save_alias).atom as libc::c_int, alias_word);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Current_Mirror_2(
    mut stm_word: WamWord,
    mut m_stm_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = Pl_Rd_Integer_Check(stm_word) as libc::c_int;
    let mut pstm: *mut StmInf = *pl_stm_tbl.offset(stm as isize);
    let mut m: *mut StmLst = (*pstm).mirror;
    if m.is_null() {
        return 0 as libc::c_int;
    }
    if !((*m).next).is_null() {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = stm as WamWord;
        *pl_reg_bank.offset(1 as libc::c_int as isize) = m_stm_word;
        *pl_reg_bank.offset(2 as libc::c_int as isize) = (*m).next as WamWord;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_2463757272656E745F6D6972726F725F616C74__a0),
                ),
            ),
            3 as libc::c_int,
        );
    }
    return Pl_Get_Integer((*m).stm as PlLong, m_stm_word);
}
pub unsafe extern "C" fn Pl_Current_Mirror_Alt_0() -> Bool {
    let mut m_stm_word: WamWord = 0;
    let mut m: *mut StmLst = 0 as *mut StmLst;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2463757272656E745F6D6972726F725F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    m_stm_word = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord);
    m = *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
        as *mut WamWord) as *mut StmLst;
    if !((*m).next).is_null() {
        *(&mut *B.offset((-(9 as libc::c_int) - 2 as libc::c_int) as isize)
            as *mut WamWord) = (*m).next as WamWord;
    } else {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh4 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh4 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
    }
    return Pl_Get_Integer((*m).stm as PlLong, m_stm_word);
}
pub unsafe extern "C" fn Pl_Stream_Position_2(
    mut sora_word: WamWord,
    mut position_word: WamWord,
) -> Bool {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut p_word: [WamWord; 4] = [0; 4];
    let mut p: [PlLong; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    Pl_Stream_Get_Position(
        pstm,
        p.as_mut_ptr(),
        p.as_mut_ptr().offset(1 as libc::c_int as isize),
        p.as_mut_ptr().offset(2 as libc::c_int as isize),
        p.as_mut_ptr().offset(3 as libc::c_int as isize),
    );
    if Pl_Get_Structure(pl_atom_stream_position, 4 as libc::c_int, position_word) == 0 {
        current_block = 9004982444554647395;
    } else {
        current_block = 11875828834189669668;
    }
    '_dom_error: loop {
        match current_block {
            9004982444554647395 => {
                Pl_Err_Domain(pl_domain_stream_position, position_word);
                current_block = 11875828834189669668;
            }
            _ => {
                i = 0 as libc::c_int;
                loop {
                    if !(i < 4 as libc::c_int) {
                        break '_dom_error;
                    }
                    p_word[i as usize] = Pl_Unify_Variable();
                    let mut deref_last_word: WamWord = 0;
                    word = p_word[i as usize];
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
                    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong
                        && tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong
                    {
                        current_block = 9004982444554647395;
                        break;
                    }
                    i += 1;
                    i;
                }
            }
        }
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if Pl_Get_Integer(p[i as usize], p_word[i as usize]) == 0 {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Set_Stream_Position_2(
    mut sora_word: WamWord,
    mut position_word: WamWord,
) -> Bool {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut p_word: [WamWord; 4] = [0; 4];
    let mut p: [libc::c_int; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    if ((*pstm).prop).reposition() == 0 {
        Pl_Err_Permission(
            pl_permission_operation_reposition,
            pl_permission_type_stream,
            sora_word,
        );
    }
    let mut deref_last_word: WamWord = 0;
    word = position_word;
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
    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
        Pl_Err_Instantiation();
    }
    if Pl_Get_Structure(pl_atom_stream_position, 4 as libc::c_int, position_word) == 0 {
        current_block = 13165113891718890642;
    } else {
        current_block = 15652330335145281839;
    }
    '_dom_error: loop {
        match current_block {
            13165113891718890642 => {
                Pl_Err_Domain(pl_domain_stream_position, position_word);
                current_block = 15652330335145281839;
            }
            _ => {
                i = 0 as libc::c_int;
                loop {
                    if !(i < 4 as libc::c_int) {
                        break '_dom_error;
                    }
                    p_word[i as usize] = Pl_Unify_Variable();
                    let mut deref_last_word_0: WamWord = 0;
                    word = p_word[i as usize];
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
                    if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
                        Pl_Err_Instantiation();
                    }
                    if tag_mask as libc::c_ulong != 0x7 as libc::c_int as PlULong {
                        current_block = 13165113891718890642;
                        break;
                    }
                    p[i
                        as usize] = (word << 0 as libc::c_int >> 3 as libc::c_int)
                        as libc::c_int;
                    i += 1;
                    i;
                }
            }
        }
    }
    return (Pl_Stream_Set_Position(
        pstm,
        0 as libc::c_int,
        p[0 as libc::c_int as usize] as PlLong,
        p[1 as libc::c_int as usize] as PlLong,
        p[2 as libc::c_int as usize] as PlLong,
        p[3 as libc::c_int as usize] as PlLong,
    ) == 0 as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Seek_4(
    mut sora_word: WamWord,
    mut whence_word: WamWord,
    mut offset_word: WamWord,
    mut new_loc_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut whence: libc::c_int = 0 as libc::c_int;
    let mut offset: PlLong = 0;
    let mut atom: libc::c_int = 0;
    let mut p: [PlLong; 4] = [0; 4];
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    if ((*pstm).prop).reposition() == 0 {
        Pl_Err_Permission(
            pl_permission_operation_reposition,
            pl_permission_type_stream,
            sora_word,
        );
    }
    if ((*pstm).prop).text() != 0 {
        Pl_Err_Permission(
            pl_permission_operation_reposition,
            pl_permission_type_text_stream,
            sora_word,
        );
    }
    atom = Pl_Rd_Atom_Check(whence_word);
    if atom == pl_atom_bof {
        whence = 0 as libc::c_int;
    } else if atom == pl_atom_current {
        whence = 1 as libc::c_int;
    } else if atom == pl_atom_eof {
        whence = 2 as libc::c_int;
    } else {
        Pl_Err_Domain(pl_domain_stream_seek_method, whence_word);
    }
    offset = Pl_Rd_Integer_Check(offset_word);
    Pl_Check_For_Un_Integer(new_loc_word);
    if Pl_Stream_Set_Position(
        pstm,
        whence,
        offset,
        offset,
        0 as libc::c_int as PlLong,
        0 as libc::c_int as PlLong,
    ) != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    Pl_Stream_Get_Position(
        pstm,
        &mut offset,
        p.as_mut_ptr().offset(1 as libc::c_int as isize),
        p.as_mut_ptr().offset(2 as libc::c_int as isize),
        p.as_mut_ptr().offset(3 as libc::c_int as isize),
    );
    return Pl_Get_Integer(offset, new_loc_word);
}
pub unsafe extern "C" fn Pl_Character_Count_2(
    mut sora_word: WamWord,
    mut count_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut offset: PlLong = 0;
    let mut char_count: PlLong = 0;
    let mut line_count: PlLong = 0;
    let mut line_pos: PlLong = 0;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    Pl_Stream_Get_Position(
        pstm,
        &mut offset,
        &mut char_count,
        &mut line_count,
        &mut line_pos,
    );
    return Pl_Un_Integer_Check(char_count, count_word);
}
pub unsafe extern "C" fn Pl_Line_Count_2(
    mut sora_word: WamWord,
    mut count_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut offset: PlLong = 0;
    let mut char_count: PlLong = 0;
    let mut line_count: PlLong = 0;
    let mut line_pos: PlLong = 0;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    if ((*pstm).prop).text() == 0 {
        Pl_Err_Permission(
            pl_permission_operation_access,
            pl_permission_type_binary_stream,
            sora_word,
        );
    }
    Pl_Stream_Get_Position(
        pstm,
        &mut offset,
        &mut char_count,
        &mut line_count,
        &mut line_pos,
    );
    return Pl_Un_Integer_Check(line_count, count_word);
}
pub unsafe extern "C" fn Pl_Line_Position_2(
    mut sora_word: WamWord,
    mut count_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut offset: PlLong = 0;
    let mut char_count: PlLong = 0;
    let mut line_count: PlLong = 0;
    let mut line_pos: PlLong = 0;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    if ((*pstm).prop).text() == 0 {
        Pl_Err_Permission(
            pl_permission_operation_access,
            pl_permission_type_binary_stream,
            sora_word,
        );
    }
    Pl_Stream_Get_Position(
        pstm,
        &mut offset,
        &mut char_count,
        &mut line_count,
        &mut line_pos,
    );
    return Pl_Un_Integer_Check(line_pos, count_word);
}
pub unsafe extern "C" fn Pl_Stream_Line_Column_3(
    mut sora_word: WamWord,
    mut line_word: WamWord,
    mut col_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut offset: PlLong = 0;
    let mut char_count: PlLong = 0;
    let mut line_count: PlLong = 0;
    let mut line_pos: PlLong = 0;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    if ((*pstm).prop).text() == 0 {
        Pl_Err_Permission(
            pl_permission_operation_access,
            pl_permission_type_binary_stream,
            sora_word,
        );
    }
    Pl_Stream_Get_Position(
        pstm,
        &mut offset,
        &mut char_count,
        &mut line_count,
        &mut line_pos,
    );
    return (Pl_Un_Integer_Check(line_count + 1 as libc::c_int as libc::c_long, line_word)
        != 0
        && Pl_Un_Integer_Check(line_pos + 1 as libc::c_int as libc::c_long, col_word)
            != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Set_Stream_Line_Column_3(
    mut sora_word: WamWord,
    mut line_word: WamWord,
    mut col_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut line_count: PlLong = 0;
    let mut line_pos: PlLong = 0;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    if ((*pstm).prop).reposition() == 0 {
        Pl_Err_Permission(
            pl_permission_operation_reposition,
            pl_permission_type_stream,
            sora_word,
        );
    }
    if ((*pstm).prop).text() == 0 {
        Pl_Err_Permission(
            pl_permission_operation_reposition,
            pl_permission_type_binary_stream,
            sora_word,
        );
    }
    line_count = Pl_Rd_Integer_Check(line_word) - 1 as libc::c_int as libc::c_long;
    line_pos = Pl_Rd_Integer_Check(col_word) - 1 as libc::c_int as libc::c_long;
    return (line_count >= 0 as libc::c_int as libc::c_long
        && line_pos >= 0 as libc::c_int as libc::c_long
        && Pl_Stream_Set_Position_LC(pstm, line_count, line_pos) == 0 as libc::c_int)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_Open_Input_Term_Stream_2(
    mut sink_term_word: WamWord,
    mut stm_word: WamWord,
) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stm: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
        == 1 as libc::c_int as libc::c_long
    {
        str = (*pl_atom_tbl.offset(Pl_Rd_Atom_Check(sink_term_word) as isize)).name;
    } else {
        n = Pl_List_Length(sink_term_word);
        if n >= 0 as libc::c_int {
            str = Pl_Malloc_Check(
                (n + 1 as libc::c_int) as libc::c_uint,
                b"stream_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                1414 as libc::c_int,
            );
        } else {
            str = pl_glob_buff.as_mut_ptr();
        }
        if *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
            == 2 as libc::c_int as libc::c_long
        {
            Pl_Rd_Chars_Str_Check(sink_term_word, str);
        } else {
            Pl_Rd_Codes_Str_Check(sink_term_word, str);
        }
    }
    stm = Pl_Add_Str_Stream(
        str,
        *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize) as libc::c_int,
    );
    Pl_Get_Integer(stm as PlLong, stm_word);
}
pub unsafe extern "C" fn Pl_Close_Input_Term_Stream_1(mut sora_word: WamWord) {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut str_stream: *mut StrSInf = 0 as *mut StrSInf;
    let mut type_0: libc::c_int = 0;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    type_0 = ((*pstm).prop).other() as libc::c_int;
    if type_0 < 1 as libc::c_int || type_0 > 3 as libc::c_int {
        Pl_Err_Domain(pl_domain_term_stream_or_alias, sora_word);
    }
    if ((*pstm).prop).output() != 0 {
        Pl_Err_Permission(
            pl_permission_operation_close,
            pl_permission_type_stream,
            sora_word,
        );
    }
    if type_0 != 1 as libc::c_int {
        str_stream = (*pstm).file as *mut StrSInf;
        free((*str_stream).buff as *mut libc::c_void);
    }
    Pl_Delete_Str_Stream(stm);
}
pub unsafe extern "C" fn Pl_Open_Output_Term_Stream_1(mut stm_word: WamWord) {
    let mut stm: libc::c_int = 0;
    stm = Pl_Add_Str_Stream(
        0 as *mut libc::c_char,
        *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize) as libc::c_int,
    );
    Pl_Get_Integer(stm as PlLong, stm_word);
}
pub unsafe extern "C" fn Pl_Close_Output_Term_Stream_2(
    mut sora_word: WamWord,
    mut sink_term_word: WamWord,
) -> Bool {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut type_0: libc::c_int = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    stm = Pl_Get_Stream_Or_Alias(sora_word, 1 as libc::c_int);
    pstm = *pl_stm_tbl.offset(stm as isize);
    type_0 = ((*pstm).prop).other() as libc::c_int;
    if type_0 < 1 as libc::c_int || type_0 > 3 as libc::c_int {
        Pl_Err_Domain(pl_domain_term_stream_or_alias, sora_word);
    }
    if ((*pstm).prop).input() != 0 {
        Pl_Err_Permission(
            pl_permission_operation_close,
            pl_permission_type_stream,
            sora_word,
        );
    }
    str = Pl_Term_Write_Str_Stream(stm);
    match *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize) {
        1 => {
            if Pl_Un_String_Check(str, sink_term_word) == 0 {
                return 0 as libc::c_int;
            }
        }
        2 => {
            if Pl_Un_Chars_Check(str, sink_term_word) == 0 {
                return 0 as libc::c_int;
            }
        }
        3 => {
            if Pl_Un_Codes_Check(str, sink_term_word) == 0 {
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    Pl_Delete_Str_Stream(stm);
    return 1 as libc::c_int;
}

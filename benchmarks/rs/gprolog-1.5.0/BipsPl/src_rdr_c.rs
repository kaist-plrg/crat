use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    static mut pl_glob_buff: [libc::c_char; 0];
    static mut pl_atom_tbl: *mut AtomInf;
    static mut pl_atom_void: libc::c_int;
    fn Pl_Malloc_Check(
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Calloc_Check(
        nb: libc::c_uint,
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Extend_Array(
        ptbl: *mut *mut libc::c_char,
        nb_elem: *mut libc::c_int,
        elem_size: libc::c_int,
        bzero: Bool,
    );
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    fn Pl_M_Tempnam(dir: *mut libc::c_char, pfx: *mut libc::c_char) -> *mut libc::c_char;
    fn Pl_Get_Atom(atom: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Get_Integer(n: PlLong, start_word: WamWord) -> Bool;
    fn Pl_Get_Nil(start_word: WamWord) -> Bool;
    fn Pl_Get_List(start_word: WamWord) -> Bool;
    fn Pl_Put_Structure(func: libc::c_int, arity: libc::c_int) -> WamWord;
    fn Pl_Unify_Variable() -> WamWord;
    fn Pl_Unify_Value(start_word: WamWord) -> Bool;
    fn Pl_Unify_Atom(atom: libc::c_int) -> Bool;
    fn Pl_Unify_Integer(n: PlLong) -> Bool;
    fn Pl_Create_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Update_Choice_Point(codep_alt: CodePtr, arity: libc::c_int);
    fn Pl_Rd_Integer_Check(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Integer(start_word: WamWord) -> PlLong;
    fn Pl_Rd_Atom_Check(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Atom(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_Boolean(start_word: WamWord) -> libc::c_int;
    fn Pl_Rd_String_Check(start_word: WamWord) -> *mut libc::c_char;
    fn Pl_Check_For_Un_Integer(start_word: WamWord);
    fn Pl_Check_For_Un_Atom(start_word: WamWord);
    fn Pl_Check_For_Un_List(start_word: WamWord);
    fn Pl_Un_Atom_Check(value: libc::c_int, start_word: WamWord) -> Bool;
    fn Pl_Un_String(value: *mut libc::c_char, start_word: WamWord) -> Bool;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_last_output_sora: WamWord;
    static mut pl_stm_input: libc::c_int;
    static mut pl_stm_output: libc::c_int;
    static mut pl_atom_stream: libc::c_int;
    fn Pl_Blt_List(x: WamWord) -> Bool;
    static mut pl_type_integer: libc::c_int;
    static mut pl_type_list: libc::c_int;
    static mut pl_existence_source_sink: libc::c_int;
    static mut pl_existence_sr_descriptor: libc::c_int;
    static mut pl_permission_operation_open: libc::c_int;
    static mut pl_permission_type_source_sink: libc::c_int;
    fn Pl_Err_Type(atom_type: libc::c_int, term: WamWord);
    fn Pl_Err_Existence(atom_object: libc::c_int, term: WamWord);
    fn Pl_Err_Permission(atom_oper: libc::c_int, atom_perm: libc::c_int, term: WamWord);
    static mut pl_sys_var: [PlLong; 0];
    fn Pl_Add_Stream_For_Stdio_File(
        path: *mut libc::c_char,
        mode: libc::c_int,
        text: Bool,
    ) -> libc::c_int;
    fn Pl_Add_Mirror_To_Stream(stm: libc::c_int, m_stm: libc::c_int);
    fn Pl_Get_Stream_Or_Alias(sora_word: WamWord, test_mask: libc::c_int) -> libc::c_int;
    fn Pl_Add_Stream(
        atom_file_name: libc::c_int,
        file: PlLong,
        prop: StmProp,
        fct_getc: StmFct,
        fct_putc: StmFct,
        fct_flush: StmFct,
        fct_close: StmFct,
        fct_tell: StmFct,
        fct_seek: StmFct,
        fct_clearerr: StmFct,
    ) -> libc::c_int;
    fn Pl_Check_Stream_Type(stm: libc::c_int, check_text: Bool, for_input: Bool);
    fn Pl_Stdio_Set_Buffering(f: *mut FILE, buffering: libc::c_int);
    fn Pl_Stream_Getc(pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Stream_Putc(c: libc::c_int, pstm: *mut StmInf);
    fn Pl_Stream_Puts(str: *mut libc::c_char, pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Stream_Printf(
        pstm: *mut StmInf,
        format: *mut libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn Pl_Stream_Set_Position(
        pstm: *mut StmInf,
        whence: libc::c_int,
        offset: PlLong,
        char_count: PlLong,
        line_count: PlLong,
        line_pos: PlLong,
    ) -> libc::c_int;
    fn Pl_Close_Stm(stm: libc::c_int, force: Bool);
    fn Pl_Op_3(prec_word: WamWord, specif_word: WamWord, oper_word: WamWord);
    fn Pl_Set_Prolog_Flag_2(flag_word: WamWord, value_word: WamWord) -> Bool;
    fn Pl_Char_Conversion_2(in_char_word: WamWord, out_char_word: WamWord);
    fn Pl_Write_2(sora_word: WamWord, term_word: WamWord);
    fn Pl_Format_3(sora_word: WamWord, format_word: WamWord, args_word: WamWord);
    fn X1_2473725F63757272656E745F64657363726970746F725F616C74__a0();
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
pub type SRDirType = libc::c_uint;
pub const CHAR_CONVERSION: SRDirType = 2;
pub const SET_PROLOG_FLAG: SRDirType = 1;
pub const OP: SRDirType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sr_one_direct {
    pub type_0: SRDirType,
    pub a: [[WamWord; 3]; 2],
    pub next: PSROneDirect,
    pub prev: PSROneDirect,
}
pub type PSROneDirect = *mut sr_one_direct;
pub type SROneDirect = sr_one_direct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRDirect {
    pub first: *mut SROneDirect,
    pub last: *mut SROneDirect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sr_file {
    pub atom_file_name: libc::c_int,
    pub stm: libc::c_int,
    pub reposition: Bool,
    pub tmp_path: *mut libc::c_char,
    pub tmp_stm: libc::c_int,
    pub next: PSRFile,
    pub eof_reached: Bool,
    pub include_line: libc::c_int,
    pub parent_file: PSRFile,
}
pub type PSRFile = *mut sr_file;
pub type SRFile = sr_file;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sr_module {
    pub atom_module_name: libc::c_int,
    pub i_atom_file_def: libc::c_int,
    pub i_line_def: libc::c_int,
    pub b_atom_file_def: libc::c_int,
    pub b_line_def: libc::c_int,
    pub direct_lst: SRDirect,
    pub next: PSRModule,
}
pub type PSRModule = *mut sr_module;
pub type SRModule = sr_module;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRInf {
    pub in_use: Bool,
    pub close_master_at_end: Bool,
    pub mask: libc::c_int,
    pub file_first: *mut SRFile,
    pub file_last: *mut SRFile,
    pub file_top: *mut SRFile,
    pub next_to_reread: *mut SRFile,
    pub cur_l1: libc::c_int,
    pub cur_l2: libc::c_int,
    pub char_count: libc::c_int,
    pub line_count: libc::c_int,
    pub error_count: libc::c_int,
    pub warning_count: libc::c_int,
    pub out_sora_word: libc::c_int,
    pub direct_lst: SRDirect,
    pub module_lst: *mut SRModule,
    pub cur_module: *mut SRModule,
    pub interface: Bool,
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
static mut sr_tbl: *mut SRInf = 0 as *const SRInf as *mut SRInf;
static mut sr_tbl_size: libc::c_int = 0 as libc::c_int;
static mut sr_last_used: libc::c_int = -(1 as libc::c_int);
static mut cur_sr: *mut SRInf = 0 as *const SRInf as *mut SRInf;
pub unsafe extern "C" fn Pl_SR_Init_Open_2(
    mut desc_word: WamWord,
    mut out_sora_word: WamWord,
) {
    let mut sr: *mut SRInf = 0 as *mut SRInf;
    let mut desc: libc::c_int = 0;
    if sr_tbl.is_null() {
        sr_tbl_size = 8 as libc::c_int;
        sr_last_used = -(1 as libc::c_int);
        sr_tbl = Pl_Calloc_Check(
            sr_tbl_size as libc::c_uint,
            ::std::mem::size_of::<SRInf>() as libc::c_ulong as libc::c_uint,
            b"src_rdr_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            253 as libc::c_int,
        ) as *mut SRInf;
    }
    desc = 0 as libc::c_int;
    while desc < sr_tbl_size {
        if (*sr_tbl.offset(desc as isize)).in_use == 0 {
            break;
        }
        desc += 1;
        desc;
    }
    if desc == sr_tbl_size {
        Pl_Extend_Array(
            &mut sr_tbl as *mut *mut SRInf as *mut *mut libc::c_char,
            &mut sr_tbl_size,
            ::std::mem::size_of::<SRInf>() as libc::c_ulong as libc::c_int,
            1 as libc::c_int,
        );
    }
    if desc > sr_last_used {
        sr_last_used = desc;
    }
    cur_sr = sr_tbl.offset(desc as isize);
    sr = cur_sr;
    if !((*sr).file_top).is_null() {
        free((*sr).file_top as *mut libc::c_void);
        (*sr).file_top = 0 as *mut SRFile;
    }
    (*sr)
        .mask = *pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
        as libc::c_int;
    (*sr).file_first = 0 as *mut SRFile;
    (*sr).file_last = 0 as *mut SRFile;
    (*sr).next_to_reread = 0 as *mut SRFile;
    (*sr).cur_l2 = 0 as libc::c_int;
    (*sr).cur_l1 = (*sr).cur_l2;
    (*sr).char_count = 0 as libc::c_int;
    (*sr).line_count = 0 as libc::c_int;
    (*sr).error_count = 0 as libc::c_int;
    (*sr).warning_count = 0 as libc::c_int;
    if *pl_sys_var.as_mut_ptr().offset(1 as libc::c_int as isize) != 0 {
        Pl_Get_Stream_Or_Alias(out_sora_word, 0 as libc::c_int);
        (*sr).out_sora_word = out_sora_word as libc::c_int;
    } else {
        (*sr)
            .out_sora_word = (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as libc::c_int;
    }
    (*sr).direct_lst.first = 0 as *mut SROneDirect;
    (*sr).direct_lst.last = 0 as *mut SROneDirect;
    (*sr).module_lst = 0 as *mut SRModule;
    (*sr).cur_module = 0 as *mut SRModule;
    (*sr).interface = 0 as libc::c_int;
    Pl_Get_Integer(desc as PlLong, desc_word);
}
pub unsafe extern "C" fn Pl_SR_Open_File_2(
    mut file_name_word: WamWord,
    mut from_stream_word: WamWord,
) {
    let mut sr: *mut SRInf = cur_sr;
    let mut atom_file_name: libc::c_int = 0;
    let mut stm: libc::c_int = 0;
    let mut file: *mut SRFile = 0 as *mut SRFile;
    let mut from_stream: Bool = Pl_Rd_Boolean(from_stream_word);
    let mut master_file: Bool = ((*sr).file_top == 0 as *mut libc::c_void as *mut SRFile)
        as libc::c_int;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut pstm_tmp: *mut StmInf = 0 as *mut StmInf;
    if ((*sr).next_to_reread).is_null() {
        if from_stream != 0 {
            stm = Pl_Get_Stream_Or_Alias(file_name_word, 2 as libc::c_int);
            Pl_Check_Stream_Type(stm, 1 as libc::c_int, 1 as libc::c_int);
            atom_file_name = (**pl_stm_tbl.offset(stm as isize)).atom_file_name;
        } else {
            atom_file_name = Pl_Rd_Atom(file_name_word);
            if strcmp(
                (*pl_atom_tbl.offset(atom_file_name as isize)).name,
                b"user\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                stm = Pl_Add_Stream(
                    0 as libc::c_int,
                    0 as libc::c_int as PlLong,
                    (**pl_stm_tbl.offset(pl_stm_input as isize)).prop,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                );
                **pl_stm_tbl
                    .offset(stm as isize) = **pl_stm_tbl.offset(pl_stm_input as isize);
            } else {
                stm = Pl_Add_Stream_For_Stdio_File(
                    (*pl_atom_tbl.offset(atom_file_name as isize)).name,
                    0 as libc::c_int,
                    1 as libc::c_int,
                );
                if stm < 0 as libc::c_int {
                    if *__errno_location() == 2 as libc::c_int
                        || *__errno_location() == 20 as libc::c_int
                    {
                        Pl_Err_Existence(pl_existence_source_sink, file_name_word);
                    } else {
                        Pl_Err_Permission(
                            pl_permission_operation_open,
                            pl_permission_type_source_sink,
                            file_name_word,
                        );
                    }
                }
            }
        }
        pstm = *pl_stm_tbl.offset(stm as isize);
        file = Pl_Malloc_Check(
            ::std::mem::size_of::<SRFile>() as libc::c_ulong as libc::c_uint,
            b"src_rdr_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            360 as libc::c_int,
        ) as *mut SRFile;
        (*file).atom_file_name = atom_file_name;
        (*file).stm = stm;
        (*file).reposition = ((*pstm).prop).reposition() as Bool;
        if (*file).reposition == 0 {
            (*file)
                .tmp_path = Pl_M_Tempnam(0 as *mut libc::c_char, 0 as *mut libc::c_char);
            (*file)
                .tmp_stm = Pl_Add_Stream_For_Stdio_File(
                (*file).tmp_path,
                1 as libc::c_int,
                1 as libc::c_int,
            );
            if (*file).tmp_stm < 0 as libc::c_int {
                Pl_Fatal_Error(
                    b"cannot create tmp file %s in %s:%d\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (*file).tmp_path,
                    b"src_rdr_c.c\0" as *const u8 as *const libc::c_char,
                    371 as libc::c_int,
                );
            }
            pstm_tmp = *pl_stm_tbl.offset((*file).tmp_stm as isize);
            (*pstm_tmp).atom_file_name = atom_file_name;
            ((*pstm_tmp).prop).set_eof_action(((*pstm).prop).eof_action());
            if ((*pstm_tmp).prop).buffering() as libc::c_int
                != ((*pstm).prop).buffering() as libc::c_int
            {
                ((*pstm_tmp).prop).set_buffering(((*pstm).prop).buffering());
                Pl_Stdio_Set_Buffering(
                    (*pstm_tmp).file as *mut FILE,
                    ((*pstm_tmp).prop).buffering() as libc::c_int,
                );
            }
            Pl_Add_Mirror_To_Stream(stm, (*file).tmp_stm);
        } else {
            (*file).tmp_path = 0 as *mut libc::c_char;
            (*file).tmp_stm = -(1 as libc::c_int);
        }
        (*file).next = 0 as PSRFile;
        if ((*sr).file_first).is_null() {
            (*sr).file_first = file;
        } else {
            (*(*sr).file_last).next = file;
        }
        (*sr).file_last = file;
    } else {
        file = (*sr).next_to_reread;
    }
    (*file).eof_reached = 0 as libc::c_int;
    (*file).parent_file = (*sr).file_top;
    if !((*sr).file_top).is_null() {
        (*(*sr).file_top).include_line = (*sr).cur_l1;
    }
    (*sr).file_top = file;
    if master_file != 0 {
        (*sr).close_master_at_end = (from_stream == 0) as libc::c_int;
        (*sr).in_use = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn Pl_SR_Close_1(mut desc_word: WamWord) {
    let mut current_block: u64;
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    let mut file: *mut SRFile = 0 as *mut SRFile;
    let mut file1: *mut SRFile = 0 as *mut SRFile;
    file = (*sr).file_first;
    if (*sr).close_master_at_end == 0 && ((*file).tmp_path).is_null() {
        current_block = 13368644360268240340;
    } else {
        current_block = 12517898123489920830;
    }
    loop {
        match current_block {
            12517898123489920830 => {
                Pl_Close_Stm((*file).stm, 1 as libc::c_int);
                if !((*file).tmp_path).is_null() {
                    unlink((*file).tmp_path);
                }
                current_block = 13368644360268240340;
            }
            _ => {
                file1 = file;
                file = (*file).next;
                free(file1 as *mut libc::c_void);
                if !file.is_null() {
                    current_block = 12517898123489920830;
                } else {
                    break;
                }
            }
        }
    }
    (*sr).file_top = 0 as *mut SRFile;
    Common_Clean(sr, 0 as libc::c_int);
    (*sr).in_use = 0 as libc::c_int;
    while sr_last_used >= 0 as libc::c_int
        && (*sr_tbl.offset(sr_last_used as isize)).in_use == 0
    {
        sr_last_used -= 1;
        sr_last_used;
    }
}
unsafe extern "C" fn Common_Clean(mut sr: *mut SRInf, mut for_reread: Bool) {
    let mut o: *mut SROneDirect = 0 as *mut SROneDirect;
    let mut o1: *mut SROneDirect = 0 as *mut SROneDirect;
    let mut m: *mut SRModule = 0 as *mut SRModule;
    let mut m1: *mut SRModule = 0 as *mut SRModule;
    if for_reread != 0
        || (*sr).mask & (1 as libc::c_int) << 18 as libc::c_int != 0 as libc::c_int
    {
        if !((*sr).cur_module).is_null() {
            Undo_Directives(&mut (*(*sr).cur_module).direct_lst);
        }
        Undo_Directives(&mut (*sr).direct_lst);
    }
    o = (*sr).direct_lst.first;
    while !o.is_null() {
        o1 = o;
        o = (*o).next;
        free(o1 as *mut libc::c_void);
    }
    m = (*sr).module_lst;
    while !m.is_null() {
        o = (*m).direct_lst.first;
        while !o.is_null() {
            o1 = o;
            o = (*o).next;
            free(o1 as *mut libc::c_void);
        }
        m1 = m;
        m = (*m).next;
        free(m1 as *mut libc::c_void);
    }
    if for_reread != 0 {
        (*sr).cur_l2 = 0 as libc::c_int;
        (*sr).cur_l1 = (*sr).cur_l2;
        (*sr).char_count = 0 as libc::c_int;
        (*sr).line_count = 0 as libc::c_int;
        (*sr).error_count = 0 as libc::c_int;
        (*sr).warning_count = 0 as libc::c_int;
        (*sr).direct_lst.first = 0 as *mut SROneDirect;
        (*sr).direct_lst.last = 0 as *mut SROneDirect;
        (*sr).module_lst = 0 as *mut SRModule;
        (*sr).cur_module = 0 as *mut SRModule;
        (*sr).interface = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn Pl_SR_New_Pass_1(mut desc_word: WamWord) -> Bool {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut file: *mut SRFile = 0 as *mut SRFile;
    if (*sr).mask & (1 as libc::c_int) << 16 as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*(*sr).file_last).reposition == 0 && (*(*sr).file_top).eof_reached == 0 {
        pstm = *pl_stm_tbl.offset((*(*sr).file_top).stm as isize);
        while Pl_Stream_Getc(pstm) != -(1 as libc::c_int) {}
    }
    (*sr).next_to_reread = (*(*sr).file_first).next;
    file = (*sr).file_first;
    while !file.is_null() {
        if (*file).reposition != 0 {
            Pl_Stream_Set_Position(
                *pl_stm_tbl.offset((*file).stm as isize),
                0 as libc::c_int,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
            );
        } else {
            if file != (*sr).file_first || (*sr).close_master_at_end != 0 {
                Pl_Close_Stm((*file).stm, 1 as libc::c_int);
            }
            Pl_Close_Stm((*file).tmp_stm, 1 as libc::c_int);
            (*file)
                .stm = Pl_Add_Stream_For_Stdio_File(
                (*file).tmp_path,
                0 as libc::c_int,
                1 as libc::c_int,
            );
            (*file).reposition = 1 as libc::c_int;
        }
        file = (*file).next;
    }
    (*sr).file_top = (*sr).file_first;
    (*(*sr).file_top).eof_reached = 0 as libc::c_int;
    Common_Clean(sr, 1 as libc::c_int);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_SR_Add_Directive_7(
    mut type_word: WamWord,
    mut d1_word: WamWord,
    mut d2_word: WamWord,
    mut d3_word: WamWord,
    mut u1_word: WamWord,
    mut u2_word: WamWord,
    mut u3_word: WamWord,
) {
    let mut sr: *mut SRInf = cur_sr;
    let mut d: *mut SRDirect = 0 as *mut SRDirect;
    let mut one: SROneDirect = SROneDirect {
        type_0: OP,
        a: [[0; 3]; 2],
        next: 0 as *mut sr_one_direct,
        prev: 0 as *mut sr_one_direct,
    };
    let mut o: *mut SROneDirect = 0 as *mut SROneDirect;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    if ((*sr).cur_module).is_null() {
        d = &mut (*sr).direct_lst;
    } else {
        d = &mut (*(*sr).cur_module).direct_lst;
    }
    o = &mut one;
    (*o).type_0 = Pl_Rd_Integer(type_word) as SRDirType;
    let mut deref_last_word: WamWord = 0;
    word = d1_word;
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
    (*o).a[0 as libc::c_int as usize][0 as libc::c_int as usize] = word;
    let mut deref_last_word_0: WamWord = 0;
    word = d2_word;
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
    (*o).a[0 as libc::c_int as usize][1 as libc::c_int as usize] = word;
    let mut deref_last_word_1: WamWord = 0;
    word = d3_word;
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
    (*o).a[0 as libc::c_int as usize][2 as libc::c_int as usize] = word;
    let mut deref_last_word_2: WamWord = 0;
    word = u1_word;
    loop {
        deref_last_word_2 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_2) {
            break;
        }
    }
    (*o).a[1 as libc::c_int as usize][0 as libc::c_int as usize] = word;
    let mut deref_last_word_3: WamWord = 0;
    word = u2_word;
    loop {
        deref_last_word_3 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_3) {
            break;
        }
    }
    (*o).a[1 as libc::c_int as usize][1 as libc::c_int as usize] = word;
    let mut deref_last_word_4: WamWord = 0;
    word = u3_word;
    loop {
        deref_last_word_4 = word;
        tag_mask = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong) as WamWord;
        if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
            break;
        }
        word = *(word as *mut WamWord);
        if !(word != deref_last_word_4) {
            break;
        }
    }
    (*o).a[1 as libc::c_int as usize][2 as libc::c_int as usize] = word;
    (*o).next = 0 as PSROneDirect;
    (*o).prev = (*d).last;
    Exec_One_Directive(o, 0 as libc::c_int);
    o = Pl_Malloc_Check(
        ::std::mem::size_of::<SROneDirect>() as libc::c_ulong as libc::c_uint,
        b"src_rdr_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        608 as libc::c_int,
    ) as *mut SROneDirect;
    *o = one;
    if ((*d).first).is_null() {
        (*d).first = o;
    } else {
        (*(*d).last).next = o;
    }
    (*d).last = o;
}
unsafe extern "C" fn Do_Directives(mut direct: *mut SRDirect) {
    let mut o: *mut SROneDirect = 0 as *mut SROneDirect;
    o = (*direct).first;
    while !o.is_null() {
        Exec_One_Directive(o, 0 as libc::c_int);
        o = (*o).next;
    }
}
unsafe extern "C" fn Undo_Directives(mut direct: *mut SRDirect) {
    let mut o: *mut SROneDirect = 0 as *mut SROneDirect;
    o = (*direct).last;
    while !o.is_null() {
        Exec_One_Directive(o, 1 as libc::c_int);
        o = (*o).prev;
    }
}
unsafe extern "C" fn Exec_One_Directive(
    mut o: *mut SROneDirect,
    mut do_undo: libc::c_int,
) {
    match (*o).type_0 as libc::c_uint {
        0 => {
            Pl_Op_3(
                (*o).a[do_undo as usize][0 as libc::c_int as usize],
                (*o).a[do_undo as usize][1 as libc::c_int as usize],
                (*o).a[do_undo as usize][2 as libc::c_int as usize],
            );
        }
        1 => {
            Pl_Set_Prolog_Flag_2(
                (*o).a[do_undo as usize][0 as libc::c_int as usize],
                (*o).a[do_undo as usize][1 as libc::c_int as usize],
            );
        }
        2 => {
            Pl_Char_Conversion_2(
                (*o).a[do_undo as usize][0 as libc::c_int as usize],
                (*o).a[do_undo as usize][1 as libc::c_int as usize],
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn Pl_SR_Change_Options_0() {
    let mut sr: *mut SRInf = cur_sr;
    let mut reread_mask: libc::c_int = (*sr).mask
        & (1 as libc::c_int) << 16 as libc::c_int;
    (*sr)
        .mask = (*pl_sys_var.as_mut_ptr().offset(0 as libc::c_int as isize)
        & !((1 as libc::c_int) << 16 as libc::c_int) as libc::c_long
        | reread_mask as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn Pl_SR_Get_Stm_For_Read_Term_1(mut stm_word: WamWord) {
    let mut sr: *mut SRInf = cur_sr;
    let mut file: *mut SRFile = 0 as *mut SRFile;
    loop {
        file = (*sr).file_top;
        if (*file).eof_reached == 0 {
            break;
        }
        if ((*file).parent_file).is_null() {
            break;
        }
        (*sr)
            .char_count = ((*sr).char_count as libc::c_long
            + (**pl_stm_tbl.offset((*file).stm as isize)).char_count) as libc::c_int;
        (*sr)
            .line_count = ((*sr).line_count as libc::c_long
            + (**pl_stm_tbl.offset((*file).stm as isize)).line_count) as libc::c_int;
        (*sr).file_top = (*file).parent_file;
    }
    Pl_Get_Integer((*(*sr).file_top).stm as PlLong, stm_word);
}
pub unsafe extern "C" fn Pl_SR_EOF_Reached_1(mut err_word: WamWord) -> Bool {
    let mut sr: *mut SRInf = cur_sr;
    (*(*sr).file_top).eof_reached = 1 as libc::c_int;
    if ((*(*sr).file_top).parent_file).is_null() {
        if !((*sr).cur_module).is_null() {
            sprintf(
                pl_glob_buff.as_mut_ptr(),
                b"end_%s(%s) not encoutered - assumed found\0" as *const u8
                    as *const libc::c_char,
                if (*sr).interface != 0 {
                    b"module\0" as *const u8 as *const libc::c_char
                } else {
                    b"body\0" as *const u8 as *const libc::c_char
                },
                (*pl_atom_tbl.offset((*(*sr).cur_module).atom_module_name as isize)).name,
            );
            Close_Current_Module();
            Pl_Un_String(pl_glob_buff.as_mut_ptr(), err_word);
        }
        return 1 as libc::c_int;
    }
    return (*sr).mask & (1 as libc::c_int) << 17 as libc::c_int;
}
pub unsafe extern "C" fn Pl_SR_Update_Position_0() {
    let mut sr: *mut SRInf = cur_sr;
    (*sr).cur_l1 = pl_last_read_line;
    (*sr)
        .cur_l2 = (**pl_stm_tbl.offset((*(*sr).file_top).stm as isize)).line_count
        as libc::c_int;
    if (**pl_stm_tbl.offset((*(*sr).file_top).stm as isize)).line_pos
        > 0 as libc::c_int as libc::c_long
    {
        (*sr).cur_l2 += 1;
        (*sr).cur_l2;
    }
}
pub unsafe extern "C" fn Pl_SR_Start_Module_3(
    mut module_name_word: WamWord,
    mut interface_word: WamWord,
    mut err_word: WamWord,
) {
    let mut current_block: u64;
    let mut sr: *mut SRInf = cur_sr;
    let mut atom_module_name: libc::c_int = Pl_Rd_Atom_Check(module_name_word);
    let mut interface: Bool = Pl_Rd_Boolean(interface_word);
    let mut m: *mut SRModule = 0 as *mut SRModule;
    *pl_glob_buff.as_mut_ptr() = '\0' as i32 as libc::c_char;
    m = (*sr).module_lst;
    while !m.is_null() {
        if (*m).atom_module_name == atom_module_name {
            break;
        }
        m = (*m).next;
    }
    if m.is_null() {
        if interface == 0 {
            sprintf(
                pl_glob_buff.as_mut_ptr(),
                b"module(%s) not encoutered - interface assumed empty\0" as *const u8
                    as *const libc::c_char,
                (*pl_atom_tbl.offset(atom_module_name as isize)).name,
            );
        }
        m = Pl_Malloc_Check(
            ::std::mem::size_of::<SRModule>() as libc::c_ulong as libc::c_uint,
            b"src_rdr_c.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            803 as libc::c_int,
        ) as *mut SRModule;
        (*m).atom_module_name = atom_module_name;
        (*m).i_atom_file_def = (*(*sr).file_top).atom_file_name;
        (*m).i_line_def = (*sr).cur_l1;
        (*m).b_atom_file_def = -(1 as libc::c_int);
        (*m).b_line_def = 0 as libc::c_int;
        (*m).direct_lst.first = 0 as *mut SROneDirect;
        (*m).direct_lst.last = 0 as *mut SROneDirect;
        (*m).next = (*sr).module_lst;
        (*sr).module_lst = m;
        current_block = 6009453772311597924;
    } else if interface != 0 {
        sprintf(
            pl_glob_buff.as_mut_ptr(),
            b"module(%s) already found at %s:%d - directive ignored\0" as *const u8
                as *const libc::c_char,
            (*pl_atom_tbl.offset(atom_module_name as isize)).name,
            (*pl_atom_tbl.offset((*m).i_atom_file_def as isize)).name,
            (*m).i_line_def,
        );
        current_block = 6845184325646427720;
    } else {
        current_block = 6009453772311597924;
    }
    match current_block {
        6009453772311597924 => {
            if !((*sr).cur_module).is_null() {
                sprintf(
                    pl_glob_buff.as_mut_ptr(),
                    b"end_%s(%s) not encoutered - assumed found\0" as *const u8
                        as *const libc::c_char,
                    if (*sr).interface != 0 {
                        b"module\0" as *const u8 as *const libc::c_char
                    } else {
                        b"body\0" as *const u8 as *const libc::c_char
                    },
                    (*pl_atom_tbl.offset((*(*sr).cur_module).atom_module_name as isize))
                        .name,
                );
                Close_Current_Module();
            }
            if interface == 0 {
                (*m).b_atom_file_def = (*(*sr).file_top).atom_file_name;
                (*m).b_line_def = (*sr).cur_l1;
            }
            (*sr).cur_module = m;
            (*sr).interface = interface;
            Do_Directives(&mut (*m).direct_lst);
            if *pl_glob_buff.as_mut_ptr() != 0 {
                current_block = 6845184325646427720;
            } else {
                current_block = 14648156034262866959;
            }
        }
        _ => {}
    }
    match current_block {
        6845184325646427720 => {
            Pl_Un_String(pl_glob_buff.as_mut_ptr(), err_word);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn Pl_SR_Stop_Module_3(
    mut module_name_word: WamWord,
    mut interface_word: WamWord,
    mut err_word: WamWord,
) {
    let mut current_block: u64;
    let mut sr: *mut SRInf = cur_sr;
    let mut atom_module_name: libc::c_int = Pl_Rd_Atom_Check(module_name_word);
    let mut interface: Bool = Pl_Rd_Boolean(interface_word);
    let mut m: *mut SRModule = (*sr).cur_module;
    *pl_glob_buff.as_mut_ptr() = '\0' as i32 as libc::c_char;
    if m.is_null() {
        sprintf(
            pl_glob_buff.as_mut_ptr(),
            b"corresponding directive %s(%s) not found - directive ignored\0"
                as *const u8 as *const libc::c_char,
            if interface != 0 {
                b"module\0" as *const u8 as *const libc::c_char
            } else {
                b"body\0" as *const u8 as *const libc::c_char
            },
            (*pl_atom_tbl.offset(atom_module_name as isize)).name,
        );
        current_block = 1102246537255141104;
    } else {
        if interface != (*sr).interface || atom_module_name != (*m).atom_module_name {
            sprintf(
                pl_glob_buff.as_mut_ptr(),
                b"directive mismatch wrt %s:%d - replaced by end_%s(%s)\0" as *const u8
                    as *const libc::c_char,
                if (*sr).interface != 0 {
                    (*pl_atom_tbl.offset((*m).i_atom_file_def as isize)).name
                } else {
                    (*pl_atom_tbl.offset((*m).b_atom_file_def as isize)).name
                },
                if (*sr).interface != 0 { (*m).i_line_def } else { (*m).b_line_def },
                if (*sr).interface != 0 {
                    b"module\0" as *const u8 as *const libc::c_char
                } else {
                    b"body\0" as *const u8 as *const libc::c_char
                },
                (*pl_atom_tbl.offset((*m).atom_module_name as isize)).name,
            );
        }
        Close_Current_Module();
        if *pl_glob_buff.as_mut_ptr() != 0 {
            current_block = 1102246537255141104;
        } else {
            current_block = 13183875560443969876;
        }
    }
    match current_block {
        1102246537255141104 => {
            Pl_Un_String(pl_glob_buff.as_mut_ptr(), err_word);
        }
        _ => {}
    };
}
unsafe extern "C" fn Close_Current_Module() {
    let mut sr: *mut SRInf = cur_sr;
    Undo_Directives(&mut (*(*sr).cur_module).direct_lst);
    (*sr).cur_module = 0 as *mut SRModule;
}
pub unsafe extern "C" fn Pl_SR_Current_Descriptor_1(mut desc_word: WamWord) -> Bool {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut desc: libc::c_int = 0 as libc::c_int;
    let mut deref_last_word: WamWord = 0;
    word = desc_word;
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
        desc = (word << 0 as libc::c_int >> 3 as libc::c_int) as libc::c_int;
        return (desc >= 0 as libc::c_int && desc <= sr_last_used
            && (*sr_tbl.offset(desc as isize)).in_use != 0) as libc::c_int;
    }
    if tag_mask as libc::c_ulong != 0 as libc::c_int as PlULong {
        Pl_Err_Type(pl_type_integer, word);
    }
    while desc <= sr_last_used {
        if (*sr_tbl.offset(desc as isize)).in_use != 0 {
            break;
        }
        desc += 1;
        desc;
    }
    if desc >= sr_last_used {
        if desc > sr_last_used {
            return 0 as libc::c_int;
        }
    } else {
        *pl_reg_bank.offset(0 as libc::c_int as isize) = desc_word;
        *pl_reg_bank
            .offset(1 as libc::c_int as isize) = (desc + 1 as libc::c_int) as WamWord;
        Pl_Create_Choice_Point(
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                CodePtr,
            >(
                Some(
                    ::std::mem::transmute::<
                        unsafe extern "C" fn() -> (),
                        unsafe extern "C" fn() -> (),
                    >(X1_2473725F63757272656E745F64657363726970746F725F616C74__a0),
                ),
            ),
            2 as libc::c_int,
        );
    }
    return Pl_Get_Integer(desc as PlLong, desc_word);
}
pub unsafe extern "C" fn Pl_SR_Current_Descriptor_Alt_0() -> Bool {
    let mut desc_word: WamWord = 0;
    let mut desc: libc::c_int = 0;
    Pl_Update_Choice_Point(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            CodePtr,
        >(
            Some(
                ::std::mem::transmute::<
                    unsafe extern "C" fn() -> (),
                    unsafe extern "C" fn() -> (),
                >(X1_2473725F63757272656E745F64657363726970746F725F616C74__a0),
            ),
        ),
        0 as libc::c_int,
    );
    desc_word = *(&mut *B.offset((-(9 as libc::c_int) - 0 as libc::c_int) as isize)
        as *mut WamWord);
    desc = *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
        as *mut WamWord) as libc::c_int;
    while desc <= sr_last_used {
        if (*sr_tbl.offset(desc as isize)).in_use != 0 {
            break;
        }
        desc += 1;
        desc;
    }
    if desc >= sr_last_used {
        B = *(&mut *B.offset(-(5 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        let ref mut fresh0 = *(pl_reg_bank as *mut WamWordP)
            .offset((256 as libc::c_int + 0 as libc::c_int) as isize);
        *fresh0 = *(&mut *B.offset(-(6 as libc::c_int) as isize) as *mut WamWord
            as *mut *mut WamWord);
        if desc > sr_last_used {
            return 0 as libc::c_int;
        }
    } else {
        *(&mut *B.offset((-(9 as libc::c_int) - 1 as libc::c_int) as isize)
            as *mut WamWord) = (desc + 1 as libc::c_int) as WamWord;
    }
    return Pl_Get_Integer(desc as PlLong, desc_word);
}
pub unsafe extern "C" fn Pl_SR_Is_Bit_Set_1(mut bit_word: WamWord) -> Bool {
    return (*cur_sr).mask & (1 as libc::c_int) << Pl_Rd_Integer(bit_word);
}
pub unsafe extern "C" fn Pl_SR_Get_Stm_2(
    mut desc_word: WamWord,
    mut stm_word: WamWord,
) -> Bool {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    return Pl_Get_Integer((*(*sr).file_top).stm as PlLong, stm_word);
}
pub unsafe extern "C" fn Pl_SR_Get_Module_3(
    mut desc_word: WamWord,
    mut module_name_word: WamWord,
    mut interface_word: WamWord,
) -> Bool {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    let mut m: *mut SRModule = (*sr).cur_module;
    Pl_Check_For_Un_Atom(module_name_word);
    Pl_Check_For_Un_Atom(interface_word);
    if m.is_null() {
        return 0 as libc::c_int;
    }
    if Pl_Get_Atom((*m).atom_module_name, module_name_word) == 0 {
        return 0 as libc::c_int;
    }
    if (*sr).interface != 0 {
        return Pl_Un_String(
            b"interface\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            interface_word,
        );
    }
    return Pl_Un_String(
        b"body\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        interface_word,
    );
}
pub unsafe extern "C" fn Pl_SR_Get_File_Name_2(
    mut desc_word: WamWord,
    mut file_name_word: WamWord,
) -> Bool {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    return Pl_Un_Atom_Check((*(*sr).file_top).atom_file_name, file_name_word);
}
pub unsafe extern "C" fn Pl_SR_Get_Position_3(
    mut desc_word: WamWord,
    mut l1_word: WamWord,
    mut l2_word: WamWord,
) -> Bool {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    Pl_Check_For_Un_Integer(l1_word);
    Pl_Check_For_Un_Integer(l2_word);
    return (Pl_Get_Integer((*sr).cur_l1 as PlLong, l1_word) != 0
        && Pl_Get_Integer((*sr).cur_l2 as PlLong, l2_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_SR_Get_Include_List_2(
    mut desc_word: WamWord,
    mut list_word: WamWord,
) -> Bool {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    let mut file: *mut SRFile = 0 as *mut SRFile;
    let mut word: WamWord = 0;
    Pl_Check_For_Un_List(list_word);
    file = (*(*sr).file_top).parent_file;
    while !file.is_null() {
        word = Pl_Put_Structure(
            ':' as i32 as libc::c_uchar as libc::c_int,
            2 as libc::c_int,
        );
        Pl_Unify_Atom((*file).atom_file_name);
        Pl_Unify_Integer((*file).include_line as PlLong);
        if Pl_Get_List(list_word) == 0 || Pl_Unify_Value(word) == 0 {
            return 0 as libc::c_int;
        }
        list_word = Pl_Unify_Variable();
        file = (*file).parent_file;
    }
    return Pl_Get_Nil(list_word);
}
pub unsafe extern "C" fn Pl_SR_Get_Include_Stream_List_2(
    mut desc_word: WamWord,
    mut list_word: WamWord,
) -> Bool {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    let mut file: *mut SRFile = 0 as *mut SRFile;
    let mut word: WamWord = 0;
    Pl_Check_For_Un_List(list_word);
    file = (*(*sr).file_top).parent_file;
    while !file.is_null() {
        word = Pl_Put_Structure(pl_atom_stream, 1 as libc::c_int);
        Pl_Unify_Integer((*file).stm as PlLong);
        if Pl_Get_List(list_word) == 0 || Pl_Unify_Value(word) == 0 {
            return 0 as libc::c_int;
        }
        list_word = Pl_Unify_Variable();
        file = (*file).parent_file;
    }
    return Pl_Get_Nil(list_word);
}
pub unsafe extern "C" fn Pl_SR_Get_Size_Counters_3(
    mut desc_word: WamWord,
    mut chars_word: WamWord,
    mut lines_word: WamWord,
) -> Bool {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    let mut file: *mut SRFile = 0 as *mut SRFile;
    let mut char_count: libc::c_int = 0;
    let mut line_count: libc::c_int = 0;
    Pl_Check_For_Un_Integer(chars_word);
    Pl_Check_For_Un_Integer(lines_word);
    char_count = (*sr).char_count;
    line_count = (*sr).line_count;
    file = (*sr).file_top;
    while !file.is_null() {
        char_count = (char_count as libc::c_long
            + (**pl_stm_tbl.offset((*file).stm as isize)).char_count) as libc::c_int;
        line_count = (line_count as libc::c_long
            + (**pl_stm_tbl.offset((*file).stm as isize)).line_count) as libc::c_int;
        file = (*file).parent_file;
    }
    return (Pl_Get_Integer(char_count as PlLong, chars_word) != 0
        && Pl_Get_Integer(line_count as PlLong, lines_word) != 0) as libc::c_int;
}
pub unsafe extern "C" fn Pl_SR_Get_Error_Counters_3(
    mut desc_word: WamWord,
    mut errors_word: WamWord,
    mut warnings_word: WamWord,
) -> Bool {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    Pl_Check_For_Un_Integer(errors_word);
    Pl_Check_For_Un_Integer(warnings_word);
    return (Pl_Get_Integer((*sr).error_count as PlLong, errors_word) != 0
        && Pl_Get_Integer((*sr).warning_count as PlLong, warnings_word) != 0)
        as libc::c_int;
}
pub unsafe extern "C" fn Pl_SR_Set_Error_Counters_3(
    mut desc_word: WamWord,
    mut errors_word: WamWord,
    mut warnings_word: WamWord,
) {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 0 as libc::c_int);
    let mut errors: libc::c_int = Pl_Rd_Integer_Check(errors_word) as libc::c_int;
    let mut warnings: libc::c_int = Pl_Rd_Integer_Check(warnings_word) as libc::c_int;
    (*sr).error_count = errors;
    (*sr).warning_count = warnings;
}
pub unsafe extern "C" fn Pl_SR_Check_Descriptor_1(mut desc_word: WamWord) {
    Get_Descriptor(desc_word, 0 as libc::c_int);
}
unsafe extern "C" fn Get_Descriptor(
    mut desc_word: WamWord,
    mut accept_none: Bool,
) -> *mut SRInf {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut desc: libc::c_int = 0;
    let mut atom: libc::c_int = 0;
    if accept_none != 0 {
        let mut deref_last_word: WamWord = 0;
        word = desc_word;
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
        atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
        if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong
            && strcmp(
                (*pl_atom_tbl.offset(atom as isize)).name,
                b"none\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            cur_sr = 0 as *mut SRInf;
            return cur_sr;
        }
    }
    desc = Pl_Rd_Integer_Check(desc_word) as libc::c_int;
    if desc < 0 as libc::c_int || desc > sr_last_used
        || (*sr_tbl.offset(desc as isize)).in_use == 0
    {
        Pl_Err_Existence(pl_existence_sr_descriptor, desc_word);
    }
    cur_sr = sr_tbl.offset(desc as isize);
    *pl_sys_var
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = (*cur_sr).mask as PlLong;
    return cur_sr;
}
pub unsafe extern "C" fn Pl_SR_Write_Message_4(
    mut desc_word: WamWord,
    mut type_word: WamWord,
    mut format_word: WamWord,
    mut args_word: WamWord,
) {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 1 as libc::c_int);
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut atom_file_name: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut l2c: libc::c_int = 0;
    let mut sora_word: WamWord = 0;
    if !sr.is_null() {
        sora_word = (*sr).out_sora_word as WamWord;
        atom_file_name = (*(*sr).file_top).atom_file_name;
        l1 = (*sr).cur_l1;
        l2c = (*sr).cur_l2;
    } else {
        sora_word = (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        atom_file_name = pl_atom_void;
        l1 = 0 as libc::c_int;
        l2c = 0 as libc::c_int;
    }
    pstm = Write_Location(
        sora_word,
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        atom_file_name,
        l1,
        l2c,
    );
    Write_Message_Text(pstm, sora_word, type_word, format_word, args_word);
}
pub unsafe extern "C" fn Pl_SR_Write_Message_6(
    mut desc_word: WamWord,
    mut l1_word: WamWord,
    mut l2c_word: WamWord,
    mut type_word: WamWord,
    mut format_word: WamWord,
    mut args_word: WamWord,
) {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 1 as libc::c_int);
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut atom_file_name: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut l2c: libc::c_int = 0;
    let mut sora_word: WamWord = 0;
    if !sr.is_null() {
        sora_word = (*sr).out_sora_word as WamWord;
        atom_file_name = (*(*sr).file_top).atom_file_name;
    } else {
        sora_word = (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
        atom_file_name = pl_atom_void;
    }
    l1 = Pl_Rd_Integer_Check(l1_word) as libc::c_int;
    l2c = Pl_Rd_Integer_Check(l2c_word) as libc::c_int;
    pstm = Write_Location(
        sora_word,
        (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord,
        atom_file_name,
        l1,
        l2c,
    );
    Write_Message_Text(pstm, sora_word, type_word, format_word, args_word);
}
pub unsafe extern "C" fn Pl_SR_Write_Message_8(
    mut desc_word: WamWord,
    mut list_word: WamWord,
    mut file_name_word: WamWord,
    mut l1_word: WamWord,
    mut l2c_word: WamWord,
    mut type_word: WamWord,
    mut format_word: WamWord,
    mut args_word: WamWord,
) {
    let mut sr: *mut SRInf = Get_Descriptor(desc_word, 1 as libc::c_int);
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut atom_file_name: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut l2c: libc::c_int = 0;
    let mut sora_word: WamWord = 0;
    if Pl_Blt_List(list_word) == 0 {
        Pl_Err_Type(pl_type_list, list_word);
    }
    if !sr.is_null() {
        sora_word = (*sr).out_sora_word as WamWord;
    } else {
        sora_word = (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) as WamWord;
    }
    sora_word = (*sr).out_sora_word as WamWord;
    atom_file_name = Pl_Rd_Atom_Check(file_name_word);
    l1 = Pl_Rd_Integer_Check(l1_word) as libc::c_int;
    l2c = Pl_Rd_Integer_Check(l2c_word) as libc::c_int;
    pstm = Write_Location(sora_word, list_word, atom_file_name, l1, l2c);
    Write_Message_Text(pstm, sora_word, type_word, format_word, args_word);
}
unsafe extern "C" fn Write_Location(
    mut sora_word: WamWord,
    mut list_word: WamWord,
    mut atom_file_name: libc::c_int,
    mut l1: libc::c_int,
    mut l2c: libc::c_int,
) -> *mut StmInf {
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut lst_adr: *mut WamWord = 0 as *mut WamWord;
    let mut first: Bool = 0;
    let mut sr: *mut SRInf = cur_sr;
    let mut file: *mut SRFile = 0 as *mut SRFile;
    let mut char_count: libc::c_int = 0;
    stm = if sora_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong)
    {
        pl_stm_output
    } else {
        Pl_Get_Stream_Or_Alias(sora_word, 3 as libc::c_int)
    };
    pstm = *pl_stm_tbl.offset(stm as isize);
    pl_last_output_sora = sora_word;
    Pl_Check_Stream_Type(stm, 1 as libc::c_int, 0 as libc::c_int);
    if list_word as libc::c_ulong
        == (0 as libc::c_int as PlLong as libc::c_ulong)
            .wrapping_add(0 as libc::c_int as PlULong) && !sr.is_null()
    {
        file = (*(*sr).file_top).parent_file;
    }
    first = 1 as libc::c_int;
    loop {
        if list_word as libc::c_ulong
            != (0 as libc::c_int as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong)
        {
            let mut deref_last_word: WamWord = 0;
            word = list_word;
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
            if word as libc::c_ulong
                == (((256 as libc::c_int as PlLong) << 3 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_add(0x3 as libc::c_int as PlULong)
            {
                break;
            }
        } else if file.is_null() {
            break;
        }
        if first != 0 {
            if (*pstm).line_pos != 0 as libc::c_int as libc::c_long {
                Pl_Stream_Putc('\n' as i32, pstm);
            }
            Pl_Stream_Puts(
                b"In file included from \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pstm,
            );
        } else {
            Pl_Stream_Printf(
                pstm,
                b",\n%*s from \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                16 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char,
            );
        }
        if list_word as libc::c_ulong
            != (0 as libc::c_int as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong)
        {
            lst_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            Pl_Write_2(sora_word, *lst_adr.offset(0 as libc::c_int as isize));
            list_word = *lst_adr.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
        } else {
            Pl_Stream_Printf(
                pstm,
                b"%s:%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*pl_atom_tbl.offset((*file).atom_file_name as isize)).name,
                (*file).include_line,
            );
            file = (*file).parent_file;
        }
        first = 0 as libc::c_int;
    }
    if first == 0 {
        Pl_Stream_Puts(
            b":\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            pstm,
        );
    }
    char_count = (*pstm).char_count as libc::c_int;
    if atom_file_name != pl_atom_void {
        Pl_Stream_Puts((*pl_atom_tbl.offset(atom_file_name as isize)).name, pstm);
    }
    if l1 > 0 as libc::c_int {
        Pl_Stream_Printf(
            pstm,
            b":%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            l1,
        );
        if l2c != l1 {
            if l2c > 0 as libc::c_int {
                Pl_Stream_Printf(
                    pstm,
                    b"--%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    l2c,
                );
            } else {
                Pl_Stream_Printf(
                    pstm,
                    b":%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    -l2c,
                );
            }
        }
    }
    if char_count as libc::c_long != (*pstm).char_count {
        Pl_Stream_Puts(
            b": \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            pstm,
        );
    }
    return pstm;
}
unsafe extern "C" fn Write_Message_Text(
    mut pstm: *mut StmInf,
    mut sora_word: WamWord,
    mut type_word: WamWord,
    mut format_word: WamWord,
    mut args_word: WamWord,
) {
    let mut sr: *mut SRInf = cur_sr;
    let mut type_0: *mut libc::c_char = Pl_Rd_String_Check(type_word);
    if *type_0 != 0 {
        Pl_Stream_Printf(
            pstm,
            b"%s: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            type_0,
        );
        if !sr.is_null() {
            if !(strstr(type_0, b"error\0" as *const u8 as *const libc::c_char))
                .is_null()
                || !(strstr(type_0, b"exception\0" as *const u8 as *const libc::c_char))
                    .is_null()
            {
                (*sr).error_count += 1;
                (*sr).error_count;
            } else if !(strstr(type_0, b"warning\0" as *const u8 as *const libc::c_char))
                .is_null()
            {
                (*sr).warning_count += 1;
                (*sr).warning_count;
            }
        }
    }
    Pl_Format_3(sora_word, format_word, args_word);
}

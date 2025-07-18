use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn clearerr(__stream: *mut FILE);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut pl_buff_signal_reg: [WamWord; 0];
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
    fn Pl_Hash_First(tbl: *mut libc::c_char, scan: *mut HashScan) -> *mut libc::c_char;
    fn Pl_Hash_Next(scan: *mut HashScan) -> *mut libc::c_char;
    static mut pl_le_mode: libc::c_int;
    fn Pl_Execute_A_Continuation(codep: CodePtr);
    fn Pl_Create_Allocate_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
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
    fn Pl_Realloc_Check(
        ptr: *mut libc::c_char,
        size: libc::c_uint,
        src_file: *mut libc::c_char,
        src_line: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_Extend_Table_If_Needed(hash_tbl: *mut *mut libc::c_char);
    fn Pl_Extend_Array(
        ptbl: *mut *mut libc::c_char,
        nb_elem: *mut libc::c_int,
        elem_size: libc::c_int,
        bzero: Bool,
    );
    fn Pl_Fatal_Error(format: *mut libc::c_char, _: ...);
    static mut pl_permission_type_stream: libc::c_int;
    fn Pl_Err_Permission(atom_oper: libc::c_int, atom_perm: libc::c_int, term: WamWord);
    static mut pl_permission_operation_output: libc::c_int;
    static mut pl_permission_operation_input: libc::c_int;
    static mut pl_existence_stream: libc::c_int;
    fn Pl_Err_Existence(atom_object: libc::c_int, term: WamWord);
    static mut pl_domain_stream_or_alias: libc::c_int;
    fn Pl_Err_Domain(atom_domain: libc::c_int, term: WamWord);
    fn Pl_Err_Instantiation();
    static mut pl_permission_type_text_stream: libc::c_int;
    static mut pl_permission_type_binary_stream: libc::c_int;
    static mut pl_sys_var: [PlLong; 0];
    static mut pl_permission_type_past_end_of_stream: libc::c_int;
    fn Pl_LE_FGets(
        str: *mut libc::c_char,
        size: libc::c_int,
        prompt: *mut libc::c_char,
        display_prompt: libc::c_int,
    ) -> *mut libc::c_char;
    fn Pl_LE_Get_Ctrl_C_Return_Value() -> PlLong;
    fn Pl_LE_Get_Key(echo: libc::c_int, catch_ctrl_c: libc::c_int) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub const LE_MODE_DEACTIVATED: C2RustUnnamed = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const LE_MODE_HOOK: C2RustUnnamed = 2;
pub const LE_MODE_TTY: C2RustUnnamed = 1;
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_byte_code: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
pub static mut pl_stm_tbl: *mut *mut StmInf = 0 as *const *mut StmInf
    as *mut *mut StmInf;
pub static mut pl_stm_tbl_size: libc::c_int = 0;
pub static mut pl_stm_last_used: libc::c_int = 0;
pub static mut pl_alias_tbl: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut pl_last_input_sora: WamWord = 0;
pub static mut pl_last_output_sora: WamWord = 0;
pub static mut pl_stm_stdin: libc::c_int = 0;
pub static mut pl_stm_stdout: libc::c_int = 0;
pub static mut pl_stm_stderr: libc::c_int = 0;
pub static mut pl_stm_input: libc::c_int = 0;
pub static mut pl_stm_output: libc::c_int = 0;
pub static mut pl_stm_error: libc::c_int = 0;
pub static mut pl_stm_top_level_input: libc::c_int = 0;
pub static mut pl_stm_top_level_output: libc::c_int = 0;
pub static mut pl_stm_debugger_input: libc::c_int = 0;
pub static mut pl_stm_debugger_output: libc::c_int = 0;
pub static mut pl_stream_use_linedit: Bool = 0;
pub static mut pl_le_prompt: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut pl_use_le_prompt: libc::c_int = 0;
pub static mut pl_atom_stream: libc::c_int = 0;
pub static mut pl_atom_user_input: libc::c_int = 0;
pub static mut pl_atom_user_output: libc::c_int = 0;
pub static mut pl_atom_user_error: libc::c_int = 0;
pub static mut pl_atom_top_level_input: libc::c_int = 0;
pub static mut pl_atom_top_level_output: libc::c_int = 0;
pub static mut pl_atom_debugger_input: libc::c_int = 0;
pub static mut pl_atom_debugger_output: libc::c_int = 0;
pub static mut pl_atom_read: libc::c_int = 0;
pub static mut pl_atom_write: libc::c_int = 0;
pub static mut pl_atom_append: libc::c_int = 0;
pub static mut pl_atom_reposition: libc::c_int = 0;
pub static mut pl_atom_stream_position: libc::c_int = 0;
pub static mut pl_atom_text: libc::c_int = 0;
pub static mut pl_atom_binary: libc::c_int = 0;
pub static mut pl_atom_error: libc::c_int = 0;
pub static mut pl_atom_eof_code: libc::c_int = 0;
pub static mut pl_atom_reset: libc::c_int = 0;
pub static mut pl_atom_none: libc::c_int = 0;
pub static mut pl_atom_line: libc::c_int = 0;
pub static mut pl_atom_block: libc::c_int = 0;
pub static mut pl_atom_not: libc::c_int = 0;
pub static mut pl_atom_at: libc::c_int = 0;
pub static mut pl_atom_past: libc::c_int = 0;
pub static mut pl_atom_bof: libc::c_int = 0;
pub static mut pl_atom_current: libc::c_int = 0;
pub static mut pl_atom_eof: libc::c_int = 0;
pub static mut pl_last_read_col: libc::c_int = 0;
pub static mut pl_last_read_line: libc::c_int = 0;
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
pub static mut pl_init_stream_supp: Option::<unsafe extern "C" fn() -> ()> = unsafe {
    Some(
        ::std::mem::transmute::<
            unsafe extern "C" fn() -> (),
            unsafe extern "C" fn() -> (),
        >(Init_Stream_Supp),
    )
};
static mut atom_constant_term_stream: libc::c_int = 0;
static mut stream_1: WamWord = 0;
static mut word_current_input_stream: WamWord = 0;
static mut word_current_output_stream: WamWord = 0;
static mut static_str_stream_rd: StrSInf = {
    let mut init = StrSInf {
        buff: 0 as *const libc::c_char as *mut libc::c_char,
        ptr: 0 as *const libc::c_char as *mut libc::c_char,
        buff_alloc_size: 0 as libc::c_int,
    };
    init
};
static mut static_str_stream_wr: StrSInf = {
    let mut init = StrSInf {
        buff: 0 as *const libc::c_char as *mut libc::c_char,
        ptr: 0 as *const libc::c_char as *mut libc::c_char,
        buff_alloc_size: 0 as libc::c_int,
    };
    init
};
static mut tty_first_buff: [libc::c_char; 1024] = [0; 1024];
static mut tty_buff: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut tty_ptr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn Init_Stream_Supp() {
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    pl_stm_tbl_size = 32 as libc::c_int;
    pl_stm_tbl = Pl_Calloc_Check(
        pl_stm_tbl_size as libc::c_uint,
        ::std::mem::size_of::<*mut StmInf>() as libc::c_ulong as libc::c_uint,
        b"stream_supp.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        176 as libc::c_int,
    ) as *mut *mut StmInf;
    pl_stm_last_used = -(1 as libc::c_int);
    pl_alias_tbl = Pl_Hash_Alloc_Table(
        128 as libc::c_int,
        ::std::mem::size_of::<AliasInf>() as libc::c_ulong as libc::c_int,
    );
    pl_atom_stream = Pl_Create_Atom(
        b"$stream\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    stream_1 = ((1 as libc::c_int as PlULong)
        << (::std::mem::size_of::<PlULong>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(10 as libc::c_int as libc::c_ulong)
        | pl_atom_stream as libc::c_ulong) as WamWord;
    atom_constant_term_stream = Pl_Create_Atom(
        b"constant term stream\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    word_current_input_stream = (((Pl_Create_Atom(
        b"current_input_stream\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    word_current_output_stream = (((Pl_Create_Atom(
        b"current_output_stream\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as PlLong) << 3 as libc::c_int) as libc::c_ulong)
        .wrapping_add(0x3 as libc::c_int as PlULong) as WamWord;
    pl_atom_user_input = Pl_Create_Atom(
        b"user_input\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_user_output = Pl_Create_Atom(
        b"user_output\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_user_error = Pl_Create_Atom(
        b"user_error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_top_level_input = Pl_Create_Atom(
        b"top_level_input\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_top_level_output = Pl_Create_Atom(
        b"top_level_output\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_debugger_input = Pl_Create_Atom(
        b"debugger_input\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_debugger_output = Pl_Create_Atom(
        b"debugger_output\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_read = Pl_Create_Atom(
        b"read\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_write = Pl_Create_Atom(
        b"write\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_append = Pl_Create_Atom(
        b"append\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_reposition = Pl_Create_Atom(
        b"reposition\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_stream_position = Pl_Create_Atom(
        b"$stream_position\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_text = Pl_Create_Atom(
        b"text\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_binary = Pl_Create_Atom(
        b"binary\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_error = Pl_Create_Atom(
        b"error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_eof_code = Pl_Create_Atom(
        b"eof_code\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_reset = Pl_Create_Atom(
        b"reset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_none = Pl_Create_Atom(
        b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_line = Pl_Create_Atom(
        b"line\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_block = Pl_Create_Atom(
        b"block\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_not = Pl_Create_Atom(
        b"not\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_at = Pl_Create_Atom(
        b"at\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_past = Pl_Create_Atom(
        b"past\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_bof = Pl_Create_Atom(
        b"bof\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_current = Pl_Create_Atom(
        b"current\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_atom_eof = Pl_Create_Atom(
        b"eof\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pl_le_prompt = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    pl_use_le_prompt = 1 as libc::c_int;
    pl_stm_stdin = Pl_Add_Stream_For_Stdio_Desc(
        stdin,
        pl_atom_user_input,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if pl_le_mode == LE_MODE_HOOK as libc::c_int
        || pl_le_mode == LE_MODE_TTY as libc::c_int && isatty(0 as libc::c_int) != 0
    {
        pstm = *pl_stm_tbl.offset(pl_stm_stdin as isize);
        (*pstm)
            .fct_getc = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            StmFct,
        >(Some(TTY_Getc as unsafe extern "C" fn() -> libc::c_int));
        (*pstm)
            .fct_putc = ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t);
        (*pstm)
            .fct_flush = ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t);
        (*pstm)
            .fct_close = ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t);
        (*pstm)
            .fct_tell = ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t);
        (*pstm)
            .fct_seek = ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t);
        (*pstm)
            .fct_clearerr = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            StmFct,
        >(Some(TTY_Clearerr as unsafe extern "C" fn() -> ()));
        pl_stream_use_linedit = 1 as libc::c_int;
    }
    Pl_Add_Alias_To_Stream(pl_atom_user_input, pl_stm_stdin);
    pl_stm_input = pl_stm_stdin;
    pl_stm_stdout = Pl_Add_Stream_For_Stdio_Desc(
        stdout,
        pl_atom_user_output,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    Pl_Add_Alias_To_Stream(pl_atom_user_output, pl_stm_stdout);
    pl_stm_output = pl_stm_stdout;
    pl_stm_stderr = Pl_Add_Stream_For_Stdio_Desc(
        stderr,
        pl_atom_user_error,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    Pl_Add_Alias_To_Stream(pl_atom_user_error, pl_stm_stderr);
    pl_stm_error = pl_stm_stderr;
    pl_stm_debugger_input = pl_stm_input;
    pl_stm_top_level_input = pl_stm_debugger_input;
    pl_stm_debugger_output = pl_stm_output;
    pl_stm_top_level_output = pl_stm_debugger_output;
    Pl_Add_Alias_To_Stream(pl_atom_top_level_input, pl_stm_top_level_input);
    Pl_Add_Alias_To_Stream(pl_atom_top_level_output, pl_stm_top_level_output);
    Pl_Add_Alias_To_Stream(pl_atom_debugger_input, pl_stm_debugger_input);
    Pl_Add_Alias_To_Stream(pl_atom_debugger_output, pl_stm_debugger_output);
}
pub unsafe extern "C" fn Pl_Prop_And_Stdio_Mode(
    mut mode: libc::c_int,
    mut text: Bool,
    mut open_str: *mut libc::c_char,
) -> StmProp {
    let mut prop: StmProp = StmProp {
        mode_input_output_text_reposition_eof_action_buffering_special_close_other: [0; 2],
        c2rust_padding: [0; 2],
    };
    prop.set_mode(mode as libc::c_uint);
    match mode {
        0 => {
            prop.set_input(1 as libc::c_int as libc::c_uint);
            prop.set_output(0 as libc::c_int as libc::c_uint);
            let fresh0 = open_str;
            open_str = open_str.offset(1);
            *fresh0 = 'r' as i32 as libc::c_char;
        }
        1 => {
            prop.set_input(0 as libc::c_int as libc::c_uint);
            prop.set_output(1 as libc::c_int as libc::c_uint);
            let fresh1 = open_str;
            open_str = open_str.offset(1);
            *fresh1 = 'w' as i32 as libc::c_char;
        }
        2 => {
            prop.set_input(0 as libc::c_int as libc::c_uint);
            prop.set_output(1 as libc::c_int as libc::c_uint);
            let fresh2 = open_str;
            open_str = open_str.offset(1);
            *fresh2 = 'a' as i32 as libc::c_char;
        }
        _ => {}
    }
    prop.set_text(text as libc::c_uint);
    prop.set_reposition(1 as libc::c_int as libc::c_uint);
    prop.set_eof_action(1 as libc::c_int as libc::c_uint);
    prop.set_buffering(2 as libc::c_int as libc::c_uint);
    prop.set_special_close(0 as libc::c_int as libc::c_uint);
    prop.set_other(0 as libc::c_int as libc::c_uint);
    let fresh3 = open_str;
    open_str = open_str.offset(1);
    *fresh3 = (if text != 0 { 't' as i32 } else { 'b' as i32 }) as libc::c_char;
    *open_str = '\0' as i32 as libc::c_char;
    return prop;
}
pub unsafe extern "C" fn Pl_Add_Stream_For_Stdio_Desc(
    mut f: *mut FILE,
    mut atom_path: libc::c_int,
    mut mode: libc::c_int,
    mut text: Bool,
    mut force_eof_reset: Bool,
) -> libc::c_int {
    let mut open_str: [libc::c_char; 10] = [0; 10];
    let mut prop: StmProp = Pl_Prop_And_Stdio_Mode(mode, text, open_str.as_mut_ptr());
    prop.set_reposition(Pl_Stdio_Is_Repositionable(f) as libc::c_uint);
    prop.set_buffering(
        (if prop.reposition() as libc::c_int != 0 {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        }) as libc::c_uint,
    );
    Pl_Stdio_Set_Buffering(f, prop.buffering() as libc::c_int);
    if force_eof_reset != 0 || isatty(fileno(f)) != 0 {
        prop.set_eof_action(2 as libc::c_int as libc::c_uint);
    }
    return Pl_Add_Stream(
        atom_path,
        f as PlLong,
        prop,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );
}
pub unsafe extern "C" fn Pl_Add_Stream_For_Stdio_File(
    mut path: *mut libc::c_char,
    mut mode: libc::c_int,
    mut text: Bool,
) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut open_str: [libc::c_char; 10] = [0; 10];
    let mut atom_path: libc::c_int = 0;
    Pl_Prop_And_Stdio_Mode(mode, text, open_str.as_mut_ptr());
    f = fopen(path, open_str.as_mut_ptr());
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    atom_path = Pl_Create_Allocate_Atom(path);
    return Pl_Add_Stream_For_Stdio_Desc(f, atom_path, mode, text, 0 as libc::c_int);
}
unsafe extern "C" fn Init_Stream_Struct(
    mut atom_file_name: libc::c_int,
    mut file: PlLong,
    mut prop: StmProp,
    mut fct_getc: StmFct,
    mut fct_putc: StmFct,
    mut fct_flush: StmFct,
    mut fct_close: StmFct,
    mut fct_tell: StmFct,
    mut fct_seek: StmFct,
    mut fct_clearerr: StmFct,
    mut pstm: *mut StmInf,
) {
    (*pstm).atom_file_name = atom_file_name;
    (*pstm).file = file;
    (*pstm).prop = prop;
    (*pstm).mirror = 0 as *mut StmLst;
    (*pstm).mirror_of = 0 as *mut StmLst;
    (*pstm)
        .fct_getc = if fct_getc.is_some() {
        fct_getc
    } else {
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_int>,
            StmFct,
        >(Some(fgetc as unsafe extern "C" fn(*mut FILE) -> libc::c_int))
    };
    (*pstm)
        .fct_putc = if fct_putc.is_some() {
        fct_putc
    } else {
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int, *mut FILE) -> libc::c_int>,
            StmFct,
        >(Some(fputc as unsafe extern "C" fn(libc::c_int, *mut FILE) -> libc::c_int))
    };
    (*pstm)
        .fct_flush = if fct_flush.is_some() {
        fct_flush
    } else {
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_int>,
            StmFct,
        >(Some(fflush as unsafe extern "C" fn(*mut FILE) -> libc::c_int))
    };
    (*pstm)
        .fct_close = if fct_close.is_some() {
        fct_close
    } else {
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_int>,
            StmFct,
        >(Some(fclose as unsafe extern "C" fn(*mut FILE) -> libc::c_int))
    };
    (*pstm)
        .fct_tell = if fct_tell.is_some() {
        fct_tell
    } else {
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_long>,
            StmFct,
        >(Some(ftell as unsafe extern "C" fn(*mut FILE) -> libc::c_long))
    };
    (*pstm)
        .fct_seek = if fct_seek.is_some() {
        fct_seek
    } else {
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut FILE, libc::c_long, libc::c_int) -> libc::c_int,
            >,
            StmFct,
        >(
            Some(
                fseek
                    as unsafe extern "C" fn(
                        *mut FILE,
                        libc::c_long,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        )
    };
    (*pstm)
        .fct_clearerr = if fct_clearerr.is_some() {
        fct_clearerr
    } else {
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FILE) -> ()>,
            StmFct,
        >(Some(clearerr as unsafe extern "C" fn(*mut FILE) -> ()))
    };
    (*pstm).eof_reached = 0 as libc::c_int;
    (*pstm).pb_char.ptr = ((*pstm).pb_char.buff).as_mut_ptr();
    (*pstm).pb_char.nb_elems = 0 as libc::c_int;
    (*pstm).char_count = 0 as libc::c_int as PlLong;
    (*pstm).line_count = 0 as libc::c_int as PlLong;
    (*pstm).line_pos = 0 as libc::c_int as PlLong;
    (*pstm).pb_line_pos.ptr = ((*pstm).pb_line_pos.buff).as_mut_ptr();
    (*pstm).pb_line_pos.nb_elems = 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Add_Stream(
    mut atom_file_name: libc::c_int,
    mut file: PlLong,
    mut prop: StmProp,
    mut fct_getc: StmFct,
    mut fct_putc: StmFct,
    mut fct_flush: StmFct,
    mut fct_close: StmFct,
    mut fct_tell: StmFct,
    mut fct_seek: StmFct,
    mut fct_clearerr: StmFct,
) -> libc::c_int {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    stm = Find_Free_Stream();
    if prop.reposition() as libc::c_int != 0
        && (fct_tell
            == ::std::mem::transmute::<
                libc::intptr_t,
                StmFct,
            >(-(1 as libc::c_int) as libc::intptr_t)
            || fct_seek
                == ::std::mem::transmute::<
                    libc::intptr_t,
                    StmFct,
                >(-(1 as libc::c_int) as libc::intptr_t))
    {
        Pl_Fatal_Error(
            b"fct tell or seek undefined\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    pstm = *pl_stm_tbl.offset(stm as isize);
    Init_Stream_Struct(
        atom_file_name,
        file,
        prop,
        fct_getc,
        fct_putc,
        fct_flush,
        fct_close,
        fct_tell,
        fct_seek,
        fct_clearerr,
        pstm,
    );
    return stm;
}
pub unsafe extern "C" fn Pl_Delete_Stream(mut stm: libc::c_int) {
    Del_Aliases_Of_Stream(stm);
    Update_Mirrors_To_Del_Stream(stm);
    free(*pl_stm_tbl.offset(stm as isize) as *mut libc::c_void);
    let ref mut fresh4 = *pl_stm_tbl.offset(stm as isize);
    *fresh4 = 0 as *mut StmInf;
    while (*pl_stm_tbl.offset(pl_stm_last_used as isize)).is_null() {
        pl_stm_last_used -= 1;
        pl_stm_last_used;
    }
}
unsafe extern "C" fn Find_Free_Stream() -> libc::c_int {
    let mut stm: libc::c_int = 0;
    stm = 0 as libc::c_int;
    while stm < pl_stm_tbl_size {
        if (*pl_stm_tbl.offset(stm as isize)).is_null() {
            break;
        }
        stm += 1;
        stm;
    }
    if stm == pl_stm_tbl_size {
        Pl_Extend_Array(
            &mut pl_stm_tbl as *mut *mut *mut StmInf as *mut *mut libc::c_char,
            &mut pl_stm_tbl_size,
            ::std::mem::size_of::<*mut StmInf>() as libc::c_ulong as libc::c_int,
            1 as libc::c_int,
        );
    }
    let ref mut fresh5 = *pl_stm_tbl.offset(stm as isize);
    *fresh5 = Pl_Malloc_Check(
        ::std::mem::size_of::<StmInf>() as libc::c_ulong as libc::c_uint,
        b"stream_supp.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        509 as libc::c_int,
    ) as *mut StmInf;
    if stm > pl_stm_last_used {
        pl_stm_last_used = stm;
    }
    return stm;
}
pub unsafe extern "C" fn Pl_Find_Stream_By_Alias(
    mut atom_alias: libc::c_int,
) -> libc::c_int {
    let mut alias: *mut AliasInf = 0 as *mut AliasInf;
    alias = Pl_Hash_Find(pl_alias_tbl, atom_alias as PlLong) as *mut AliasInf;
    return if alias.is_null() { -(1 as libc::c_int) } else { (*alias).stm };
}
pub unsafe extern "C" fn Pl_Add_Alias_To_Stream(
    mut atom_alias: libc::c_int,
    mut stm: libc::c_int,
) -> Bool {
    let mut alias: *mut AliasInf = 0 as *mut AliasInf;
    let mut alias_info: AliasInf = AliasInf { atom: 0, stm: 0 };
    alias = Pl_Hash_Find(pl_alias_tbl, atom_alias as PlLong) as *mut AliasInf;
    if !alias.is_null() {
        return ((*alias).stm == stm) as libc::c_int;
    }
    Pl_Extend_Table_If_Needed(&mut pl_alias_tbl);
    alias_info.atom = atom_alias as PlLong;
    alias_info.stm = stm;
    alias = Pl_Hash_Insert(
        pl_alias_tbl,
        &mut alias_info as *mut AliasInf as *mut libc::c_char,
        0 as libc::c_int,
    ) as *mut AliasInf;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Reassign_Alias(
    mut atom_alias: libc::c_int,
    mut stm: libc::c_int,
) {
    let mut alias: *mut AliasInf = 0 as *mut AliasInf;
    alias = Pl_Hash_Find(pl_alias_tbl, atom_alias as PlLong) as *mut AliasInf;
    if !alias.is_null() {
        (*alias).stm = stm;
    }
}
unsafe extern "C" fn Del_Aliases_Of_Stream(mut stm: libc::c_int) {
    let mut scan: HashScan = HashScan {
        endt: 0 as *mut libc::c_char,
        cur_t: 0 as *mut libc::c_char,
        cur_p: 0 as *mut libc::c_char,
    };
    let mut alias: *mut AliasInf = 0 as *mut AliasInf;
    alias = Pl_Hash_First(pl_alias_tbl, &mut scan) as *mut AliasInf;
    while !alias.is_null() {
        if (*alias).stm == stm {
            Pl_Hash_Delete(pl_alias_tbl, (*alias).atom);
        }
        alias = Pl_Hash_Next(&mut scan) as *mut AliasInf;
    }
}
pub unsafe extern "C" fn Pl_Add_Mirror_To_Stream(
    mut stm: libc::c_int,
    mut m_stm: libc::c_int,
) {
    let mut pstm: *mut StmInf = *pl_stm_tbl.offset(stm as isize);
    let mut m_pstm: *mut StmInf = *pl_stm_tbl.offset(m_stm as isize);
    let mut m: *mut StmLst = 0 as *mut StmLst;
    if stm == m_stm {
        return;
    }
    m = (*pstm).mirror;
    while !m.is_null() {
        if (*m).stm == m_stm {
            return;
        }
        m = (*m).next;
    }
    m = Pl_Malloc_Check(
        ::std::mem::size_of::<StmLst>() as libc::c_ulong as libc::c_uint,
        b"stream_supp.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        619 as libc::c_int,
    ) as *mut StmLst;
    (*m).stm = m_stm;
    (*m).next = (*pstm).mirror;
    (*pstm).mirror = m;
    m = Pl_Malloc_Check(
        ::std::mem::size_of::<StmLst>() as libc::c_ulong as libc::c_uint,
        b"stream_supp.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        624 as libc::c_int,
    ) as *mut StmLst;
    (*m).stm = stm;
    (*m).next = (*m_pstm).mirror_of;
    (*m_pstm).mirror_of = m;
}
pub unsafe extern "C" fn Pl_Del_Mirror_From_Stream(
    mut stm: libc::c_int,
    mut m_stm: libc::c_int,
) -> Bool {
    let mut pstm: *mut StmInf = *pl_stm_tbl.offset(stm as isize);
    let mut m_pstm: *mut StmInf = *pl_stm_tbl.offset(m_stm as isize);
    if Remove_In_Stream_List(m_stm, &mut (*pstm).mirror) == 0 {
        return 0 as libc::c_int;
    }
    Remove_In_Stream_List(stm, &mut (*m_pstm).mirror_of);
    return 1 as libc::c_int;
}
unsafe extern "C" fn Update_Mirrors_To_Del_Stream(mut stm: libc::c_int) {
    let mut pstm: *mut StmInf = *pl_stm_tbl.offset(stm as isize);
    let mut m_pstm: *mut StmInf = 0 as *mut StmInf;
    let mut m: *mut StmLst = 0 as *mut StmLst;
    let mut m1: *mut StmLst = 0 as *mut StmLst;
    m = (*pstm).mirror;
    while !m.is_null() {
        m1 = m;
        m_pstm = *pl_stm_tbl.offset((*m).stm as isize);
        m = (*m).next;
        free(m1 as *mut libc::c_void);
        Remove_In_Stream_List(stm, &mut (*m_pstm).mirror_of);
    }
    m = (*pstm).mirror_of;
    while !m.is_null() {
        m1 = m;
        m_pstm = *pl_stm_tbl.offset((*m).stm as isize);
        m = (*m).next;
        free(m1 as *mut libc::c_void);
        Remove_In_Stream_List(stm, &mut (*m_pstm).mirror);
    }
}
unsafe extern "C" fn Remove_In_Stream_List(
    mut stm: libc::c_int,
    mut p_start: *mut *mut StmLst,
) -> Bool {
    let mut m: *mut StmLst = 0 as *mut StmLst;
    loop {
        m = *p_start;
        if m.is_null() {
            break;
        }
        if (*m).stm == stm {
            *p_start = (*m).next;
            free(m as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        p_start = &mut (*m).next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Find_Stream_From_PStm(mut pstm: *mut StmInf) -> libc::c_int {
    let mut stm: libc::c_int = 0;
    stm = 0 as libc::c_int;
    while stm <= pl_stm_last_used {
        if *pl_stm_tbl.offset(stm as isize) == pstm {
            return stm;
        }
        stm += 1;
        stm;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Flush_All_Streams() {
    let mut stm: libc::c_int = 0;
    stm = 0 as libc::c_int;
    while stm <= pl_stm_last_used {
        if !(*pl_stm_tbl.offset(stm as isize)).is_null() {
            Pl_Stream_Flush(*pl_stm_tbl.offset(stm as isize));
        }
        stm += 1;
        stm;
    }
}
pub unsafe extern "C" fn Pl_Set_Stream_Buffering(mut stm: libc::c_int) {
    let mut pstm: *mut StmInf = *pl_stm_tbl.offset(stm as isize);
    let mut f: *mut FILE = 0 as *mut FILE;
    f = Pl_Stdio_Desc_Of_Stream(stm);
    if f.is_null() {
        ((*pstm).prop).set_buffering(0 as libc::c_int as libc::c_uint);
        return;
    }
    if ((*pstm).file == stdout as PlLong || (*pstm).file == stderr as PlLong)
        && pl_le_hook_set_line_buffering.is_some()
    {
        ::std::mem::transmute::<
            _,
            fn(_),
        >(
            (Some(pl_le_hook_set_line_buffering.unwrap())).unwrap(),
        )(
            (((*pstm).prop).buffering() as libc::c_int != 0 as libc::c_int)
                as libc::c_int,
        );
    } else {
        Pl_Stdio_Set_Buffering(f, ((*pstm).prop).buffering() as libc::c_int);
    };
}
pub unsafe extern "C" fn Pl_Get_Stream_Or_Alias(
    mut sora_word: WamWord,
    mut test: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut word: WamWord = 0;
    let mut tag_mask: WamWord = 0;
    let mut tag_mask1: WamWord = 0;
    let mut atom: libc::c_int = 0;
    let mut stc_adr: *mut WamWord = 0 as *mut WamWord;
    let mut stm: PlLong = 0 as libc::c_int as PlLong;
    let mut perm_oper: libc::c_int = 0;
    let mut deref_last_word: WamWord = 0;
    word = sora_word;
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
    if tag_mask as libc::c_ulong == 0x3 as libc::c_int as PlULong {
        atom = (word as PlULong >> 3 as libc::c_int) as libc::c_int;
        stm = Pl_Find_Stream_By_Alias(atom) as PlLong;
    } else {
        if tag_mask as libc::c_ulong == 0x2 as libc::c_int as PlULong {
            stc_adr = (word as libc::c_ulong & 0xfffffffffffffff8 as libc::c_ulong)
                as *mut WamWord;
            let mut deref_last_word_0: WamWord = 0;
            word = *stc_adr.offset((1 as libc::c_int + 0 as libc::c_int) as isize);
            loop {
                deref_last_word_0 = word;
                tag_mask1 = (word as libc::c_ulong & 0x7 as libc::c_int as PlULong)
                    as WamWord;
                if tag_mask1 as libc::c_ulong != 0 as libc::c_int as PlULong {
                    break;
                }
                word = *(word as *mut WamWord);
                if !(word != deref_last_word_0) {
                    break;
                }
            }
            stm = word << 0 as libc::c_int >> 3 as libc::c_int;
            if *stc_adr.offset(0 as libc::c_int as isize) == stream_1
                && tag_mask1 as libc::c_ulong == 0x7 as libc::c_int as PlULong
            {
                current_block = 13409297157721066830;
            } else {
                current_block = 7172762164747879670;
            }
        } else {
            current_block = 7172762164747879670;
        }
        match current_block {
            13409297157721066830 => {}
            _ => {
                if tag_mask as libc::c_ulong == 0 as libc::c_int as PlULong {
                    Pl_Err_Instantiation();
                }
                Pl_Err_Domain(pl_domain_stream_or_alias, sora_word);
            }
        }
    }
    if stm as PlULong > pl_stm_last_used as PlULong
        || (*pl_stm_tbl.offset(stm as isize)).is_null()
    {
        if test == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        Pl_Err_Existence(pl_existence_stream, sora_word);
    }
    if !(test == 0 as libc::c_int || test == 1 as libc::c_int) {
        if test == 2 as libc::c_int {
            if ((**pl_stm_tbl.offset(stm as isize)).prop).input() != 0 {
                current_block = 17650143022224026054;
            } else {
                perm_oper = pl_permission_operation_input;
                current_block = 7659304154607701039;
            }
        } else if ((**pl_stm_tbl.offset(stm as isize)).prop).output() != 0 {
            current_block = 17650143022224026054;
        } else {
            perm_oper = pl_permission_operation_output;
            current_block = 7659304154607701039;
        }
        match current_block {
            17650143022224026054 => {}
            _ => {
                Pl_Err_Permission(perm_oper, pl_permission_type_stream, sora_word);
            }
        }
    }
    return stm as libc::c_int;
}
pub unsafe extern "C" fn Pl_Check_Stream_Type(
    mut stm: libc::c_int,
    mut check_text: Bool,
    mut for_input: Bool,
) {
    let mut perm_oper: libc::c_int = 0;
    let mut perm_type: libc::c_int = 0;
    let mut sora_word: WamWord = 0;
    if check_text != 0 {
        if ((**pl_stm_tbl.offset(stm as isize)).prop).text() != 0 {
            return;
        }
        perm_type = pl_permission_type_binary_stream;
    } else {
        if ((**pl_stm_tbl.offset(stm as isize)).prop).text() == 0 {
            return;
        }
        perm_type = pl_permission_type_text_stream;
    }
    if for_input != 0 {
        perm_oper = pl_permission_operation_input;
        sora_word = if pl_last_input_sora as libc::c_ulong
            == (0 as libc::c_int as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong)
        {
            word_current_input_stream
        } else {
            pl_last_input_sora
        };
    } else {
        perm_oper = pl_permission_operation_output;
        sora_word = if pl_last_output_sora as libc::c_ulong
            == (0 as libc::c_int as PlLong as libc::c_ulong)
                .wrapping_add(0 as libc::c_int as PlULong)
        {
            word_current_output_stream
        } else {
            pl_last_output_sora
        };
    }
    Pl_Err_Permission(perm_oper, perm_type, sora_word);
}
pub unsafe extern "C" fn Pl_Make_Stream_Tagged_Word(mut stm: libc::c_int) -> WamWord {
    static mut h: [WamWord; 2] = [0; 2];
    h[0 as libc::c_int as usize] = stream_1;
    h[1 as libc::c_int
        as usize] = ((stm as PlULong) << 3 as libc::c_int
        | 0x7 as libc::c_int as PlULong) as WamWord;
    return (h.as_mut_ptr() as PlLong as libc::c_ulong)
        .wrapping_add(0x2 as libc::c_int as PlULong) as WamWord;
}
pub unsafe extern "C" fn Pl_Stdio_Is_Repositionable(mut f: *mut FILE) -> Bool {
    let mut fd: libc::c_int = fileno(f);
    return (isatty(fd) == 0
        && lseek(fd, 0 as libc::c_int as __off_t, 1 as libc::c_int)
            >= 0 as libc::c_int as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn Pl_Stdio_Set_Buffering(
    mut f: *mut FILE,
    mut buffering: libc::c_int,
) {
    let mut buff_flag: libc::c_int = 0 as libc::c_int;
    match buffering {
        0 => {
            buff_flag = 2 as libc::c_int;
        }
        1 => {
            buff_flag = 1 as libc::c_int;
        }
        2 => {
            buff_flag = 0 as libc::c_int;
        }
        _ => {}
    }
    setvbuf(f, 0 as *mut libc::c_char, buff_flag, 8192 as libc::c_int as size_t);
}
pub unsafe extern "C" fn Pl_Stdio_Desc_Of_Stream(mut stm: libc::c_int) -> *mut FILE {
    let mut pstm: *mut StmInf = *pl_stm_tbl.offset(stm as isize);
    if stm == pl_stm_stdin {
        return stdin;
    }
    if (*pstm).fct_getc
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut FILE) -> libc::c_int>,
            StmFct,
        >(Some(fgetc as unsafe extern "C" fn(*mut FILE) -> libc::c_int))
    {
        return (*pstm).file as *mut FILE;
    }
    return 0 as *mut FILE;
}
pub unsafe extern "C" fn Pl_Io_Fileno_Of_Stream(mut stm: libc::c_int) -> libc::c_int {
    let mut f: *mut FILE = 0 as *mut FILE;
    f = Pl_Stdio_Desc_Of_Stream(stm);
    if !f.is_null() {
        return fileno(f);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn TTY_Getc() -> libc::c_int {
    let mut current_block: u64;
    let mut c: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    static mut tty_linedit_depth: libc::c_int = 0 as libc::c_int;
    if tty_ptr.is_null() {
        let fresh6 = tty_linedit_depth;
        tty_linedit_depth = tty_linedit_depth + 1;
        if fresh6 == 0 as libc::c_int {
            tty_buff = tty_first_buff.as_mut_ptr();
        } else {
            tty_buff = Pl_Malloc_Check(
                1024 as libc::c_int as libc::c_uint,
                b"stream_supp.c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                1063 as libc::c_int,
            );
        }
        let mut save_sys_var_option_mask: libc::c_int = *pl_sys_var
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as libc::c_int;
        let mut save_last_read_line: libc::c_int = pl_last_read_line;
        let mut save_last_read_col: libc::c_int = pl_last_read_col;
        tty_buff = Pl_LE_FGets(
            tty_buff,
            1024 as libc::c_int,
            pl_le_prompt,
            pl_use_le_prompt,
        );
        pl_use_le_prompt = 0 as libc::c_int;
        *pl_sys_var
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) = save_sys_var_option_mask as PlLong;
        pl_last_read_line = save_last_read_line;
        pl_last_read_col = save_last_read_col;
        tty_linedit_depth -= 1;
        tty_linedit_depth;
        if tty_buff as PlLong == -(2 as libc::c_int) as PlLong {
            Pl_Execute_A_Continuation(
                ::std::mem::transmute::<
                    libc::intptr_t,
                    CodePtr,
                >(Pl_LE_Get_Ctrl_C_Return_Value() as libc::intptr_t),
            );
        }
        if tty_buff.is_null() {
            c = -(1 as libc::c_int);
            current_block = 8907204036091796403;
        } else {
            tty_ptr = tty_buff;
            pstm = *pl_stm_tbl.offset(pl_stm_stdout as isize);
            (*pstm)
                .char_count = ((*pstm).char_count as libc::c_ulong)
                .wrapping_add(strlen(tty_buff)) as PlLong as PlLong;
            (*pstm).line_count += 1;
            (*pstm).line_count;
            (*pstm).line_pos = 0 as libc::c_int as PlLong;
            current_block = 5689001924483802034;
        }
    } else {
        current_block = 5689001924483802034;
    }
    match current_block {
        5689001924483802034 => {
            let fresh7 = tty_ptr;
            tty_ptr = tty_ptr.offset(1);
            c = *fresh7 as libc::c_int;
            if *tty_ptr as libc::c_int == '\0' as i32 {
                current_block = 8907204036091796403;
            } else {
                current_block = 1109700713171191020;
            }
        }
        _ => {}
    }
    match current_block {
        8907204036091796403 => {
            if tty_buff != tty_first_buff.as_mut_ptr() {
                free(tty_buff as *mut libc::c_void);
            }
            tty_ptr = 0 as *mut libc::c_char;
        }
        _ => {}
    }
    return c;
}
unsafe extern "C" fn TTY_Get_Key(mut echo: Bool, mut catch_ctrl_c: Bool) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if !tty_ptr.is_null() {
        let fresh8 = tty_ptr;
        tty_ptr = tty_ptr.offset(1);
        c = *fresh8 as libc::c_int;
        if *tty_ptr as libc::c_int == '\0' as i32 {
            if tty_buff != tty_first_buff.as_mut_ptr() {
                free(tty_buff as *mut libc::c_void);
            }
            tty_ptr = 0 as *mut libc::c_char;
        }
        return c;
    }
    let mut save_sys_var_option_mask: libc::c_int = *pl_sys_var
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as libc::c_int;
    let mut save_last_read_line: libc::c_int = pl_last_read_line;
    let mut save_last_read_col: libc::c_int = pl_last_read_col;
    c = Pl_LE_Get_Key(echo, catch_ctrl_c);
    *pl_sys_var
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) = save_sys_var_option_mask as PlLong;
    pl_last_read_line = save_last_read_line;
    pl_last_read_col = save_last_read_col;
    if c as PlLong == -(2 as libc::c_int) as PlLong {
        Pl_Execute_A_Continuation(
            ::std::mem::transmute::<
                libc::intptr_t,
                CodePtr,
            >(Pl_LE_Get_Ctrl_C_Return_Value() as libc::intptr_t),
        );
    }
    return c;
}
unsafe extern "C" fn TTY_Clearerr() {
    clearerr(stdin);
}
unsafe extern "C" fn Basic_Call_Fct_Getc(mut pstm: *mut StmInf) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut m: *mut StmLst = 0 as *mut StmLst;
    if *pl_sys_var.as_mut_ptr().offset(20 as libc::c_int as isize) != 0
        && (*pstm).file == stdin as PlLong
    {
        putchar('\u{1}' as i32);
        fflush(stdout);
    }
    c = ::std::mem::transmute::<
        _,
        fn(_) -> libc::c_int,
    >((Some(((*pstm).fct_getc).unwrap())).unwrap())((*pstm).file);
    if c != -(1 as libc::c_int) {
        m = (*pstm).mirror;
        while !m.is_null() {
            Pl_Stream_Putc(c, *pl_stm_tbl.offset((*m).stm as isize));
            m = (*m).next;
        }
    }
    return c;
}
unsafe extern "C" fn Basic_Call_Fct_Putc(mut c: libc::c_int, mut pstm: *mut StmInf) {
    let mut m: *mut StmLst = 0 as *mut StmLst;
    ::std::mem::transmute::<
        _,
        fn(_, _) -> libc::c_int,
    >((Some(((*pstm).fct_putc).unwrap())).unwrap())(c, (*pstm).file);
    m = (*pstm).mirror;
    while !m.is_null() {
        Pl_Stream_Putc(c, *pl_stm_tbl.offset((*m).stm as isize));
        m = (*m).next;
    }
}
pub unsafe extern "C" fn Pl_PB_Empty_Buffer(mut pstm: *mut StmInf) {
    (*pstm).pb_char.ptr = ((*pstm).pb_char.buff).as_mut_ptr();
    (*pstm).pb_char.nb_elems = 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Stream_Get_Key(
    mut pstm: *mut StmInf,
    mut echo: libc::c_int,
    mut catch_ctrl_c: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut file: PlLong = (*pstm).file;
    let mut simulate: Bool = 0;
    if pl_le_mode != LE_MODE_DEACTIVATED as libc::c_int
        && pstm == *pl_stm_tbl.offset(pl_stm_stdin as isize)
    {
        simulate = 0 as libc::c_int;
    } else {
        simulate = 1 as libc::c_int;
    }
    if (*pstm).eof_reached != 0 {
        if ((*pstm).prop).eof_action() as libc::c_int == 0 as libc::c_int {
            Pl_Err_Permission(
                pl_permission_operation_input,
                pl_permission_type_past_end_of_stream,
                if pl_last_input_sora as libc::c_ulong
                    == (0 as libc::c_int as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong)
                {
                    word_current_input_stream
                } else {
                    pl_last_input_sora
                },
            );
        }
        if ((*pstm).prop).eof_action() as libc::c_int == 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
        (*pstm).eof_reached = 0 as libc::c_int;
        if ((*pstm).prop).reposition() != 0 {
            Pl_Stream_Set_Position(
                pstm,
                0 as libc::c_int,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
            );
        }
        if (*pstm).fct_clearerr
            != ::std::mem::transmute::<
                libc::intptr_t,
                StmFct,
            >(-(1 as libc::c_int) as libc::intptr_t)
        {
            ::std::mem::transmute::<
                _,
                fn(_) -> libc::c_int,
            >((Some(((*pstm).fct_clearerr).unwrap())).unwrap())(file);
        }
    }
    if !((*pstm).pb_char.nb_elems == 0 as libc::c_int) {
        if (*pstm).pb_char.ptr != ((*pstm).pb_char.buff).as_mut_ptr() {
            (*pstm).pb_char.ptr = ((*pstm).pb_char.ptr).offset(-1);
            (*pstm).pb_char.ptr;
        } else {
            (*pstm)
                .pb_char
                .ptr = ((*pstm).pb_char.buff)
                .as_mut_ptr()
                .offset(8 as libc::c_int as isize)
                .offset(-(1 as libc::c_int as isize));
        }
        c = *(*pstm).pb_char.ptr;
        (*pstm).pb_char.nb_elems -= 1;
        (*pstm).pb_char.nb_elems;
    } else {
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) = pl_reg_bank as WamWord;
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize) = TR as WamWord;
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) = B as WamWord;
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize) = H as WamWord;
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize) = 1 as libc::c_int as WamWord;
        if simulate != 0 {
            c = Basic_Call_Fct_Getc(pstm);
        } else {
            c = TTY_Get_Key(echo, catch_ctrl_c);
        }
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize) = 0 as libc::c_int as WamWord;
    }
    if simulate != 0 && c != '\n' as i32 {
        while Basic_Call_Fct_Getc(pstm) >= ' ' as i32 {}
        if '\n' as i32 != -(1 as libc::c_int) {
            let ref mut fresh9 = (**pl_stm_tbl.offset(pl_stm_stdout as isize))
                .char_count;
            *fresh9 += 1;
            *fresh9;
        }
        if '\n' as i32 == '\n' as i32 {
            let ref mut fresh10 = (**pl_stm_tbl.offset(pl_stm_stdout as isize))
                .line_count;
            *fresh10 += 1;
            *fresh10;
            (**pl_stm_tbl.offset(pl_stm_stdout as isize))
                .line_pos = 0 as libc::c_int as PlLong;
        } else {
            let ref mut fresh11 = (**pl_stm_tbl.offset(pl_stm_stdout as isize)).line_pos;
            *fresh11 += 1;
            *fresh11;
        }
    }
    if c == -(1 as libc::c_int) {
        (*pstm).eof_reached = 1 as libc::c_int;
    }
    if c == '\n' as i32 {
        *(*pstm).pb_line_pos.ptr = (*pstm).line_pos as libc::c_int;
        if (*pstm).pb_line_pos.ptr
            != ((*pstm).pb_line_pos.buff)
                .as_mut_ptr()
                .offset(8 as libc::c_int as isize)
                .offset(-(1 as libc::c_int as isize))
        {
            (*pstm).pb_line_pos.ptr = ((*pstm).pb_line_pos.ptr).offset(1);
            (*pstm).pb_line_pos.ptr;
        } else {
            (*pstm).pb_line_pos.ptr = ((*pstm).pb_line_pos.buff).as_mut_ptr();
        }
        if (*pstm).pb_line_pos.nb_elems < 8 as libc::c_int {
            (*pstm).pb_line_pos.nb_elems += 1;
            (*pstm).pb_line_pos.nb_elems;
        }
    }
    if c != -(1 as libc::c_int) {
        (*pstm).char_count += 1;
        (*pstm).char_count;
    }
    if c == '\n' as i32 {
        (*pstm).line_count += 1;
        (*pstm).line_count;
        (*pstm).line_pos = 0 as libc::c_int as PlLong;
    } else {
        (*pstm).line_pos += 1;
        (*pstm).line_pos;
    }
    return c;
}
pub unsafe extern "C" fn Pl_Stream_Getc(mut pstm: *mut StmInf) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut file: PlLong = (*pstm).file;
    if (*pstm).eof_reached != 0 {
        if ((*pstm).prop).eof_action() as libc::c_int == 0 as libc::c_int {
            Pl_Err_Permission(
                pl_permission_operation_input,
                pl_permission_type_past_end_of_stream,
                if pl_last_input_sora as libc::c_ulong
                    == (0 as libc::c_int as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong)
                {
                    word_current_input_stream
                } else {
                    pl_last_input_sora
                },
            );
        }
        if ((*pstm).prop).eof_action() as libc::c_int == 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
        (*pstm).eof_reached = 0 as libc::c_int;
        if ((*pstm).prop).reposition() != 0 {
            Pl_Stream_Set_Position(
                pstm,
                0 as libc::c_int,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
            );
        }
        if (*pstm).fct_clearerr
            != ::std::mem::transmute::<
                libc::intptr_t,
                StmFct,
            >(-(1 as libc::c_int) as libc::intptr_t)
        {
            ::std::mem::transmute::<
                _,
                fn(_) -> libc::c_int,
            >((Some(((*pstm).fct_clearerr).unwrap())).unwrap())(file);
        }
    }
    if !((*pstm).pb_char.nb_elems == 0 as libc::c_int) {
        if (*pstm).pb_char.ptr != ((*pstm).pb_char.buff).as_mut_ptr() {
            (*pstm).pb_char.ptr = ((*pstm).pb_char.ptr).offset(-1);
            (*pstm).pb_char.ptr;
        } else {
            (*pstm)
                .pb_char
                .ptr = ((*pstm).pb_char.buff)
                .as_mut_ptr()
                .offset(8 as libc::c_int as isize)
                .offset(-(1 as libc::c_int as isize));
        }
        c = *(*pstm).pb_char.ptr;
        (*pstm).pb_char.nb_elems -= 1;
        (*pstm).pb_char.nb_elems;
    } else {
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) = pl_reg_bank as WamWord;
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize) = TR as WamWord;
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) = B as WamWord;
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize) = H as WamWord;
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize) = 1 as libc::c_int as WamWord;
        c = Basic_Call_Fct_Getc(pstm);
        *pl_buff_signal_reg
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize) = 0 as libc::c_int as WamWord;
    }
    if c == -(1 as libc::c_int) {
        (*pstm).eof_reached = 1 as libc::c_int;
    }
    if c == '\n' as i32 {
        *(*pstm).pb_line_pos.ptr = (*pstm).line_pos as libc::c_int;
        if (*pstm).pb_line_pos.ptr
            != ((*pstm).pb_line_pos.buff)
                .as_mut_ptr()
                .offset(8 as libc::c_int as isize)
                .offset(-(1 as libc::c_int as isize))
        {
            (*pstm).pb_line_pos.ptr = ((*pstm).pb_line_pos.ptr).offset(1);
            (*pstm).pb_line_pos.ptr;
        } else {
            (*pstm).pb_line_pos.ptr = ((*pstm).pb_line_pos.buff).as_mut_ptr();
        }
        if (*pstm).pb_line_pos.nb_elems < 8 as libc::c_int {
            (*pstm).pb_line_pos.nb_elems += 1;
            (*pstm).pb_line_pos.nb_elems;
        }
    }
    if c != -(1 as libc::c_int) {
        (*pstm).char_count += 1;
        (*pstm).char_count;
    }
    if c == '\n' as i32 {
        (*pstm).line_count += 1;
        (*pstm).line_count;
        (*pstm).line_pos = 0 as libc::c_int as PlLong;
    } else {
        (*pstm).line_pos += 1;
        (*pstm).line_pos;
    }
    return c;
}
pub unsafe extern "C" fn Pl_Stream_Ungetc(mut c: libc::c_int, mut pstm: *mut StmInf) {
    *(*pstm).pb_char.ptr = c;
    if (*pstm).pb_char.ptr
        != ((*pstm).pb_char.buff)
            .as_mut_ptr()
            .offset(8 as libc::c_int as isize)
            .offset(-(1 as libc::c_int as isize))
    {
        (*pstm).pb_char.ptr = ((*pstm).pb_char.ptr).offset(1);
        (*pstm).pb_char.ptr;
    } else {
        (*pstm).pb_char.ptr = ((*pstm).pb_char.buff).as_mut_ptr();
    }
    if (*pstm).pb_char.nb_elems < 8 as libc::c_int {
        (*pstm).pb_char.nb_elems += 1;
        (*pstm).pb_char.nb_elems;
    }
    (*pstm).eof_reached = 0 as libc::c_int;
    if (*pstm).char_count > 0 as libc::c_int as libc::c_long {
        (*pstm).char_count -= 1;
        (*pstm).char_count;
    }
    if c == '\n' as i32 {
        if (*pstm).line_count > 0 as libc::c_int as libc::c_long {
            (*pstm).line_count -= 1;
            (*pstm).line_count;
        }
        if !((*pstm).pb_line_pos.nb_elems == 0 as libc::c_int) {
            if (*pstm).pb_line_pos.ptr != ((*pstm).pb_line_pos.buff).as_mut_ptr() {
                (*pstm).pb_line_pos.ptr = ((*pstm).pb_line_pos.ptr).offset(-1);
                (*pstm).pb_line_pos.ptr;
            } else {
                (*pstm)
                    .pb_line_pos
                    .ptr = ((*pstm).pb_line_pos.buff)
                    .as_mut_ptr()
                    .offset(8 as libc::c_int as isize)
                    .offset(-(1 as libc::c_int as isize));
            }
            (*pstm).line_pos = *(*pstm).pb_line_pos.ptr as PlLong;
            (*pstm).pb_line_pos.nb_elems -= 1;
            (*pstm).pb_line_pos.nb_elems;
        } else {
            (*pstm).line_pos = 0 as libc::c_int as PlLong;
        }
    } else if (*pstm).line_pos > 0 as libc::c_int as libc::c_long {
        (*pstm).line_pos -= 1;
        (*pstm).line_pos;
    }
}
pub unsafe extern "C" fn Pl_Stream_Peekc(mut pstm: *mut StmInf) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut file: PlLong = (*pstm).file;
    if (*pstm).eof_reached != 0 {
        if ((*pstm).prop).eof_action() as libc::c_int == 0 as libc::c_int {
            Pl_Err_Permission(
                pl_permission_operation_input,
                pl_permission_type_past_end_of_stream,
                if pl_last_input_sora as libc::c_ulong
                    == (0 as libc::c_int as PlLong as libc::c_ulong)
                        .wrapping_add(0 as libc::c_int as PlULong)
                {
                    word_current_input_stream
                } else {
                    pl_last_input_sora
                },
            );
        }
        if ((*pstm).prop).eof_action() as libc::c_int == 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
        (*pstm).eof_reached = 0 as libc::c_int;
        if ((*pstm).prop).reposition() != 0 {
            Pl_Stream_Set_Position(
                pstm,
                0 as libc::c_int,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
                0 as libc::c_int as PlLong,
            );
        }
        if (*pstm).fct_clearerr
            != ::std::mem::transmute::<
                libc::intptr_t,
                StmFct,
            >(-(1 as libc::c_int) as libc::intptr_t)
        {
            ::std::mem::transmute::<
                _,
                fn(_) -> libc::c_int,
            >((Some(((*pstm).fct_clearerr).unwrap())).unwrap())(file);
        }
    }
    if !((*pstm).pb_char.nb_elems == 0 as libc::c_int) {
        if (*pstm).pb_char.ptr != ((*pstm).pb_char.buff).as_mut_ptr() {
            c = *((*pstm).pb_char.ptr).offset(-(1 as libc::c_int) as isize);
        } else {
            c = (*pstm).pb_char.buff[(8 as libc::c_int - 1 as libc::c_int) as usize];
        }
    } else {
        c = Basic_Call_Fct_Getc(pstm);
        *(*pstm).pb_char.ptr = c;
        if (*pstm).pb_char.ptr
            != ((*pstm).pb_char.buff)
                .as_mut_ptr()
                .offset(8 as libc::c_int as isize)
                .offset(-(1 as libc::c_int as isize))
        {
            (*pstm).pb_char.ptr = ((*pstm).pb_char.ptr).offset(1);
            (*pstm).pb_char.ptr;
        } else {
            (*pstm).pb_char.ptr = ((*pstm).pb_char.buff).as_mut_ptr();
        }
        if (*pstm).pb_char.nb_elems < 8 as libc::c_int {
            (*pstm).pb_char.nb_elems += 1;
            (*pstm).pb_char.nb_elems;
        }
    }
    return c;
}
pub unsafe extern "C" fn Pl_Stream_Gets(
    mut str: *mut libc::c_char,
    mut size: libc::c_int,
    mut pstm: *mut StmInf,
) -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    let mut p: *mut libc::c_char = str;
    while !(p.offset_from(str) as libc::c_long >= size as libc::c_long) {
        c = Pl_Stream_Getc(pstm);
        if c == -(1 as libc::c_int) {
            break;
        }
        let fresh12 = p;
        p = p.offset(1);
        *fresh12 = c as libc::c_char;
        if c == '\n' as i32 {
            break;
        }
    }
    if c == -(1 as libc::c_int) && p == str {
        return 0 as *mut libc::c_char;
    }
    *p = '\0' as i32 as libc::c_char;
    return str;
}
pub unsafe extern "C" fn Pl_Stream_Gets_Prompt(
    mut prompt: *mut libc::c_char,
    mut pstm_o: *mut StmInf,
    mut str: *mut libc::c_char,
    mut size: libc::c_int,
    mut pstm_i: *mut StmInf,
) -> *mut libc::c_char {
    let mut save_le_prompt: *mut libc::c_char = pl_le_prompt;
    let mut save_use_le_prompt: libc::c_int = pl_use_le_prompt;
    pl_le_prompt = prompt;
    pl_use_le_prompt = 1 as libc::c_int;
    if (*pstm_i).fct_getc
        != ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            StmFct,
        >(Some(TTY_Getc as unsafe extern "C" fn() -> libc::c_int))
    {
        Pl_Stream_Printf(pstm_o, prompt);
    }
    str = Pl_Stream_Gets(str, size, pstm_i);
    pl_use_le_prompt = save_use_le_prompt;
    pl_le_prompt = save_le_prompt;
    return str;
}
pub unsafe extern "C" fn Pl_Stream_Putc(mut c: libc::c_int, mut pstm: *mut StmInf) {
    Basic_Call_Fct_Putc(c, pstm);
    if c != -(1 as libc::c_int) {
        (*pstm).char_count += 1;
        (*pstm).char_count;
    }
    if c == '\n' as i32 {
        (*pstm).line_count += 1;
        (*pstm).line_count;
        (*pstm).line_pos = 0 as libc::c_int as PlLong;
    } else {
        (*pstm).line_pos += 1;
        (*pstm).line_pos;
    };
}
pub unsafe extern "C" fn Pl_Stream_Puts(
    mut str: *mut libc::c_char,
    mut pstm: *mut StmInf,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    p = str;
    while *p != 0 {
        c = *p as libc::c_int;
        Basic_Call_Fct_Putc(c, pstm);
        if c != -(1 as libc::c_int) {
            (*pstm).char_count += 1;
            (*pstm).char_count;
        }
        if c == '\n' as i32 {
            (*pstm).line_count += 1;
            (*pstm).line_count;
            (*pstm).line_pos = 0 as libc::c_int as PlLong;
        } else {
            (*pstm).line_pos += 1;
            (*pstm).line_pos;
        }
        p = p.offset(1);
        p;
    }
    return p.offset_from(str) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn Pl_Stream_Printf(
    mut pstm: *mut StmInf,
    mut format: *mut libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut arg_ptr: ::std::ffi::VaListImpl;
    static mut str: [libc::c_char; 65535] = [0; 65535];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    arg_ptr = args.clone();
    vsprintf(str.as_mut_ptr(), format, arg_ptr.as_va_list());
    p = str.as_mut_ptr();
    while *p != 0 {
        c = *p as libc::c_int;
        Basic_Call_Fct_Putc(c, pstm);
        if c != -(1 as libc::c_int) {
            (*pstm).char_count += 1;
            (*pstm).char_count;
        }
        if c == '\n' as i32 {
            (*pstm).line_count += 1;
            (*pstm).line_count;
            (*pstm).line_pos = 0 as libc::c_int as PlLong;
        } else {
            (*pstm).line_pos += 1;
            (*pstm).line_pos;
        }
        p = p.offset(1);
        p;
    }
    return p.offset_from(str.as_mut_ptr()) as libc::c_long as libc::c_int;
}
pub unsafe extern "C" fn Pl_Stream_Flush(mut pstm: *mut StmInf) {
    let mut file: PlLong = (*pstm).file;
    if ((*pstm).prop).output() as libc::c_int != 0
        && (*pstm).fct_flush
            != ::std::mem::transmute::<
                libc::intptr_t,
                StmFct,
            >(-(1 as libc::c_int) as libc::intptr_t)
    {
        ::std::mem::transmute::<
            _,
            fn(_) -> libc::c_int,
        >((Some(((*pstm).fct_flush).unwrap())).unwrap())(file);
    }
}
pub unsafe extern "C" fn Pl_Stream_Close(mut pstm: *mut StmInf) -> libc::c_int {
    let mut file: PlLong = (*pstm).file;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if (*pstm).fct_close
        != ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t)
    {
        ret = ::std::mem::transmute::<
            _,
            fn(_) -> libc::c_int,
        >((Some(((*pstm).fct_close).unwrap())).unwrap())(file);
    }
    return ret;
}
pub unsafe extern "C" fn Pl_Stream_End_Of_Stream(mut pstm: *mut StmInf) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if ((*pstm).prop).eof_action() as libc::c_int == 2 as libc::c_int
        || ((*pstm).prop).input() == 0
    {
        return 0 as libc::c_int;
    }
    if (*pstm).eof_reached != 0 {
        return 2 as libc::c_int;
    }
    c = Pl_Stream_Peekc(pstm);
    if c == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Stream_Get_Position(
    mut pstm: *mut StmInf,
    mut offset: *mut PlLong,
    mut char_count: *mut PlLong,
    mut line_count: *mut PlLong,
    mut line_pos: *mut PlLong,
) {
    let mut file: PlLong = (*pstm).file;
    *offset = 0 as libc::c_int as PlLong;
    if ((*pstm).prop).reposition() as libc::c_int != 0
        && (*pstm).fct_tell
            != ::std::mem::transmute::<
                libc::intptr_t,
                StmFct,
            >(-(1 as libc::c_int) as libc::intptr_t)
    {
        *offset = ::std::mem::transmute::<
            _,
            fn(_) -> libc::c_int,
        >((Some(((*pstm).fct_tell).unwrap())).unwrap())(file) as PlLong;
        if *offset < 0 as libc::c_int as libc::c_long {
            *offset = 0 as libc::c_int as PlLong;
        } else {
            *offset = *offset - (*pstm).pb_char.nb_elems as libc::c_long;
            if *offset < 0 as libc::c_int as libc::c_long {
                *offset = 0 as libc::c_int as PlLong;
            }
        }
    }
    *char_count = (*pstm).char_count;
    if ((*pstm).prop).text() != 0 {
        *line_count = (*pstm).line_count;
        *line_pos = (*pstm).line_pos;
    } else {
        *line_count = 0 as libc::c_int as PlLong;
        *line_pos = 0 as libc::c_int as PlLong;
    };
}
pub unsafe extern "C" fn Pl_Stream_Set_Position(
    mut pstm: *mut StmInf,
    mut whence: libc::c_int,
    mut offset: PlLong,
    mut char_count: PlLong,
    mut line_count: PlLong,
    mut line_pos: PlLong,
) -> libc::c_int {
    let mut file: PlLong = (*pstm).file;
    let mut x: libc::c_int = 0;
    x = ::std::mem::transmute::<
        _,
        fn(_, _, _) -> libc::c_int,
    >((Some(((*pstm).fct_seek).unwrap())).unwrap())(file, offset, whence);
    if x != 0 as libc::c_int {
        return x;
    }
    (*pstm).char_count = char_count;
    if ((*pstm).prop).text() != 0 {
        (*pstm).line_count = line_count;
        (*pstm).line_pos = line_pos;
    }
    if (*pstm).eof_reached != 0 {
        (*pstm).eof_reached = 0 as libc::c_int;
        if (*pstm).fct_clearerr
            != ::std::mem::transmute::<
                libc::intptr_t,
                StmFct,
            >(-(1 as libc::c_int) as libc::intptr_t)
        {
            ::std::mem::transmute::<
                _,
                fn(_) -> libc::c_int,
            >((Some(((*pstm).fct_clearerr).unwrap())).unwrap())(file);
        }
    }
    (*pstm).pb_char.ptr = ((*pstm).pb_char.buff).as_mut_ptr();
    (*pstm).pb_char.nb_elems = 0 as libc::c_int;
    (*pstm).pb_line_pos.ptr = ((*pstm).pb_line_pos.buff).as_mut_ptr();
    (*pstm).pb_line_pos.nb_elems = 0 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn Pl_Stream_Set_Position_LC(
    mut pstm: *mut StmInf,
    mut line_count: PlLong,
    mut line_pos: PlLong,
) -> libc::c_int {
    let mut current_block: u64;
    let mut file: PlLong = (*pstm).file;
    let mut x: libc::c_int = 0;
    let mut p: *mut PlLong = 0 as *mut PlLong;
    let mut c: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut save_eof_reached: Bool = 0;
    let mut save_char_count: libc::c_int = 0;
    let mut save_line_count: libc::c_int = 0;
    let mut save_line_pos: libc::c_int = 0;
    let mut save_char_nb_elems: libc::c_int = 0;
    offset = ::std::mem::transmute::<
        _,
        fn(_) -> libc::c_int,
    >((Some(((*pstm).fct_tell).unwrap())).unwrap())(file);
    if offset < 0 as libc::c_int {
        return offset;
    }
    x = ::std::mem::transmute::<
        _,
        fn(_, _, _) -> libc::c_int,
    >(
        (Some(((*pstm).fct_seek).unwrap())).unwrap(),
    )(file, 0 as libc::c_int as PlLong, 0 as libc::c_int);
    if x != 0 as libc::c_int {
        return x;
    }
    save_eof_reached = (*pstm).eof_reached;
    save_char_count = (*pstm).char_count as libc::c_int;
    save_line_count = (*pstm).line_count as libc::c_int;
    save_line_pos = (*pstm).line_pos as libc::c_int;
    save_char_nb_elems = (*pstm).pb_char.nb_elems;
    (*pstm).char_count = 0 as libc::c_int as PlLong;
    (*pstm).line_count = 0 as libc::c_int as PlLong;
    (*pstm).line_pos = 0 as libc::c_int as PlLong;
    (*pstm).pb_char.nb_elems = 0 as libc::c_int;
    if (*pstm).eof_reached != 0 {
        (*pstm).eof_reached = 0 as libc::c_int;
        if (*pstm).fct_clearerr
            != ::std::mem::transmute::<
                libc::intptr_t,
                StmFct,
            >(-(1 as libc::c_int) as libc::intptr_t)
        {
            ::std::mem::transmute::<
                _,
                fn(_) -> libc::c_int,
            >((Some(((*pstm).fct_clearerr).unwrap())).unwrap())(file);
        }
    }
    p = &mut (*pstm).line_count;
    loop {
        if !(*p < line_count) {
            current_block = 2370887241019905314;
            break;
        }
        if Pl_Stream_Getc(pstm) == -(1 as libc::c_int) {
            current_block = 4799744081623093562;
            break;
        }
    }
    match current_block {
        2370887241019905314 => {
            p = &mut (*pstm).line_pos;
            loop {
                if !(*p < line_pos) {
                    current_block = 5634871135123216486;
                    break;
                }
                c = Pl_Stream_Getc(pstm);
                if c == -(1 as libc::c_int) {
                    current_block = 4799744081623093562;
                    break;
                }
                if c == '\n' as i32 {
                    current_block = 4799744081623093562;
                    break;
                }
            }
            match current_block {
                4799744081623093562 => {}
                _ => {
                    (*pstm).pb_char.ptr = ((*pstm).pb_char.buff).as_mut_ptr();
                    (*pstm).pb_char.nb_elems = 0 as libc::c_int;
                    (*pstm).pb_line_pos.ptr = ((*pstm).pb_line_pos.buff).as_mut_ptr();
                    (*pstm).pb_line_pos.nb_elems = 0 as libc::c_int;
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    (*pstm).eof_reached = save_eof_reached;
    (*pstm).char_count = save_char_count as PlLong;
    (*pstm).line_count = save_line_count as PlLong;
    (*pstm).line_pos = save_line_pos as PlLong;
    (*pstm).pb_char.nb_elems = save_char_nb_elems;
    x = ::std::mem::transmute::<
        _,
        fn(_, _, _) -> libc::c_int,
    >(
        (Some(((*pstm).fct_seek).unwrap())).unwrap(),
    )(file, offset as PlLong, 0 as libc::c_int);
    if x != 0 as libc::c_int {
        return x;
    }
    return -(2 as libc::c_int);
}
pub unsafe extern "C" fn Pl_Add_Str_Stream(
    mut buff: *mut libc::c_char,
    mut prop_other: libc::c_int,
) -> libc::c_int {
    let mut stm: libc::c_int = 0;
    let mut pstm: *mut StmInf = 0 as *mut StmInf;
    let mut prop: StmProp = StmProp {
        mode_input_output_text_reposition_eof_action_buffering_special_close_other: [0; 2],
        c2rust_padding: [0; 2],
    };
    let mut str_stream: *mut StrSInf = 0 as *mut StrSInf;
    str_stream = if !buff.is_null() {
        &mut static_str_stream_rd
    } else {
        &mut static_str_stream_wr
    };
    if !((*str_stream).ptr).is_null() {
        str_stream = Pl_Malloc_Check(
            ::std::mem::size_of::<StrSInf>() as libc::c_ulong as libc::c_uint,
            b"stream_supp.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1808 as libc::c_int,
        ) as *mut StrSInf;
        (*str_stream).buff_alloc_size = 0 as libc::c_int;
    }
    if !buff.is_null() {
        (*str_stream).buff = buff;
        prop.set_mode(0 as libc::c_int as libc::c_uint);
        prop.set_input(1 as libc::c_int as libc::c_uint);
        prop.set_output(0 as libc::c_int as libc::c_uint);
    } else {
        if (*str_stream).buff_alloc_size == 0 as libc::c_int {
            (*str_stream)
                .buff = Pl_Malloc_Check(
                1024 as libc::c_int as libc::c_uint,
                b"stream_supp.c\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                1824 as libc::c_int,
            );
            (*str_stream).buff_alloc_size = 1024 as libc::c_int;
        }
        prop.set_mode(1 as libc::c_int as libc::c_uint);
        prop.set_input(0 as libc::c_int as libc::c_uint);
        prop.set_output(1 as libc::c_int as libc::c_uint);
    }
    (*str_stream).ptr = (*str_stream).buff;
    prop.set_text(1 as libc::c_int as libc::c_uint);
    prop.set_reposition(0 as libc::c_int as libc::c_uint);
    prop.set_buffering(0 as libc::c_int as libc::c_uint);
    prop.set_eof_action(1 as libc::c_int as libc::c_uint);
    prop.set_special_close(1 as libc::c_int as libc::c_uint);
    prop.set_other(prop_other as libc::c_uint);
    stm = Find_Free_Stream();
    pstm = *pl_stm_tbl.offset(stm as isize);
    Init_Stream_Struct(
        atom_constant_term_stream,
        str_stream as PlLong,
        prop,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut StrSInf) -> libc::c_int>,
            StmFct,
        >(Some(Str_Stream_Getc as unsafe extern "C" fn(*mut StrSInf) -> libc::c_int)),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int, *mut StrSInf) -> ()>,
            StmFct,
        >(
            Some(
                Str_Stream_Putc as unsafe extern "C" fn(libc::c_int, *mut StrSInf) -> (),
            ),
        ),
        ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t),
        ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t),
        ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t),
        ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t),
        ::std::mem::transmute::<
            libc::intptr_t,
            StmFct,
        >(-(1 as libc::c_int) as libc::intptr_t),
        pstm,
    );
    return stm;
}
pub unsafe extern "C" fn Pl_Delete_Str_Stream(mut stm: libc::c_int) {
    let mut str_stream: *mut StrSInf = (**pl_stm_tbl.offset(stm as isize)).file
        as *mut StrSInf;
    if str_stream == &mut static_str_stream_rd as *mut StrSInf
        || str_stream == &mut static_str_stream_wr as *mut StrSInf
    {
        (*str_stream).ptr = 0 as *mut libc::c_char;
    } else {
        if (*str_stream).buff_alloc_size != 0 {
            free((*str_stream).buff as *mut libc::c_void);
        }
        free(str_stream as *mut libc::c_void);
    }
    Pl_Delete_Stream(stm);
}
pub unsafe extern "C" fn Pl_Term_Write_Str_Stream(
    mut stm: libc::c_int,
) -> *mut libc::c_char {
    let mut str_stream: *mut StrSInf = 0 as *mut StrSInf;
    str_stream = (**pl_stm_tbl.offset(stm as isize)).file as *mut StrSInf;
    *(*str_stream).ptr = '\0' as i32 as libc::c_char;
    return (*str_stream).buff;
}
unsafe extern "C" fn Str_Stream_Getc(mut str_stream: *mut StrSInf) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = *(*str_stream).ptr as libc::c_int;
    if c == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    (*str_stream).ptr = ((*str_stream).ptr).offset(1);
    (*str_stream).ptr;
    return c;
}
unsafe extern "C" fn Str_Stream_Putc(mut c: libc::c_int, mut str_stream: *mut StrSInf) {
    let mut size: libc::c_int = ((*str_stream).ptr).offset_from((*str_stream).buff)
        as libc::c_long as libc::c_int;
    let mut new_size: libc::c_int = 0;
    if size >= (*str_stream).buff_alloc_size - 1 as libc::c_int {
        new_size = (*str_stream).buff_alloc_size + 1024 as libc::c_int;
        (*str_stream)
            .buff = Pl_Realloc_Check(
            (*str_stream).buff,
            new_size as libc::c_uint,
            b"stream_supp.c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1939 as libc::c_int,
        );
        (*str_stream).buff_alloc_size = new_size;
        (*str_stream).ptr = ((*str_stream).buff).offset(size as isize);
    }
    let fresh13 = (*str_stream).ptr;
    (*str_stream).ptr = ((*str_stream).ptr).offset(1);
    *fresh13 = c as libc::c_char;
}

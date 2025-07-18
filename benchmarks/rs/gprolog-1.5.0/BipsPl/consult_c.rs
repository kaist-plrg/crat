use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn Pl_Create_Atom(name: *mut libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn Pl_M_Spawn_Redirect(
        arg: *mut *mut libc::c_char,
        detach: libc::c_int,
        f_in: *mut *mut FILE,
        f_out: *mut *mut FILE,
        f_err: *mut *mut FILE,
    ) -> libc::c_int;
    fn Pl_M_Get_Status(pid: libc::c_int) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn Pl_Rd_String_Check(start_word: WamWord) -> *mut libc::c_char;
    static mut pl_stm_tbl: *mut *mut StmInf;
    static mut pl_stm_top_level_input: libc::c_int;
    static mut pl_stm_top_level_output: libc::c_int;
    static mut pl_use_le_prompt: libc::c_int;
    fn Pl_Write_Pl_State_File(file_word: WamWord) -> Bool;
    static mut pl_flag_suspicious_warning: *mut FlagInf;
    static mut pl_flag_singleton_warning: *mut FlagInf;
    static mut pl_flag_multifile_warning: *mut FlagInf;
    static mut pl_sys_var: [PlLong; 0];
    fn Pl_Os_Error(ret_val: libc::c_int);
    fn Pl_Err_System(pl_atom_error: libc::c_int);
    fn Pl_Flush_All_Streams();
    fn Pl_Stream_Getc(pstm: *mut StmInf) -> libc::c_int;
    fn Pl_Stream_Putc(c: libc::c_int, pstm: *mut StmInf);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
pub type Bool = libc::c_int;
pub type WamWord = intptr_t;
pub type WamWordP = *mut WamWord;
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
pub type FlagInf = flag_inf;
pub static mut TR: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut B: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut H: WamWordP = 0 as *const WamWord as *mut WamWord;
pub static mut pl_def_max_atom: PlLong = 0;
pub static mut pl_reg_bank: *mut WamWord = 0 as *const WamWord as *mut WamWord;
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
pub unsafe extern "C" fn Pl_Consult_2(
    mut tmp_file_word: WamWord,
    mut pl_file_word: WamWord,
) -> Bool {
    let mut tmp_file: *mut libc::c_char = Pl_Rd_String_Check(tmp_file_word);
    let mut pl_file: *mut libc::c_char = Pl_Rd_String_Check(pl_file_word);
    let mut pstm_o: *mut StmInf = *pl_stm_tbl.offset(pl_stm_top_level_output as isize);
    let mut pstm_i: *mut StmInf = *pl_stm_tbl.offset(pl_stm_top_level_input as isize);
    let mut pid: libc::c_int = 0;
    let mut f_out: *mut FILE = 0 as *mut FILE;
    let mut f_in: *mut FILE = 0 as *mut FILE;
    let mut pf_in: *mut *mut FILE = 0 as *mut *mut FILE;
    let mut save: PlLong = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut status: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut save_use_le_prompt: libc::c_int = 0;
    let mut arg: [*mut libc::c_char; 13] = [
        b"pl2wam\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"--compile-msg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"--no-redef-error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"--pl-state\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        tmp_file,
        b"-o\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        tmp_file,
        pl_file,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    let mut warn_i: libc::c_int = (::std::mem::size_of::<[*mut libc::c_char; 13]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        .wrapping_sub(4 as libc::c_int as libc::c_ulong) as libc::c_int;
    if (*pl_flag_suspicious_warning).value == 0 {
        let fresh0 = warn_i;
        warn_i = warn_i + 1;
        arg[fresh0
            as usize] = b"--no-susp-warn\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*pl_flag_singleton_warning).value == 0 {
        let fresh1 = warn_i;
        warn_i = warn_i + 1;
        arg[fresh1
            as usize] = b"--no-singl-warn\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if (*pl_flag_multifile_warning).value == 0 {
        let fresh2 = warn_i;
        warn_i = warn_i + 1;
        arg[fresh2
            as usize] = b"--no-mult-warn\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    save = *pl_sys_var.as_mut_ptr().offset(20 as libc::c_int as isize);
    *pl_sys_var
        .as_mut_ptr()
        .offset(20 as libc::c_int as isize) = 1 as libc::c_int as PlLong;
    pf_in = &mut f_in;
    Pl_Write_Pl_State_File(tmp_file_word);
    *pl_sys_var.as_mut_ptr().offset(20 as libc::c_int as isize) = save;
    Pl_Flush_All_Streams();
    pid = Pl_M_Spawn_Redirect(
        arg.as_mut_ptr(),
        0 as libc::c_int,
        pf_in,
        &mut f_out,
        &mut f_out,
    );
    if pid == -(1 as libc::c_int) && *__errno_location() != 2 as libc::c_int {
        let mut _tst: libc::c_int = pid;
        if _tst < 0 as libc::c_int {
            Pl_Os_Error(_tst);
            return 0 as libc::c_int;
        }
    }
    if !(pid < 0 as libc::c_int) {
        save_use_le_prompt = pl_use_le_prompt;
        pl_use_le_prompt = 0 as libc::c_int;
        loop {
            c = fgetc(f_out);
            if c == -(1 as libc::c_int) {
                break;
            }
            if c == '\u{1}' as i32 {
                let mut current_block_28: u64;
                if p.is_null() {
                    c = Pl_Stream_Getc(pstm_i);
                    if c == -(1 as libc::c_int) {
                        current_block_28 = 16148733069943663812;
                    } else {
                        current_block_28 = 11385396242402735691;
                    }
                } else if *p as libc::c_int == '\0' as i32 {
                    current_block_28 = 16148733069943663812;
                } else {
                    let fresh4 = p;
                    p = p.offset(1);
                    c = *fresh4 as libc::c_int;
                    current_block_28 = 11385396242402735691;
                }
                match current_block_28 {
                    16148733069943663812 => {
                        p = b"end_of_file.\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_uchar;
                        let fresh3 = p;
                        p = p.offset(1);
                        c = *fresh3 as libc::c_int;
                    }
                    _ => {}
                }
                fputc(c, f_in);
                fflush(f_in);
            } else {
                Pl_Stream_Putc(c, pstm_o);
            }
        }
        pl_use_le_prompt = save_use_le_prompt;
        if !f_in.is_null() {
            fclose(f_in);
        }
        fclose(f_out);
        status = Pl_M_Get_Status(pid);
        if !(status < 0 as libc::c_int) {
            return (status == 0 as libc::c_int) as libc::c_int;
        }
    }
    Pl_Err_System(
        Pl_Create_Atom(
            b"error trying to execute pl2wam (maybe not found)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        ),
    );
    return 0 as libc::c_int;
}

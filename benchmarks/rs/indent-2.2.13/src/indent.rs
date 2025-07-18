use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn addkey(key: *mut libc::c_char, val: rwcodes_ty);
    fn textdomain(__domainname: *const libc::c_char) -> *mut libc::c_char;
    fn bindtextdomain(
        __domainname: *const libc::c_char,
        __dirname: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn lexi() -> codes_ty;
    fn cleanup_user_specials();
    static mut in_prog_pos: *mut libc::c_char;
    static mut com_lines: libc::c_int;
    static mut had_eof: BOOLEAN;
    static mut out_lines: libc::c_int;
    static mut settings: user_options_ty;
    static mut buf_ptr: *mut libc::c_char;
    static mut buf_end: *mut libc::c_char;
    static mut token: *mut libc::c_char;
    static mut parser_state_tos: *mut parser_state_ty;
    fn read_file(filename: *mut libc::c_char, _: *mut stat) -> *mut file_buffer_ty;
    fn read_stdin() -> *mut file_buffer_ty;
    fn fill_buffer();
    fn current_column() -> libc::c_int;
    fn initialize_backups();
    fn make_backup(file: *mut file_buffer_ty, file_stats: *const stat);
    fn xmalloc(size: libc::c_uint) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: libc::c_uint) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn message(
        kind: *mut libc::c_char,
        string: *mut libc::c_char,
        a0: *mut libc::c_char,
        a1: *mut libc::c_char,
    );
    fn init_parser();
    fn uninit_parser();
    fn reset_parser();
    fn set_defaults();
    fn set_defaults_after();
    fn set_option(
        option: *const libc::c_char,
        param: *const libc::c_char,
        explicit: libc::c_int,
        option_source: *const libc::c_char,
    ) -> libc::c_int;
    fn set_profile() -> *mut libc::c_char;
    static mut buf_break: *mut buf_break_st_ty;
    fn clear_buf_break_list(pbreak_line: *mut BOOLEAN);
    fn set_buf_break(code: bb_code_ty, paren_targ: libc::c_int);
    fn dump_line(
        force_nl: libc::c_int,
        paren_targ: *mut libc::c_int,
        break_line: *mut BOOLEAN,
    );
    fn flush_output();
    fn open_output(filename: *const libc::c_char, mode: *const libc::c_char);
    fn reopen_output_trunc(filename: *const libc::c_char);
    fn close_output(file_stats: *mut stat, filename: *const libc::c_char);
    fn output_line_length() -> libc::c_int;
    fn handle_the_token(
        type_code: codes_ty,
        scase: *mut BOOLEAN,
        force_nl: *mut BOOLEAN,
        sp_sw: *mut BOOLEAN,
        flushed_nl: *mut BOOLEAN,
        hd_type: *mut codes_ty,
        dec_ind: *mut libc::c_int,
        last_token_ends_sp: *mut BOOLEAN,
        file_exit_value: *mut exit_values_ty,
        can_break: bb_code_ty,
        last_else: *mut BOOLEAN,
        is_procname_definition: BOOLEAN,
        pbreak_line: *mut BOOLEAN,
    );
    fn check_code_size();
    fn need_chars(bp: *mut buf_ty, needed: size_t);
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type rwcodes = libc::c_uint;
pub const rw_return: rwcodes = 12;
pub const rw_sizeof: rwcodes = 11;
pub const rw_sp_else: rwcodes = 10;
pub const rw_sp_nparen: rwcodes = 9;
pub const rw_sp_paren: rwcodes = 8;
pub const rw_decl: rwcodes = 7;
pub const rw_enum: rwcodes = 6;
pub const rw_struct_like: rwcodes = 5;
pub const rw_case: rwcodes = 4;
pub const rw_switch: rwcodes = 3;
pub const rw_break: rwcodes = 2;
pub const rw_operator: rwcodes = 1;
pub const rw_none: rwcodes = 0;
pub type rwcodes_ty = rwcodes;
pub type codes = libc::c_uint;
pub const number_of_codes: codes = 42;
pub const attribute: codes = 41;
pub const struct_delim: codes = 40;
pub const elsehead: codes = 39;
pub const ifhead: codes = 38;
pub const dostmt: codes = 37;
pub const dohead: codes = 36;
pub const dolit: codes = 35;
pub const elselit: codes = 34;
pub const stmtl: codes = 33;
pub const stmt: codes = 32;
pub const forstmt: codes = 31;
pub const whilestmt: codes = 30;
pub const elseifstmt: codes = 29;
pub const ifstmt: codes = 28;
pub const sp_else: codes = 27;
pub const sp_nparen: codes = 26;
pub const sp_paren: codes = 25;
pub const decl: codes = 24;
pub const form_feed: codes = 23;
pub const preesc: codes = 22;
pub const swstmt: codes = 21;
pub const cplus_comment: codes = 20;
pub const comment: codes = 19;
pub const comma: codes = 18;
pub const cpp_operator: codes = 17;
pub const overloaded: codes = 16;
pub const ident: codes = 15;
pub const rbrace: codes = 14;
pub const lbrace: codes = 13;
pub const semicolon: codes = 12;
pub const doublecolon: codes = 11;
pub const colon: codes = 10;
pub const casestmt: codes = 9;
pub const question: codes = 8;
pub const postop: codes = 7;
pub const binary_op: codes = 6;
pub const unary_op: codes = 5;
pub const start_token: codes = 4;
pub const rparen: codes = 3;
pub const lparen: codes = 2;
pub const newline: codes = 1;
pub const code_eof: codes = 0;
pub type codes_ty = codes;
pub type exit_values = libc::c_uint;
pub const system_error: exit_values = 5;
pub const indent_fatal: exit_values = 4;
pub const indent_punt: exit_values = 3;
pub const indent_error: exit_values = 2;
pub const invocation_error: exit_values = 64;
pub const total_success: exit_values = 0;
pub type exit_values_ty = exit_values;
pub type BOOLEAN = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_buffer {
    pub name: *mut libc::c_char,
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub type file_buffer_ty = file_buffer;
pub type bb_code = libc::c_uint;
pub const bb_cast: bb_code = 28;
pub const bb_doublecolon: bb_code = 27;
pub const bb_operator6: bb_code = 26;
pub const bb_operator5: bb_code = 25;
pub const bb_operator4: bb_code = 24;
pub const bb_operator2: bb_code = 23;
pub const bb_struct_delim: bb_code = 22;
pub const bb_attribute: bb_code = 21;
pub const bb_ident: bb_code = 20;
pub const bb_const_qualifier: bb_code = 19;
pub const bb_overloaded: bb_code = 18;
pub const bb_rbrace: bb_code = 17;
pub const bb_lbrace: bb_code = 16;
pub const bb_semicolon: bb_code = 15;
pub const bb_label: bb_code = 14;
pub const bb_colon: bb_code = 13;
pub const bb_question: bb_code = 12;
pub const bb_comparisation: bb_code = 11;
pub const bb_after_equal_sign: bb_code = 10;
pub const bb_after_boolean_binary_op: bb_code = 9;
pub const bb_before_boolean_binary_op: bb_code = 8;
pub const bb_binary_op: bb_code = 7;
pub const bb_unary_op: bb_code = 6;
pub const bb_dec_ind: bb_code = 5;
pub const bb_proc_call: bb_code = 4;
pub const bb_embedded_comment_end: bb_code = 3;
pub const bb_embedded_comment_start: bb_code = 2;
pub const bb_comma: bb_code = 1;
pub const bb_none: bb_code = 0;
pub type bb_code_ty = bb_code;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf {
    pub ptr: *mut libc::c_char,
    pub end: *mut libc::c_char,
    pub size: libc::c_int,
    pub len: libc::c_int,
    pub start_column: libc::c_int,
    pub column: libc::c_int,
}
pub type buf_ty = buf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user_options_st {
    pub verbose: libc::c_int,
    pub use_tabs: libc::c_int,
    pub tabsize: libc::c_int,
    pub use_stdout: libc::c_int,
    pub space_sp_semicolon: libc::c_int,
    pub swallow_optional_blanklines: libc::c_int,
    pub star_comment_cont: libc::c_int,
    pub struct_brace_indent: libc::c_int,
    pub space_after_while: libc::c_int,
    pub space_after_if: libc::c_int,
    pub space_after_for: libc::c_int,
    pub procnames_start_line: libc::c_int,
    pub parentheses_space: libc::c_int,
    pub preserve_mtime: libc::c_int,
    pub paren_indent: libc::c_int,
    pub proc_calls_space: libc::c_int,
    pub leave_preproc_space: libc::c_int,
    pub force_preproc_width: libc::c_int,
    pub lineup_to_parens: libc::c_int,
    pub honour_newlines: libc::c_int,
    pub fix_nested_comments: libc::c_int,
    pub format_comments: libc::c_int,
    pub format_col1_comments: libc::c_int,
    pub extra_expression_indent: libc::c_int,
    pub ljust_decl: libc::c_int,
    pub cast_space: libc::c_int,
    pub cuddle_else: libc::c_int,
    pub cuddle_do_while: libc::c_int,
    pub comment_delimiter_on_blankline: libc::c_int,
    pub blank_after_sizeof: libc::c_int,
    pub break_function_decl_args: libc::c_int,
    pub break_function_decl_args_end: libc::c_int,
    pub leave_comma: libc::c_int,
    pub break_before_boolean_operator: libc::c_int,
    pub blanklines_before_blockcomments: libc::c_int,
    pub blanklines_after_declarations: libc::c_int,
    pub blanklines_after_procs: libc::c_int,
    pub blanklines_after_declarations_at_proctop: libc::c_int,
    pub blanklines_around_conditional_compilation: libc::c_int,
    pub comment_max_col: libc::c_int,
    pub max_col: libc::c_int,
    pub label_offset: libc::c_int,
    pub ind_size: libc::c_int,
    pub indent_parameters: libc::c_int,
    pub decl_indent: libc::c_int,
    pub unindent_displace: libc::c_int,
    pub else_endif_col: libc::c_int,
    pub case_indent: libc::c_int,
    pub continuation_indent: libc::c_int,
    pub decl_com_ind: libc::c_int,
    pub case_brace_indent: libc::c_int,
    pub c_plus_plus: libc::c_int,
    pub com_ind: libc::c_int,
    pub braces_on_struct_decl_line: libc::c_int,
    pub braces_on_func_def_line: libc::c_int,
    pub btype_2: libc::c_int,
    pub brace_indent: libc::c_int,
    pub expect_output_file: libc::c_int,
    pub pointer_align_right: libc::c_int,
    pub gettext_strings: libc::c_int,
    pub allow_single_line_conditionals: libc::c_int,
    pub align_with_spaces: libc::c_int,
    pub spaces_around_initializers: libc::c_int,
    pub dont_tab_align_comments: libc::c_int,
}
pub type user_options_ty = user_options_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_state {
    pub next: *mut parser_state,
    pub last_token: codes_ty,
    pub p_stack: *mut codes_ty,
    pub p_stack_size: libc::c_int,
    pub il: *mut libc::c_int,
    pub last_rw: rwcodes_ty,
    pub last_rw_depth: libc::c_int,
    pub cstk: *mut libc::c_int,
    pub tos: libc::c_int,
    pub box_com: libc::c_int,
    pub cast_mask: libc::c_int,
    pub noncast_mask: libc::c_int,
    pub sizeof_mask: libc::c_int,
    pub block_init: libc::c_int,
    pub block_init_level: libc::c_int,
    pub last_nl: libc::c_int,
    pub last_saw_nl: libc::c_int,
    pub saw_double_colon: libc::c_int,
    pub is_func_ptr_decl: libc::c_int,
    pub broken_at_non_nl: libc::c_int,
    pub in_or_st: libc::c_int,
    pub col_1: libc::c_int,
    pub com_col: libc::c_int,
    pub dec_nest: libc::c_int,
    pub decl_on_line: libc::c_int,
    pub i_l_follow: libc::c_int,
    pub in_decl: BOOLEAN,
    pub in_stmt: libc::c_int,
    pub in_parameter_declaration: libc::c_int,
    pub ind_level: libc::c_int,
    pub ind_stmt: libc::c_int,
    pub last_u_d: libc::c_int,
    pub p_l_follow: libc::c_int,
    pub paren_level: libc::c_int,
    pub paren_depth: libc::c_int,
    pub paren_indents: *mut libc::c_short,
    pub paren_indents_size: libc::c_int,
    pub pcase: libc::c_int,
    pub search_brace: libc::c_int,
    pub use_ff: libc::c_int,
    pub want_blank: libc::c_int,
    pub can_break: bb_code_ty,
    pub its_a_keyword: libc::c_int,
    pub sizeof_keyword: libc::c_int,
    pub procname: *mut libc::c_char,
    pub procname_end: *mut libc::c_char,
    pub classname: *mut libc::c_char,
    pub classname_end: *mut libc::c_char,
    pub just_saw_decl: libc::c_int,
    pub matching_brace_on_same_line: libc::c_int,
}
pub type parser_state_ty = parser_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_break_st {
    pub next: *mut buf_break_st,
    pub prev: *mut buf_break_st,
    pub offset: libc::c_int,
    pub corresponds_to: *mut libc::c_char,
    pub target_col: libc::c_int,
    pub first_level: libc::c_int,
    pub level: libc::c_int,
    pub col: libc::c_int,
    pub priority_code_length: libc::c_int,
    pub priority_code: bb_code_ty,
    pub priority_newline: libc::c_int,
    pub priority: libc::c_int,
}
pub type buf_break_st_ty = buf_break_st;
pub static mut labbuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut s_lab: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut e_lab: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut l_lab: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut codebuf: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut s_code: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut e_code: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut l_code: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut combuf: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut s_com: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut e_com: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut l_com: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut s_code_corresponds_to: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut save_com: buf_ty = buf_ty {
    ptr: 0 as *const libc::c_char as *mut libc::c_char,
    end: 0 as *const libc::c_char as *mut libc::c_char,
    size: 0,
    len: 0,
    start_column: 0,
    column: 0,
};
pub static mut bp_save: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut be_save: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut code_lines: libc::c_int = 0 as libc::c_int;
pub static mut line_no: libc::c_int = 0 as libc::c_int;
pub static mut break_comma: libc::c_int = 0 as libc::c_int;
pub static mut n_real_blanklines: libc::c_int = 0 as libc::c_int;
pub static mut prefix_blankline_requested: libc::c_int = 0 as libc::c_int;
pub static mut prefix_blankline_requested_code: codes_ty = code_eof;
pub static mut postfix_blankline_requested: libc::c_int = 0 as libc::c_int;
pub static mut postfix_blankline_requested_code: codes_ty = code_eof;
pub static mut in_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut current_input: *mut file_buffer_ty = 0 as *const file_buffer_ty
    as *mut file_buffer_ty;
pub static mut embedded_comment_on_line: libc::c_int = 0 as libc::c_int;
pub static mut else_or_endif: libc::c_int = 0 as libc::c_int;
pub static mut di_stack: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
pub static mut di_stack_alloc: libc::c_int = 0 as libc::c_int;
pub static mut squest: libc::c_int = 0 as libc::c_int;
pub static mut in_prog_size: libc::c_ulong = 0 as libc::c_uint as libc::c_ulong;
pub static mut in_prog: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut paren_target: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn sw_buffer() {
    (*parser_state_tos).search_brace = 0 as libc::c_int;
    bp_save = buf_ptr;
    be_save = buf_end;
    buf_ptr = save_com.ptr;
    need_chars(&mut save_com, 1 as libc::c_int as size_t);
    buf_end = save_com.end;
    save_com.end = save_com.ptr;
}
unsafe extern "C" fn search_brace(
    mut type_code: *mut codes_ty,
    mut force_nl: *mut BOOLEAN,
    mut flushed_nl: *mut BOOLEAN,
    mut last_else: *mut BOOLEAN,
    mut is_procname_definition: *mut BOOLEAN,
    mut pbreak_line: *mut BOOLEAN,
) -> BOOLEAN {
    let mut cur_token: libc::c_int = 0;
    while (*parser_state_tos).search_brace != 0 {
        cur_token = *type_code as libc::c_int;
        let mut current_block_77: u64;
        match cur_token {
            1 => {
                line_no += 1;
                line_no;
                *flushed_nl = 1 as libc::c_int as BOOLEAN;
                current_block_77 = 5807581744382915773;
            }
            23 => {
                current_block_77 = 5807581744382915773;
            }
            13 => {
                if save_com.end == save_com.ptr {
                    (*parser_state_tos).search_brace = 0 as libc::c_int;
                    return 1 as libc::c_int as BOOLEAN;
                }
                if settings.btype_2 != 0
                    && (*parser_state_tos).last_token as libc::c_uint
                        != rbrace as libc::c_int as libc::c_uint
                {
                    *(save_com.ptr)
                        .offset(0 as libc::c_int as isize) = '{' as i32 as libc::c_char;
                    save_com.len = 1 as libc::c_int;
                    save_com.column = current_column();
                } else {
                    let fresh0 = save_com.end;
                    save_com.end = (save_com.end).offset(1);
                    *fresh0 = '\n' as i32 as libc::c_char;
                    let fresh1 = save_com.end;
                    save_com.end = (save_com.end).offset(1);
                    *fresh1 = '{' as i32 as libc::c_char;
                    save_com.len += 2 as libc::c_int;
                    sw_buffer();
                }
                current_block_77 = 5807581744382915773;
            }
            19 => {
                if *flushed_nl == 0 || save_com.end != save_com.ptr {
                    need_chars(&mut save_com, 10 as libc::c_int as size_t);
                    if save_com.end == save_com.ptr {
                        save_com.start_column = current_column();
                        let ref mut fresh2 = *(save_com.ptr)
                            .offset(1 as libc::c_int as isize);
                        *fresh2 = ' ' as i32 as libc::c_char;
                        *(save_com.ptr).offset(0 as libc::c_int as isize) = *fresh2;
                        save_com.end = (save_com.ptr).offset(2 as libc::c_int as isize);
                        save_com.len = 2 as libc::c_int;
                        save_com.column = current_column();
                    } else {
                        let fresh3 = save_com.end;
                        save_com.end = (save_com.end).offset(1);
                        *fresh3 = '\n' as i32 as libc::c_char;
                        let fresh4 = save_com.end;
                        save_com.end = (save_com.end).offset(1);
                        *fresh4 = ' ' as i32 as libc::c_char;
                        save_com.len += 2 as libc::c_int;
                        line_no -= 1;
                        line_no;
                    }
                    let fresh5 = save_com.end;
                    save_com.end = (save_com.end).offset(1);
                    *fresh5 = '/' as i32 as libc::c_char;
                    let fresh6 = save_com.end;
                    save_com.end = (save_com.end).offset(1);
                    *fresh6 = '*' as i32 as libc::c_char;
                    loop {
                        need_chars(&mut save_com, 2 as libc::c_int as size_t);
                        let fresh7 = buf_ptr;
                        buf_ptr = buf_ptr.offset(1);
                        *save_com.end = *fresh7;
                        save_com.len += 1;
                        save_com.len;
                        if buf_ptr >= buf_end {
                            fill_buffer();
                            if had_eof != 0 {
                                message(
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"Error\0" as *const u8 as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    dcgettext(
                                        0 as *const libc::c_char,
                                        b"EOF encountered in comment\0" as *const u8
                                            as *const libc::c_char,
                                        5 as libc::c_int,
                                    ),
                                    0 as *mut libc::c_void as *mut libc::c_char,
                                    0 as *mut libc::c_void as *mut libc::c_char,
                                );
                                return indent_punt as libc::c_int as BOOLEAN;
                            }
                        }
                        let fresh8 = save_com.end;
                        save_com.end = (save_com.end).offset(1);
                        if *fresh8 as libc::c_int == '*' as i32
                            && *buf_ptr as libc::c_int == '/' as i32
                        {
                            break;
                        }
                    }
                    let fresh9 = save_com.end;
                    save_com.end = (save_com.end).offset(1);
                    *fresh9 = '/' as i32 as libc::c_char;
                    save_com.len += 1;
                    save_com.len;
                    buf_ptr = buf_ptr.offset(1);
                    if buf_ptr >= buf_end {
                        fill_buffer();
                    }
                    current_block_77 = 5807581744382915773;
                } else {
                    current_block_77 = 10332453687116070454;
                }
            }
            _ => {
                current_block_77 = 10332453687116070454;
            }
        }
        match current_block_77 {
            10332453687116070454 => {
                if *type_code as libc::c_uint == sp_paren as libc::c_int as libc::c_uint
                    && *token as libc::c_int == 'i' as i32
                    && *last_else as libc::c_int != 0
                    || *type_code as libc::c_uint
                        == sp_else as libc::c_int as libc::c_uint && e_code != s_code
                        && *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == '}' as i32 && save_com.end == save_com.ptr
                {
                    *force_nl = 0 as libc::c_int as BOOLEAN;
                } else if *flushed_nl != 0 {
                    *force_nl = 1 as libc::c_int as BOOLEAN;
                }
                if save_com.end == save_com.ptr {
                    (*parser_state_tos).search_brace = 0 as libc::c_int;
                    return 1 as libc::c_int as BOOLEAN;
                }
                if *force_nl != 0 {
                    *force_nl = 0 as libc::c_int as BOOLEAN;
                    line_no -= 1;
                    line_no;
                    need_chars(&mut save_com, 2 as libc::c_int as size_t);
                    let fresh10 = save_com.end;
                    save_com.end = (save_com.end).offset(1);
                    *fresh10 = '\n' as i32 as libc::c_char;
                    save_com.len += 1;
                    save_com.len;
                    if settings.verbose != 0 && *flushed_nl == 0 {
                        message(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Warning\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Line broken\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            0 as *mut libc::c_void as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut libc::c_char,
                        );
                    }
                    *flushed_nl = 0 as libc::c_int as BOOLEAN;
                }
                let fresh11 = save_com.end;
                save_com.end = (save_com.end).offset(1);
                *fresh11 = ' ' as i32 as libc::c_char;
                save_com.len += 1;
                save_com.len;
                buf_ptr = token;
                (*parser_state_tos)
                    .procname = b"\0\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                (*parser_state_tos)
                    .procname_end = b"\0\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                (*parser_state_tos)
                    .classname = b"\0\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                (*parser_state_tos)
                    .classname_end = b"\0\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                sw_buffer();
            }
            _ => {}
        }
        if *type_code as libc::c_uint != code_eof as libc::c_int as libc::c_uint {
            *type_code = lexi();
            if cur_token == newline as libc::c_int
                && (*type_code as libc::c_uint == newline as libc::c_int as libc::c_uint
                    || *type_code as libc::c_uint
                        == comment as libc::c_int as libc::c_uint)
                && (*parser_state_tos).last_token as libc::c_uint
                    == rbrace as libc::c_int as libc::c_uint
            {
                if save_com.len == 0 {
                    dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
                    *flushed_nl = 1 as libc::c_int as BOOLEAN;
                } else if *type_code as libc::c_uint
                    == newline as libc::c_int as libc::c_uint
                {
                    let fresh12 = save_com.end;
                    save_com.end = (save_com.end).offset(1);
                    *fresh12 = '\n' as i32 as libc::c_char;
                    save_com.len += 1;
                    save_com.len;
                }
            }
            *is_procname_definition = (*((*parser_state_tos).procname)
                .offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
                && (*parser_state_tos).in_parameter_declaration != 0) as libc::c_int
                as BOOLEAN;
        }
        if *type_code as libc::c_uint == ident as libc::c_int as libc::c_uint
            && *flushed_nl as libc::c_int != 0 && settings.procnames_start_line == 0
            && (*parser_state_tos).in_decl as libc::c_int != 0
            && *((*parser_state_tos).procname).offset(0 as libc::c_int as isize)
                as libc::c_int != '\0' as i32
        {
            *flushed_nl = 0 as libc::c_int as BOOLEAN;
        }
    }
    *last_else = 0 as libc::c_int as BOOLEAN;
    return 1 as libc::c_int as BOOLEAN;
}
unsafe extern "C" fn indent_main_loop(mut pbreak_line: *mut BOOLEAN) -> exit_values_ty {
    let mut hd_type: codes_ty = code_eof;
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_code: codes_ty = start_token;
    let mut file_exit_value: exit_values_ty = total_success;
    let mut dec_ind: libc::c_int = 0 as libc::c_int;
    let mut scase: BOOLEAN = 0 as libc::c_int as BOOLEAN;
    let mut flushed_nl: BOOLEAN = 0;
    let mut sp_sw: BOOLEAN = 0 as libc::c_int as BOOLEAN;
    let mut force_nl: BOOLEAN = 0 as libc::c_int as BOOLEAN;
    let mut last_token_ends_sp: BOOLEAN = 0 as libc::c_int as BOOLEAN;
    let mut last_else: BOOLEAN = 0 as libc::c_int as BOOLEAN;
    loop {
        let mut is_procname_definition: BOOLEAN = 0;
        let mut can_break: bb_code_ty = bb_none;
        if type_code as libc::c_uint != newline as libc::c_int as libc::c_uint {
            can_break = (*parser_state_tos).can_break;
        }
        (*parser_state_tos).last_saw_nl = 0 as libc::c_int;
        (*parser_state_tos).can_break = bb_none;
        type_code = lexi();
        if settings.max_col > 0 as libc::c_int && !buf_break.is_null()
            && ((*parser_state_tos).last_token as libc::c_uint
                == ident as libc::c_int as libc::c_uint
                && type_code as libc::c_uint != comma as libc::c_int as libc::c_uint
                && type_code as libc::c_uint != semicolon as libc::c_int as libc::c_uint
                && type_code as libc::c_uint != newline as libc::c_int as libc::c_uint
                && type_code as libc::c_uint != form_feed as libc::c_int as libc::c_uint
                && type_code as libc::c_uint != rparen as libc::c_int as libc::c_uint
                && type_code as libc::c_uint
                    != struct_delim as libc::c_int as libc::c_uint
                || (*parser_state_tos).last_token as libc::c_uint
                    == rparen as libc::c_int as libc::c_uint
                    && type_code as libc::c_uint != comma as libc::c_int as libc::c_uint
                    && type_code as libc::c_uint
                        != rparen as libc::c_int as libc::c_uint)
            && output_line_length() > settings.max_col
        {
            *pbreak_line = 1 as libc::c_int as BOOLEAN;
        }
        if last_token_ends_sp as libc::c_int > 0 as libc::c_int {
            last_token_ends_sp = last_token_ends_sp.wrapping_sub(1);
            last_token_ends_sp;
        }
        is_procname_definition = (*((*parser_state_tos).procname)
            .offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
            && (*parser_state_tos).in_parameter_declaration != 0
            || *((*parser_state_tos).classname).offset(0 as libc::c_int as isize)
                as libc::c_int != '\0' as i32) as libc::c_int as BOOLEAN;
        flushed_nl = 0 as libc::c_int as BOOLEAN;
        if settings.allow_single_line_conditionals != 0
            && ((*parser_state_tos).last_token as libc::c_uint
                == rparen as libc::c_int as libc::c_uint
                || (*parser_state_tos).last_token as libc::c_uint
                    == sp_else as libc::c_int as libc::c_uint)
        {
            force_nl = 0 as libc::c_int as BOOLEAN;
        }
        if (*parser_state_tos).block_init != 0
            && (*parser_state_tos).last_token as libc::c_uint
                == rbrace as libc::c_int as libc::c_uint
            && *token as libc::c_int == ',' as i32
        {
            force_nl = 0 as libc::c_int as BOOLEAN;
        }
        if search_brace(
            &mut type_code,
            &mut force_nl,
            &mut flushed_nl,
            &mut last_else,
            &mut is_procname_definition,
            pbreak_line,
        ) == 0
        {
            return indent_punt;
        }
        if type_code as libc::c_uint == code_eof as libc::c_int as libc::c_uint {
            if s_lab != e_lab || s_code != e_code || s_com != e_com {
                dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
            }
            if (*parser_state_tos).tos > 1 as libc::c_int {
                message(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unexpected end of file\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
                file_exit_value = indent_error;
            }
            if settings.verbose != 0 {
                printf(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"There were %d non-blank output lines and %d comments\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    out_lines,
                    com_lines,
                );
                if com_lines > 0 as libc::c_int && code_lines > 0 as libc::c_int {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"(Lines with comments)/(Lines with code): %6.3f\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        1.0f64 * com_lines as libc::c_double
                            / code_lines as libc::c_double,
                    );
                }
            }
            flush_output();
            return file_exit_value;
        }
        if type_code as libc::c_uint != comment as libc::c_int as libc::c_uint
            && type_code as libc::c_uint != cplus_comment as libc::c_int as libc::c_uint
            && type_code as libc::c_uint != newline as libc::c_int as libc::c_uint
            && type_code as libc::c_uint != preesc as libc::c_int as libc::c_uint
            && type_code as libc::c_uint != form_feed as libc::c_int as libc::c_uint
        {
            if force_nl as libc::c_int != 0
                && type_code as libc::c_uint != semicolon as libc::c_int as libc::c_uint
                && (type_code as libc::c_uint != lbrace as libc::c_int as libc::c_uint
                    || (*parser_state_tos).in_decl == 0 && settings.btype_2 == 0
                    || (*parser_state_tos).in_decl as libc::c_int != 0
                        && settings.braces_on_struct_decl_line == 0
                    || (*parser_state_tos).last_token as libc::c_uint
                        == rbrace as libc::c_int as libc::c_uint)
            {
                if settings.verbose != 0 && flushed_nl == 0 {
                    message(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Warning\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Line broken 2\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        0 as *mut libc::c_void as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_char,
                    );
                }
                flushed_nl = 0 as libc::c_int as BOOLEAN;
                dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
                (*parser_state_tos).want_blank = 0 as libc::c_int;
                force_nl = 0 as libc::c_int as BOOLEAN;
            }
            (*parser_state_tos).in_stmt = 1 as libc::c_int;
            if s_com != e_com {
                if e_code != s_code {
                    set_buf_break(bb_embedded_comment_start, paren_target);
                    let fresh13 = e_code;
                    e_code = e_code.offset(1);
                    *fresh13 = ' ' as i32 as libc::c_char;
                    embedded_comment_on_line = 2 as libc::c_int;
                } else {
                    embedded_comment_on_line = 1 as libc::c_int;
                }
                t_ptr = s_com;
                while *t_ptr != 0 {
                    check_code_size();
                    let fresh14 = e_code;
                    e_code = e_code.offset(1);
                    *fresh14 = *t_ptr;
                    t_ptr = t_ptr.offset(1);
                    t_ptr;
                }
                set_buf_break(bb_embedded_comment_end, paren_target);
                let fresh15 = e_code;
                e_code = e_code.offset(1);
                *fresh15 = ' ' as i32 as libc::c_char;
                *e_code = '\0' as i32 as libc::c_char;
                (*parser_state_tos).want_blank = 0 as libc::c_int;
                e_com = s_com;
            }
        } else if type_code as libc::c_uint != comment as libc::c_int as libc::c_uint
            && type_code as libc::c_uint != cplus_comment as libc::c_int as libc::c_uint
            && !(settings.break_function_decl_args != 0
                && (*parser_state_tos).last_token as libc::c_uint
                    == comma as libc::c_int as libc::c_uint)
            && !((*parser_state_tos).last_token as libc::c_uint
                == comma as libc::c_int as libc::c_uint && settings.leave_comma == 0)
        {
            force_nl = 0 as libc::c_int as BOOLEAN;
        }
        check_code_size();
        handle_the_token(
            type_code,
            &mut scase,
            &mut force_nl,
            &mut sp_sw,
            &mut flushed_nl,
            &mut hd_type,
            &mut dec_ind,
            &mut last_token_ends_sp,
            &mut file_exit_value,
            can_break,
            &mut last_else,
            is_procname_definition,
            pbreak_line,
        );
        *e_code = '\0' as i32 as libc::c_char;
        if type_code as libc::c_uint != comment as libc::c_int as libc::c_uint
            && type_code as libc::c_uint != cplus_comment as libc::c_int as libc::c_uint
            && type_code as libc::c_uint != newline as libc::c_int as libc::c_uint
            && type_code as libc::c_uint != preesc as libc::c_int as libc::c_uint
            && type_code as libc::c_uint != form_feed as libc::c_int as libc::c_uint
        {
            (*parser_state_tos).last_token = type_code;
        }
        if settings.max_col > 0 as libc::c_int && !buf_break.is_null() {
            if (type_code as libc::c_uint == binary_op as libc::c_int as libc::c_uint
                || type_code as libc::c_uint == postop as libc::c_int as libc::c_uint
                || type_code as libc::c_uint == question as libc::c_int as libc::c_uint
                || type_code as libc::c_uint == colon as libc::c_int as libc::c_uint
                    && (scase as libc::c_int != 0 || squest <= 0 as libc::c_int)
                || type_code as libc::c_uint == semicolon as libc::c_int as libc::c_uint
                || type_code as libc::c_uint == sp_nparen as libc::c_int as libc::c_uint
                || type_code as libc::c_uint == sp_else as libc::c_int as libc::c_uint
                || type_code as libc::c_uint == ident as libc::c_int as libc::c_uint
                    && *token as libc::c_int == '"' as i32
                || type_code as libc::c_uint
                    == struct_delim as libc::c_int as libc::c_uint
                || type_code as libc::c_uint == comma as libc::c_int as libc::c_uint)
                && output_line_length() > settings.max_col
            {
                *pbreak_line = 1 as libc::c_int as BOOLEAN;
            }
        }
    };
}
unsafe extern "C" fn indent(mut this_file: *mut file_buffer_ty) -> exit_values_ty {
    let mut break_line: BOOLEAN = 0 as libc::c_int as BOOLEAN;
    in_prog = (*this_file).data;
    in_prog_pos = (*this_file).data;
    in_prog_size = (*this_file).size;
    squest = 0 as libc::c_int;
    n_real_blanklines = 0 as libc::c_int;
    postfix_blankline_requested = 0 as libc::c_int;
    clear_buf_break_list(&mut break_line);
    if settings.decl_com_ind <= 0 as libc::c_int {
        settings
            .decl_com_ind = if settings.ljust_decl != 0 {
            if settings.com_ind <= 10 as libc::c_int {
                2 as libc::c_int
            } else {
                settings.com_ind - 8 as libc::c_int
            }
        } else {
            settings.com_ind
        };
    }
    if settings.continuation_indent == 0 as libc::c_int {
        settings.continuation_indent = settings.ind_size;
    }
    if settings.paren_indent == -(1 as libc::c_int) {
        settings.paren_indent = settings.continuation_indent;
    }
    if settings.case_brace_indent == -(1 as libc::c_int) {
        settings.case_brace_indent = settings.ind_size;
    }
    fill_buffer();
    return indent_main_loop(&mut break_line);
}
unsafe extern "C" fn handle_profile(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut profile_pathname: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"-npro\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"--ignore-profile\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(i as isize),
                b"+ignore-profile\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            break;
        }
        i += 1;
        i;
    }
    if i >= argc {
        profile_pathname = set_profile();
    }
    return profile_pathname;
}
static mut out_name: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut input_files: libc::c_int = 0 as libc::c_int;
static mut in_file_names: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut max_input_files: libc::c_int = 128 as libc::c_int;
unsafe extern "C" fn process_args(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut using_stdin: *mut BOOLEAN,
) -> exit_values_ty {
    let mut i: libc::c_int = 0;
    let mut exit_status: exit_values_ty = total_success;
    i = 1 as libc::c_int;
    while i < argc {
        if **argv.offset(i as isize) as libc::c_int != '-' as i32
            && **argv.offset(i as isize) as libc::c_int != '+' as i32
        {
            if settings.expect_output_file == 1 as libc::c_int {
                if !out_name.is_null() {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"indent: only one output file (2nd was %s)\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        *argv.offset(i as isize),
                    );
                    exit_status = invocation_error;
                    break;
                } else if input_files > 1 as libc::c_int {
                    fprintf(
                        stderr,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"indent: only one input file when output file is specified\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                    );
                    exit_status = invocation_error;
                    break;
                } else {
                    out_name = *argv.offset(i as isize);
                    settings.expect_output_file = 0 as libc::c_int;
                }
            } else if *using_stdin != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"indent: can't have filenames when specifying standard input\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                exit_status = invocation_error;
                break;
            } else {
                input_files += 1;
                input_files;
                if input_files > 1 as libc::c_int {
                    if !out_name.is_null() {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"indent: only one input file when output file is specified\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        exit_status = invocation_error;
                        break;
                    } else if settings.use_stdout != 0 as libc::c_int {
                        fprintf(
                            stderr,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"indent: only one input file when stdout is used\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                        );
                        exit_status = invocation_error;
                        break;
                    } else if input_files > max_input_files {
                        max_input_files = 2 as libc::c_int * max_input_files;
                        in_file_names = xrealloc(
                            in_file_names as *mut libc::c_void,
                            (max_input_files as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                                ) as libc::c_uint,
                        ) as *mut *mut libc::c_char;
                    }
                }
                let ref mut fresh16 = *in_file_names
                    .offset((input_files - 1 as libc::c_int) as isize);
                *fresh16 = *argv.offset(i as isize);
            }
        } else if strcmp(
            *argv.offset(i as isize),
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if input_files > 0 as libc::c_int {
                fprintf(
                    stderr,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"indent: can't have filenames when specifying standard input\n\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
                exit_status = invocation_error;
                break;
            } else {
                *using_stdin = 1 as libc::c_int as BOOLEAN;
            }
        } else {
            i
                += set_option(
                    *argv.offset(i as isize),
                    if i < argc {
                        *argv.offset((i + 1 as libc::c_int) as isize)
                    } else {
                        0 as *mut libc::c_char
                    },
                    1 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"command line\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                );
        }
        i += 1;
        i;
    }
    return exit_status;
}
unsafe extern "C" fn indent_multiple_files() -> exit_values_ty {
    let mut exit_status: exit_values_ty = total_success;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while input_files != 0 {
        let mut status: exit_values_ty = total_success;
        let mut file_stats: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        in_name = *in_file_names.offset(i as isize);
        out_name = *in_file_names.offset(i as isize);
        current_input = read_file(*in_file_names.offset(i as isize), &mut file_stats);
        open_output(out_name, b"r+\0" as *const u8 as *const libc::c_char);
        make_backup(current_input, &mut file_stats);
        reopen_output_trunc(out_name);
        reset_parser();
        status = indent(current_input);
        if status as libc::c_uint > exit_status as libc::c_uint {
            exit_status = status;
        }
        if settings.preserve_mtime != 0 {
            close_output(&mut file_stats, out_name);
        } else {
            close_output(0 as *mut stat, out_name);
        }
        i += 1;
        i;
        input_files -= 1;
        input_files;
    }
    return exit_status;
}
unsafe extern "C" fn indent_single_file(mut using_stdin: BOOLEAN) -> exit_values_ty {
    let mut is_stdin: libc::c_int = 0 as libc::c_int;
    let mut exit_status: exit_values_ty = total_success;
    let mut file_stats: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if input_files == 0 as libc::c_int || using_stdin as libc::c_int != 0 {
        input_files = 1 as libc::c_int;
        let ref mut fresh17 = *in_file_names.offset(0 as libc::c_int as isize);
        *fresh17 = b"Standard input\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        in_name = *in_file_names.offset(0 as libc::c_int as isize);
        current_input = read_stdin();
        is_stdin = 1 as libc::c_int;
    } else {
        in_name = *in_file_names.offset(0 as libc::c_int as isize);
        current_input = read_file(
            *in_file_names.offset(0 as libc::c_int as isize),
            &mut file_stats,
        );
        if out_name.is_null() && settings.use_stdout == 0 {
            out_name = *in_file_names.offset(0 as libc::c_int as isize);
            make_backup(current_input, &mut file_stats);
        }
    }
    if settings.use_stdout != 0 || out_name.is_null() {
        open_output(0 as *const libc::c_char, 0 as *const libc::c_char);
    } else {
        open_output(out_name, b"w\0" as *const u8 as *const libc::c_char);
    }
    reset_parser();
    exit_status = indent(current_input);
    if input_files > 0 as libc::c_int && using_stdin == 0 && settings.preserve_mtime != 0
    {
        close_output(&mut file_stats, out_name);
    } else {
        close_output(0 as *mut stat, out_name);
    }
    if !current_input.is_null() {
        if is_stdin == 0 && !((*current_input).name).is_null() {
            xfree((*current_input).name as *mut libc::c_void);
        }
        xfree((*current_input).data as *mut libc::c_void);
    }
    return exit_status;
}
unsafe extern "C" fn indent_all(mut using_stdin: BOOLEAN) -> exit_values_ty {
    let mut exit_status: exit_values_ty = total_success;
    if input_files > 1 as libc::c_int {
        exit_status = indent_multiple_files();
    } else {
        exit_status = indent_single_file(using_stdin);
    }
    return exit_status;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut profile_pathname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut using_stdin: BOOLEAN = 0 as libc::c_int as BOOLEAN;
    let mut exit_status: exit_values_ty = total_success;
    bindtextdomain(
        b"indent\0" as *const u8 as *const libc::c_char,
        b"/usr/local/share/locale\0" as *const u8 as *const libc::c_char,
    );
    textdomain(b"indent\0" as *const u8 as *const libc::c_char);
    memset(
        &mut settings as *mut user_options_ty as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<user_options_ty>() as libc::c_ulong,
    );
    tmp = xmalloc(7 as libc::c_int as libc::c_uint) as *mut libc::c_char;
    memcpy(
        tmp as *mut libc::c_void,
        b"size_t\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        7 as libc::c_int as libc::c_ulong,
    );
    addkey(tmp, rw_decl);
    tmp = xmalloc(8 as libc::c_int as libc::c_uint) as *mut libc::c_char;
    memcpy(
        tmp as *mut libc::c_void,
        b"wchar_t\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    addkey(tmp, rw_decl);
    tmp = xmalloc(10 as libc::c_int as libc::c_uint) as *mut libc::c_char;
    memcpy(
        tmp as *mut libc::c_void,
        b"ptrdiff_t\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        10 as libc::c_int as libc::c_ulong,
    );
    addkey(tmp, rw_decl);
    init_parser();
    initialize_backups();
    exit_status = total_success;
    input_files = 0 as libc::c_int;
    in_file_names = xmalloc(
        (max_input_files as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_uint,
    ) as *mut *mut libc::c_char;
    set_defaults();
    profile_pathname = handle_profile(argc, argv);
    exit_status = process_args(argc, argv, &mut using_stdin);
    if exit_status as libc::c_uint == total_success as libc::c_int as libc::c_uint {
        if settings.verbose != 0 && !profile_pathname.is_null() {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Read profile %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                profile_pathname,
            );
        }
        set_defaults_after();
        exit_status = indent_all(using_stdin);
    }
    if !profile_pathname.is_null() {
        xfree(profile_pathname as *mut libc::c_void);
    }
    xfree(in_file_names as *mut libc::c_void);
    uninit_parser();
    cleanup_user_specials();
    return exit_status as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

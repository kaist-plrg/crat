use ::libc;
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut in_prog: *mut libc::c_char;
    static mut in_prog_size: libc::c_ulong;
    static mut s_code: *mut libc::c_char;
    static mut e_code: *mut libc::c_char;
    static mut e_com: *mut libc::c_char;
    static mut buf_ptr: *mut libc::c_char;
    static mut buf_end: *mut libc::c_char;
    static mut squest: libc::c_int;
    static mut parser_state_tos: *mut parser_state_ty;
    static mut settings: user_options_ty;
    static mut line_no: libc::c_int;
    static mut had_eof: BOOLEAN;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn fill_buffer();
    fn skip_buffered_space();
    fn xmalloc(size: libc::c_uint) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: libc::c_uint) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn message(
        kind: *mut libc::c_char,
        string: *mut libc::c_char,
        a0: *mut libc::c_char,
        a1: *mut libc::c_char,
    );
}
pub type size_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct templ_ty {
    pub rwd: *mut libc::c_char,
    pub rwcode: rwcodes_ty,
}
pub type user_options_ty = user_options_st;
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
pub type parser_state_ty = parser_state;
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
pub type bb_code_ty = bb_code;
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
pub type BOOLEAN = libc::c_uchar;
pub static mut token: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut token_end: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut user_specials: *mut templ_ty = 0 as *const templ_ty as *mut templ_ty;
static mut user_specials_max: libc::c_uint = 0 as libc::c_int as libc::c_uint;
static mut user_specials_idx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
pub static mut chartype: [libc::c_char; 256] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    1 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    3 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn hash(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> libc::c_uint {
    static mut asso_values: [libc::c_uchar; 256] = [
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        39 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        27 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
    ];
    return len
        .wrapping_add(
            asso_values[*str
                .offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_uchar as usize] as libc::c_ulong,
        )
        .wrapping_add(
            asso_values[*str.offset(0 as libc::c_int as isize) as libc::c_uchar as usize]
                as libc::c_ulong,
        ) as libc::c_uint;
}
pub unsafe extern "C" fn is_reserved(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *mut templ_ty {
    static mut lengthtable: [libc::c_uchar; 45] = [
        3 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
    ];
    static mut wordlist: [templ_ty; 45] = [
        {
            let mut init = templ_ty {
                rwd: b"int\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"if\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_paren,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"short\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"struct\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_struct_like,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"float\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"sizeof\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_sizeof,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"typedef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"for\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_paren,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"const\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"static\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"restrict\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"alignas\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_sizeof,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"char\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"alignof\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_sizeof,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"return\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_return,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"register\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"case\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_case,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"noreturn\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"extern\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"else\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_else,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"signed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"default\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_case,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"volatile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"_Alignas\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_sizeof,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"va_dcl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"_Alignof\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_sizeof,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"void\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"imaginary\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"complex\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"double\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"_Noreturn\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"union\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_struct_like,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"long\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"_Bool\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"global\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"do\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_nparen,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"while\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_paren,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"enum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_enum,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"goto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_break,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"bool\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"break\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_break,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"_Complex\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"switch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_switch,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"unsigned\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"_Imaginary\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
    ];
    static mut lookup: [libc::c_schar; 50] = [
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        5 as libc::c_int as libc::c_schar,
        6 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
        8 as libc::c_int as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        10 as libc::c_int as libc::c_schar,
        11 as libc::c_int as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        13 as libc::c_int as libc::c_schar,
        14 as libc::c_int as libc::c_schar,
        15 as libc::c_int as libc::c_schar,
        16 as libc::c_int as libc::c_schar,
        17 as libc::c_int as libc::c_schar,
        18 as libc::c_int as libc::c_schar,
        19 as libc::c_int as libc::c_schar,
        20 as libc::c_int as libc::c_schar,
        21 as libc::c_int as libc::c_schar,
        22 as libc::c_int as libc::c_schar,
        23 as libc::c_int as libc::c_schar,
        24 as libc::c_int as libc::c_schar,
        25 as libc::c_int as libc::c_schar,
        26 as libc::c_int as libc::c_schar,
        27 as libc::c_int as libc::c_schar,
        28 as libc::c_int as libc::c_schar,
        29 as libc::c_int as libc::c_schar,
        30 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
        32 as libc::c_int as libc::c_schar,
        33 as libc::c_int as libc::c_schar,
        34 as libc::c_int as libc::c_schar,
        35 as libc::c_int as libc::c_schar,
        36 as libc::c_int as libc::c_schar,
        37 as libc::c_int as libc::c_schar,
        38 as libc::c_int as libc::c_schar,
        39 as libc::c_int as libc::c_schar,
        40 as libc::c_int as libc::c_schar,
        41 as libc::c_int as libc::c_schar,
        42 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        43 as libc::c_int as libc::c_schar,
        44 as libc::c_int as libc::c_schar,
    ];
    if len <= 10 as libc::c_int as libc::c_ulong
        && len >= 2 as libc::c_int as libc::c_ulong
    {
        let mut key: libc::c_uint = hash(str, len);
        if key <= 49 as libc::c_int as libc::c_uint {
            let mut idx: libc::c_int = lookup[key as usize] as libc::c_int;
            if idx >= 0 as libc::c_int {
                if len == lengthtable[idx as usize] as libc::c_ulong {
                    let mut s: *const libc::c_char = wordlist[idx as usize].rwd;
                    if *str as libc::c_int == *s as libc::c_int
                        && memcmp(
                            str.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) == 0
                    {
                        return &mut *wordlist.as_mut_ptr().offset(idx as isize)
                            as *mut templ_ty;
                    }
                }
            }
        }
    }
    return 0 as *mut templ_ty;
}
unsafe extern "C" fn hash_cc(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> libc::c_uint {
    static mut asso_values: [libc::c_uchar; 256] = [
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        30 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        37 as libc::c_int as libc::c_uchar,
        22 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar,
        27 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
    ];
    let mut hval: libc::c_uint = len as libc::c_uint;
    match hval {
        2 | 1 => {}
        _ => {
            hval = hval
                .wrapping_add(
                    asso_values[*str.offset(2 as libc::c_int as isize) as libc::c_uchar
                        as usize] as libc::c_uint,
                );
        }
    }
    hval = hval
        .wrapping_add(
            asso_values[*str.offset(0 as libc::c_int as isize) as libc::c_uchar as usize]
                as libc::c_uint,
        );
    return hval;
}
pub unsafe extern "C" fn is_reserved_cc(
    mut str: *const libc::c_char,
    mut len: size_t,
) -> *mut templ_ty {
    static mut lengthtable: [libc::c_uchar; 48] = [
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
    ];
    static mut wordlist: [templ_ty; 48] = [
        {
            let mut init = templ_ty {
                rwd: b"if\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_paren,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"int\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"throw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_return,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"return\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_return,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"goto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_break,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"switch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_switch,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"struct\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_struct_like,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"inline\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"sigof\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sizeof,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"signed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"register\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"catch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_paren,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"signature\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_struct_like,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"case\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_case,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"static\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"short\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"extern\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"else\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_else,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"global\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"union\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_struct_like,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"char\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"class\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_struct_like,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"do\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_nparen,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"classof\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_sizeof,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"unsigned\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"for\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_paren,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"long\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"friend\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"while\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_sp_paren,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"sizeof\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_sizeof,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"operator\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_operator,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"delete\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_return,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"enum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_enum,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"const\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"void\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"typeof\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_sizeof,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"typedef\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"float\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"virtual\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"bool\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"template\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"headof\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_sizeof,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"volatile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"break\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_break,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"double\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"va_dcl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_decl,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"new\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rwcode: rw_return,
            };
            init
        },
        {
            let mut init = templ_ty {
                rwd: b"default\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rwcode: rw_case,
            };
            init
        },
    ];
    static mut lookup: [libc::c_schar; 58] = [
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        0 as libc::c_int as libc::c_schar,
        1 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        2 as libc::c_int as libc::c_schar,
        3 as libc::c_int as libc::c_schar,
        4 as libc::c_int as libc::c_schar,
        5 as libc::c_int as libc::c_schar,
        6 as libc::c_int as libc::c_schar,
        7 as libc::c_int as libc::c_schar,
        8 as libc::c_int as libc::c_schar,
        9 as libc::c_int as libc::c_schar,
        10 as libc::c_int as libc::c_schar,
        11 as libc::c_int as libc::c_schar,
        12 as libc::c_int as libc::c_schar,
        13 as libc::c_int as libc::c_schar,
        14 as libc::c_int as libc::c_schar,
        15 as libc::c_int as libc::c_schar,
        16 as libc::c_int as libc::c_schar,
        17 as libc::c_int as libc::c_schar,
        18 as libc::c_int as libc::c_schar,
        19 as libc::c_int as libc::c_schar,
        20 as libc::c_int as libc::c_schar,
        21 as libc::c_int as libc::c_schar,
        22 as libc::c_int as libc::c_schar,
        23 as libc::c_int as libc::c_schar,
        24 as libc::c_int as libc::c_schar,
        25 as libc::c_int as libc::c_schar,
        26 as libc::c_int as libc::c_schar,
        27 as libc::c_int as libc::c_schar,
        28 as libc::c_int as libc::c_schar,
        29 as libc::c_int as libc::c_schar,
        30 as libc::c_int as libc::c_schar,
        31 as libc::c_int as libc::c_schar,
        32 as libc::c_int as libc::c_schar,
        33 as libc::c_int as libc::c_schar,
        34 as libc::c_int as libc::c_schar,
        35 as libc::c_int as libc::c_schar,
        36 as libc::c_int as libc::c_schar,
        37 as libc::c_int as libc::c_schar,
        38 as libc::c_int as libc::c_schar,
        39 as libc::c_int as libc::c_schar,
        40 as libc::c_int as libc::c_schar,
        41 as libc::c_int as libc::c_schar,
        42 as libc::c_int as libc::c_schar,
        43 as libc::c_int as libc::c_schar,
        44 as libc::c_int as libc::c_schar,
        45 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        46 as libc::c_int as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        -(1 as libc::c_int) as libc::c_schar,
        47 as libc::c_int as libc::c_schar,
    ];
    if len <= 9 as libc::c_int as libc::c_ulong
        && len >= 2 as libc::c_int as libc::c_ulong
    {
        let mut key: libc::c_uint = hash_cc(str, len);
        if key <= 57 as libc::c_int as libc::c_uint {
            let mut idx: libc::c_int = lookup[key as usize] as libc::c_int;
            if idx >= 0 as libc::c_int {
                if len == lengthtable[idx as usize] as libc::c_ulong {
                    let mut s: *const libc::c_char = wordlist[idx as usize].rwd;
                    if *str as libc::c_int == *s as libc::c_int
                        && memcmp(
                            str.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            s.offset(1 as libc::c_int as isize) as *const libc::c_void,
                            len.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) == 0
                    {
                        return &mut *wordlist.as_mut_ptr().offset(idx as isize)
                            as *mut templ_ty;
                    }
                }
            }
        }
    }
    return 0 as *mut templ_ty;
}
pub unsafe extern "C" fn lexi() -> codes_ty {
    let mut current_block: u64;
    let mut unary_delim: libc::c_int = 0 as libc::c_int;
    static mut last_code: codes_ty = code_eof;
    static mut l_struct: libc::c_int = 0 as libc::c_int;
    static mut l_enum: libc::c_int = 0 as libc::c_int;
    let mut code: codes_ty = code_eof;
    let mut qchar: libc::c_char = 0;
    let mut tmpchar: [libc::c_char; 2] = [0; 2];
    (*parser_state_tos).col_1 = (*parser_state_tos).last_nl;
    (*parser_state_tos).last_saw_nl = (*parser_state_tos).last_nl;
    (*parser_state_tos).last_nl = 0 as libc::c_int;
    if buf_ptr >= buf_end {
        fill_buffer();
    }
    if *buf_ptr as libc::c_int == ' ' as i32 || *buf_ptr as libc::c_int == '\t' as i32 {
        (*parser_state_tos).col_1 = 0 as libc::c_int;
        skip_buffered_space();
    }
    token = buf_ptr;
    if !(*buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int == 'L' as i32
        && (*buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int == '"' as i32
            || *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int == '\'' as i32))
        && !(settings.gettext_strings != 0
            && *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
            && *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int == '(' as i32
            && *buf_ptr.offset(2 as libc::c_int as isize) as libc::c_int == '"' as i32)
        && !(settings.gettext_strings != 0
            && *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int == 'N' as i32
            && *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
            && *buf_ptr.offset(2 as libc::c_int as isize) as libc::c_int == '(' as i32
            && *buf_ptr.offset(3 as libc::c_int as isize) as libc::c_int == '"' as i32)
        && chartype[(0xff as libc::c_int & *buf_ptr as libc::c_int) as usize]
            as libc::c_int == 1 as libc::c_int
        || *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
            && *(*__ctype_b_loc())
                .offset(
                    *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int as isize,
                ) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let mut p: *mut templ_ty = 0 as *mut templ_ty;
        if (*parser_state_tos).last_rw as libc::c_uint
            == rw_return as libc::c_int as libc::c_uint
        {
            (*parser_state_tos).last_rw = rw_none;
        }
        if *(*__ctype_b_loc()).offset(*buf_ptr as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            || *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                && *(*__ctype_b_loc())
                    .offset(
                        *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                            as isize,
                    ) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            let mut seendot: libc::c_int = 0 as libc::c_int;
            let mut seenexp: libc::c_int = 0 as libc::c_int;
            let mut ishexa: libc::c_int = 0 as libc::c_int;
            let mut isbinary: libc::c_int = 0 as libc::c_int;
            if *buf_ptr as libc::c_int == '0' as i32 {
                if *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'x' as i32
                    || *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'X' as i32
                {
                    ishexa = 1 as libc::c_int;
                    buf_ptr = buf_ptr.offset(1 as libc::c_int as isize);
                } else if *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'b' as i32
                    || *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'B' as i32
                {
                    isbinary = 1 as libc::c_int;
                    buf_ptr = buf_ptr.offset(1 as libc::c_int as isize);
                }
            }
            loop {
                if *buf_ptr as libc::c_int == '.' as i32 {
                    if seendot != 0 {
                        break;
                    }
                    seendot += 1;
                    seendot;
                }
                buf_ptr = buf_ptr.offset(1);
                buf_ptr;
                if !(!(ishexa != 0 && seenexp == 0
                    && *(*__ctype_b_loc()).offset(*buf_ptr as libc::c_int as isize)
                        as libc::c_int
                        & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || isbinary != 0 && seenexp == 0
                        && (*buf_ptr as libc::c_int == '0' as i32
                            || *buf_ptr as libc::c_int == '1' as i32)
                    || (!(ishexa != 0 || isbinary != 0) || seenexp != 0)
                        && *(*__ctype_b_loc()).offset(*buf_ptr as libc::c_int as isize)
                            as libc::c_int
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0) && *buf_ptr as libc::c_int != '.' as i32)
                {
                    continue;
                }
                if (if ishexa != 0 {
                    (*buf_ptr as libc::c_int != 'P' as i32
                        && *buf_ptr as libc::c_int != 'p' as i32) as libc::c_int
                } else {
                    (*buf_ptr as libc::c_int != 'E' as i32
                        && *buf_ptr as libc::c_int != 'e' as i32) as libc::c_int
                }) != 0 || seenexp != 0
                {
                    break;
                }
                seenexp += 1;
                seenexp;
                seendot += 1;
                seendot;
                buf_ptr = buf_ptr.offset(1);
                buf_ptr;
                if *buf_ptr as libc::c_int == '+' as i32
                    || *buf_ptr as libc::c_int == '-' as i32
                {
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                }
            }
            if *buf_ptr as libc::c_int == 'F' as i32
                || *buf_ptr as libc::c_int == 'f' as i32
                || *buf_ptr as libc::c_int == 'i' as i32
                || *buf_ptr as libc::c_int == 'j' as i32
            {
                buf_ptr = buf_ptr.offset(1);
                buf_ptr;
            } else if *buf_ptr as libc::c_int == 'D' as i32
                || *buf_ptr as libc::c_int == 'd' as i32
            {
                if *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                    == 'F' as i32
                    || *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'f' as i32
                    || *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'D' as i32
                    || *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'd' as i32
                    || *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'L' as i32
                    || *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        == 'l' as i32
                {
                    buf_ptr = buf_ptr.offset(2 as libc::c_int as isize);
                }
            } else {
                while *buf_ptr as libc::c_int == 'U' as i32
                    || *buf_ptr as libc::c_int == 'u' as i32
                    || *buf_ptr as libc::c_int == 'L' as i32
                    || *buf_ptr as libc::c_int == 'l' as i32
                {
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                }
            }
        } else {
            while chartype[(0xff as libc::c_int & *buf_ptr as libc::c_int) as usize]
                as libc::c_int == 1 as libc::c_int
            {
                buf_ptr = buf_ptr.offset(1);
                buf_ptr;
                if buf_ptr >= buf_end {
                    fill_buffer();
                }
            }
        }
        token_end = buf_ptr;
        if token_end.offset_from(token) as libc::c_long
            == 13 as libc::c_int as libc::c_long
            && strncmp(
                token,
                b"__attribute__\0" as *const u8 as *const libc::c_char,
                13 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            last_code = decl;
            (*parser_state_tos).last_u_d = 1 as libc::c_int;
            return attribute;
        }
        skip_buffered_space();
        if token_end.offset_from(token) as libc::c_long
            == 8 as libc::c_int as libc::c_long
            && strncmp(
                token,
                b"operator\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            while chartype[(0xff as libc::c_int & *buf_ptr as libc::c_int) as usize]
                as libc::c_int == 3 as libc::c_int
            {
                buf_ptr = buf_ptr.offset(1);
                buf_ptr;
                if buf_ptr >= buf_end {
                    fill_buffer();
                }
            }
            token_end = buf_ptr;
            skip_buffered_space();
        }
        (*parser_state_tos).its_a_keyword = 0 as libc::c_int;
        (*parser_state_tos).sizeof_keyword = 0 as libc::c_int;
        if l_struct != 0 {
            l_struct = 0 as libc::c_int;
            last_code = ident;
            (*parser_state_tos).last_u_d = 1 as libc::c_int;
            if (*parser_state_tos).last_token as libc::c_uint
                == cpp_operator as libc::c_int as libc::c_uint
            {
                return overloaded;
            }
            return decl;
        }
        (*parser_state_tos).last_u_d = 0 as libc::c_int;
        last_code = ident;
        if settings.c_plus_plus != 0 {
            p = is_reserved_cc(
                token,
                token_end.offset_from(token) as libc::c_long as size_t,
            );
        } else {
            p = is_reserved(
                token,
                token_end.offset_from(token) as libc::c_long as size_t,
            );
        }
        if p.is_null() && !user_specials.is_null() {
            p = &mut *user_specials.offset(0 as libc::c_int as isize) as *mut templ_ty;
            's_397: loop {
                if !(p
                    < (&mut *user_specials.offset(0 as libc::c_int as isize)
                        as *mut templ_ty)
                        .offset(user_specials_idx as isize))
                {
                    current_block = 10859911333150192671;
                    break;
                }
                let mut q: *mut libc::c_char = token;
                let mut r: *mut libc::c_char = (*p).rwd;
                loop {
                    if q >= token_end && *r == 0 {
                        current_block = 11344492717589242087;
                        break 's_397;
                    }
                    if q >= token_end || *r == 0 {
                        break;
                    }
                    let fresh0 = q;
                    q = q.offset(1);
                    let fresh1 = r;
                    r = r.offset(1);
                    if *fresh0 as libc::c_int != *fresh1 as libc::c_int {
                        break;
                    }
                }
                p = p.offset(1);
                p;
            }
            match current_block {
                11344492717589242087 => {}
                _ => {
                    p = 0 as *mut templ_ty;
                }
            }
        }
        if !p.is_null() {
            let mut value: codes_ty = code_eof;
            value = ident;
            (*parser_state_tos).its_a_keyword = 1 as libc::c_int;
            (*parser_state_tos).last_u_d = 1 as libc::c_int;
            (*parser_state_tos).last_rw = (*p).rwcode;
            (*parser_state_tos).last_rw_depth = (*parser_state_tos).paren_depth;
            let mut current_block_114: u64;
            match (*p).rwcode as libc::c_uint {
                1 => {
                    value = cpp_operator;
                    (*parser_state_tos).in_parameter_declaration = 1 as libc::c_int;
                    current_block_114 = 3951782611973701285;
                }
                3 => {
                    value = swstmt;
                    current_block_114 = 3951782611973701285;
                }
                4 => {
                    value = casestmt;
                    current_block_114 = 3951782611973701285;
                }
                6 => {
                    l_enum = 1 as libc::c_int;
                    current_block_114 = 17633496012193172303;
                }
                5 => {
                    current_block_114 = 17633496012193172303;
                }
                7 => {
                    current_block_114 = 4973564045581569773;
                }
                8 => {
                    value = sp_paren;
                    if *token as libc::c_int == 'i' as i32
                        && (*parser_state_tos).last_token as libc::c_uint
                            == sp_else as libc::c_int as libc::c_uint
                    {
                        (*parser_state_tos).i_l_follow -= settings.ind_size;
                    }
                    current_block_114 = 3951782611973701285;
                }
                9 => {
                    value = sp_nparen;
                    current_block_114 = 3951782611973701285;
                }
                10 => {
                    value = sp_else;
                    current_block_114 = 3951782611973701285;
                }
                11 => {
                    (*parser_state_tos).sizeof_keyword = 1 as libc::c_int;
                    value = ident;
                    current_block_114 = 3951782611973701285;
                }
                12 | 2 | _ => {
                    value = ident;
                    current_block_114 = 3951782611973701285;
                }
            }
            match current_block_114 {
                17633496012193172303 => {
                    if (*parser_state_tos).p_l_follow != 0
                        && (*parser_state_tos).noncast_mask
                            & (1 as libc::c_int) << (*parser_state_tos).p_l_follow == 0
                    {
                        (*parser_state_tos).cast_mask
                            |= (1 as libc::c_int) << (*parser_state_tos).p_l_follow;
                        current_block_114 = 3951782611973701285;
                    } else {
                        l_struct = 1 as libc::c_int;
                        current_block_114 = 4973564045581569773;
                    }
                }
                _ => {}
            }
            match current_block_114 {
                4973564045581569773 => {
                    if (*parser_state_tos).p_l_follow != 0
                        && (*parser_state_tos).noncast_mask
                            & (1 as libc::c_int) << (*parser_state_tos).p_l_follow == 0
                    {
                        (*parser_state_tos).cast_mask
                            |= (1 as libc::c_int) << (*parser_state_tos).p_l_follow;
                    } else {
                        last_code = decl;
                        value = decl;
                    }
                }
                _ => {}
            }
            if (*parser_state_tos).last_token as libc::c_uint
                == cpp_operator as libc::c_int as libc::c_uint
            {
                return overloaded;
            }
            return value;
        } else {
            if *buf_ptr as libc::c_int == '(' as i32
                && (*parser_state_tos).tos <= 1 as libc::c_int
                && (*parser_state_tos).ind_level == 0 as libc::c_int
                && (*parser_state_tos).paren_depth == 0 as libc::c_int
            {
                let mut tp: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut paren_count: libc::c_int = 1 as libc::c_int;
                if (*parser_state_tos).last_token as libc::c_uint
                    == ident as libc::c_int as libc::c_uint
                    && (*parser_state_tos).last_saw_nl != 0
                {
                    (*parser_state_tos).in_decl = 1 as libc::c_int as BOOLEAN;
                }
                tp = buf_ptr.offset(1 as libc::c_int as isize);
                loop {
                    if !(paren_count > 0 as libc::c_int
                        && tp < in_prog.offset(in_prog_size as isize))
                    {
                        current_block = 12693738997172594219;
                        break;
                    }
                    if buf_ptr >= buf_end {
                        fill_buffer();
                    }
                    if had_eof != 0 {
                        current_block = 8587840431104022724;
                        break;
                    }
                    if *tp as libc::c_int == '(' as i32 {
                        paren_count += 1;
                        paren_count;
                    }
                    if *tp as libc::c_int == ')' as i32 {
                        paren_count -= 1;
                        paren_count;
                    }
                    if *tp as libc::c_int == ';' as i32 {
                        current_block = 8587840431104022724;
                        break;
                    }
                    tp = tp.offset(1);
                    tp;
                }
                match current_block {
                    8587840431104022724 => {}
                    _ => {
                        if paren_count == 0 as libc::c_int {
                            (*parser_state_tos).procname = token;
                            (*parser_state_tos).procname_end = token_end;
                            while *(*__ctype_b_loc()).offset(*tp as libc::c_int as isize)
                                as libc::c_int
                                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                            {
                                tp = tp.offset(1);
                                tp;
                            }
                            if !(*tp as libc::c_int == '_' as i32
                                && in_prog.offset(in_prog_size as isize).offset_from(tp)
                                    as libc::c_long >= 13 as libc::c_int as libc::c_long
                                && strncmp(
                                    tp,
                                    b"__attribute__\0" as *const u8 as *const libc::c_char,
                                    13 as libc::c_int as libc::c_ulong,
                                ) == 0)
                            {
                                if *tp as libc::c_int != ';' as i32
                                    && *tp as libc::c_int != ',' as i32
                                    && *tp as libc::c_int != '(' as i32
                                    && *tp as libc::c_int != '=' as i32
                                {
                                    (*parser_state_tos)
                                        .in_parameter_declaration = 1 as libc::c_int;
                                }
                            }
                        }
                    }
                }
            } else if *buf_ptr as libc::c_int == ':' as i32
                && *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                    == ':' as i32 && (*parser_state_tos).tos <= 1 as libc::c_int
                && (*parser_state_tos).ind_level == 0 as libc::c_int
                && (*parser_state_tos).paren_depth == 0 as libc::c_int
            {
                (*parser_state_tos).classname = token;
                (*parser_state_tos).classname_end = token_end;
            } else if (*buf_ptr as libc::c_int == '*' as i32
                && *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                    != '=' as i32
                || *(*__ctype_b_loc()).offset(*buf_ptr as libc::c_int as isize)
                    as libc::c_int
                    & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int != 0
                || *buf_ptr as libc::c_int == '_' as i32)
                && (*parser_state_tos).p_l_follow == 0
                && (*parser_state_tos).block_init == 0
                && ((*parser_state_tos).last_token as libc::c_uint
                    == rparen as libc::c_int as libc::c_uint
                    || (*parser_state_tos).last_token as libc::c_uint
                        == semicolon as libc::c_int as libc::c_uint
                    || (*parser_state_tos).last_token as libc::c_uint
                        == rbrace as libc::c_int as libc::c_uint
                    || (*parser_state_tos).last_token as libc::c_uint
                        == decl as libc::c_int as libc::c_uint
                    || (*parser_state_tos).last_token as libc::c_uint
                        == lbrace as libc::c_int as libc::c_uint
                    || (*parser_state_tos).last_token as libc::c_uint
                        == start_token as libc::c_int as libc::c_uint)
            {
                (*parser_state_tos).its_a_keyword = 1 as libc::c_int;
                (*parser_state_tos).last_u_d = 1 as libc::c_int;
                last_code = decl;
                if (*parser_state_tos).last_token as libc::c_uint
                    == cpp_operator as libc::c_int as libc::c_uint
                {
                    return overloaded;
                }
                return decl;
            }
            last_code = ident;
            if (*parser_state_tos).last_token as libc::c_uint
                == cpp_operator as libc::c_int as libc::c_uint
            {
                return overloaded;
            }
            return ident;
        }
    } else {
        token_end = buf_ptr.offset(1 as libc::c_int as isize);
        buf_ptr = buf_ptr.offset(1);
        if buf_ptr >= buf_end {
            fill_buffer();
        }
        if *token as libc::c_int == '\\' as i32
            && *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
        {
            token = buf_ptr;
            buf_ptr = buf_ptr.offset(1);
            if buf_ptr >= buf_end {
                fill_buffer();
            }
        }
        match *token as libc::c_int {
            0 => {
                code = code_eof;
                current_block = 8501358306579972139;
            }
            10 => {
                (*parser_state_tos).matching_brace_on_same_line = -(1 as libc::c_int);
                unary_delim = (*parser_state_tos).last_u_d;
                (*parser_state_tos).last_nl = 1 as libc::c_int;
                code = newline;
                current_block = 8501358306579972139;
            }
            95 => {
                if settings.gettext_strings == 0
                    || *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                        != '(' as i32
                    || *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        != '"' as i32
                {
                    token_end = buf_ptr;
                    code = ident;
                    current_block = 8501358306579972139;
                } else {
                    qchar = *buf_ptr.offset(1 as libc::c_int as isize);
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    current_block = 18299784206167311994;
                }
            }
            78 => {
                if settings.gettext_strings == 0
                    || *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                        != '_' as i32
                    || *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        != '(' as i32
                    || *buf_ptr.offset(2 as libc::c_int as isize) as libc::c_int
                        != '"' as i32
                {
                    token_end = buf_ptr;
                    code = ident;
                    current_block = 8501358306579972139;
                } else {
                    qchar = *buf_ptr.offset(2 as libc::c_int as isize);
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    current_block = 18299784206167311994;
                }
            }
            76 => {
                if *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                    != '"' as i32
                    && *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                        != '\'' as i32
                {
                    token_end = buf_ptr;
                    code = ident;
                    current_block = 8501358306579972139;
                } else {
                    qchar = *buf_ptr.offset(0 as libc::c_int as isize);
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    current_block = 18299784206167311994;
                }
            }
            39 | 34 => {
                qchar = *token;
                current_block = 18299784206167311994;
            }
            40 => {
                l_enum = 0 as libc::c_int;
                unary_delim = 1 as libc::c_int;
                code = lparen;
                current_block = 8501358306579972139;
            }
            91 => {
                if (*parser_state_tos).in_or_st != 0 {
                    (*parser_state_tos).in_or_st += 1;
                    (*parser_state_tos).in_or_st;
                }
                unary_delim = 1 as libc::c_int;
                code = lparen;
                current_block = 8501358306579972139;
            }
            41 => {
                l_enum = 0 as libc::c_int;
                code = rparen;
                current_block = 8501358306579972139;
            }
            93 => {
                if (*parser_state_tos).in_or_st > 1 as libc::c_int {
                    (*parser_state_tos).in_or_st -= 1;
                    (*parser_state_tos).in_or_st;
                }
                code = rparen;
                current_block = 8501358306579972139;
            }
            35 => {
                unary_delim = (*parser_state_tos).last_u_d;
                code = preesc;
                while *buf_ptr as libc::c_int == ' ' as i32 && buf_ptr < buf_end {
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                }
                if settings.leave_preproc_space != 0 {
                    token_end = buf_ptr;
                }
                current_block = 8501358306579972139;
            }
            63 => {
                unary_delim = 1 as libc::c_int;
                code = question;
                current_block = 8501358306579972139;
            }
            58 => {
                if *buf_ptr as libc::c_int == ':' as i32 {
                    code = doublecolon;
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    token_end = buf_ptr;
                } else {
                    code = colon;
                    unary_delim = 1 as libc::c_int;
                    if squest != 0 && *e_com as libc::c_int != ' ' as i32 {
                        if e_code == s_code {
                            (*parser_state_tos).want_blank = 0 as libc::c_int;
                        } else {
                            (*parser_state_tos).want_blank = 1 as libc::c_int;
                        }
                    }
                }
                current_block = 8501358306579972139;
            }
            59 => {
                l_enum = 0 as libc::c_int;
                unary_delim = 1 as libc::c_int;
                code = semicolon;
                current_block = 8501358306579972139;
            }
            123 => {
                if (*parser_state_tos).matching_brace_on_same_line < 0 as libc::c_int {
                    (*parser_state_tos).matching_brace_on_same_line = 1 as libc::c_int;
                } else {
                    (*parser_state_tos).matching_brace_on_same_line += 1;
                    (*parser_state_tos).matching_brace_on_same_line;
                }
                if l_enum != 0 {
                    (*parser_state_tos).block_init = 2 as libc::c_int;
                    (*parser_state_tos).block_init_level = 0 as libc::c_int;
                    l_enum = 0 as libc::c_int;
                }
                unary_delim = 1 as libc::c_int;
                code = lbrace;
                current_block = 8501358306579972139;
            }
            125 => {
                (*parser_state_tos).matching_brace_on_same_line -= 1;
                (*parser_state_tos).matching_brace_on_same_line;
                l_enum = 0 as libc::c_int;
                unary_delim = 1 as libc::c_int;
                code = rbrace;
                current_block = 8501358306579972139;
            }
            12 => {
                unary_delim = (*parser_state_tos).last_u_d;
                (*parser_state_tos).last_nl = 1 as libc::c_int;
                code = form_feed;
                current_block = 8501358306579972139;
            }
            44 => {
                unary_delim = 1 as libc::c_int;
                code = comma;
                current_block = 8501358306579972139;
            }
            46 => {
                if *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                    == '.' as i32
                    && *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                        == '.' as i32
                {
                    buf_ptr = buf_ptr.offset(2 as libc::c_int as isize);
                    if buf_ptr >= buf_end {
                        fill_buffer();
                    }
                    unary_delim = 1 as libc::c_int;
                    token_end = buf_ptr;
                    if (*parser_state_tos).in_decl != 0 {
                        code = decl;
                    } else {
                        code = binary_op;
                    }
                } else {
                    unary_delim = 0 as libc::c_int;
                    code = struct_delim;
                    if *buf_ptr as libc::c_int == '*' as i32 {
                        buf_ptr = buf_ptr.offset(1);
                        buf_ptr;
                        token_end = buf_ptr;
                    }
                }
                current_block = 8501358306579972139;
            }
            45 | 43 => {
                code = (if (*parser_state_tos).last_u_d != 0 {
                    unary_op as libc::c_int
                } else {
                    binary_op as libc::c_int
                }) as codes_ty;
                unary_delim = 1 as libc::c_int;
                if *buf_ptr as libc::c_int
                    == *token.offset(0 as libc::c_int as isize) as libc::c_int
                {
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    if last_code as libc::c_uint == ident as libc::c_int as libc::c_uint
                        || last_code as libc::c_uint
                            == rparen as libc::c_int as libc::c_uint
                    {
                        code = (if (*parser_state_tos).last_u_d != 0 {
                            unary_op as libc::c_int
                        } else {
                            postop as libc::c_int
                        }) as codes_ty;
                        unary_delim = 0 as libc::c_int;
                    }
                } else if *buf_ptr as libc::c_int == '=' as i32 {
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                } else if *buf_ptr as libc::c_int == '>' as i32 {
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    code = struct_delim;
                    if *buf_ptr as libc::c_int == '*' as i32 {
                        buf_ptr = buf_ptr.offset(1);
                        buf_ptr;
                    }
                }
                token_end = buf_ptr;
                current_block = 8501358306579972139;
            }
            61 => {
                if (*parser_state_tos).in_or_st != 0
                    && (*parser_state_tos).last_token as libc::c_uint
                        != cpp_operator as libc::c_int as libc::c_uint
                {
                    (*parser_state_tos).block_init = 1 as libc::c_int;
                    (*parser_state_tos).block_init_level = 0 as libc::c_int;
                }
                if *buf_ptr as libc::c_int == '=' as i32 {
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                } else if *buf_ptr as libc::c_int == '-' as i32
                    || *buf_ptr as libc::c_int == '+' as i32
                    || *buf_ptr as libc::c_int == '*' as i32
                    || *buf_ptr as libc::c_int == '&' as i32
                {
                    tmpchar[0 as libc::c_int as usize] = *buf_ptr;
                    tmpchar[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                    message(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Warning\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"old style assignment ambiguity in \"=%s\". Assuming \"= %s\"\n\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        tmpchar.as_mut_ptr(),
                        tmpchar.as_mut_ptr(),
                    );
                }
                code = binary_op;
                unary_delim = 1 as libc::c_int;
                token_end = buf_ptr;
                current_block = 8501358306579972139;
            }
            62 | 60 | 33 => {
                while *buf_ptr as libc::c_int == '>' as i32
                    || *buf_ptr as libc::c_int == '<' as i32
                    || *buf_ptr as libc::c_int == '=' as i32
                    || settings.c_plus_plus != 0 && *buf_ptr as libc::c_int == '?' as i32
                {
                    buf_ptr = buf_ptr.offset(1);
                    if buf_ptr >= buf_end {
                        fill_buffer();
                    }
                    if *buf_ptr as libc::c_int == '=' as i32 {
                        buf_ptr = buf_ptr.offset(1);
                        if buf_ptr >= buf_end {
                            fill_buffer();
                        }
                    }
                }
                code = (if (*parser_state_tos).last_u_d != 0 {
                    unary_op as libc::c_int
                } else {
                    binary_op as libc::c_int
                }) as codes_ty;
                unary_delim = 1 as libc::c_int;
                token_end = buf_ptr;
                current_block = 8501358306579972139;
            }
            _ => {
                if *token.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                    && (*buf_ptr as libc::c_int == '*' as i32
                        || *buf_ptr as libc::c_int == '/' as i32)
                {
                    if *buf_ptr as libc::c_int == '*' as i32 {
                        code = comment;
                    } else {
                        code = cplus_comment;
                    }
                    buf_ptr = buf_ptr.offset(1);
                    if buf_ptr >= buf_end {
                        fill_buffer();
                    }
                    if code as libc::c_uint == comment as libc::c_int as libc::c_uint {
                        let mut p_0: *mut libc::c_char = buf_ptr;
                        loop {
                            let fresh2 = p_0;
                            p_0 = p_0.offset(1);
                            if !(*(*__ctype_b_loc())
                                .offset(*fresh2 as libc::c_int as isize) as libc::c_int
                                & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
                                != 0)
                            {
                                break;
                            }
                        }
                        if p_0 < buf_end
                            && *p_0.offset(-(1 as libc::c_int) as isize) as libc::c_int
                                == '*' as i32 && *p_0 as libc::c_int == '/' as i32
                        {
                            buf_ptr = p_0.offset(1 as libc::c_int as isize);
                            code = ident;
                            (*parser_state_tos).want_blank = 1 as libc::c_int;
                        }
                    }
                    unary_delim = (*parser_state_tos).last_u_d;
                } else if (*parser_state_tos).last_token as libc::c_uint
                    == cpp_operator as libc::c_int as libc::c_uint
                {
                    code = overloaded;
                    last_code = overloaded;
                } else {
                    while *buf_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                        == *buf_ptr as libc::c_int
                        || *buf_ptr as libc::c_int == '=' as i32
                    {
                        buf_ptr = buf_ptr.offset(1);
                        if buf_ptr >= buf_end {
                            fill_buffer();
                        }
                    }
                    code = (if (*parser_state_tos).last_u_d != 0 {
                        unary_op as libc::c_int
                    } else {
                        binary_op as libc::c_int
                    }) as codes_ty;
                    unary_delim = 1 as libc::c_int;
                }
                token_end = buf_ptr;
                current_block = 8501358306579972139;
            }
        }
        match current_block {
            18299784206167311994 => {
                while *buf_ptr as libc::c_int != qchar as libc::c_int
                    && *buf_ptr as libc::c_int != 0 as libc::c_int
                {
                    if *buf_ptr as libc::c_int == '\n' as i32 {
                        line_no += 1;
                        line_no;
                    }
                    if *buf_ptr as libc::c_int == '\\' as i32 {
                        buf_ptr = buf_ptr.offset(1);
                        buf_ptr;
                        if buf_ptr >= buf_end {
                            fill_buffer();
                        }
                        if *buf_ptr as libc::c_int == '\n' as i32 {
                            line_no += 1;
                            line_no;
                        }
                        if *buf_ptr as libc::c_int == 0 as libc::c_int {
                            break;
                        }
                    }
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    if buf_ptr >= buf_end {
                        fill_buffer();
                    }
                }
                if *buf_ptr as libc::c_int == '\n' as i32
                    || *buf_ptr as libc::c_int == 0 as libc::c_int
                {
                    message(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"Warning\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        if qchar as libc::c_int == '\'' as i32 {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Unterminated character constant\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        } else {
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Unterminated string constant\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            )
                        },
                        0 as *mut libc::c_void as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut libc::c_char,
                    );
                } else {
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    if buf_ptr >= buf_end {
                        fill_buffer();
                    }
                }
                if settings.gettext_strings != 0
                    && (*token.offset(0 as libc::c_int as isize) as libc::c_int
                        == '_' as i32
                        || *token.offset(0 as libc::c_int as isize) as libc::c_int
                            == 'N' as i32
                            && *token.offset(1 as libc::c_int as isize) as libc::c_int
                                == '_' as i32)
                {
                    if *buf_ptr as libc::c_int != ')' as i32 {
                        message(
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Warning\0" as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            dcgettext(
                                0 as *const libc::c_char,
                                b"Unterminated string constant\0" as *const u8
                                    as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            0 as *mut libc::c_void as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut libc::c_char,
                        );
                    } else {
                        buf_ptr = buf_ptr.offset(1);
                        buf_ptr;
                        if buf_ptr >= buf_end {
                            fill_buffer();
                        }
                    }
                }
                token_end = buf_ptr;
                code = ident;
            }
            _ => {}
        }
        if code as libc::c_uint != newline as libc::c_int as libc::c_uint {
            l_struct = 0 as libc::c_int;
            last_code = code;
        }
        if buf_ptr >= buf_end {
            fill_buffer();
        }
        (*parser_state_tos).last_u_d = unary_delim;
        if (*parser_state_tos).last_token as libc::c_uint
            == cpp_operator as libc::c_int as libc::c_uint
        {
            return overloaded;
        }
        return code;
    };
}
pub unsafe extern "C" fn addkey(mut key: *mut libc::c_char, mut val: rwcodes_ty) {
    let mut p: *mut templ_ty = 0 as *mut templ_ty;
    if !(settings.c_plus_plus != 0 && !(is_reserved_cc(key, strlen(key))).is_null()
        || settings.c_plus_plus == 0 && !(is_reserved(key, strlen(key))).is_null())
    {
        if user_specials.is_null() {
            user_specials = xmalloc(
                (5 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<templ_ty>() as libc::c_ulong)
                    as libc::c_uint,
            ) as *mut templ_ty;
            user_specials_max = 5 as libc::c_int as libc::c_uint;
            user_specials_idx = 0 as libc::c_int as libc::c_uint;
        } else if user_specials_idx == user_specials_max {
            user_specials_max = user_specials_max
                .wrapping_add(5 as libc::c_int as libc::c_uint);
            user_specials = xrealloc(
                user_specials as *mut libc::c_void,
                (user_specials_max as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<templ_ty>() as libc::c_ulong)
                    as libc::c_uint,
            ) as *mut templ_ty;
        }
        let fresh3 = user_specials_idx;
        user_specials_idx = user_specials_idx.wrapping_add(1);
        p = &mut *user_specials.offset(fresh3 as isize) as *mut templ_ty;
        (*p).rwd = key;
        (*p).rwcode = val;
    }
}
pub unsafe extern "C" fn cleanup_user_specials() {
    if !user_specials.is_null() {
        loop {
            user_specials_idx = user_specials_idx.wrapping_sub(1);
            if !(user_specials_idx > 0 as libc::c_int as libc::c_uint) {
                break;
            }
            xfree(
                (*user_specials.offset(user_specials_idx as isize)).rwd
                    as *mut libc::c_void,
            );
        }
        xfree(
            (*user_specials.offset(0 as libc::c_int as isize)).rwd as *mut libc::c_void,
        );
        xfree(user_specials as *mut libc::c_void);
    }
}

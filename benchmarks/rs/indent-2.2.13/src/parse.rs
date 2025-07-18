use ::libc;
extern "C" {
    static mut labbuf: *mut libc::c_char;
    static mut s_lab: *mut libc::c_char;
    static mut e_lab: *mut libc::c_char;
    static mut l_lab: *mut libc::c_char;
    static mut codebuf: *mut libc::c_char;
    static mut s_code: *mut libc::c_char;
    static mut e_code: *mut libc::c_char;
    static mut l_code: *mut libc::c_char;
    static mut combuf: *mut libc::c_char;
    static mut s_com: *mut libc::c_char;
    static mut e_com: *mut libc::c_char;
    static mut l_com: *mut libc::c_char;
    static mut save_com: buf_ty;
    static mut bp_save: *mut libc::c_char;
    static mut be_save: *mut libc::c_char;
    static mut prefix_blankline_requested: libc::c_int;
    static mut break_comma: libc::c_int;
    static mut settings: user_options_ty;
    static mut di_stack_alloc: libc::c_int;
    static mut di_stack: *mut libc::c_int;
    static mut else_or_endif: libc::c_int;
    static mut had_eof: BOOLEAN;
    static mut line_no: libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(size: libc::c_uint) -> *mut libc::c_void;
    fn xrealloc(ptr: *mut libc::c_void, size: libc::c_uint) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn fatal(string: *const libc::c_char, a0: *const libc::c_char);
    fn message(
        kind: *mut libc::c_char,
        string: *mut libc::c_char,
        a0: *mut libc::c_char,
        a1: *mut libc::c_char,
    );
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
pub static mut parser_state_tos: *mut parser_state_ty = 0 as *const parser_state_ty
    as *mut parser_state_ty;
pub unsafe extern "C" fn init_parser() {
    parser_state_tos = xmalloc(
        ::std::mem::size_of::<parser_state_ty>() as libc::c_ulong as libc::c_uint,
    ) as *mut parser_state_ty;
    (*parser_state_tos).p_stack_size = 2 as libc::c_int;
    (*parser_state_tos)
        .p_stack = xmalloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<codes_ty>() as libc::c_ulong)
            as libc::c_uint,
    ) as *mut codes_ty;
    (*parser_state_tos)
        .il = xmalloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_uint,
    ) as *mut libc::c_int;
    (*parser_state_tos)
        .cstk = xmalloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_uint,
    ) as *mut libc::c_int;
    (*parser_state_tos).paren_indents_size = 8 as libc::c_int;
    (*parser_state_tos)
        .paren_indents = xmalloc(
        ((*parser_state_tos).paren_indents_size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong)
            as libc::c_uint,
    ) as *mut libc::c_short;
    combuf = xmalloc(1024 as libc::c_int as libc::c_uint) as *mut libc::c_char;
    labbuf = xmalloc(1024 as libc::c_int as libc::c_uint) as *mut libc::c_char;
    codebuf = xmalloc(1024 as libc::c_int as libc::c_uint) as *mut libc::c_char;
    save_com.size = 1024 as libc::c_int;
    save_com.ptr = xmalloc(save_com.size as libc::c_uint) as *mut libc::c_char;
    save_com.end = save_com.ptr;
    save_com.column = 0 as libc::c_int;
    save_com.len = save_com.column;
    di_stack_alloc = 2 as libc::c_int;
    di_stack = xmalloc(
        (di_stack_alloc as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_uint,
    ) as *mut libc::c_int;
}
pub unsafe extern "C" fn uninit_parser() {
    if parser_state_tos.is_null() {
        return;
    }
    xfree((*parser_state_tos).p_stack as *mut libc::c_void);
    xfree((*parser_state_tos).il as *mut libc::c_void);
    xfree((*parser_state_tos).cstk as *mut libc::c_void);
    xfree((*parser_state_tos).paren_indents as *mut libc::c_void);
    xfree(parser_state_tos as *mut libc::c_void);
    xfree(save_com.ptr as *mut libc::c_void);
    xfree(combuf as *mut libc::c_void);
    xfree(labbuf as *mut libc::c_void);
    xfree(codebuf as *mut libc::c_void);
    xfree(di_stack as *mut libc::c_void);
    parser_state_tos = 0 as *mut parser_state_ty;
}
pub unsafe extern "C" fn reset_parser() {
    (*parser_state_tos).next = 0 as *mut parser_state;
    (*parser_state_tos).tos = 0 as libc::c_int;
    *((*parser_state_tos).p_stack).offset(0 as libc::c_int as isize) = stmt;
    (*parser_state_tos).last_nl = 1 as libc::c_int;
    (*parser_state_tos).last_token = start_token;
    (*parser_state_tos).last_saw_nl = 0 as libc::c_int;
    (*parser_state_tos).broken_at_non_nl = 0 as libc::c_int;
    (*parser_state_tos).box_com = 0 as libc::c_int;
    (*parser_state_tos).cast_mask = 0 as libc::c_int;
    (*parser_state_tos).noncast_mask = 0 as libc::c_int;
    (*parser_state_tos).sizeof_mask = 0 as libc::c_int;
    (*parser_state_tos).block_init = 0 as libc::c_int;
    (*parser_state_tos).block_init_level = 0 as libc::c_int;
    (*parser_state_tos).col_1 = 0 as libc::c_int;
    (*parser_state_tos).com_col = 0 as libc::c_int;
    (*parser_state_tos).dec_nest = 0 as libc::c_int;
    (*parser_state_tos).i_l_follow = 0 as libc::c_int;
    (*parser_state_tos).ind_level = 0 as libc::c_int;
    (*parser_state_tos).last_u_d = 0 as libc::c_int;
    (*parser_state_tos).p_l_follow = 0 as libc::c_int;
    (*parser_state_tos).paren_level = 0 as libc::c_int;
    (*parser_state_tos).paren_depth = 0 as libc::c_int;
    (*parser_state_tos).search_brace = 0 as libc::c_int;
    (*parser_state_tos).use_ff = 0 as libc::c_int;
    (*parser_state_tos).its_a_keyword = 0 as libc::c_int;
    (*parser_state_tos).sizeof_keyword = 0 as libc::c_int;
    (*parser_state_tos).in_parameter_declaration = 0 as libc::c_int;
    (*parser_state_tos).just_saw_decl = 0 as libc::c_int;
    (*parser_state_tos).in_decl = 0 as libc::c_int as BOOLEAN;
    (*parser_state_tos).decl_on_line = 0 as libc::c_int;
    (*parser_state_tos).in_or_st = 0 as libc::c_int;
    (*parser_state_tos).want_blank = 0 as libc::c_int;
    (*parser_state_tos).in_stmt = 0 as libc::c_int;
    (*parser_state_tos).ind_stmt = 0 as libc::c_int;
    (*parser_state_tos)
        .procname = b"\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*parser_state_tos)
        .procname_end = b"\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*parser_state_tos)
        .classname = b"\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*parser_state_tos)
        .classname_end = b"\0\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    (*parser_state_tos).pcase = 0 as libc::c_int;
    (*parser_state_tos).dec_nest = 0 as libc::c_int;
    (*parser_state_tos).can_break = bb_none;
    (*parser_state_tos).saw_double_colon = 0 as libc::c_int;
    (*parser_state_tos).is_func_ptr_decl = 0 as libc::c_int;
    *((*parser_state_tos).il).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *((*parser_state_tos).cstk).offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    save_com.column = 0 as libc::c_int;
    save_com.len = save_com.column;
    *di_stack.offset((*parser_state_tos).dec_nest as isize) = 0 as libc::c_int;
    l_com = combuf
        .offset(1024 as libc::c_int as isize)
        .offset(-(5 as libc::c_int as isize));
    l_lab = labbuf
        .offset(1024 as libc::c_int as isize)
        .offset(-(5 as libc::c_int as isize));
    l_code = codebuf
        .offset(1024 as libc::c_int as isize)
        .offset(-(5 as libc::c_int as isize));
    let ref mut fresh0 = *labbuf.offset(0 as libc::c_int as isize);
    *fresh0 = ' ' as i32 as libc::c_char;
    let ref mut fresh1 = *codebuf.offset(0 as libc::c_int as isize);
    *fresh1 = *fresh0;
    *combuf.offset(0 as libc::c_int as isize) = *fresh1;
    let ref mut fresh2 = *labbuf.offset(1 as libc::c_int as isize);
    *fresh2 = '\0' as i32 as libc::c_char;
    let ref mut fresh3 = *codebuf.offset(1 as libc::c_int as isize);
    *fresh3 = *fresh2;
    *combuf.offset(1 as libc::c_int as isize) = *fresh3;
    else_or_endif = 0 as libc::c_int;
    e_lab = labbuf.offset(1 as libc::c_int as isize);
    s_lab = e_lab;
    e_code = codebuf.offset(1 as libc::c_int as isize);
    s_code = e_code;
    e_com = combuf.offset(1 as libc::c_int as isize);
    s_com = e_com;
    line_no = 1 as libc::c_int;
    had_eof = 0 as libc::c_int as BOOLEAN;
    break_comma = 0 as libc::c_int;
    bp_save = 0 as *mut libc::c_char;
    be_save = 0 as *mut libc::c_char;
    if settings.tabsize <= 0 as libc::c_int {
        settings.tabsize = 1 as libc::c_int;
    }
    prefix_blankline_requested = 0 as libc::c_int;
}
pub unsafe extern "C" fn inc_pstack() -> libc::c_int {
    (*parser_state_tos).tos += 1;
    if (*parser_state_tos).tos >= (*parser_state_tos).p_stack_size {
        (*parser_state_tos).p_stack_size *= 2 as libc::c_int;
        (*parser_state_tos)
            .p_stack = xrealloc(
            (*parser_state_tos).p_stack as *mut libc::c_void,
            ((*parser_state_tos).p_stack_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<codes_ty>() as libc::c_ulong)
                as libc::c_uint,
        ) as *mut codes_ty;
        (*parser_state_tos)
            .il = xrealloc(
            (*parser_state_tos).il as *mut libc::c_void,
            ((*parser_state_tos).p_stack_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_uint,
        ) as *mut libc::c_int;
        (*parser_state_tos)
            .cstk = xrealloc(
            (*parser_state_tos).cstk as *mut libc::c_void,
            ((*parser_state_tos).p_stack_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_uint,
        ) as *mut libc::c_int;
    }
    *((*parser_state_tos).cstk)
        .offset(
            (*parser_state_tos).tos as isize,
        ) = *((*parser_state_tos).cstk)
        .offset(((*parser_state_tos).tos - 1 as libc::c_int) as isize);
    return (*parser_state_tos).tos;
}
pub unsafe extern "C" fn parse(mut tk: codes_ty) -> exit_values_ty {
    let mut i: libc::c_int = 0;
    while *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
        as libc::c_uint == ifhead as libc::c_int as libc::c_uint
        && tk as libc::c_uint != elselit as libc::c_int as libc::c_uint
    {
        *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize) = stmt;
        reduce();
    }
    let mut current_block_110: u64;
    match tk as libc::c_uint {
        24 => {
            (*parser_state_tos).search_brace = settings.braces_on_struct_decl_line;
            if *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
                as libc::c_uint != decl as libc::c_int as libc::c_uint
                && (*parser_state_tos).block_init == 0 as libc::c_int
            {
                break_comma = 1 as libc::c_int;
                inc_pstack();
                *((*parser_state_tos).p_stack)
                    .offset((*parser_state_tos).tos as isize) = decl;
                *((*parser_state_tos).il)
                    .offset(
                        (*parser_state_tos).tos as isize,
                    ) = (*parser_state_tos).i_l_follow;
                if settings.ljust_decl != 0 {
                    (*parser_state_tos).ind_level = 0 as libc::c_int;
                    i = (*parser_state_tos).tos - 1 as libc::c_int;
                    while i > 0 as libc::c_int {
                        if *((*parser_state_tos).p_stack).offset(i as isize)
                            as libc::c_uint == decl as libc::c_int as libc::c_uint
                        {
                            (*parser_state_tos).ind_level += settings.ind_size;
                        }
                        i -= 1;
                        i;
                    }
                    (*parser_state_tos).i_l_follow = (*parser_state_tos).ind_level;
                }
            }
            current_block_110 = 16037123508100270995;
        }
        28 => {
            if *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
                as libc::c_uint == elsehead as libc::c_int as libc::c_uint
            {
                (*parser_state_tos)
                    .i_l_follow = *((*parser_state_tos).il)
                    .offset((*parser_state_tos).tos as isize);
            }
            current_block_110 = 18414224136269064119;
        }
        35 => {
            current_block_110 = 18414224136269064119;
        }
        31 | 9 => {
            current_block_110 = 3948058512416047487;
        }
        13 => {
            break_comma = 0 as libc::c_int;
            if *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
                as libc::c_uint == stmt as libc::c_int as libc::c_uint
                || *((*parser_state_tos).p_stack)
                    .offset((*parser_state_tos).tos as isize) as libc::c_uint
                    == stmtl as libc::c_int as libc::c_uint
            {
                (*parser_state_tos).i_l_follow += settings.ind_size;
            } else if *((*parser_state_tos).p_stack)
                .offset((*parser_state_tos).tos as isize) as libc::c_uint
                == decl as libc::c_int as libc::c_uint
            {
                (*parser_state_tos).i_l_follow += settings.ind_size;
                if ((*parser_state_tos).last_rw as libc::c_uint
                    == rw_struct_like as libc::c_int as libc::c_uint
                    || (*parser_state_tos).last_rw as libc::c_uint
                        == rw_enum as libc::c_int as libc::c_uint)
                    && ((*parser_state_tos).block_init != 1 as libc::c_int
                        || (*parser_state_tos).block_init_level == 0 as libc::c_int)
                    && (*parser_state_tos).last_token as libc::c_uint
                        != rparen as libc::c_int as libc::c_uint
                    && settings.braces_on_struct_decl_line == 0
                {
                    (*parser_state_tos).ind_level += settings.struct_brace_indent;
                    (*parser_state_tos).i_l_follow += settings.struct_brace_indent;
                }
            } else if *((*parser_state_tos).p_stack)
                .offset((*parser_state_tos).tos as isize) as libc::c_uint
                == casestmt as libc::c_int as libc::c_uint
            {
                (*parser_state_tos).ind_level
                    += settings.case_brace_indent - settings.ind_size;
                (*parser_state_tos).i_l_follow += settings.case_brace_indent;
            } else {
                if s_code == e_code {
                    (*parser_state_tos).ind_level -= settings.ind_size;
                }
                if settings.btype_2 == 0 {
                    (*parser_state_tos).ind_level += settings.brace_indent;
                    (*parser_state_tos).i_l_follow += settings.brace_indent;
                }
                if *((*parser_state_tos).p_stack)
                    .offset((*parser_state_tos).tos as isize) as libc::c_uint
                    == swstmt as libc::c_int as libc::c_uint
                {
                    (*parser_state_tos).i_l_follow += settings.case_indent;
                }
            }
            inc_pstack();
            *((*parser_state_tos).p_stack)
                .offset((*parser_state_tos).tos as isize) = lbrace;
            *((*parser_state_tos).il)
                .offset(
                    (*parser_state_tos).tos as isize,
                ) = (*parser_state_tos).ind_level;
            inc_pstack();
            *((*parser_state_tos).p_stack)
                .offset((*parser_state_tos).tos as isize) = stmt;
            *((*parser_state_tos).il)
                .offset(
                    (*parser_state_tos).tos as isize,
                ) = (*parser_state_tos).i_l_follow;
            current_block_110 = 16037123508100270995;
        }
        30 => {
            if *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
                as libc::c_uint == dohead as libc::c_int as libc::c_uint
            {
                (*parser_state_tos)
                    .i_l_follow = *((*parser_state_tos).il)
                    .offset((*parser_state_tos).tos as isize);
                (*parser_state_tos)
                    .ind_level = *((*parser_state_tos).il)
                    .offset((*parser_state_tos).tos as isize);
                inc_pstack();
                *((*parser_state_tos).p_stack)
                    .offset((*parser_state_tos).tos as isize) = whilestmt;
                (*parser_state_tos).ind_level = (*parser_state_tos).i_l_follow;
                *((*parser_state_tos).il)
                    .offset(
                        (*parser_state_tos).tos as isize,
                    ) = (*parser_state_tos).i_l_follow;
            } else {
                inc_pstack();
                *((*parser_state_tos).p_stack)
                    .offset((*parser_state_tos).tos as isize) = whilestmt;
                *((*parser_state_tos).il)
                    .offset(
                        (*parser_state_tos).tos as isize,
                    ) = (*parser_state_tos).i_l_follow;
                (*parser_state_tos).i_l_follow += settings.ind_size;
                (*parser_state_tos).search_brace = settings.btype_2;
            }
            current_block_110 = 16037123508100270995;
        }
        34 => {
            if *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
                as libc::c_uint != ifhead as libc::c_int as libc::c_uint
            {
                message(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unmatched 'else'\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
            } else {
                (*parser_state_tos)
                    .ind_level = *((*parser_state_tos).il)
                    .offset((*parser_state_tos).tos as isize);
                (*parser_state_tos)
                    .i_l_follow = (*parser_state_tos).ind_level + settings.ind_size;
                *((*parser_state_tos).p_stack)
                    .offset((*parser_state_tos).tos as isize) = elsehead;
                (*parser_state_tos).search_brace = 1 as libc::c_int;
            }
            current_block_110 = 16037123508100270995;
        }
        14 => {
            if (*parser_state_tos).tos > 0 as libc::c_int
                && *((*parser_state_tos).p_stack)
                    .offset(((*parser_state_tos).tos - 1 as libc::c_int) as isize)
                    as libc::c_uint == lbrace as libc::c_int as libc::c_uint
            {
                (*parser_state_tos).tos -= 1;
                (*parser_state_tos)
                    .i_l_follow = *((*parser_state_tos).il)
                    .offset((*parser_state_tos).tos as isize);
                (*parser_state_tos).ind_level = (*parser_state_tos).i_l_follow;
                *((*parser_state_tos).p_stack)
                    .offset((*parser_state_tos).tos as isize) = stmt;
            } else {
                message(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Error\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Stmt nesting error.\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    0 as *mut libc::c_void as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut libc::c_char,
                );
            }
            current_block_110 = 16037123508100270995;
        }
        21 => {
            inc_pstack();
            *((*parser_state_tos).p_stack)
                .offset((*parser_state_tos).tos as isize) = swstmt;
            *((*parser_state_tos).cstk)
                .offset(
                    (*parser_state_tos).tos as isize,
                ) = settings.case_indent + (*parser_state_tos).i_l_follow;
            if settings.btype_2 == 0 {
                *((*parser_state_tos).cstk).offset((*parser_state_tos).tos as isize)
                    += settings.brace_indent;
            }
            *((*parser_state_tos).il)
                .offset(
                    (*parser_state_tos).tos as isize,
                ) = (*parser_state_tos).i_l_follow;
            (*parser_state_tos).i_l_follow += settings.ind_size;
            (*parser_state_tos).search_brace = settings.btype_2;
            current_block_110 = 16037123508100270995;
        }
        12 => {
            break_comma = 0 as libc::c_int;
            if *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
                as libc::c_uint == dostmt as libc::c_int as libc::c_uint
            {
                *((*parser_state_tos).p_stack)
                    .offset((*parser_state_tos).tos as isize) = stmt;
            } else {
                inc_pstack();
                *((*parser_state_tos).p_stack)
                    .offset((*parser_state_tos).tos as isize) = stmt;
                *((*parser_state_tos).il)
                    .offset(
                        (*parser_state_tos).tos as isize,
                    ) = (*parser_state_tos).ind_level;
            }
            current_block_110 = 16037123508100270995;
        }
        _ => {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unknown code to parser\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *const libc::c_char,
            );
            current_block_110 = 16037123508100270995;
        }
    }
    match current_block_110 {
        18414224136269064119 => {
            current_block_110 = 3948058512416047487;
        }
        _ => {}
    }
    match current_block_110 {
        3948058512416047487 => {
            inc_pstack();
            *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize) = tk;
            (*parser_state_tos).ind_level = (*parser_state_tos).i_l_follow;
            *((*parser_state_tos).il)
                .offset(
                    (*parser_state_tos).tos as isize,
                ) = (*parser_state_tos).ind_level;
            if tk as libc::c_uint != casestmt as libc::c_int as libc::c_uint {
                (*parser_state_tos).i_l_follow += settings.ind_size;
            }
            (*parser_state_tos).search_brace = settings.btype_2;
        }
        _ => {}
    }
    reduce();
    return total_success;
}
pub unsafe extern "C" fn reduce() {
    let mut i: libc::c_int = 0;
    loop {
        match *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
            as libc::c_uint
        {
            32 => {
                if (*parser_state_tos).tos == 0 as libc::c_int {
                    return;
                }
                let mut current_block_12: u64;
                match *((*parser_state_tos).p_stack)
                    .offset(((*parser_state_tos).tos - 1 as libc::c_int) as isize)
                    as libc::c_uint
                {
                    32 | 33 => {
                        (*parser_state_tos).tos -= 1;
                        *((*parser_state_tos).p_stack)
                            .offset((*parser_state_tos).tos as isize) = stmtl;
                        current_block_12 = 5689001924483802034;
                    }
                    35 => {
                        (*parser_state_tos).tos -= 1;
                        *((*parser_state_tos).p_stack)
                            .offset((*parser_state_tos).tos as isize) = dohead;
                        (*parser_state_tos)
                            .i_l_follow = *((*parser_state_tos).il)
                            .offset((*parser_state_tos).tos as isize);
                        current_block_12 = 5689001924483802034;
                    }
                    28 => {
                        (*parser_state_tos).tos -= 1;
                        *((*parser_state_tos).p_stack)
                            .offset((*parser_state_tos).tos as isize) = ifhead;
                        i = (*parser_state_tos).tos - 1 as libc::c_int;
                        while *((*parser_state_tos).p_stack).offset(i as isize)
                            as libc::c_uint != stmt as libc::c_int as libc::c_uint
                            && *((*parser_state_tos).p_stack).offset(i as isize)
                                as libc::c_uint != stmtl as libc::c_int as libc::c_uint
                            && *((*parser_state_tos).p_stack).offset(i as isize)
                                as libc::c_uint != lbrace as libc::c_int as libc::c_uint
                        {
                            i -= 1;
                            i;
                        }
                        (*parser_state_tos)
                            .i_l_follow = *((*parser_state_tos).il).offset(i as isize);
                        current_block_12 = 5689001924483802034;
                    }
                    21 => {
                        current_block_12 = 2803813255614228254;
                    }
                    24 => {
                        current_block_12 = 2803813255614228254;
                    }
                    39 => {
                        current_block_12 = 18440665178214134792;
                    }
                    31 => {
                        current_block_12 = 17529221511509422402;
                    }
                    9 | 30 => {
                        current_block_12 = 17925473840362818883;
                    }
                    _ => return,
                }
                match current_block_12 {
                    2803813255614228254 => {
                        current_block_12 = 18440665178214134792;
                    }
                    _ => {}
                }
                match current_block_12 {
                    18440665178214134792 => {
                        current_block_12 = 17529221511509422402;
                    }
                    _ => {}
                }
                match current_block_12 {
                    17529221511509422402 => {
                        current_block_12 = 17925473840362818883;
                    }
                    _ => {}
                }
                match current_block_12 {
                    17925473840362818883 => {
                        (*parser_state_tos).tos -= 1;
                        *((*parser_state_tos).p_stack)
                            .offset((*parser_state_tos).tos as isize) = stmt;
                        (*parser_state_tos)
                            .i_l_follow = *((*parser_state_tos).il)
                            .offset((*parser_state_tos).tos as isize);
                    }
                    _ => {}
                }
            }
            30 => {
                if *((*parser_state_tos).p_stack)
                    .offset(((*parser_state_tos).tos - 1 as libc::c_int) as isize)
                    as libc::c_uint == dohead as libc::c_int as libc::c_uint
                {
                    (*parser_state_tos).tos -= 1;
                    *((*parser_state_tos).p_stack)
                        .offset((*parser_state_tos).tos as isize) = dostmt;
                } else {
                    return
                }
            }
            _ => return,
        }
    };
}
pub unsafe extern "C" fn parse_lparen_in_decl() {
    inc_pstack();
    *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize) = stmt;
    *((*parser_state_tos).il)
        .offset((*parser_state_tos).tos as isize) = (*parser_state_tos).ind_level;
    reduce();
}

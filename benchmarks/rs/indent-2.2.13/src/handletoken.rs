use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn abort() -> !;
    static mut be_save: *mut libc::c_char;
    static mut in_prog_pos: *mut libc::c_char;
    static mut labbuf: *mut libc::c_char;
    static mut s_lab: *mut libc::c_char;
    static mut e_lab: *mut libc::c_char;
    static mut l_lab: *mut libc::c_char;
    static mut codebuf: *mut libc::c_char;
    static mut s_code: *mut libc::c_char;
    static mut s_code_corresponds_to: *mut libc::c_char;
    static mut e_code: *mut libc::c_char;
    static mut l_code: *mut libc::c_char;
    static mut s_com: *mut libc::c_char;
    static mut e_com: *mut libc::c_char;
    static mut buf_ptr: *mut libc::c_char;
    static mut buf_end: *mut libc::c_char;
    static mut token: *mut libc::c_char;
    static mut token_end: *mut libc::c_char;
    static mut squest: libc::c_int;
    static mut save_com: buf_ty;
    static mut bp_save: *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut n_real_blanklines: libc::c_int;
    static mut prefix_blankline_requested: libc::c_int;
    static mut prefix_blankline_requested_code: codes_ty;
    static mut postfix_blankline_requested: libc::c_int;
    static mut postfix_blankline_requested_code: codes_ty;
    static mut break_comma: libc::c_int;
    static mut else_or_endif: libc::c_int;
    static mut di_stack: *mut libc::c_int;
    static mut di_stack_alloc: libc::c_int;
    static mut embedded_comment_on_line: libc::c_int;
    static mut settings: user_options_ty;
    static mut had_eof: BOOLEAN;
    static mut line_no: libc::c_int;
    static mut paren_target: libc::c_int;
    static mut parser_state_tos: *mut parser_state_ty;
    fn skip_horiz_space(p: *const libc::c_char) -> *mut libc::c_char;
    fn current_column() -> libc::c_int;
    fn fill_buffer();
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
    fn parse_lparen_in_decl();
    fn parse(tk: codes_ty) -> exit_values_ty;
    fn print_comment(paren_targ: *mut libc::c_int, pbreak_line: *mut BOOLEAN);
    static mut buf_break_used: libc::c_int;
    fn clear_buf_break_list(pbreak_line: *mut BOOLEAN);
    fn set_buf_break(code: bb_code_ty, paren_targ: libc::c_int);
    fn dump_line(
        force_nl: libc::c_int,
        paren_targ: *mut libc::c_int,
        break_line: *mut BOOLEAN,
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
pub unsafe extern "C" fn need_chars(mut bp: *mut buf_ty, mut needed: size_t) {
    let mut current_size: size_t = ((*bp).end).offset_from((*bp).ptr) as libc::c_long
        as size_t;
    if current_size.wrapping_add(needed) >= (*bp).size as size_t {
        (*bp)
            .size = (current_size
            .wrapping_add(needed)
            .wrapping_add(1024 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            & !(1024 as libc::c_int - 1 as libc::c_int) as libc::c_ulong) as libc::c_int;
        (*bp)
            .ptr = xrealloc((*bp).ptr as *mut libc::c_void, (*bp).size as libc::c_uint)
            as *mut libc::c_char;
        if ((*bp).ptr).is_null() {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Ran out of memory\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *const libc::c_char,
            );
        }
        (*bp).end = ((*bp).ptr).offset(current_size as isize);
    }
}
pub unsafe extern "C" fn check_code_size() {
    if e_code >= l_code {
        let mut nsize: libc::c_int = (l_code.offset_from(s_code) as libc::c_long
            + 400 as libc::c_int as libc::c_long) as libc::c_int;
        codebuf = xrealloc(codebuf as *mut libc::c_void, nsize as libc::c_uint)
            as *mut libc::c_char;
        e_code = codebuf
            .offset(e_code.offset_from(s_code) as libc::c_long as isize)
            .offset(1 as libc::c_int as isize);
        l_code = codebuf.offset(nsize as isize).offset(-(5 as libc::c_int as isize));
        s_code = codebuf.offset(1 as libc::c_int as isize);
    }
}
unsafe extern "C" fn check_lab_size() {
    if e_lab >= l_lab {
        let mut nsize: libc::c_int = (l_lab.offset_from(s_lab) as libc::c_long
            + 400 as libc::c_int as libc::c_long) as libc::c_int;
        labbuf = xrealloc(labbuf as *mut libc::c_void, nsize as libc::c_uint)
            as *mut libc::c_char;
        e_lab = labbuf
            .offset(e_lab.offset_from(s_lab) as libc::c_long as isize)
            .offset(1 as libc::c_int as isize);
        l_lab = labbuf.offset(nsize as isize).offset(-(5 as libc::c_int as isize));
        s_lab = labbuf.offset(1 as libc::c_int as isize);
    }
}
unsafe extern "C" fn copy_id(
    type_code: codes_ty,
    mut force_nl: *mut BOOLEAN,
    mut file_exit_value: *mut exit_values_ty,
    can_break: bb_code_ty,
) {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*parser_state_tos).want_blank != 0 {
        set_buf_break(bb_ident, paren_target);
        let fresh0 = e_code;
        e_code = e_code.offset(1);
        *fresh0 = ' ' as i32 as libc::c_char;
    } else if can_break as u64 != 0 {
        set_buf_break(can_break, paren_target);
    }
    if s_code == e_code {
        s_code_corresponds_to = token;
    }
    t_ptr = token;
    while t_ptr < token_end {
        check_code_size();
        let fresh1 = e_code;
        e_code = e_code.offset(1);
        *fresh1 = *t_ptr;
        t_ptr = t_ptr.offset(1);
        t_ptr;
    }
    *e_code = '\0' as i32 as libc::c_char;
    (*parser_state_tos).want_blank = 1 as libc::c_int;
    if type_code as libc::c_uint == sp_paren as libc::c_int as libc::c_uint
        && (settings.space_after_if == 0 && *token as libc::c_int == 'i' as i32
            || settings.space_after_for == 0 && *token as libc::c_int == 'f' as i32
            || settings.space_after_while == 0 && *token as libc::c_int == 'w' as i32)
    {
        (*parser_state_tos).want_blank = 0 as libc::c_int;
    }
    if token_end.offset_from(token) as libc::c_long == 1 as libc::c_int as libc::c_long
        && *token as libc::c_int == '_' as i32
        || token_end.offset_from(token) as libc::c_long
            == 2 as libc::c_int as libc::c_long && *token as libc::c_int == 'N' as i32
            && *token.offset(1 as libc::c_int as isize) as libc::c_int == '_' as i32
    {
        (*parser_state_tos).want_blank = 0 as libc::c_int;
    }
    if token_end.offset_from(token) as libc::c_long == 6 as libc::c_int as libc::c_long
        && strncmp(
            token,
            b"va_dcl\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        (*parser_state_tos).in_or_st = 0 as libc::c_int;
        (*parser_state_tos).just_saw_decl -= 1;
        (*parser_state_tos).just_saw_decl;
        (*parser_state_tos).in_decl = 0 as libc::c_int as BOOLEAN;
        if parse(semicolon) as libc::c_uint
            != total_success as libc::c_int as libc::c_uint
        {
            *file_exit_value = indent_error;
        }
        *force_nl = 1 as libc::c_int as BOOLEAN;
    }
}
unsafe extern "C" fn handle_token_form_feed(mut pbreak_line: *mut BOOLEAN) {
    (*parser_state_tos).use_ff = 1 as libc::c_int;
    dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
    (*parser_state_tos).want_blank = 0 as libc::c_int;
}
unsafe extern "C" fn handle_token_newline(
    mut force_nl: *mut BOOLEAN,
    mut pbreak_line: *mut BOOLEAN,
) {
    if s_lab != e_lab && *s_lab as libc::c_int == '#' as i32 {
        dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
        if s_code == e_code {
            (*parser_state_tos).want_blank = 0 as libc::c_int;
        }
        *force_nl = 0 as libc::c_int as BOOLEAN;
    } else if ((*parser_state_tos).last_token as libc::c_uint
        != comma as libc::c_int as libc::c_uint || settings.leave_comma == 0
        || break_comma == 0 || (*parser_state_tos).p_l_follow > 0 as libc::c_int
        || (*parser_state_tos).block_init != 0 || s_com != e_com)
        && ((*parser_state_tos).last_token as libc::c_uint
            != rbrace as libc::c_int as libc::c_uint
            || !(settings.braces_on_struct_decl_line != 0
                && (*parser_state_tos).in_decl as libc::c_int != 0))
    {
        if settings.procnames_start_line == 0 && s_lab == e_lab
            && (*parser_state_tos).last_token as libc::c_uint
                != lparen as libc::c_int as libc::c_uint
            && (*parser_state_tos).last_token as libc::c_uint
                != semicolon as libc::c_int as libc::c_uint
            && (*parser_state_tos).last_token as libc::c_uint
                != comma as libc::c_int as libc::c_uint
            && (*parser_state_tos).last_rw as libc::c_uint
                == rw_decl as libc::c_int as libc::c_uint
            && (*parser_state_tos).last_rw_depth == 0 as libc::c_int
            && (*parser_state_tos).block_init == 0
            && (*parser_state_tos).in_decl as libc::c_int != 0
        {
            if !(e_code > s_code
                && *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '*' as i32)
            {
                (*parser_state_tos).want_blank = 1 as libc::c_int;
            }
        }
        if (*parser_state_tos).in_stmt == 0 || s_com != e_com
            || embedded_comment_on_line != 0
        {
            dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
            if s_code == e_code {
                (*parser_state_tos).want_blank = 0 as libc::c_int;
            }
            *force_nl = 0 as libc::c_int as BOOLEAN;
        }
    }
    else_or_endif = 0 as libc::c_int;
    line_no += 1;
    line_no;
}
unsafe extern "C" fn handle_token_lparen(
    mut force_nl: *mut BOOLEAN,
    mut sp_sw: *mut BOOLEAN,
    mut dec_ind: *mut libc::c_int,
    mut pbreak_line: *mut BOOLEAN,
) {
    if *token as libc::c_int == '{' as i32
        && (s_code != e_code || s_com != e_com || s_lab != e_lab)
    {
        dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
        (*parser_state_tos).want_blank = 0 as libc::c_int;
    }
    (*parser_state_tos).p_l_follow += 1;
    (*parser_state_tos).p_l_follow;
    if (*parser_state_tos).p_l_follow >= (*parser_state_tos).paren_indents_size {
        (*parser_state_tos).paren_indents_size *= 2 as libc::c_int;
        (*parser_state_tos)
            .paren_indents = xrealloc(
            (*parser_state_tos).paren_indents as *mut libc::c_void,
            ((*parser_state_tos).paren_indents_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong)
                as libc::c_uint,
        ) as *mut libc::c_short;
    }
    (*parser_state_tos).paren_depth += 1;
    (*parser_state_tos).paren_depth;
    if (*parser_state_tos).is_func_ptr_decl != 0 && settings.proc_calls_space == 0 {
        (*parser_state_tos)
            .want_blank = (*token.offset(-(1 as libc::c_int as isize)) as libc::c_int
            != ')' as i32
            && *token.offset(-(1 as libc::c_int as isize)) as libc::c_int != ' ' as i32)
            as libc::c_int;
    }
    if (*parser_state_tos).want_blank != 0 && *token as libc::c_int != '[' as i32
        && ((*parser_state_tos).last_token as libc::c_uint
            != ident as libc::c_int as libc::c_uint || settings.proc_calls_space != 0
            || (*parser_state_tos).its_a_keyword != 0
                && ((*parser_state_tos).sizeof_keyword == 0
                    || settings.blank_after_sizeof != 0))
    {
        set_buf_break(bb_proc_call, paren_target);
        let fresh2 = e_code;
        e_code = e_code.offset(1);
        *fresh2 = ' ' as i32 as libc::c_char;
        *e_code = '\0' as i32 as libc::c_char;
    } else {
        set_buf_break(bb_proc_call, paren_target);
    }
    if *token.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
        && (*parser_state_tos).last_rw as libc::c_uint
            == rw_decl as libc::c_int as libc::c_uint
        && ((*parser_state_tos).last_token as libc::c_uint
            == decl as libc::c_int as libc::c_uint
            || (*parser_state_tos).last_token as libc::c_uint
                == unary_op as libc::c_int as libc::c_uint)
    {
        (*parser_state_tos).is_func_ptr_decl = 1 as libc::c_int;
    }
    if (*parser_state_tos).in_decl as libc::c_int != 0
        && (*parser_state_tos).block_init == 0
    {
        if *token as libc::c_int != '[' as i32 && buf_break_used == 0 {
            while (e_code.offset_from(s_code) as libc::c_long) < *dec_ind as libc::c_long
            {
                check_code_size();
                set_buf_break(bb_dec_ind, paren_target);
                let fresh3 = e_code;
                e_code = e_code.offset(1);
                *fresh3 = ' ' as i32 as libc::c_char;
            }
            let fresh4 = e_code;
            e_code = e_code.offset(1);
            *fresh4 = *token.offset(0 as libc::c_int as isize);
            (*parser_state_tos).ind_stmt = 0 as libc::c_int;
        } else {
            let fresh5 = e_code;
            e_code = e_code.offset(1);
            *fresh5 = *token.offset(0 as libc::c_int as isize);
        }
    } else {
        let fresh6 = e_code;
        e_code = e_code.offset(1);
        *fresh6 = *token.offset(0 as libc::c_int as isize);
    }
    if settings.parentheses_space != 0 && *token as libc::c_int != '[' as i32 {
        let fresh7 = e_code;
        e_code = e_code.offset(1);
        *fresh7 = ' ' as i32 as libc::c_char;
    }
    *((*parser_state_tos).paren_indents)
        .offset(
            ((*parser_state_tos).p_l_follow - 1 as libc::c_int) as isize,
        ) = e_code.offset_from(s_code) as libc::c_long as libc::c_short;
    if *sp_sw as libc::c_int != 0 && (*parser_state_tos).p_l_follow == 1 as libc::c_int
        && settings.extra_expression_indent != 0
        && (*((*parser_state_tos).paren_indents).offset(0 as libc::c_int as isize)
            as libc::c_int) < 2 as libc::c_int * settings.ind_size
    {
        *((*parser_state_tos).paren_indents)
            .offset(
                0 as libc::c_int as isize,
            ) = (2 as libc::c_int * settings.ind_size) as libc::c_short;
    }
    (*parser_state_tos).want_blank = 0 as libc::c_int;
    if (*parser_state_tos).in_or_st == 1 as libc::c_int
        && *token as libc::c_int == '(' as i32
    {
        parse_lparen_in_decl();
        (*parser_state_tos).in_or_st = 0 as libc::c_int;
    }
    if *token as libc::c_int == '(' as i32 && settings.break_function_decl_args != 0
        && (*parser_state_tos).in_stmt != 0
        && (*parser_state_tos).in_decl as libc::c_int != 0
        && (*parser_state_tos).paren_depth == 1 as libc::c_int
    {
        dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
        *force_nl = 0 as libc::c_int as BOOLEAN;
        paren_target = (*parser_state_tos).paren_depth * settings.ind_size
            + 1 as libc::c_int;
        *((*parser_state_tos).paren_indents)
            .offset(
                ((*parser_state_tos).p_l_follow - 1 as libc::c_int) as isize,
            ) = -paren_target as libc::c_short;
    }
    if (*parser_state_tos).sizeof_keyword != 0 {
        (*parser_state_tos).sizeof_mask
            |= (1 as libc::c_int) << (*parser_state_tos).p_l_follow;
    }
    if (*parser_state_tos).last_token as libc::c_uint
        == decl as libc::c_int as libc::c_uint
        || (*parser_state_tos).last_token as libc::c_uint
            == ident as libc::c_int as libc::c_uint
            && (*parser_state_tos).last_rw as libc::c_uint
                != rw_return as libc::c_int as libc::c_uint
    {
        (*parser_state_tos).noncast_mask
            |= (1 as libc::c_int) << (*parser_state_tos).p_l_follow;
    } else {
        (*parser_state_tos).noncast_mask
            &= !((1 as libc::c_int) << (*parser_state_tos).p_l_follow);
    };
}
unsafe extern "C" fn handle_token_rparen(
    mut force_nl: *mut BOOLEAN,
    mut sp_sw: *mut BOOLEAN,
    mut hd_type: *mut codes_ty,
    mut last_token_ends_sp: *mut BOOLEAN,
    mut file_exit_value: *mut exit_values_ty,
    mut pbreak_line: *mut BOOLEAN,
) {
    let mut tmpchar: [libc::c_char; 2] = [0; 2];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    (*parser_state_tos).paren_depth -= 1;
    (*parser_state_tos).paren_depth;
    if *token as libc::c_int == ')' as i32 && settings.break_function_decl_args_end != 0
        && (*parser_state_tos).in_or_st == 0 && (*parser_state_tos).in_stmt != 0
        && (*parser_state_tos).in_decl as libc::c_int != 0
        && (*parser_state_tos).paren_depth == 0 as libc::c_int
    {
        if s_code != e_code || s_lab != e_lab || s_com != e_com {
            dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
        }
        paren_target = (*parser_state_tos).paren_depth * settings.ind_size;
        *((*parser_state_tos).paren_indents)
            .offset(
                ((*parser_state_tos).p_l_follow - 1 as libc::c_int) as isize,
            ) = paren_target as libc::c_short;
        (*parser_state_tos).ind_stmt = 0 as libc::c_int;
    }
    if (*parser_state_tos).cast_mask
        & (1 as libc::c_int) << (*parser_state_tos).p_l_follow
        & !(*parser_state_tos).sizeof_mask != 0
    {
        (*parser_state_tos).last_u_d = 1 as libc::c_int;
        (*parser_state_tos).cast_mask
            &= ((1 as libc::c_int) << (*parser_state_tos).p_l_follow) - 1 as libc::c_int;
        if (*parser_state_tos).cast_mask == 0 && settings.cast_space != 0 {
            (*parser_state_tos).want_blank = 1 as libc::c_int;
        } else {
            (*parser_state_tos).want_blank = 0 as libc::c_int;
            (*parser_state_tos).can_break = bb_cast;
        }
        tmp = token.offset(1 as libc::c_int as isize);
        while *(*__ctype_b_loc()).offset(*tmp as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            tmp = tmp.offset(1);
            tmp;
        }
        if *tmp as libc::c_int == '{' as i32 {
            (*parser_state_tos).block_init = 3 as libc::c_int;
        }
    } else if (*parser_state_tos).in_decl as libc::c_int != 0
        && (*parser_state_tos).block_init == 0
        && (*parser_state_tos).paren_depth == 0 as libc::c_int
    {
        (*parser_state_tos).want_blank = 1 as libc::c_int;
    }
    (*parser_state_tos).sizeof_mask
        &= ((1 as libc::c_int) << (*parser_state_tos).p_l_follow) - 1 as libc::c_int;
    (*parser_state_tos).p_l_follow -= 1;
    if (*parser_state_tos).p_l_follow < 0 as libc::c_int {
        (*parser_state_tos).p_l_follow = 0 as libc::c_int;
        tmpchar[0 as libc::c_int as usize] = *token;
        tmpchar[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        message(
            dcgettext(
                0 as *const libc::c_char,
                b"Warning\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            dcgettext(
                0 as *const libc::c_char,
                b"Extra %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            tmpchar.as_mut_ptr(),
            0 as *mut libc::c_void as *mut libc::c_char,
        );
    }
    if e_code == s_code {
        let mut level: libc::c_int = (*parser_state_tos).p_l_follow;
        (*parser_state_tos).paren_level = level;
        if level > 0 as libc::c_int {
            paren_target = -(*((*parser_state_tos).paren_indents)
                .offset((level - 1 as libc::c_int) as isize) as libc::c_int);
        } else {
            paren_target = 0 as libc::c_int;
        }
    }
    if settings.parentheses_space != 0 && *token as libc::c_int != ']' as i32 {
        let fresh8 = e_code;
        e_code = e_code.offset(1);
        *fresh8 = ' ' as i32 as libc::c_char;
    }
    let fresh9 = e_code;
    e_code = e_code.offset(1);
    *fresh9 = *token.offset(0 as libc::c_int as isize);
    if settings.allow_single_line_conditionals != 0
        && *token.offset(-(1 as libc::c_int as isize)) as libc::c_int == ')' as i32
        && *token.offset(2 as libc::c_int as isize) as libc::c_int != '{' as i32
        && (*parser_state_tos).paren_depth == 0
    {
        (*parser_state_tos).want_blank = 1 as libc::c_int;
    }
    if *sp_sw as libc::c_int != 0 && (*parser_state_tos).p_l_follow == 0 as libc::c_int {
        if *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
            as libc::c_uint != dohead as libc::c_int as libc::c_uint
        {
            *last_token_ends_sp = 2 as libc::c_int as BOOLEAN;
        }
        *sp_sw = 0 as libc::c_int as BOOLEAN;
        *force_nl = (settings.allow_single_line_conditionals == 0) as libc::c_int
            as BOOLEAN;
        (*parser_state_tos).last_u_d = 1 as libc::c_int;
        (*parser_state_tos).in_stmt = 0 as libc::c_int;
        if parse(*hd_type) as libc::c_uint
            != total_success as libc::c_int as libc::c_uint
        {
            *file_exit_value = indent_error;
        }
    }
    (*parser_state_tos).search_brace = settings.btype_2;
}
unsafe extern "C" fn handle_token_unary_op(
    mut dec_ind: *mut libc::c_int,
    can_break: bb_code_ty,
) {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*parser_state_tos).want_blank != 0
        && !((*parser_state_tos).in_decl as libc::c_int != 0
            && settings.pointer_align_right == 0 && *token as libc::c_int == '*' as i32)
    {
        set_buf_break(bb_unary_op, paren_target);
        let fresh10 = e_code;
        e_code = e_code.offset(1);
        *fresh10 = ' ' as i32 as libc::c_char;
        *e_code = '\0' as i32 as libc::c_char;
        (*parser_state_tos).want_blank = 0 as libc::c_int;
    } else if can_break as u64 != 0 {
        set_buf_break(can_break, paren_target);
    }
    let mut res: *mut libc::c_char = token;
    let mut res_end: *mut libc::c_char = token_end;
    if (*parser_state_tos).paren_depth == 0 as libc::c_int
        && (*parser_state_tos).in_decl as libc::c_int != 0 && buf_break_used == 0
        && (*parser_state_tos).block_init == 0
    {
        while (e_code.offset_from(s_code) as libc::c_long)
            < *dec_ind as libc::c_long - token_end.offset_from(token) as libc::c_long
        {
            check_code_size();
            set_buf_break(bb_dec_ind, paren_target);
            let fresh11 = e_code;
            e_code = e_code.offset(1);
            *fresh11 = ' ' as i32 as libc::c_char;
        }
        (*parser_state_tos).ind_stmt = 0 as libc::c_int;
    } else if (*parser_state_tos).last_token as libc::c_uint
        == unary_op as libc::c_int as libc::c_uint && e_code > s_code
        && *res as libc::c_int != '!' as i32
        && *e_code.offset(-(1 as libc::c_int as isize)) as libc::c_int
            == *res as libc::c_int
    {
        let fresh12 = e_code;
        e_code = e_code.offset(1);
        *fresh12 = ' ' as i32 as libc::c_char;
    }
    t_ptr = res;
    while t_ptr < res_end {
        check_code_size();
        let fresh13 = e_code;
        e_code = e_code.offset(1);
        *fresh13 = *t_ptr;
        t_ptr = t_ptr.offset(1);
        t_ptr;
    }
    if (*parser_state_tos).want_blank != 0
        && !((*parser_state_tos).in_decl as libc::c_int != 0
            && settings.pointer_align_right != 0 && *token as libc::c_int == '*' as i32)
    {
        set_buf_break(bb_unary_op, paren_target);
        let fresh14 = e_code;
        e_code = e_code.offset(1);
        *fresh14 = ' ' as i32 as libc::c_char;
    }
    *e_code = '\0' as i32 as libc::c_char;
    (*parser_state_tos).want_blank = 0 as libc::c_int;
}
unsafe extern "C" fn handle_token_binary_op(can_break: bb_code_ty) {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*parser_state_tos).want_blank != 0
        || e_code > s_code && *e_code as libc::c_int != ' ' as i32)
        && !((*parser_state_tos).in_parameter_declaration != 0
            && settings.pointer_align_right == 0 && *token as libc::c_int == '*' as i32)
    {
        set_buf_break(bb_binary_op, paren_target);
        let fresh15 = e_code;
        e_code = e_code.offset(1);
        *fresh15 = ' ' as i32 as libc::c_char;
        *e_code = '\0' as i32 as libc::c_char;
    } else if can_break as u64 != 0 {
        set_buf_break(can_break, paren_target);
    }
    let mut res: *mut libc::c_char = token;
    let mut res_end: *mut libc::c_char = token_end;
    t_ptr = res;
    while t_ptr < res_end {
        check_code_size();
        let fresh16 = e_code;
        e_code = e_code.offset(1);
        *fresh16 = *t_ptr;
        t_ptr = t_ptr.offset(1);
        t_ptr;
    }
    if *token as libc::c_int == '=' as i32 {
        (*parser_state_tos).in_decl = 0 as libc::c_int as BOOLEAN;
    }
    (*parser_state_tos)
        .want_blank = !((*parser_state_tos).in_parameter_declaration != 0
        && settings.pointer_align_right != 0 && *token as libc::c_int == '*' as i32)
        as libc::c_int;
}
unsafe extern "C" fn handle_token_postop() {
    let fresh17 = e_code;
    e_code = e_code.offset(1);
    *fresh17 = *token.offset(0 as libc::c_int as isize);
    let fresh18 = e_code;
    e_code = e_code.offset(1);
    *fresh18 = *token.offset(1 as libc::c_int as isize);
    (*parser_state_tos).want_blank = 1 as libc::c_int;
}
unsafe extern "C" fn handle_token_question(can_break: bb_code_ty) {
    squest += 1;
    squest;
    if (*parser_state_tos).want_blank != 0 {
        set_buf_break(bb_question, paren_target);
        let fresh19 = e_code;
        e_code = e_code.offset(1);
        *fresh19 = ' ' as i32 as libc::c_char;
    } else if can_break as u64 != 0 {
        set_buf_break(can_break, paren_target);
    }
    let fresh20 = e_code;
    e_code = e_code.offset(1);
    *fresh20 = '?' as i32 as libc::c_char;
    (*parser_state_tos).want_blank = 1 as libc::c_int;
    *e_code = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn handle_token_casestmt(
    mut scase: *mut BOOLEAN,
    mut file_exit_value: *mut exit_values_ty,
) {
    *scase = 1 as libc::c_int as BOOLEAN;
    if parse(casestmt) as libc::c_uint != total_success as libc::c_int as libc::c_uint {
        *file_exit_value = indent_error;
    }
}
unsafe extern "C" fn handle_token_colon(
    mut scase: *mut BOOLEAN,
    mut force_nl: *mut BOOLEAN,
    mut dec_ind: *mut libc::c_int,
    can_break: bb_code_ty,
    mut pbreak_line: *mut BOOLEAN,
) {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if squest > 0 as libc::c_int {
        squest -= 1;
        squest;
        if (*parser_state_tos).want_blank != 0 {
            set_buf_break(bb_colon, paren_target);
            let fresh21 = e_code;
            e_code = e_code.offset(1);
            *fresh21 = ' ' as i32 as libc::c_char;
        } else if can_break as u64 != 0 {
            set_buf_break(can_break, paren_target);
        }
        let fresh22 = e_code;
        e_code = e_code.offset(1);
        *fresh22 = ':' as i32 as libc::c_char;
        *e_code = '\0' as i32 as libc::c_char;
        (*parser_state_tos).want_blank = 1 as libc::c_int;
    } else {
        if (*parser_state_tos).in_decl != 0 {
            if !(e_code.offset_from(s_code) as libc::c_long
                > 6 as libc::c_int as libc::c_long
                && strncmp(
                    &mut *buf_ptr.offset(-(8 as libc::c_int) as isize),
                    b"private:\0" as *const u8 as *const libc::c_char,
                    8 as libc::c_int as libc::c_ulong,
                ) == 0
                && *(*__ctype_b_loc()).offset(*buf_ptr as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0)
                && !(e_code.offset_from(s_code) as libc::c_long
                    > 8 as libc::c_int as libc::c_long
                    && strncmp(
                        &mut *buf_ptr.offset(-(10 as libc::c_int) as isize),
                        b"protected:\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as libc::c_ulong,
                    ) == 0
                    && *(*__ctype_b_loc()).offset(*buf_ptr as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0)
                && !(e_code.offset_from(s_code) as libc::c_long
                    > 5 as libc::c_int as libc::c_long
                    && strncmp(
                        &mut *buf_ptr.offset(-(7 as libc::c_int) as isize),
                        b"public:\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0
                    && *(*__ctype_b_loc()).offset(*buf_ptr as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0)
            {
                let fresh23 = e_code;
                e_code = e_code.offset(1);
                *fresh23 = ':' as i32 as libc::c_char;
                (*parser_state_tos).want_blank = 0 as libc::c_int;
                return;
            } else if *s_code as libc::c_int == ' ' as i32 {
                let mut p1: *mut libc::c_char = s_code;
                let mut p2: *mut libc::c_char = s_code.offset(*dec_ind as isize);
                while p2 < e_code {
                    let fresh24 = p2;
                    p2 = p2.offset(1);
                    let fresh25 = p1;
                    p1 = p1.offset(1);
                    *fresh25 = *fresh24;
                }
                e_code = e_code.offset(-(*dec_ind as isize));
                *e_code = '\0' as i32 as libc::c_char;
            }
        }
        (*parser_state_tos).in_stmt = 0 as libc::c_int;
        t_ptr = s_code;
        while *t_ptr != 0 {
            check_lab_size();
            let fresh26 = e_lab;
            e_lab = e_lab.offset(1);
            *fresh26 = *t_ptr;
            t_ptr = t_ptr.offset(1);
            t_ptr;
        }
        e_code = s_code;
        clear_buf_break_list(pbreak_line);
        let fresh27 = e_lab;
        e_lab = e_lab.offset(1);
        *fresh27 = ':' as i32 as libc::c_char;
        set_buf_break(bb_label, paren_target);
        let fresh28 = e_lab;
        e_lab = e_lab.offset(1);
        *fresh28 = ' ' as i32 as libc::c_char;
        *e_lab = '\0' as i32 as libc::c_char;
        (*parser_state_tos).pcase = *scase as libc::c_int;
        *force_nl = (*parser_state_tos).pcase as BOOLEAN;
        *scase = 0 as libc::c_int as BOOLEAN;
        (*parser_state_tos).want_blank = 0 as libc::c_int;
    };
}
unsafe extern "C" fn handle_token_doublecolon() {
    let fresh29 = e_code;
    e_code = e_code.offset(1);
    *fresh29 = ':' as i32 as libc::c_char;
    let fresh30 = e_code;
    e_code = e_code.offset(1);
    *fresh30 = ':' as i32 as libc::c_char;
    (*parser_state_tos).want_blank = 0 as libc::c_int;
    (*parser_state_tos).can_break = bb_doublecolon;
    (*parser_state_tos).last_u_d = 1 as libc::c_int;
    (*parser_state_tos).saw_double_colon = 1 as libc::c_int;
}
unsafe extern "C" fn handle_token_semicolon(
    mut scase: *mut BOOLEAN,
    mut force_nl: *mut BOOLEAN,
    mut sp_sw: *mut BOOLEAN,
    mut dec_ind: *mut libc::c_int,
    mut last_token_ends_sp: *mut BOOLEAN,
    mut file_exit_value: *mut exit_values_ty,
) {
    (*parser_state_tos).in_or_st = 0 as libc::c_int;
    (*parser_state_tos).saw_double_colon = 0 as libc::c_int;
    *scase = 0 as libc::c_int as BOOLEAN;
    squest = 0 as libc::c_int;
    (*parser_state_tos).cast_mask = 0 as libc::c_int;
    (*parser_state_tos).sizeof_mask = 0 as libc::c_int;
    (*parser_state_tos).block_init = 0 as libc::c_int;
    (*parser_state_tos).block_init_level = 0 as libc::c_int;
    (*parser_state_tos).just_saw_decl -= 1;
    (*parser_state_tos).just_saw_decl;
    (*parser_state_tos).is_func_ptr_decl = 0 as libc::c_int;
    if (*parser_state_tos).in_decl as libc::c_int != 0 && s_code == e_code
        && buf_break_used == 0 && (*parser_state_tos).block_init == 0
    {
        while (e_code.offset_from(s_code) as libc::c_long)
            < (*dec_ind - 1 as libc::c_int) as libc::c_long
        {
            check_code_size();
            set_buf_break(bb_dec_ind, paren_target);
            let fresh31 = e_code;
            e_code = e_code.offset(1);
            *fresh31 = ' ' as i32 as libc::c_char;
        }
        (*parser_state_tos).ind_stmt = 0 as libc::c_int;
    }
    *e_code = '\0' as i32 as libc::c_char;
    (*parser_state_tos)
        .in_decl = (if (*parser_state_tos).dec_nest > 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as BOOLEAN;
    if *last_token_ends_sp as libc::c_int != 0 && settings.space_sp_semicolon != 0 {
        let fresh32 = e_code;
        e_code = e_code.offset(1);
        *fresh32 = ' ' as i32 as libc::c_char;
    }
    let fresh33 = e_code;
    e_code = e_code.offset(1);
    *fresh33 = ';' as i32 as libc::c_char;
    *e_code = '\0' as i32 as libc::c_char;
    (*parser_state_tos).want_blank = 1 as libc::c_int;
    (*parser_state_tos)
        .in_stmt = ((*parser_state_tos).p_l_follow > 0 as libc::c_int) as libc::c_int;
    if *sp_sw == 0 {
        if parse(semicolon) as libc::c_uint
            != total_success as libc::c_int as libc::c_uint
        {
            *file_exit_value = indent_error;
        }
        *force_nl = 1 as libc::c_int as BOOLEAN;
    }
}
unsafe extern "C" fn handle_token_lbrace(
    mut force_nl: *mut BOOLEAN,
    mut dec_ind: *mut libc::c_int,
    mut file_exit_value: *mut exit_values_ty,
    mut pbreak_line: *mut BOOLEAN,
) {
    (*parser_state_tos).saw_double_colon = 0 as libc::c_int;
    if (*parser_state_tos).last_token as libc::c_uint
        == binary_op as libc::c_int as libc::c_uint
    {
        (*parser_state_tos).block_init = 1 as libc::c_int;
    }
    if (*parser_state_tos).block_init == 0 {
        *force_nl = 1 as libc::c_int as BOOLEAN;
        (*parser_state_tos).in_stmt = 0 as libc::c_int;
    } else {
        let mut p: *mut libc::c_char = buf_ptr;
        loop {
            p = skip_horiz_space(p);
            if *p as libc::c_int == '\n' as i32
                || *p as libc::c_int == '/' as i32
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                (*parser_state_tos).in_stmt = 0 as libc::c_int;
                break;
            } else {
                if !(*p as libc::c_int == '/' as i32
                    && *p.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32)
                {
                    break;
                }
                p = p.offset(2 as libc::c_int as isize);
                while *p as libc::c_int != 0 && *p as libc::c_int != '\n' as i32
                    && (*p as libc::c_int != '*' as i32
                        || *p.offset(1 as libc::c_int as isize) as libc::c_int
                            != '/' as i32)
                {
                    p = p.offset(1);
                    p;
                }
                if *p == 0 || *p as libc::c_int == '\n' as i32 {
                    (*parser_state_tos).in_stmt = 0 as libc::c_int;
                    break;
                } else {
                    p = p.offset(2 as libc::c_int as isize);
                    if *p == 0 {
                        break;
                    }
                }
            }
        }
        if (*parser_state_tos).block_init_level <= 0 as libc::c_int {
            (*parser_state_tos).block_init_level = 1 as libc::c_int;
        } else {
            (*parser_state_tos).block_init_level += 1;
            (*parser_state_tos).block_init_level;
        }
    }
    if s_code != e_code && (*parser_state_tos).block_init != 1 as libc::c_int {
        if (*parser_state_tos).in_decl == 0 && settings.btype_2 == 0
            || (*parser_state_tos).in_decl as libc::c_int != 0
                && settings.braces_on_struct_decl_line == 0
                && settings.braces_on_func_def_line == 0
        {
            dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
            (*parser_state_tos).want_blank = 0 as libc::c_int;
        } else if (*parser_state_tos).in_parameter_declaration != 0
            && (*parser_state_tos).in_or_st == 0
        {
            (*parser_state_tos).i_l_follow = 0 as libc::c_int;
            if settings.braces_on_func_def_line == 0 {
                dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
            } else {
                let fresh34 = e_code;
                e_code = e_code.offset(1);
                *fresh34 = ' ' as i32 as libc::c_char;
            }
            (*parser_state_tos).want_blank = 0 as libc::c_int;
        } else {
            (*parser_state_tos).want_blank = 1 as libc::c_int;
        }
    }
    if (*parser_state_tos).in_parameter_declaration != 0 {
        prefix_blankline_requested = 0 as libc::c_int;
    }
    if s_code == e_code {
        (*parser_state_tos).ind_stmt = 0 as libc::c_int;
    }
    if (*parser_state_tos).in_decl as libc::c_int != 0
        && (*parser_state_tos).in_or_st != 0
    {
        if (*parser_state_tos).dec_nest >= di_stack_alloc {
            di_stack_alloc *= 2 as libc::c_int;
            di_stack = xrealloc(
                di_stack as *mut libc::c_void,
                (di_stack_alloc as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    as libc::c_uint,
            ) as *mut libc::c_int;
        }
        let fresh35 = (*parser_state_tos).dec_nest;
        (*parser_state_tos).dec_nest = (*parser_state_tos).dec_nest + 1;
        *di_stack.offset(fresh35 as isize) = *dec_ind;
    } else {
        (*parser_state_tos).in_decl = 0 as libc::c_int as BOOLEAN;
        (*parser_state_tos).decl_on_line = 0 as libc::c_int;
        (*parser_state_tos).in_parameter_declaration = 0 as libc::c_int;
    }
    *dec_ind = 0 as libc::c_int;
    (*parser_state_tos).in_or_st = 0 as libc::c_int;
    if parse(lbrace) as libc::c_uint != total_success as libc::c_int as libc::c_uint {
        *file_exit_value = indent_error;
    }
    set_buf_break(bb_lbrace, paren_target);
    if (*parser_state_tos).want_blank != 0 && s_code != e_code {
        let fresh36 = e_code;
        e_code = e_code.offset(1);
        *fresh36 = ' ' as i32 as libc::c_char;
    }
    (*parser_state_tos).want_blank = 0 as libc::c_int;
    let fresh37 = e_code;
    e_code = e_code.offset(1);
    *fresh37 = '{' as i32 as libc::c_char;
    *e_code = '\0' as i32 as libc::c_char;
    (*parser_state_tos).just_saw_decl = 0 as libc::c_int;
    if (*parser_state_tos).block_init != 0
        && (*parser_state_tos).block_init_level >= 2 as libc::c_int
    {
        (*parser_state_tos).p_l_follow += 1;
        if (*parser_state_tos).p_l_follow >= (*parser_state_tos).paren_indents_size {
            (*parser_state_tos).paren_indents_size *= 2 as libc::c_int;
            (*parser_state_tos)
                .paren_indents = xrealloc(
                (*parser_state_tos).paren_indents as *mut libc::c_void,
                ((*parser_state_tos).paren_indents_size as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ) as libc::c_uint,
            ) as *mut libc::c_short;
        }
        (*parser_state_tos).paren_depth += 1;
        (*parser_state_tos).paren_depth;
        *((*parser_state_tos).paren_indents)
            .offset(
                ((*parser_state_tos).p_l_follow - 1 as libc::c_int) as isize,
            ) = e_code.offset_from(s_code) as libc::c_long as libc::c_short;
        if settings.spaces_around_initializers != 0 {
            (*parser_state_tos).want_blank = 1 as libc::c_int;
        }
    } else if (*parser_state_tos).block_init != 0
        && (*parser_state_tos).block_init_level == 1 as libc::c_int
    {
        (*parser_state_tos).want_blank = 1 as libc::c_int;
    }
}
unsafe extern "C" fn handle_token_rbrace(
    mut force_nl: *mut BOOLEAN,
    mut dec_ind: *mut libc::c_int,
    mut file_exit_value: *mut exit_values_ty,
    mut pbreak_line: *mut BOOLEAN,
) {
    let mut tmpchar: [libc::c_char; 2] = [0; 2];
    if *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
        as libc::c_uint == decl as libc::c_int as libc::c_uint
        && (*parser_state_tos).block_init == 0
        || *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
            as libc::c_uint == casestmt as libc::c_int as libc::c_uint
    {
        if parse(semicolon) as libc::c_uint
            != total_success as libc::c_int as libc::c_uint
        {
            *file_exit_value = indent_error;
        }
    }
    (*parser_state_tos).just_saw_decl = 0 as libc::c_int;
    (*parser_state_tos).ind_stmt = 0 as libc::c_int;
    (*parser_state_tos).in_stmt = 0 as libc::c_int;
    (*parser_state_tos).block_init_level -= 1;
    (*parser_state_tos).block_init_level;
    if (*parser_state_tos).block_init_level == 0 as libc::c_int && s_code != e_code {
        if (*parser_state_tos).matching_brace_on_same_line < 0 as libc::c_int {
            dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
        } else {
            set_buf_break(bb_rbrace, paren_target);
            let fresh38 = e_code;
            e_code = e_code.offset(1);
            *fresh38 = ' ' as i32 as libc::c_char;
        }
    } else if (*parser_state_tos).block_init_level == 1 as libc::c_int
        && settings.spaces_around_initializers != 0
    {
        set_buf_break(bb_rbrace, paren_target);
        let fresh39 = e_code;
        e_code = e_code.offset(1);
        *fresh39 = ' ' as i32 as libc::c_char;
    }
    let fresh40 = e_code;
    e_code = e_code.offset(1);
    *fresh40 = '}' as i32 as libc::c_char;
    (*parser_state_tos).want_blank = 1 as libc::c_int;
    if (*parser_state_tos).block_init != 0
        && (*parser_state_tos).block_init_level > 0 as libc::c_int
    {
        (*parser_state_tos).paren_depth -= 1;
        (*parser_state_tos).paren_depth;
        (*parser_state_tos).p_l_follow -= 1;
        if (*parser_state_tos).p_l_follow < 0 as libc::c_int {
            (*parser_state_tos).p_l_follow = 0 as libc::c_int;
            tmpchar[0 as libc::c_int as usize] = *token;
            tmpchar[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            message(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Warning\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Extra %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                tmpchar.as_mut_ptr(),
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        }
    } else if (*parser_state_tos).dec_nest > 0 as libc::c_int {
        (*parser_state_tos).dec_nest -= 1;
        *dec_ind = *di_stack.offset((*parser_state_tos).dec_nest as isize);
        if (*parser_state_tos).dec_nest == 0 as libc::c_int
            && (*parser_state_tos).in_parameter_declaration == 0
        {
            (*parser_state_tos).just_saw_decl = 2 as libc::c_int;
        }
        (*parser_state_tos).in_decl = 1 as libc::c_int as BOOLEAN;
    }
    prefix_blankline_requested = 0 as libc::c_int;
    if parse(rbrace) as libc::c_uint != total_success as libc::c_int as libc::c_uint {
        *file_exit_value = indent_error;
    }
    (*parser_state_tos)
        .search_brace = (settings.cuddle_else != 0
        && *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
            as libc::c_uint == ifhead as libc::c_int as libc::c_uint
        || settings.cuddle_do_while != 0
            && *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
                as libc::c_uint == dohead as libc::c_int as libc::c_uint) as libc::c_int;
    if *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
        as libc::c_uint == stmtl as libc::c_int as libc::c_uint
    {
        if (*parser_state_tos).last_rw as libc::c_uint
            != rw_struct_like as libc::c_int as libc::c_uint
            && (*parser_state_tos).last_rw as libc::c_uint
                != rw_enum as libc::c_int as libc::c_uint
            && (*parser_state_tos).last_rw as libc::c_uint
                != rw_decl as libc::c_int as libc::c_uint
        {
            *force_nl = 1 as libc::c_int as BOOLEAN;
        }
    }
    if *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
        as libc::c_uint == ifhead as libc::c_int as libc::c_uint
        || *((*parser_state_tos).p_stack).offset((*parser_state_tos).tos as isize)
            as libc::c_uint == dohead as libc::c_int as libc::c_uint
            && settings.cuddle_do_while == 0 && settings.btype_2 == 0
    {
        *force_nl = 1 as libc::c_int as BOOLEAN;
    }
    if (*parser_state_tos).in_decl == 0 && (*parser_state_tos).tos <= 0 as libc::c_int
        && settings.blanklines_after_procs != 0
        && (*parser_state_tos).dec_nest <= 0 as libc::c_int
    {
        postfix_blankline_requested = 1 as libc::c_int;
        postfix_blankline_requested_code = (if (*parser_state_tos).in_decl as libc::c_int
            != 0
        {
            decl as libc::c_int
        } else {
            rbrace as libc::c_int
        }) as codes_ty;
    }
}
unsafe extern "C" fn handle_token_swstmt(
    mut sp_sw: *mut BOOLEAN,
    mut hd_type: *mut codes_ty,
) {
    *sp_sw = 1 as libc::c_int as BOOLEAN;
    *hd_type = swstmt;
    (*parser_state_tos).in_decl = 0 as libc::c_int as BOOLEAN;
}
unsafe extern "C" fn handle_token_sp_paren(
    mut sp_sw: *mut BOOLEAN,
    mut hd_type: *mut codes_ty,
) {
    *sp_sw = 1 as libc::c_int as BOOLEAN;
    *hd_type = (if *token as libc::c_int == 'i' as i32 {
        ifstmt as libc::c_int
    } else if *token as libc::c_int == 'w' as i32 {
        whilestmt as libc::c_int
    } else {
        forstmt as libc::c_int
    }) as codes_ty;
}
unsafe extern "C" fn handle_token_nparen(
    mut force_nl: *mut BOOLEAN,
    mut file_exit_value: *mut exit_values_ty,
    mut last_else: *mut BOOLEAN,
    mut pbreak_line: *mut BOOLEAN,
) {
    (*parser_state_tos).in_stmt = 0 as libc::c_int;
    if *token as libc::c_int == 'e' as i32 {
        if e_code != s_code
            && (settings.cuddle_else == 0
                || *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    != '}' as i32)
        {
            if settings.verbose != 0 {
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
            dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
            (*parser_state_tos).want_blank = 0 as libc::c_int;
        }
        *force_nl = 1 as libc::c_int as BOOLEAN;
        *last_else = 1 as libc::c_int as BOOLEAN;
        if parse(elselit) as libc::c_uint != total_success as libc::c_int as libc::c_uint
        {
            *file_exit_value = indent_error;
        }
    } else {
        if e_code != s_code {
            if settings.verbose != 0 {
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
            dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
            (*parser_state_tos).want_blank = 0 as libc::c_int;
        }
        *force_nl = 1 as libc::c_int as BOOLEAN;
        *last_else = 0 as libc::c_int as BOOLEAN;
        if parse(dolit) as libc::c_uint != total_success as libc::c_int as libc::c_uint {
            *file_exit_value = indent_error;
        }
    };
}
unsafe extern "C" fn handle_token_overloaded(can_break: bb_code_ty) {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*parser_state_tos).want_blank != 0 {
        set_buf_break(bb_overloaded, paren_target);
        let fresh41 = e_code;
        e_code = e_code.offset(1);
        *fresh41 = ' ' as i32 as libc::c_char;
    } else if can_break as u64 != 0 {
        set_buf_break(can_break, paren_target);
    }
    (*parser_state_tos).want_blank = 1 as libc::c_int;
    t_ptr = token;
    while t_ptr < token_end {
        check_code_size();
        let fresh42 = e_code;
        e_code = e_code.offset(1);
        *fresh42 = *t_ptr;
        t_ptr = t_ptr.offset(1);
        t_ptr;
    }
    *e_code = '\0' as i32 as libc::c_char;
}
unsafe extern "C" fn handle_token_decl(
    mut dec_ind: *mut libc::c_int,
    mut file_exit_value: *mut exit_values_ty,
    mut pbreak_line: *mut BOOLEAN,
) {
    if (*parser_state_tos).last_token as libc::c_uint
        == rparen as libc::c_int as libc::c_uint
        && (*parser_state_tos).in_parameter_declaration != 0
        && (*parser_state_tos).saw_double_colon != 0
        && strncmp(
            token,
            b"const\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        set_buf_break(bb_const_qualifier, paren_target);
    } else {
        if (*parser_state_tos).sizeof_mask == 0 {
            if parse(decl) as libc::c_uint
                != total_success as libc::c_int as libc::c_uint
            {
                *file_exit_value = indent_error;
            }
        }
        if (*parser_state_tos).last_token as libc::c_uint
            == rparen as libc::c_int as libc::c_uint
            && (*parser_state_tos).tos <= 1 as libc::c_int
        {
            (*parser_state_tos).in_parameter_declaration = 1 as libc::c_int;
            if s_code != e_code {
                dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
                (*parser_state_tos).want_blank = 0 as libc::c_int;
            }
        }
        if (*parser_state_tos).in_parameter_declaration != 0
            && (*parser_state_tos).dec_nest == 0 as libc::c_int
            && (*parser_state_tos).p_l_follow == 0 as libc::c_int
        {
            (*parser_state_tos).i_l_follow = settings.indent_parameters;
            (*parser_state_tos).ind_level = (*parser_state_tos).i_l_follow;
            (*parser_state_tos).ind_stmt = 0 as libc::c_int;
        }
        if (*parser_state_tos).paren_depth == 0 {
            (*parser_state_tos).in_or_st = 1 as libc::c_int;
        }
        if (*parser_state_tos).sizeof_mask == 0 {
            (*parser_state_tos).in_decl = 1 as libc::c_int as BOOLEAN;
            (*parser_state_tos).decl_on_line = 1 as libc::c_int;
            if (*parser_state_tos).dec_nest <= 0 as libc::c_int {
                (*parser_state_tos).just_saw_decl = 2 as libc::c_int;
            }
        }
        *dec_ind = (if settings.decl_indent > 0 as libc::c_int {
            settings.decl_indent as libc::c_long
        } else {
            token_end.offset_from(token) as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as libc::c_int;
    };
}
unsafe extern "C" fn handle_token_ident(
    mut force_nl: *mut BOOLEAN,
    mut sp_sw: *mut BOOLEAN,
    mut hd_type: *mut codes_ty,
    mut dec_ind: *mut libc::c_int,
    mut file_exit_value: *mut exit_values_ty,
    can_break: bb_code_ty,
    mut is_procname_definition: BOOLEAN,
    mut pbreak_line: *mut BOOLEAN,
) {
    if (*parser_state_tos).in_decl as libc::c_int != 0
        && (*parser_state_tos).p_l_follow == 0 as libc::c_int
        && (*parser_state_tos).last_token as libc::c_uint
            != rbrace as libc::c_int as libc::c_uint
    {
        if (*parser_state_tos).want_blank != 0 {
            set_buf_break(bb_ident, paren_target);
            let fresh43 = e_code;
            e_code = e_code.offset(1);
            *fresh43 = ' ' as i32 as libc::c_char;
            *e_code = '\0' as i32 as libc::c_char;
        } else if can_break as u64 != 0 {
            set_buf_break(can_break, paren_target);
        }
        (*parser_state_tos).want_blank = 0 as libc::c_int;
        if is_procname_definition as libc::c_int == 0 as libc::c_int
            || settings.procnames_start_line == 0 && s_code != e_code
        {
            if (*parser_state_tos).block_init == 0 && buf_break_used == 0 {
                if is_procname_definition != 0 {
                    *dec_ind = 0 as libc::c_int;
                }
                while (e_code.offset_from(s_code) as libc::c_long)
                    < *dec_ind as libc::c_long
                {
                    check_code_size();
                    set_buf_break(bb_dec_ind, paren_target);
                    let fresh44 = e_code;
                    e_code = e_code.offset(1);
                    *fresh44 = ' ' as i32 as libc::c_char;
                }
                *e_code = '\0' as i32 as libc::c_char;
                (*parser_state_tos).ind_stmt = 0 as libc::c_int;
            }
        } else {
            if s_code != e_code
                && (*parser_state_tos).last_token as libc::c_uint
                    != doublecolon as libc::c_int as libc::c_uint
            {
                dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
            }
            *dec_ind = 0 as libc::c_int;
            (*parser_state_tos).want_blank = 0 as libc::c_int;
        }
    } else if *sp_sw as libc::c_int != 0
        && (*parser_state_tos).p_l_follow == 0 as libc::c_int
    {
        *sp_sw = 0 as libc::c_int as BOOLEAN;
        *force_nl = 1 as libc::c_int as BOOLEAN;
        (*parser_state_tos).last_u_d = 1 as libc::c_int;
        (*parser_state_tos).in_stmt = 0 as libc::c_int;
        if parse(*hd_type) as libc::c_uint
            != total_success as libc::c_int as libc::c_uint
        {
            *file_exit_value = indent_error;
        }
    }
}
unsafe extern "C" fn handle_token_struct_delim() {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    t_ptr = token;
    while t_ptr < token_end {
        check_code_size();
        let fresh45 = e_code;
        e_code = e_code.offset(1);
        *fresh45 = *t_ptr;
        t_ptr = t_ptr.offset(1);
        t_ptr;
    }
    (*parser_state_tos).want_blank = 0 as libc::c_int;
    (*parser_state_tos).can_break = bb_struct_delim;
}
unsafe extern "C" fn handle_token_comma(
    mut force_nl: *mut BOOLEAN,
    mut dec_ind: *mut libc::c_int,
    mut is_procname_definition: BOOLEAN,
) {
    (*parser_state_tos).want_blank = 1 as libc::c_int;
    if (*parser_state_tos).paren_depth == 0 as libc::c_int
        && (*parser_state_tos).in_decl as libc::c_int != 0 && buf_break_used == 0
        && is_procname_definition as libc::c_int == 0 as libc::c_int
        && (*parser_state_tos).block_init == 0
    {
        while (e_code.offset_from(s_code) as libc::c_long)
            < (*dec_ind - 1 as libc::c_int) as libc::c_long
        {
            check_code_size();
            set_buf_break(bb_dec_ind, paren_target);
            let fresh46 = e_code;
            e_code = e_code.offset(1);
            *fresh46 = ' ' as i32 as libc::c_char;
        }
        (*parser_state_tos).ind_stmt = 0 as libc::c_int;
    }
    let fresh47 = e_code;
    e_code = e_code.offset(1);
    *fresh47 = ',' as i32 as libc::c_char;
    if (*parser_state_tos).p_l_follow == 0 as libc::c_int {
        if (*parser_state_tos).block_init_level <= 0 as libc::c_int {
            (*parser_state_tos).block_init = 0 as libc::c_int;
        }
        if break_comma != 0 && settings.leave_comma == 0 {
            *force_nl = 1 as libc::c_int as BOOLEAN;
        }
    }
    if (*parser_state_tos).block_init != 0 {
        (*parser_state_tos).in_stmt = 0 as libc::c_int;
    }
    if settings.break_function_decl_args != 0
        && ((*parser_state_tos).in_or_st == 0 && (*parser_state_tos).in_stmt != 0
            && (*parser_state_tos).in_decl as libc::c_int != 0)
    {
        *force_nl = 1 as libc::c_int as BOOLEAN;
    }
}
unsafe extern "C" fn handle_token_preesc(
    mut file_exit_value: *mut exit_values_ty,
    mut pbreak_line: *mut BOOLEAN,
) {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if s_com != e_com || s_lab != e_lab || s_code != e_code {
        dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
    }
    let mut in_comment: libc::c_int = 0 as libc::c_int;
    let mut in_cplus_comment: libc::c_int = 0 as libc::c_int;
    let mut com_start: libc::c_int = 0 as libc::c_int;
    let mut quote: libc::c_char = 0 as libc::c_int as libc::c_char;
    let mut com_end: libc::c_int = 0 as libc::c_int;
    t_ptr = token;
    while t_ptr < token_end {
        check_lab_size();
        let fresh48 = e_lab;
        e_lab = e_lab.offset(1);
        *fresh48 = *t_ptr;
        t_ptr = t_ptr.offset(1);
        t_ptr;
    }
    while had_eof == 0 && (*buf_ptr as libc::c_int != '\n' as i32 || in_comment != 0) {
        check_lab_size();
        let fresh49 = buf_ptr;
        buf_ptr = buf_ptr.offset(1);
        *e_lab = *fresh49;
        if buf_ptr >= buf_end {
            fill_buffer();
        }
        let fresh50 = e_lab;
        e_lab = e_lab.offset(1);
        match *fresh50 as libc::c_int {
            92 => {
                if in_comment == 0 && in_cplus_comment == 0 {
                    let fresh51 = buf_ptr;
                    buf_ptr = buf_ptr.offset(1);
                    let fresh52 = e_lab;
                    e_lab = e_lab.offset(1);
                    *fresh52 = *fresh51;
                    if buf_ptr >= buf_end {
                        fill_buffer();
                    }
                }
            }
            47 => {
                if (*buf_ptr as libc::c_int == '*' as i32
                    || *buf_ptr as libc::c_int == '/' as i32) && in_comment == 0
                    && in_cplus_comment == 0 && quote == 0
                {
                    save_com.column = current_column() - 1 as libc::c_int;
                    if *buf_ptr as libc::c_int == '/' as i32 {
                        in_cplus_comment = 1 as libc::c_int;
                    } else {
                        in_comment = 1 as libc::c_int;
                    }
                    let fresh53 = buf_ptr;
                    buf_ptr = buf_ptr.offset(1);
                    let fresh54 = e_lab;
                    e_lab = e_lab.offset(1);
                    *fresh54 = *fresh53;
                    com_start = (e_lab.offset_from(s_lab) as libc::c_long
                        - 2 as libc::c_int as libc::c_long) as libc::c_int;
                    if save_com.ptr == save_com.end {
                        save_com.start_column = current_column() - 2 as libc::c_int;
                    }
                }
            }
            34 | 39 => {
                if quote == 0 {
                    quote = *e_lab.offset(-(1 as libc::c_int) as isize);
                } else if *e_lab.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == quote as libc::c_int
                {
                    quote = 0 as libc::c_int as libc::c_char;
                }
            }
            42 => {
                if *buf_ptr as libc::c_int == '/' as i32 && in_comment != 0 {
                    in_comment = 0 as libc::c_int;
                    let fresh55 = buf_ptr;
                    buf_ptr = buf_ptr.offset(1);
                    let fresh56 = e_lab;
                    e_lab = e_lab.offset(1);
                    *fresh56 = *fresh55;
                    com_end = e_lab.offset_from(s_lab) as libc::c_long as libc::c_int;
                }
            }
            _ => {}
        }
    }
    while e_lab > s_lab
        && (*e_lab.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
            || *e_lab.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\t' as i32)
    {
        e_lab = e_lab.offset(-1);
        e_lab;
    }
    if in_cplus_comment != 0 {
        in_cplus_comment = 0 as libc::c_int;
        let fresh57 = buf_ptr;
        buf_ptr = buf_ptr.offset(1);
        let fresh58 = e_lab;
        e_lab = e_lab.offset(1);
        *fresh58 = *fresh57;
        com_end = e_lab.offset_from(s_lab) as libc::c_long as libc::c_int;
    }
    if e_lab.offset_from(s_lab) as libc::c_long == com_end as libc::c_long
        && bp_save.is_null()
    {
        if save_com.end != save_com.ptr {
            need_chars(&mut save_com, 2 as libc::c_int as size_t);
            let fresh59 = save_com.end;
            save_com.end = (save_com.end).offset(1);
            *fresh59 = '\n' as i32 as libc::c_char;
            let fresh60 = save_com.end;
            save_com.end = (save_com.end).offset(1);
            *fresh60 = ' ' as i32 as libc::c_char;
            save_com.len += 2 as libc::c_int;
            line_no -= 1;
            line_no;
        }
        need_chars(&mut save_com, (com_end - com_start + 1 as libc::c_int) as size_t);
        strncpy(
            save_com.end,
            s_lab.offset(com_start as isize),
            (com_end - com_start) as libc::c_ulong,
        );
        *(save_com.end)
            .offset((com_end - com_start) as isize) = '\0' as i32 as libc::c_char;
        save_com.end = (save_com.end).offset((com_end - com_start) as isize);
        save_com.len += com_end - com_start;
        e_lab = s_lab.offset(com_start as isize);
        while e_lab > s_lab
            && (*e_lab.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
                || *e_lab.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '\t' as i32)
        {
            e_lab = e_lab.offset(-1);
            e_lab;
        }
        bp_save = buf_ptr;
        be_save = buf_end;
        need_chars(&mut save_com, 1 as libc::c_int as size_t);
        buf_ptr = save_com.ptr;
        buf_end = save_com.end;
        save_com.end = save_com.ptr;
    }
    *e_lab = '\0' as i32 as libc::c_char;
    (*parser_state_tos).pcase = 0 as libc::c_int;
    p = s_lab.offset(1 as libc::c_int as isize);
    p = skip_horiz_space(p);
    if strncmp(
        p,
        b"if\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        if settings.blanklines_around_conditional_compilation != 0 {
            prefix_blankline_requested += 1;
            prefix_blankline_requested;
            prefix_blankline_requested_code = preesc;
            loop {
                let fresh61 = in_prog_pos;
                in_prog_pos = in_prog_pos.offset(1);
                if !(*fresh61 as libc::c_int == '\n' as i32) {
                    break;
                }
            }
            in_prog_pos = in_prog_pos.offset(-1);
            in_prog_pos;
        }
        let mut new: *mut parser_state_ty = 0 as *mut parser_state_ty;
        new = xmalloc(
            ::std::mem::size_of::<parser_state_ty>() as libc::c_ulong as libc::c_uint,
        ) as *mut parser_state_ty;
        memcpy(
            new as *mut libc::c_void,
            parser_state_tos as *const libc::c_void,
            ::std::mem::size_of::<parser_state_ty>() as libc::c_ulong,
        );
        (*new)
            .p_stack = xmalloc(
            ((*parser_state_tos).p_stack_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<codes_ty>() as libc::c_ulong)
                as libc::c_uint,
        ) as *mut codes_ty;
        memcpy(
            (*new).p_stack as *mut libc::c_void,
            (*parser_state_tos).p_stack as *const libc::c_void,
            ((*parser_state_tos).p_stack_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<codes_ty>() as libc::c_ulong),
        );
        (*new)
            .il = xmalloc(
            ((*parser_state_tos).p_stack_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_uint,
        ) as *mut libc::c_int;
        memcpy(
            (*new).il as *mut libc::c_void,
            (*parser_state_tos).il as *const libc::c_void,
            ((*parser_state_tos).p_stack_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*new)
            .cstk = xmalloc(
            ((*parser_state_tos).p_stack_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_uint,
        ) as *mut libc::c_int;
        memcpy(
            (*new).cstk as *mut libc::c_void,
            (*parser_state_tos).cstk as *const libc::c_void,
            ((*parser_state_tos).p_stack_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*new)
            .paren_indents = xmalloc(
            ((*parser_state_tos).paren_indents_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong)
                as libc::c_uint,
        ) as *mut libc::c_short;
        memcpy(
            (*new).paren_indents as *mut libc::c_void,
            (*parser_state_tos).paren_indents as *const libc::c_void,
            ((*parser_state_tos).paren_indents_size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        (*new).next = parser_state_tos;
        parser_state_tos = new;
    } else if strncmp(
        p,
        b"else\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
        || strncmp(
            p,
            b"elif\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        else_or_endif = (strncmp(
            p,
            b"else\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int) as libc::c_int;
        prefix_blankline_requested = 0 as libc::c_int;
        if !((*parser_state_tos).next).is_null() {
            let mut tos_p_stack: *mut codes_ty = (*parser_state_tos).p_stack;
            let mut tos_il: *mut libc::c_int = (*parser_state_tos).il;
            let mut tos_cstk: *mut libc::c_int = (*parser_state_tos).cstk;
            let mut tos_paren_indents: *mut libc::c_short = (*parser_state_tos)
                .paren_indents;
            let mut second: *mut parser_state_ty = (*parser_state_tos).next;
            memcpy(
                parser_state_tos as *mut libc::c_void,
                second as *const libc::c_void,
                ::std::mem::size_of::<parser_state_ty>() as libc::c_ulong,
            );
            (*parser_state_tos).next = second;
            (*parser_state_tos).p_stack = tos_p_stack;
            memcpy(
                (*parser_state_tos).p_stack as *mut libc::c_void,
                (*(*parser_state_tos).next).p_stack as *const libc::c_void,
                ((*parser_state_tos).p_stack_size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<codes_ty>() as libc::c_ulong),
            );
            (*parser_state_tos).il = tos_il;
            memcpy(
                (*parser_state_tos).il as *mut libc::c_void,
                (*(*parser_state_tos).next).il as *const libc::c_void,
                ((*parser_state_tos).p_stack_size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            (*parser_state_tos).cstk = tos_cstk;
            memcpy(
                (*parser_state_tos).cstk as *mut libc::c_void,
                (*(*parser_state_tos).next).cstk as *const libc::c_void,
                ((*parser_state_tos).p_stack_size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            (*parser_state_tos).paren_indents = tos_paren_indents;
            memcpy(
                (*parser_state_tos).paren_indents as *mut libc::c_void,
                (*(*parser_state_tos).next).paren_indents as *const libc::c_void,
                ((*parser_state_tos).paren_indents_size as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_short>() as libc::c_ulong,
                    ),
            );
        } else {
            message(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                if else_or_endif != 0 {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unmatched #else\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    )
                } else {
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Unmatched #elif\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    )
                },
                0 as *mut libc::c_char,
                0 as *mut libc::c_char,
            );
            *file_exit_value = indent_error;
        }
    } else if strncmp(
        p,
        b"endif\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        else_or_endif = 1 as libc::c_int;
        prefix_blankline_requested = 0 as libc::c_int;
        if !((*parser_state_tos).next).is_null() {
            let mut second_0: *mut parser_state_ty = (*parser_state_tos).next;
            (*parser_state_tos).next = (*second_0).next;
            xfree((*second_0).p_stack as *mut libc::c_void);
            xfree((*second_0).il as *mut libc::c_void);
            xfree((*second_0).cstk as *mut libc::c_void);
            xfree((*second_0).paren_indents as *mut libc::c_void);
            xfree(second_0 as *mut libc::c_void);
        } else {
            message(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Error\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unmatched #endif\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *mut libc::c_char,
                0 as *mut libc::c_char,
            );
            *file_exit_value = indent_error;
        }
        if settings.blanklines_around_conditional_compilation != 0 {
            postfix_blankline_requested += 1;
            postfix_blankline_requested;
            postfix_blankline_requested_code = preesc;
            n_real_blanklines = 0 as libc::c_int;
        }
    }
    if else_or_endif != 0
        && prefix_blankline_requested_code as libc::c_uint
            == decl as libc::c_int as libc::c_uint
    {
        prefix_blankline_requested = 0 as libc::c_int;
    }
    if (*parser_state_tos).last_token as libc::c_uint
        == comma as libc::c_int as libc::c_uint
        && (*parser_state_tos).p_l_follow <= 0 as libc::c_int
        && settings.leave_comma != 0 && (*parser_state_tos).block_init == 0
        && break_comma != 0 && s_com == e_com
    {
        dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
        (*parser_state_tos).want_blank = 0 as libc::c_int;
    }
}
unsafe extern "C" fn handle_token_comment(
    mut force_nl: *mut BOOLEAN,
    mut flushed_nl: *mut BOOLEAN,
    mut pbreak_line: *mut BOOLEAN,
) {
    if (*parser_state_tos).last_saw_nl != 0 && s_code != e_code {
        *flushed_nl = 0 as libc::c_int as BOOLEAN;
        dump_line(1 as libc::c_int, &mut paren_target, pbreak_line);
        (*parser_state_tos).want_blank = 0 as libc::c_int;
        *force_nl = 0 as libc::c_int as BOOLEAN;
    }
    print_comment(&mut paren_target, pbreak_line);
}
unsafe extern "C" fn handle_token_attribute() {
    let mut t_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    if s_code != e_code {
        set_buf_break(bb_attribute, paren_target);
        let fresh62 = e_code;
        e_code = e_code.offset(1);
        *fresh62 = ' ' as i32 as libc::c_char;
    }
    t_ptr = token;
    while t_ptr < token_end {
        check_code_size();
        let fresh63 = e_code;
        e_code = e_code.offset(1);
        *fresh63 = *t_ptr;
        t_ptr = t_ptr.offset(1);
        t_ptr;
    }
    (*parser_state_tos).in_decl = 0 as libc::c_int as BOOLEAN;
    (*parser_state_tos).want_blank = settings.blank_after_sizeof;
}
pub unsafe extern "C" fn handle_the_token(
    type_code: codes_ty,
    mut scase: *mut BOOLEAN,
    mut force_nl: *mut BOOLEAN,
    mut sp_sw: *mut BOOLEAN,
    mut flushed_nl: *mut BOOLEAN,
    mut hd_type: *mut codes_ty,
    mut dec_ind: *mut libc::c_int,
    mut last_token_ends_sp: *mut BOOLEAN,
    mut file_exit_value: *mut exit_values_ty,
    can_break: bb_code_ty,
    mut last_else: *mut BOOLEAN,
    mut is_procname_definition: BOOLEAN,
    mut pbreak_line: *mut BOOLEAN,
) {
    match type_code as libc::c_uint {
        23 => {
            handle_token_form_feed(pbreak_line);
        }
        1 => {
            handle_token_newline(force_nl, pbreak_line);
        }
        2 => {
            handle_token_lparen(force_nl, sp_sw, dec_ind, pbreak_line);
        }
        3 => {
            handle_token_rparen(
                force_nl,
                sp_sw,
                hd_type,
                last_token_ends_sp,
                file_exit_value,
                pbreak_line,
            );
        }
        5 => {
            handle_token_unary_op(dec_ind, can_break);
        }
        6 => {
            handle_token_binary_op(can_break);
        }
        7 => {
            handle_token_postop();
        }
        8 => {
            handle_token_question(can_break);
        }
        9 => {
            handle_token_casestmt(scase, file_exit_value);
            copy_id(type_code, force_nl, file_exit_value, can_break);
        }
        10 => {
            handle_token_colon(scase, force_nl, dec_ind, can_break, pbreak_line);
        }
        11 => {
            handle_token_doublecolon();
        }
        12 => {
            handle_token_semicolon(
                scase,
                force_nl,
                sp_sw,
                dec_ind,
                last_token_ends_sp,
                file_exit_value,
            );
        }
        13 => {
            handle_token_lbrace(force_nl, dec_ind, file_exit_value, pbreak_line);
        }
        14 => {
            handle_token_rbrace(force_nl, dec_ind, file_exit_value, pbreak_line);
        }
        21 => {
            handle_token_swstmt(sp_sw, hd_type);
            copy_id(type_code, force_nl, file_exit_value, can_break);
        }
        25 => {
            handle_token_sp_paren(sp_sw, hd_type);
            copy_id(type_code, force_nl, file_exit_value, can_break);
        }
        27 | 26 => {
            handle_token_nparen(force_nl, file_exit_value, last_else, pbreak_line);
            copy_id(type_code, force_nl, file_exit_value, can_break);
        }
        16 => {
            handle_token_overloaded(can_break);
        }
        24 => {
            handle_token_decl(dec_ind, file_exit_value, pbreak_line);
            copy_id(type_code, force_nl, file_exit_value, can_break);
        }
        17 | 15 => {
            handle_token_ident(
                force_nl,
                sp_sw,
                hd_type,
                dec_ind,
                file_exit_value,
                can_break,
                is_procname_definition,
                pbreak_line,
            );
            copy_id(type_code, force_nl, file_exit_value, can_break);
        }
        40 => {
            handle_token_struct_delim();
        }
        18 => {
            handle_token_comma(force_nl, dec_ind, is_procname_definition);
        }
        22 => {
            handle_token_preesc(file_exit_value, pbreak_line);
        }
        19 | 20 => {
            handle_token_comment(force_nl, flushed_nl, pbreak_line);
        }
        41 => {
            handle_token_attribute();
        }
        _ => {
            abort();
        }
    };
}

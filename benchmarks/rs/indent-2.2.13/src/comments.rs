use ::libc;
extern "C" {
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut s_lab: *mut libc::c_char;
    static mut e_lab: *mut libc::c_char;
    static mut s_code: *mut libc::c_char;
    static mut e_code: *mut libc::c_char;
    static mut combuf: *mut libc::c_char;
    static mut s_com: *mut libc::c_char;
    static mut e_com: *mut libc::c_char;
    static mut l_com: *mut libc::c_char;
    static mut buf_ptr: *mut libc::c_char;
    static mut buf_end: *mut libc::c_char;
    static mut token: *mut libc::c_char;
    static mut save_com: buf_ty;
    static mut prefix_blankline_requested: libc::c_int;
    static mut settings: user_options_ty;
    static mut else_or_endif: libc::c_int;
    static mut com_lines: libc::c_int;
    static mut had_eof: BOOLEAN;
    static mut line_no: libc::c_int;
    static mut parser_state_tos: *mut parser_state_ty;
    fn skip_horiz_space(p: *const libc::c_char) -> *mut libc::c_char;
    fn current_column() -> libc::c_int;
    fn fill_buffer();
    fn skip_buffered_space();
    fn compute_code_target(paren_targ: libc::c_int) -> libc::c_int;
    fn compute_label_target() -> libc::c_int;
    fn count_columns(
        column: libc::c_int,
        bp: *mut libc::c_char,
        stop_char: libc::c_int,
    ) -> libc::c_int;
    fn dump_line(
        force_nl: libc::c_int,
        paren_targ: *mut libc::c_int,
        break_line: *mut BOOLEAN,
    );
    fn xrealloc(ptr: *mut libc::c_void, size: libc::c_uint) -> *mut libc::c_void;
    fn inc_pstack() -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn print_comment(
    mut paren_targ: *mut libc::c_int,
    mut pbreak_line: *mut BOOLEAN,
) {
    let mut current_block: u64;
    let mut column: libc::c_int = 0;
    let mut format: libc::c_int = 0;
    let mut comment_type: codes_ty = code_eof;
    let mut start_column: libc::c_int = 0;
    let mut found_column: libc::c_int = 0;
    let mut first_comment_line: libc::c_int = 0;
    let mut right_margin: libc::c_int = 0;
    let mut boxed_comment: libc::c_int = 0;
    let mut stars: libc::c_int = 0;
    let mut blankline_delims: libc::c_int = 0;
    let mut paragraph_break: libc::c_int = 0;
    let mut merge_blank_comment_lines: libc::c_int = 0;
    let mut two_contiguous_comments: libc::c_int = 0 as libc::c_int;
    let mut save_length: libc::c_int = 0 as libc::c_int;
    let mut save_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text_on_line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_break_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_delim: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_preamble: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_preamble_length: libc::c_int = 0;
    let mut visible_preamble: libc::c_int = 0;
    let mut suppress_cdb: libc::c_int = 0 as libc::c_int;
    inc_pstack();
    if *token.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        comment_type = cplus_comment;
    } else {
        comment_type = comment;
    }
    if comment_type as libc::c_uint == cplus_comment as libc::c_int as libc::c_uint {
        start_delim = b"//\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        line_preamble = b"// \0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        line_preamble_length = strlen(line_preamble) as libc::c_int;
        visible_preamble = 1 as libc::c_int;
        boxed_comment = 0 as libc::c_int;
        stars = 0 as libc::c_int;
        blankline_delims = 0 as libc::c_int;
    } else if *buf_ptr as libc::c_int == '*' as i32
        || *buf_ptr as libc::c_int == '-' as i32 || *buf_ptr as libc::c_int == '=' as i32
        || *buf_ptr as libc::c_int == '_' as i32
        || (*parser_state_tos).col_1 != 0 && settings.format_col1_comments == 0
    {
        let mut comment_lines_count: libc::c_int = 1 as libc::c_int;
        stars = 0 as libc::c_int;
        boxed_comment = 0 as libc::c_int;
        blankline_delims = 0 as libc::c_int;
        line_preamble_length = 0 as libc::c_int;
        visible_preamble = 0 as libc::c_int;
        if buf_ptr >= save_com.ptr
            && buf_ptr <= (save_com.ptr).offset(save_com.len as isize)
        {
            start_column = save_com.start_column;
        } else {
            start_column = current_column() - 2 as libc::c_int;
        }
        found_column = start_column;
        (*parser_state_tos).box_com = 1 as libc::c_int;
        (*parser_state_tos).com_col = found_column;
        if settings.blanklines_before_blockcomments != 0 {
            prefix_blankline_requested = 1 as libc::c_int;
        }
        let fresh0 = e_com;
        e_com = e_com.offset(1);
        *fresh0 = '/' as i32 as libc::c_char;
        let fresh1 = e_com;
        e_com = e_com.offset(1);
        *fresh1 = '*' as i32 as libc::c_char;
        loop {
            loop {
                if *buf_ptr as libc::c_int == '\n' as i32 {
                    line_no += 1;
                    line_no;
                }
                if e_com >= l_com.offset(-(1 as libc::c_int as isize)) {
                    let mut nsize: size_t = (l_com.offset_from(s_com) as libc::c_long
                        + 400 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long) as size_t;
                    combuf = xrealloc(combuf as *mut libc::c_void, nsize as libc::c_uint)
                        as *mut libc::c_char;
                    e_com = combuf
                        .offset(e_com.offset_from(s_com) as libc::c_long as isize)
                        .offset(1 as libc::c_int as isize);
                    l_com = combuf
                        .offset(nsize as isize)
                        .offset(-(5 as libc::c_int as isize));
                    s_com = combuf.offset(1 as libc::c_int as isize);
                }
                let fresh2 = buf_ptr;
                buf_ptr = buf_ptr.offset(1);
                let fresh3 = e_com;
                e_com = e_com.offset(1);
                *fresh3 = *fresh2;
                if !(*buf_ptr as libc::c_int != '*' as i32 && buf_ptr < buf_end) {
                    break;
                }
            }
            if buf_ptr > buf_end {
                buf_ptr = buf_end;
            }
            if settings.fix_nested_comments != 0 {
                if *buf_ptr as libc::c_int == '*' as i32
                    && *buf_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                        == '/' as i32
                {
                    *e_com
                        .offset(
                            -(1 as libc::c_int as isize),
                        ) = ' ' as i32 as libc::c_char;
                    *e_com = '*' as i32 as libc::c_char;
                }
            }
            if *buf_ptr as libc::c_int == '*' as i32
                && *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                    == '/' as i32
            {
                if buf_ptr == buf_end {
                    fill_buffer();
                }
                buf_ptr = buf_ptr.offset(2 as libc::c_int as isize);
                if buf_ptr == buf_end {
                    fill_buffer();
                }
                let fresh4 = e_com;
                e_com = e_com.offset(1);
                *fresh4 = '*' as i32 as libc::c_char;
                let fresh5 = e_com;
                e_com = e_com.offset(1);
                *fresh5 = '/' as i32 as libc::c_char;
                *e_com = '\0' as i32 as libc::c_char;
                (*parser_state_tos).tos -= 1;
                (*parser_state_tos).tos;
                if comment_lines_count > 1 as libc::c_int {
                    (*parser_state_tos).com_col = 1 as libc::c_int;
                } else {
                    (*parser_state_tos).com_col = found_column;
                }
                return;
            }
            if buf_ptr == buf_end {
                if *buf_ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == '\n' as i32
                {
                    e_com = e_com.offset(-1);
                    *e_com = '\0' as i32 as libc::c_char;
                    dump_line(1 as libc::c_int, paren_targ, pbreak_line);
                    comment_lines_count += 1;
                    comment_lines_count;
                    (*parser_state_tos).com_col = 1 as libc::c_int;
                }
                fill_buffer();
                if had_eof != 0 {
                    let fresh6 = e_com;
                    e_com = e_com.offset(1);
                    *fresh6 = '\0' as i32 as libc::c_char;
                    (*parser_state_tos).tos -= 1;
                    (*parser_state_tos).tos;
                    (*parser_state_tos).com_col = start_column;
                    return;
                }
            }
        }
    } else {
        start_delim = b"/*\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        line_preamble = 0 as *mut libc::c_char;
        line_preamble_length = 0 as libc::c_int;
        visible_preamble = 0 as libc::c_int;
        boxed_comment = 0 as libc::c_int;
        stars = settings.star_comment_cont;
        blankline_delims = settings.comment_delimiter_on_blankline;
    }
    paragraph_break = 0 as libc::c_int;
    merge_blank_comment_lines = 0 as libc::c_int;
    first_comment_line = com_lines;
    right_margin = settings.comment_max_col;
    found_column = current_column() - 2 as libc::c_int;
    if s_lab == e_lab && s_code == e_code {
        if (*parser_state_tos).col_1 != 0 && settings.format_col1_comments == 0 {
            format = settings.format_col1_comments;
            start_column = 1 as libc::c_int;
        } else {
            format = settings.format_comments;
            if (*parser_state_tos).ind_level <= 0 as libc::c_int
                && ((*parser_state_tos).in_stmt == 0
                    || (*parser_state_tos).in_decl as libc::c_int != 0
                        && (*parser_state_tos).paren_level == 0 as libc::c_int)
            {
                start_column = found_column;
            } else {
                start_column = compute_code_target(*paren_targ)
                    - settings.unindent_displace;
                if start_column < 0 as libc::c_int {
                    start_column = 1 as libc::c_int;
                }
            }
        }
    } else {
        let mut target: libc::c_int = 0;
        suppress_cdb = 1 as libc::c_int;
        if (*parser_state_tos).decl_on_line != 0 {
            target = settings.decl_com_ind;
        } else if else_or_endif != 0 {
            target = settings.else_endif_col;
        } else {
            target = settings.com_ind;
        }
        if s_code != e_code {
            start_column = count_columns(
                compute_code_target(*paren_targ),
                s_code,
                '\0' as i32,
            );
        } else {
            start_column = count_columns(compute_label_target(), s_lab, '\0' as i32);
        }
        if start_column < target {
            start_column = target;
        } else if else_or_endif != 0 || settings.dont_tab_align_comments != 0 {
            start_column += 1;
            start_column;
            else_or_endif = 0 as libc::c_int;
        } else {
            start_column
                += settings.tabsize
                    - (start_column - 1 as libc::c_int) % settings.tabsize;
        }
        format = settings.format_comments;
    }
    if line_preamble.is_null() {
        line_preamble_length = 3 as libc::c_int;
        if stars != 0 {
            line_preamble = b" * \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            visible_preamble = 1 as libc::c_int;
        } else {
            line_preamble = b"   \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            visible_preamble = 0 as libc::c_int;
        }
    }
    (*parser_state_tos)
        .com_col = if two_contiguous_comments != 0 {
        1 as libc::c_int
    } else {
        start_column
    };
    (*parser_state_tos).box_com = boxed_comment;
    if e_com >= l_com.offset(-(2 as libc::c_int as isize)) {
        let mut nsize_0: size_t = (l_com.offset_from(s_com) as libc::c_long
            + 400 as libc::c_int as libc::c_long + 2 as libc::c_int as libc::c_long)
            as size_t;
        combuf = xrealloc(combuf as *mut libc::c_void, nsize_0 as libc::c_uint)
            as *mut libc::c_char;
        e_com = combuf
            .offset(e_com.offset_from(s_com) as libc::c_long as isize)
            .offset(1 as libc::c_int as isize);
        l_com = combuf.offset(nsize_0 as isize).offset(-(5 as libc::c_int as isize));
        s_com = combuf.offset(1 as libc::c_int as isize);
    }
    let fresh7 = e_com;
    e_com = e_com.offset(1);
    *fresh7 = *start_delim;
    let fresh8 = e_com;
    e_com = e_com.offset(1);
    *fresh8 = *start_delim.offset(1 as libc::c_int as isize);
    column = start_column + 2 as libc::c_int;
    if blankline_delims != 0 && suppress_cdb == 0 {
        let mut p: *mut libc::c_char = buf_ptr;
        *e_com = '\0' as i32 as libc::c_char;
        dump_line(1 as libc::c_int, paren_targ, pbreak_line);
        p = skip_horiz_space(p);
        if *p as libc::c_int == '\n' as i32 {
            buf_ptr = p.offset(1 as libc::c_int as isize);
        } else if format != 0 {
            buf_ptr = p;
        }
        if buf_ptr >= buf_end {
            fill_buffer();
        }
        column = start_column;
        current_block = 6122576587362036437;
    } else {
        if format != 0 {
            let fresh9 = e_com;
            e_com = e_com.offset(1);
            *fresh9 = ' ' as i32 as libc::c_char;
            column = start_column + 3 as libc::c_int;
            skip_buffered_space();
        }
        current_block = 8880031775101799352;
    }
    loop {
        match current_block {
            6122576587362036437 => {
                if had_eof != 0 {
                    break;
                }
                if !line_preamble.is_null() && line_preamble_length > 0 as libc::c_int {
                    if e_com >= l_com.offset(-(line_preamble_length as isize)) {
                        let mut nsize_4: size_t = (l_com.offset_from(s_com)
                            as libc::c_long + 400 as libc::c_int as libc::c_long
                            + line_preamble_length as libc::c_long) as size_t;
                        combuf = xrealloc(
                            combuf as *mut libc::c_void,
                            nsize_4 as libc::c_uint,
                        ) as *mut libc::c_char;
                        e_com = combuf
                            .offset(e_com.offset_from(s_com) as libc::c_long as isize)
                            .offset(1 as libc::c_int as isize);
                        l_com = combuf
                            .offset(nsize_4 as isize)
                            .offset(-(5 as libc::c_int as isize));
                        s_com = combuf.offset(1 as libc::c_int as isize);
                    }
                    memcpy(
                        e_com as *mut libc::c_void,
                        line_preamble as *const libc::c_void,
                        line_preamble_length as libc::c_ulong,
                    );
                    e_com = e_com.offset(line_preamble_length as isize);
                    column = start_column + line_preamble_length;
                } else {
                    column = start_column;
                }
                line_break_ptr = 0 as *mut libc::c_char;
                if !save_ptr.is_null() {
                    while (*save_ptr as libc::c_int == ' ' as i32
                        || *save_ptr as libc::c_int == '\t' as i32) && save_length != 0
                    {
                        save_ptr = save_ptr.offset(1);
                        save_ptr;
                        save_length -= 1;
                        save_length;
                    }
                    if e_com >= l_com.offset(-(save_length as isize)) {
                        let mut nsize_5: size_t = (l_com.offset_from(s_com)
                            as libc::c_long + 400 as libc::c_int as libc::c_long
                            + save_length as libc::c_long) as size_t;
                        combuf = xrealloc(
                            combuf as *mut libc::c_void,
                            nsize_5 as libc::c_uint,
                        ) as *mut libc::c_char;
                        e_com = combuf
                            .offset(e_com.offset_from(s_com) as libc::c_long as isize)
                            .offset(1 as libc::c_int as isize);
                        l_com = combuf
                            .offset(nsize_5 as isize)
                            .offset(-(5 as libc::c_int as isize));
                        s_com = combuf.offset(1 as libc::c_int as isize);
                    }
                    memmove(
                        e_com as *mut libc::c_void,
                        save_ptr as *const libc::c_void,
                        save_length as libc::c_ulong,
                    );
                    text_on_line = e_com;
                    e_com = e_com.offset(save_length as isize);
                    column += save_length;
                    save_ptr = 0 as *mut libc::c_char;
                    save_length = 0 as libc::c_int;
                } else {
                    skip_buffered_space();
                    text_on_line = 0 as *mut libc::c_char;
                }
                current_block = 8880031775101799352;
            }
            _ => {
                if !(had_eof == 0) {
                    break;
                }
                while had_eof == 0 {
                    if e_com >= l_com.offset(-(1 as libc::c_int as isize)) {
                        let mut nsize_1: size_t = (l_com.offset_from(s_com)
                            as libc::c_long + 400 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as size_t;
                        combuf = xrealloc(
                            combuf as *mut libc::c_void,
                            nsize_1 as libc::c_uint,
                        ) as *mut libc::c_char;
                        e_com = combuf
                            .offset(e_com.offset_from(s_com) as libc::c_long as isize)
                            .offset(1 as libc::c_int as isize);
                        l_com = combuf
                            .offset(nsize_1 as isize)
                            .offset(-(5 as libc::c_int as isize));
                        s_com = combuf.offset(1 as libc::c_int as isize);
                    }
                    match *buf_ptr as libc::c_int {
                        32 | 9 => {
                            if format != 0 && line_break_ptr < text_on_line {
                                line_break_ptr = e_com;
                            }
                            if format != 0 {
                                if e_com == s_com
                                    || *e_com.offset(-(1 as libc::c_int) as isize)
                                        as libc::c_int != ' ' as i32
                                    || e_com.offset(-(1 as libc::c_int as isize)) == s_com
                                    || *e_com.offset(-(2 as libc::c_int) as isize)
                                        as libc::c_int == '.' as i32
                                {
                                    let fresh10 = e_com;
                                    e_com = e_com.offset(1);
                                    *fresh10 = ' ' as i32 as libc::c_char;
                                    column += 1;
                                    column;
                                }
                            } else if *buf_ptr as libc::c_int == ' ' as i32 {
                                let fresh11 = e_com;
                                e_com = e_com.offset(1);
                                *fresh11 = ' ' as i32 as libc::c_char;
                                column += 1;
                                column;
                            } else {
                                let mut tab_width: libc::c_int = settings.tabsize
                                    - (column + found_column - start_column - 1 as libc::c_int)
                                        % settings.tabsize;
                                column += tab_width;
                                if e_com >= l_com.offset(-(tab_width as isize)) {
                                    let mut nsize_2: size_t = (l_com.offset_from(s_com)
                                        as libc::c_long + 400 as libc::c_int as libc::c_long
                                        + tab_width as libc::c_long) as size_t;
                                    combuf = xrealloc(
                                        combuf as *mut libc::c_void,
                                        nsize_2 as libc::c_uint,
                                    ) as *mut libc::c_char;
                                    e_com = combuf
                                        .offset(e_com.offset_from(s_com) as libc::c_long as isize)
                                        .offset(1 as libc::c_int as isize);
                                    l_com = combuf
                                        .offset(nsize_2 as isize)
                                        .offset(-(5 as libc::c_int as isize));
                                    s_com = combuf.offset(1 as libc::c_int as isize);
                                }
                                loop {
                                    let fresh12 = tab_width;
                                    tab_width = tab_width - 1;
                                    if !(fresh12 != 0) {
                                        break;
                                    }
                                    let fresh13 = e_com;
                                    e_com = e_com.offset(1);
                                    *fresh13 = ' ' as i32 as libc::c_char;
                                }
                            }
                            current_block = 14908777651318078790;
                        }
                        10 => {
                            if comment_type as libc::c_uint
                                == cplus_comment as libc::c_int as libc::c_uint
                            {
                                current_block = 2088797871576213844;
                            } else {
                                if !(format != 0) {
                                    break;
                                }
                                if *buf_ptr as libc::c_int == '\n' as i32 {
                                    line_no += 1;
                                    line_no;
                                }
                                buf_ptr = buf_ptr.offset(1);
                                buf_ptr;
                                if buf_ptr >= buf_end {
                                    fill_buffer();
                                }
                                if e_com > line_break_ptr && text_on_line < line_break_ptr {
                                    e_com = line_break_ptr;
                                }
                                skip_buffered_space();
                                if *buf_ptr as libc::c_int == '\n' as i32
                                    || text_on_line.is_null()
                                {
                                    paragraph_break = 1 as libc::c_int;
                                    break;
                                } else {
                                    if boxed_comment == 0
                                        && current_column() == found_column + 1 as libc::c_int
                                        && *buf_ptr.offset(0 as libc::c_int as isize) as libc::c_int
                                            == '*' as i32
                                        && *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                                            != '/' as i32
                                    {
                                        buf_ptr = buf_ptr.offset(1);
                                        if buf_ptr >= buf_end {
                                            fill_buffer();
                                        }
                                        if *buf_ptr as libc::c_int == ' ' as i32
                                            && {
                                                buf_ptr = buf_ptr.offset(1);
                                                buf_ptr >= buf_end
                                            }
                                        {
                                            fill_buffer();
                                        }
                                    }
                                    if *e_com.offset(-(1 as libc::c_int) as isize)
                                        as libc::c_int != ' ' as i32
                                    {
                                        line_break_ptr = e_com;
                                        let fresh14 = e_com;
                                        e_com = e_com.offset(1);
                                        *fresh14 = ' ' as i32 as libc::c_char;
                                        column += 1;
                                        column;
                                    }
                                    continue;
                                }
                            }
                        }
                        42 => {
                            if comment_type as libc::c_uint
                                == comment as libc::c_int as libc::c_uint
                            {
                                if *buf_ptr.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '/' as i32
                                {
                                    if boxed_comment == 0 {
                                        if !text_on_line.is_null() {
                                            if blankline_delims != 0 && suppress_cdb == 0 {
                                                *e_com = '\0' as i32 as libc::c_char;
                                                dump_line(1 as libc::c_int, paren_targ, pbreak_line);
                                                let fresh15 = e_com;
                                                e_com = e_com.offset(1);
                                                *fresh15 = ' ' as i32 as libc::c_char;
                                            } else if *e_com.offset(-(1 as libc::c_int as isize))
                                                as libc::c_int != ' ' as i32
                                                && *e_com.offset(-(1 as libc::c_int as isize))
                                                    as libc::c_int != '\t' as i32
                                            {
                                                let fresh16 = e_com;
                                                e_com = e_com.offset(1);
                                                *fresh16 = ' ' as i32 as libc::c_char;
                                            }
                                        } else if s_com == e_com
                                            || *s_com as libc::c_int != '/' as i32
                                        {
                                            e_com = s_com;
                                            let fresh17 = e_com;
                                            e_com = e_com.offset(1);
                                            *fresh17 = ' ' as i32 as libc::c_char;
                                        } else if *e_com.offset(-(1 as libc::c_int as isize))
                                            as libc::c_int != ' ' as i32
                                            && *e_com.offset(-(1 as libc::c_int as isize))
                                                as libc::c_int != '\t' as i32
                                        {
                                            let fresh18 = e_com;
                                            e_com = e_com.offset(1);
                                            *fresh18 = ' ' as i32 as libc::c_char;
                                        }
                                    }
                                    if e_com >= l_com.offset(-(3 as libc::c_int as isize)) {
                                        let mut nsize_3: size_t = (l_com.offset_from(s_com)
                                            as libc::c_long + 400 as libc::c_int as libc::c_long
                                            + 3 as libc::c_int as libc::c_long) as size_t;
                                        combuf = xrealloc(
                                            combuf as *mut libc::c_void,
                                            nsize_3 as libc::c_uint,
                                        ) as *mut libc::c_char;
                                        e_com = combuf
                                            .offset(e_com.offset_from(s_com) as libc::c_long as isize)
                                            .offset(1 as libc::c_int as isize);
                                        l_com = combuf
                                            .offset(nsize_3 as isize)
                                            .offset(-(5 as libc::c_int as isize));
                                        s_com = combuf.offset(1 as libc::c_int as isize);
                                    }
                                    let fresh19 = e_com;
                                    e_com = e_com.offset(1);
                                    *fresh19 = '*' as i32 as libc::c_char;
                                    let fresh20 = e_com;
                                    e_com = e_com.offset(1);
                                    *fresh20 = '/' as i32 as libc::c_char;
                                    *e_com = '\0' as i32 as libc::c_char;
                                    buf_ptr = buf_ptr.offset(2 as libc::c_int as isize);
                                    buf_ptr = skip_horiz_space(buf_ptr);
                                    if buf_ptr >= buf_end {
                                        fill_buffer();
                                    }
                                    (*parser_state_tos).tos -= 1;
                                    (*parser_state_tos).tos;
                                    (*parser_state_tos)
                                        .com_col = if two_contiguous_comments != 0 {
                                        1 as libc::c_int
                                    } else {
                                        start_column
                                    };
                                    (*parser_state_tos).box_com = boxed_comment;
                                    return;
                                }
                                if first_comment_line == com_lines - 1 as libc::c_int
                                    && e_com == s_com.offset(line_preamble_length as isize)
                                {
                                    column -= line_preamble_length - 1 as libc::c_int;
                                    line_preamble = b" \0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char;
                                    line_preamble_length = 1 as libc::c_int;
                                    boxed_comment = 1 as libc::c_int;
                                    format = 0 as libc::c_int;
                                    blankline_delims = 0 as libc::c_int;
                                    *s_com = ' ' as i32 as libc::c_char;
                                    *s_com
                                        .offset(
                                            1 as libc::c_int as isize,
                                        ) = '*' as i32 as libc::c_char;
                                    e_com = s_com.offset(2 as libc::c_int as isize);
                                    text_on_line = e_com;
                                    column += 1;
                                    column;
                                    current_block = 14908777651318078790;
                                } else {
                                    current_block = 18422366903412862849;
                                }
                            } else {
                                current_block = 18422366903412862849;
                            }
                        }
                        _ => {
                            current_block = 18422366903412862849;
                        }
                    }
                    match current_block {
                        18422366903412862849 => {
                            text_on_line = e_com;
                            let fresh21 = e_com;
                            e_com = e_com.offset(1);
                            *fresh21 = *buf_ptr;
                            column += 1;
                            column;
                            current_block = 14908777651318078790;
                        }
                        _ => {}
                    }
                    match current_block {
                        14908777651318078790 => {
                            if format != 0 && column > right_margin
                                && !line_break_ptr.is_null()
                            {
                                if line_break_ptr
                                    < e_com.offset(-(1 as libc::c_int as isize))
                                {
                                    *line_break_ptr = '\0' as i32 as libc::c_char;
                                    save_ptr = line_break_ptr.offset(1 as libc::c_int as isize);
                                    save_length = e_com.offset_from(save_ptr) as libc::c_long
                                        as libc::c_int;
                                    e_com = line_break_ptr;
                                    if column - save_length > right_margin {
                                        right_margin = column - save_length;
                                    }
                                    break;
                                } else {
                                    if comment_type as libc::c_uint
                                        == cplus_comment as libc::c_int as libc::c_uint
                                    {
                                        buf_ptr = skip_horiz_space(buf_ptr);
                                        buf_ptr = buf_ptr.offset(-1);
                                        buf_ptr;
                                        if *buf_ptr as libc::c_int == '\n' as i32 {
                                            current_block = 2088797871576213844;
                                        } else {
                                            current_block = 8169728755214547442;
                                        }
                                    } else {
                                        while *buf_ptr as libc::c_int == '\t' as i32
                                            || *buf_ptr as libc::c_int == ' ' as i32
                                            || *buf_ptr as libc::c_int == '\n' as i32
                                        {
                                            if *buf_ptr as libc::c_int == '\n' as i32 {
                                                line_no += 1;
                                                line_no;
                                            }
                                            buf_ptr = buf_ptr.offset(1);
                                            buf_ptr;
                                            if buf_ptr >= buf_end {
                                                fill_buffer();
                                            }
                                        }
                                        buf_ptr = buf_ptr.offset(-1);
                                        buf_ptr;
                                        current_block = 8169728755214547442;
                                    }
                                    match current_block {
                                        2088797871576213844 => {}
                                        _ => {
                                            *e_com = '\0' as i32 as libc::c_char;
                                            break;
                                        }
                                    }
                                }
                            } else {
                                if *buf_ptr as libc::c_int == '\n' as i32 {
                                    line_no += 1;
                                    line_no;
                                }
                                buf_ptr = buf_ptr.offset(1);
                                buf_ptr;
                                if buf_ptr == buf_end {
                                    fill_buffer();
                                }
                                continue;
                            }
                        }
                        _ => {}
                    }
                    (*parser_state_tos).tos -= 1;
                    (*parser_state_tos).tos;
                    (*parser_state_tos)
                        .com_col = if two_contiguous_comments != 0 {
                        1 as libc::c_int
                    } else {
                        start_column
                    };
                    (*parser_state_tos).box_com = boxed_comment;
                    *e_com = 0 as libc::c_int as libc::c_char;
                    return;
                }
                if text_on_line.is_null() && visible_preamble == 0
                    && !(first_comment_line == com_lines)
                {
                    e_com = s_com;
                }
                *e_com = '\0' as i32 as libc::c_char;
                dump_line(1 as libc::c_int, paren_targ, pbreak_line);
                prefix_blankline_requested = 0 as libc::c_int;
                if paragraph_break != 0 {
                    if merge_blank_comment_lines != 0 {
                        while *buf_ptr as libc::c_int == '\n' as i32
                            || *buf_ptr as libc::c_int == ' ' as i32
                            || *buf_ptr as libc::c_int == '\t' as i32
                        {
                            if *buf_ptr as libc::c_int == '\n' as i32 {
                                line_no += 1;
                                line_no;
                            }
                            buf_ptr = buf_ptr.offset(1);
                            if buf_ptr >= buf_end {
                                fill_buffer();
                            }
                        }
                    }
                    paragraph_break = 0 as libc::c_int;
                } else {
                    if *buf_ptr as libc::c_int == '\n' as i32 {
                        line_no += 1;
                        line_no;
                    }
                    buf_ptr = buf_ptr.offset(1);
                    buf_ptr;
                    if buf_ptr >= buf_end {
                        fill_buffer();
                    }
                }
                current_block = 6122576587362036437;
            }
        }
    }
    *e_com = '\0' as i32 as libc::c_char;
    (*parser_state_tos).tos -= 1;
    (*parser_state_tos).tos;
    (*parser_state_tos)
        .com_col = if two_contiguous_comments != 0 {
        1 as libc::c_int
    } else {
        start_column
    };
    (*parser_state_tos).box_com = boxed_comment;
}

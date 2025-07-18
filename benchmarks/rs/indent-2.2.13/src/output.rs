use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn utime(__file: *const libc::c_char, __file_times: *const utimbuf) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    static mut in_prog_pos: *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut s_com: *mut libc::c_char;
    static mut e_com: *mut libc::c_char;
    static mut buf_ptr: *mut libc::c_char;
    static mut buf_end: *mut libc::c_char;
    static mut token: *mut libc::c_char;
    static mut current_input: *mut file_buffer_ty;
    static mut s_code_corresponds_to: *mut libc::c_char;
    static mut e_code: *mut libc::c_char;
    static mut s_code: *mut libc::c_char;
    static mut e_lab: *mut libc::c_char;
    static mut s_lab: *mut libc::c_char;
    static mut cur_line: *mut libc::c_char;
    static mut line_no: libc::c_int;
    static mut code_lines: libc::c_int;
    static mut embedded_comment_on_line: libc::c_int;
    static mut had_eof: BOOLEAN;
    static mut settings: user_options_ty;
    static mut postfix_blankline_requested_code: codes_ty;
    static mut postfix_blankline_requested: libc::c_int;
    static mut prefix_blankline_requested_code: codes_ty;
    static mut prefix_blankline_requested: libc::c_int;
    static mut n_real_blanklines: libc::c_int;
    static mut paren_target: libc::c_int;
    static mut parser_state_tos: *mut parser_state_ty;
    fn xmalloc(size: libc::c_uint) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn fatal(string: *const libc::c_char, a0: *const libc::c_char);
    fn message(
        kind: *mut libc::c_char,
        string: *mut libc::c_char,
        a0: *mut libc::c_char,
        a1: *mut libc::c_char,
    );
    fn fill_buffer();
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utimbuf {
    pub actime: __time_t,
    pub modtime: __time_t,
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
static mut output: *mut FILE = 0 as *const FILE as *mut FILE;
static mut inhibited: BOOLEAN = 0 as libc::c_int as BOOLEAN;
static mut buf_break_list: *mut buf_break_st_ty = 0 as *const buf_break_st_ty
    as *mut buf_break_st_ty;
pub static mut buf_break: *mut buf_break_st_ty = 0 as *const buf_break_st_ty
    as *mut buf_break_st_ty;
pub static mut out_lines: libc::c_int = 0 as libc::c_int;
pub static mut com_lines: libc::c_int = 0 as libc::c_int;
pub static mut prev_target_col_break: libc::c_int = 0 as libc::c_int;
pub static mut buf_break_used: libc::c_int = 0 as libc::c_int;
pub static mut preproc_indent: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn better_break(
    mut b1: *mut buf_break_st_ty,
    mut b2: *const buf_break_st_ty,
) -> BOOLEAN {
    static mut first_level: libc::c_int = 0;
    let mut is_better: BOOLEAN = 0;
    if b2.is_null() {
        first_level = (*b1).level;
        (*b1).first_level = first_level;
        is_better = 1 as libc::c_int as BOOLEAN;
    } else {
        if (*b2).target_col >= (*b2).col + 1 as libc::c_int {
            is_better = 1 as libc::c_int as BOOLEAN;
        } else if settings.honour_newlines != 0 && (*b2).priority_newline != 0 {
            is_better = 0 as libc::c_int as BOOLEAN;
        } else if settings.honour_newlines != 0 && (*b1).priority_newline != 0 {
            is_better = 1 as libc::c_int as BOOLEAN;
        } else {
            let mut only_parens_till_b2: libc::c_int = 0 as libc::c_int;
            is_better = ((*b1).priority > (*b2).priority) as libc::c_int as BOOLEAN;
            if is_better != 0 {
                let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                p = &mut *s_code.offset((*b2).offset as isize) as *mut libc::c_char;
                while p >= s_code {
                    if *p as libc::c_int == '!' as i32 {
                        p = p.offset(-1);
                        p;
                    }
                    if *p as libc::c_int != '(' as i32 {
                        break;
                    }
                    p = p.offset(-1);
                    p;
                }
                if p < s_code {
                    only_parens_till_b2 = 1 as libc::c_int;
                }
            }
            if settings.lineup_to_parens != 0
                && (*b1).level > first_level + 1 as libc::c_int
                && !(only_parens_till_b2 != 0
                    && (*b1).target_col
                        <= (*b2).col
                            + (1 as libc::c_int + 2 as libc::c_int * (*b1).level))
                && (*b1).level > (*b2).level
            {
                is_better = 0 as libc::c_int as BOOLEAN;
            }
        }
        if is_better != 0 {
            (*b1).first_level = first_level;
        }
    }
    return is_better;
}
unsafe extern "C" fn set_priority(mut bb: *mut buf_break_st_ty) {
    (*bb).priority = (*bb).priority_code_length;
    match (*bb).priority_code as libc::c_uint {
        15 => {
            (*bb).priority += 6000 as libc::c_int;
        }
        8 => {
            (*bb).priority += 5000 as libc::c_int;
        }
        9 => {
            if (*bb).priority_code_length > 2 as libc::c_int {
                (*bb).priority += 5000 as libc::c_int;
            }
            if settings.break_before_boolean_operator != 0 {
                (*bb).priority -= 3 as libc::c_int;
            }
        }
        10 => {
            (*bb).priority += 4000 as libc::c_int;
        }
        21 => {
            (*bb).priority += 3000 as libc::c_int;
        }
        1 => {
            (*bb).priority += 2000 as libc::c_int;
        }
        11 => {
            (*bb).priority += 1000 as libc::c_int;
        }
        4 => {
            (*bb).priority -= 1000 as libc::c_int;
        }
        26 => {
            (*bb).priority += 600 as libc::c_int;
        }
        25 => {
            (*bb).priority += 500 as libc::c_int;
        }
        24 => {
            (*bb).priority += 400 as libc::c_int;
        }
        23 => {
            (*bb).priority += 200 as libc::c_int;
        }
        27 => {
            (*bb).priority += 100 as libc::c_int;
        }
        _ => {}
    };
}
pub unsafe extern "C" fn set_buf_break(
    mut code: bb_code_ty,
    mut paren_targ: libc::c_int,
) {
    let mut target_col: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut code_target: libc::c_int = compute_code_target(paren_targ);
    let mut bb: *mut buf_break_st_ty = 0 as *mut buf_break_st_ty;
    target_col = (*parser_state_tos).i_l_follow + 1 as libc::c_int;
    if *token as libc::c_int == '{' as i32 {
        target_col -= settings.ind_size;
    }
    level = (*parser_state_tos).p_l_follow;
    if *token as libc::c_int == '(' as i32 || *token as libc::c_int == '[' as i32 {
        level -= 1;
        level;
    }
    if *((*parser_state_tos).procname).offset(0 as libc::c_int as isize) as libc::c_int
        != 0 && token == (*parser_state_tos).procname
    {
        target_col = 1 as libc::c_int;
    } else if level == 0 as libc::c_int {
        if (*parser_state_tos).in_stmt != 0 {
            target_col += settings.continuation_indent;
        }
    } else if settings.lineup_to_parens == 0 {
        target_col
            += settings.continuation_indent
                + settings.paren_indent * (level - 1 as libc::c_int);
    } else if (*((*parser_state_tos).paren_indents)
        .offset((level - 1 as libc::c_int) as isize) as libc::c_int) < 0 as libc::c_int
    {
        target_col = -(*((*parser_state_tos).paren_indents)
            .offset((level - 1 as libc::c_int) as isize) as libc::c_int);
    } else {
        target_col = code_target
            + *((*parser_state_tos).paren_indents)
                .offset((level - 1 as libc::c_int) as isize) as libc::c_int;
    }
    bb = xmalloc(
        ::std::mem::size_of::<buf_break_st_ty>() as libc::c_ulong as libc::c_uint,
    ) as *mut buf_break_st_ty;
    (*bb).offset = e_code.offset_from(s_code) as libc::c_long as libc::c_int;
    (*bb).level = level;
    (*bb).target_col = target_col;
    (*bb).corresponds_to = token;
    *e_code = 0 as libc::c_int as libc::c_char;
    (*bb).col = count_columns(code_target, s_code, '\0' as i32) - 1 as libc::c_int;
    (*bb)
        .priority_code_length = e_code.offset_from(s_code) as libc::c_long
        as libc::c_int;
    (*bb)
        .priority_newline = ((*parser_state_tos).last_saw_nl != 0
        && (*parser_state_tos).broken_at_non_nl == 0) as libc::c_int;
    if !buf_break.is_null() {
        (*bb).first_level = (*buf_break).first_level;
    }
    match (*parser_state_tos).last_token as libc::c_uint {
        6 => {
            if e_code.offset_from(s_code) as libc::c_long
                >= 3 as libc::c_int as libc::c_long
                && *e_code.offset(-(3 as libc::c_int) as isize) as libc::c_int
                    == ' ' as i32
                && (*e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '&' as i32
                    && *e_code.offset(-(2 as libc::c_int) as isize) as libc::c_int
                        == '&' as i32
                    || *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '|' as i32
                        && *e_code.offset(-(2 as libc::c_int) as isize) as libc::c_int
                            == '|' as i32)
            {
                (*bb).priority_code = bb_after_boolean_binary_op;
            } else if e_code.offset_from(s_code) as libc::c_long
                >= 2 as libc::c_int as libc::c_long
                && *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '=' as i32
                && (*e_code.offset(-(2 as libc::c_int) as isize) as libc::c_int
                    == ' ' as i32
                    || e_code.offset_from(s_code) as libc::c_long
                        >= 3 as libc::c_int as libc::c_long
                        && *e_code.offset(-(3 as libc::c_int) as isize) as libc::c_int
                            == ' ' as i32
                        && (*e_code.offset(-(2 as libc::c_int) as isize) as libc::c_int
                            == '%' as i32
                            || *e_code.offset(-(2 as libc::c_int) as isize)
                                as libc::c_int == '^' as i32
                            || *e_code.offset(-(2 as libc::c_int) as isize)
                                as libc::c_int == '&' as i32
                            || *e_code.offset(-(2 as libc::c_int) as isize)
                                as libc::c_int == '*' as i32
                            || *e_code.offset(-(2 as libc::c_int) as isize)
                                as libc::c_int == '-' as i32
                            || *e_code.offset(-(2 as libc::c_int) as isize)
                                as libc::c_int == '+' as i32
                            || *e_code.offset(-(2 as libc::c_int) as isize)
                                as libc::c_int == '|' as i32))
            {
                (*bb).priority_code = bb_after_equal_sign;
            } else if e_code.offset_from(s_code) as libc::c_long
                >= 2 as libc::c_int as libc::c_long
                && *e_code.offset(-(2 as libc::c_int) as isize) as libc::c_int
                    == ' ' as i32
                && (*e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '<' as i32
                    || *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '>' as i32)
                || e_code.offset_from(s_code) as libc::c_long
                    >= 3 as libc::c_int as libc::c_long
                    && *e_code.offset(-(3 as libc::c_int) as isize) as libc::c_int
                        == ' ' as i32
                    && *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '=' as i32
                    && (*e_code.offset(-(2 as libc::c_int) as isize) as libc::c_int
                        == '=' as i32
                        || *e_code.offset(-(2 as libc::c_int) as isize) as libc::c_int
                            == '!' as i32
                        || *e_code.offset(-(2 as libc::c_int) as isize) as libc::c_int
                            == '<' as i32
                        || *e_code.offset(-(2 as libc::c_int) as isize) as libc::c_int
                            == '>' as i32)
            {
                (*bb).priority_code = bb_comparisation;
            } else if *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == '+' as i32
                || *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '-' as i32
            {
                (*bb).priority_code = bb_operator6;
            } else if *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == '*' as i32
                || *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '/' as i32
                || *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                    == '%' as i32
            {
                (*bb).priority_code = bb_operator5;
            } else {
                (*bb).priority_code = bb_binary_op;
            }
        }
        18 => {
            (*bb).priority_code = bb_comma;
        }
        _ => {
            if code as libc::c_uint == bb_binary_op as libc::c_int as libc::c_uint
                && (*token as libc::c_int == '&' as i32
                    || *token as libc::c_int == '|' as i32)
                && *token as libc::c_int
                    == *token.offset(1 as libc::c_int as isize) as libc::c_int
            {
                (*bb).priority_code = bb_before_boolean_binary_op;
            } else if *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == ';' as i32
            {
                (*bb).priority_code = bb_semicolon;
            } else {
                (*bb).priority_code = code;
                if code as libc::c_uint == bb_struct_delim as libc::c_int as libc::c_uint
                {
                    if *e_code.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '*' as i32
                    {
                        (*bb).priority_code = bb_operator4;
                    } else {
                        (*bb).priority_code = bb_operator2;
                    }
                }
            }
        }
    }
    set_priority(bb);
    if !buf_break_list.is_null() {
        (*buf_break_list).next = bb;
    }
    (*bb).prev = buf_break_list;
    (*bb).next = 0 as *mut buf_break_st;
    buf_break_list = bb;
    if buf_break.is_null() || (*bb).col <= settings.max_col {
        if better_break(bb, buf_break) != 0 {
            buf_break = bb;
            bb = (*bb).prev;
            while !bb.is_null() {
                let mut obb: *mut buf_break_st_ty = bb;
                bb = (*bb).prev;
                xfree(obb as *mut libc::c_void);
            }
            (*buf_break).prev = 0 as *mut buf_break_st;
        }
    }
}
pub unsafe extern "C" fn clear_buf_break_list(mut pbreak_line: *mut BOOLEAN) {
    let mut bb: *mut buf_break_st_ty = 0 as *mut buf_break_st_ty;
    bb = buf_break_list;
    while !bb.is_null() {
        let mut obb: *mut buf_break_st_ty = bb;
        bb = (*bb).prev;
        xfree(obb as *mut libc::c_void);
    }
    buf_break_list = 0 as *mut buf_break_st_ty;
    buf_break = buf_break_list;
    *pbreak_line = 0 as libc::c_int as BOOLEAN;
}
unsafe extern "C" fn set_next_buf_break(
    mut prev_code_target: libc::c_int,
    mut new_code_target: libc::c_int,
    mut offset: libc::c_int,
    mut pbreak_line: *mut BOOLEAN,
) {
    let mut bb: *mut buf_break_st_ty = 0 as *mut buf_break_st_ty;
    better_break(buf_break, 0 as *const buf_break_st_ty);
    if buf_break_list == buf_break {
        clear_buf_break_list(pbreak_line);
    } else {
        bb = buf_break_list;
        while !bb.is_null() {
            if (*bb).target_col > (*buf_break).target_col
                && settings.lineup_to_parens != 0
            {
                (*bb).target_col -= prev_code_target + offset - new_code_target;
            }
            (*bb).col -= prev_code_target + offset - new_code_target;
            (*bb).offset -= offset;
            (*bb).priority_code_length -= offset;
            (*bb).first_level = (*buf_break).first_level;
            if (*buf_break).priority_newline == 0 {
                (*bb).priority_newline = 0 as libc::c_int;
            }
            set_priority(bb);
            if (*bb).prev == buf_break {
                break;
            }
            bb = (*bb).prev;
        }
        xfree(buf_break as *mut libc::c_void);
        if !bb.is_null() {
            buf_break = bb;
            (*buf_break).prev = 0 as *mut buf_break_st;
            bb = buf_break;
            while !bb.is_null() {
                if !((*bb).col > settings.max_col) {
                    if better_break(bb, buf_break) != 0 {
                        buf_break = bb;
                        bb = (*bb).prev;
                        while !bb.is_null() {
                            let mut obb: *mut buf_break_st_ty = bb;
                            bb = (*bb).prev;
                            xfree(obb as *mut libc::c_void);
                        }
                        bb = buf_break;
                        (*buf_break).prev = 0 as *mut buf_break_st;
                    }
                }
                bb = (*bb).next;
            }
        }
    };
}
unsafe extern "C" fn pad_output(
    mut cur_col: libc::c_int,
    mut target_column: libc::c_int,
) -> libc::c_int {
    let mut offset: libc::c_int = 0 as libc::c_int;
    let mut align_target: libc::c_int = target_column;
    let mut tos: libc::c_int = (*parser_state_tos).tos;
    if cur_col < target_column {
        if settings.use_tabs != 0 && settings.tabsize > 1 as libc::c_int {
            if settings.align_with_spaces != 0 {
                if align_target >= (*parser_state_tos).ind_level {
                    align_target = (*parser_state_tos).ind_level;
                }
                if (*parser_state_tos).last_rw as libc::c_uint
                    == rw_sp_paren as libc::c_int as libc::c_uint
                    && *((*parser_state_tos).p_stack).offset(tos as isize)
                        as libc::c_uint == stmt as libc::c_int as libc::c_uint
                    && *s_code as libc::c_int != 0 && tos > 0 as libc::c_int
                {
                    while !(*((*parser_state_tos).p_stack).offset(tos as isize)
                        as libc::c_uint == ifstmt as libc::c_int as libc::c_uint
                        || *((*parser_state_tos).p_stack).offset(tos as isize)
                            as libc::c_uint == forstmt as libc::c_int as libc::c_uint
                        || *((*parser_state_tos).p_stack).offset(tos as isize)
                            as libc::c_uint == whilestmt as libc::c_int as libc::c_uint)
                    {
                        tos -= 1;
                        if !(tos != 0) {
                            break;
                        }
                    }
                    if tos != 0 {
                        align_target = *((*parser_state_tos).il).offset(tos as isize);
                    }
                } else if *((*parser_state_tos).p_stack).offset(tos as isize)
                    as libc::c_uint == ifstmt as libc::c_int as libc::c_uint
                    || *((*parser_state_tos).p_stack).offset(tos as isize)
                        as libc::c_uint == forstmt as libc::c_int as libc::c_uint
                    || *((*parser_state_tos).p_stack).offset(tos as isize)
                        as libc::c_uint == whilestmt as libc::c_int as libc::c_uint
                {
                    align_target = *((*parser_state_tos).il).offset(tos as isize);
                }
                offset = (align_target - cur_col + 1 as libc::c_int) / settings.tabsize;
                align_target = cur_col + offset * settings.tabsize;
            }
            offset = settings.tabsize - (cur_col - 1 as libc::c_int) % settings.tabsize;
            while cur_col + offset <= align_target {
                putc('\t' as i32, output);
                cur_col += offset;
                offset = settings.tabsize;
            }
        }
        while cur_col < target_column {
            putc(' ' as i32, output);
            cur_col += 1;
            cur_col;
        }
    }
    return cur_col;
}
unsafe extern "C" fn output_substring(
    mut file: *mut FILE,
    mut begin: *const libc::c_char,
    mut end: *const libc::c_char,
) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = begin;
    while p < end {
        putc(*p as libc::c_int, file);
        p = p.offset(1);
        p;
    }
}
unsafe extern "C" fn dump_line_label() -> libc::c_int {
    let mut cur_col: libc::c_int = 0;
    while e_lab > s_lab
        && (*e_lab.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
            || *e_lab.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\t' as i32)
    {
        e_lab = e_lab.offset(-1);
        e_lab;
    }
    cur_col = pad_output(1 as libc::c_int, compute_label_target());
    if settings.force_preproc_width > 0 as libc::c_int
        && *s_lab.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
    {
        let mut preproc_postcrement: libc::c_int = 0;
        let mut p: *mut libc::c_char = &mut *s_lab.offset(1 as libc::c_int as isize)
            as *mut libc::c_char;
        while *p as libc::c_int == ' ' as i32 {
            p = p.offset(1);
            p;
        }
        preproc_postcrement = settings.force_preproc_width;
        if strncmp(
            p,
            b"else\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            preproc_indent -= settings.force_preproc_width;
        } else if !(strncmp(
            p,
            b"if\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || strncmp(
                p,
                b"ifdef\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
        {
            if strncmp(
                p,
                b"elif\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                preproc_indent -= settings.force_preproc_width;
            } else if strncmp(
                p,
                b"endif\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                preproc_indent -= settings.force_preproc_width;
                preproc_postcrement = 0 as libc::c_int;
            } else {
                preproc_postcrement = 0 as libc::c_int;
            }
        }
        if preproc_indent == 0 as libc::c_int {
            fprintf(output, b"#\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(
                output,
                b"#%*s\0" as *const u8 as *const libc::c_char,
                preproc_indent,
                b" \0" as *const u8 as *const libc::c_char,
            );
        }
        fprintf(
            output,
            b"%.*s\0" as *const u8 as *const libc::c_char,
            e_lab.offset_from(p) as libc::c_long as libc::c_int,
            p,
        );
        cur_col = count_columns(
            cur_col + preproc_indent + 1 as libc::c_int,
            p,
            '\0' as i32,
        );
        preproc_indent += preproc_postcrement;
    } else if *s_lab.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
        && (strncmp(
            &mut *s_lab.offset(1 as libc::c_int as isize),
            b"else\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || strncmp(
                &mut *s_lab.offset(1 as libc::c_int as isize),
                b"endif\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
    {
        let mut s: *mut libc::c_char = s_lab;
        if *e_lab.offset(-(1 as libc::c_int) as isize) as libc::c_int == '\n' as i32 {
            e_lab = e_lab.offset(-1);
            e_lab;
        }
        loop {
            let fresh0 = s;
            s = s.offset(1);
            putc(*fresh0 as libc::c_int, output);
            cur_col += 1;
            cur_col;
            if !(s < e_lab && 'a' as i32 <= *s as libc::c_int
                && *s as libc::c_int <= 'z' as i32)
            {
                break;
            }
        }
        while (*s as libc::c_int == ' ' as i32 || *s as libc::c_int == '\t' as i32)
            && s < e_lab
        {
            s = s.offset(1);
            s;
        }
        if s < e_lab {
            if settings.tabsize > 1 as libc::c_int {
                cur_col = pad_output(
                    cur_col,
                    cur_col + settings.tabsize
                        - (cur_col - 1 as libc::c_int) % settings.tabsize,
                );
            } else {
                cur_col = pad_output(cur_col, cur_col + 2 as libc::c_int);
            }
            if *s.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
                && (*s.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
                    || *s.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
            {
                fprintf(
                    output,
                    b"%.*s\0" as *const u8 as *const libc::c_char,
                    e_lab.offset_from(s) as libc::c_long as libc::c_int,
                    s,
                );
            } else {
                fprintf(
                    output,
                    b"/* %.*s */\0" as *const u8 as *const libc::c_char,
                    e_lab.offset_from(s) as libc::c_long as libc::c_int,
                    s,
                );
            }
        }
    } else {
        fprintf(
            output,
            b"%.*s\0" as *const u8 as *const libc::c_char,
            e_lab.offset_from(s_lab) as libc::c_long as libc::c_int,
            s_lab,
        );
        cur_col = count_columns(cur_col, s_lab, '\0' as i32);
    }
    return cur_col;
}
unsafe extern "C" fn count_parens(mut string: *const libc::c_char) -> libc::c_int {
    let mut paren_level: libc::c_int = 0 as libc::c_int;
    while *string != 0 {
        match *string as libc::c_int {
            40 | 91 => {
                paren_level += 1;
                paren_level;
            }
            41 | 93 => {
                paren_level -= 1;
                paren_level;
            }
            _ => {}
        }
        string = string.offset(1);
        string;
    }
    return paren_level;
}
unsafe extern "C" fn dump_line_code(
    mut pcur_col: *mut libc::c_int,
    mut pnot_truncated: *mut libc::c_int,
    mut paren_targ: libc::c_int,
    mut pbreak_line: *mut BOOLEAN,
    mut target_col_break: libc::c_int,
) {
    let mut paren_level: libc::c_int = 0 as libc::c_int;
    if s_code != e_code {
        let mut i: libc::c_int = 0;
        let mut target_col: libc::c_int = 0 as libc::c_int;
        if embedded_comment_on_line == 1 as libc::c_int {
            target_col = (*parser_state_tos).com_col;
        } else if target_col_break != -(1 as libc::c_int) {
            target_col = target_col_break;
        } else {
            target_col = compute_code_target(paren_targ);
        }
        if (*parser_state_tos).last_token as libc::c_uint
            == lparen as libc::c_int as libc::c_uint
        {
            let ref mut fresh1 = *((*parser_state_tos).paren_indents)
                .offset(((*parser_state_tos).p_l_follow - 1 as libc::c_int) as isize);
            *fresh1 = (*fresh1 as libc::c_int + (settings.ind_size - 1 as libc::c_int))
                as libc::c_short;
        }
        *pcur_col = pad_output(*pcur_col, target_col);
        if *pbreak_line as libc::c_int != 0 && s_com == e_com
            && (*buf_break).target_col <= (*buf_break).col
        {
            let mut offset: libc::c_int = 0;
            let mut len: libc::c_int = 0;
            let mut c: libc::c_char = 0;
            let mut ptr: *mut libc::c_char = &mut *s_code
                .offset((*buf_break).offset as isize) as *mut libc::c_char;
            if *ptr as libc::c_int != ' ' as i32 {
                ptr = ptr.offset(-1);
                ptr;
            }
            offset = (ptr.offset_from(s_code) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_int;
            i = 0 as libc::c_int;
            while i < (*parser_state_tos).p_l_follow {
                if *((*parser_state_tos).paren_indents).offset(i as isize) as libc::c_int
                    >= 0 as libc::c_int
                {
                    if (*((*parser_state_tos).paren_indents).offset(i as isize)
                        as libc::c_long) < ptr.offset_from(s_code) as libc::c_long
                    {
                        *((*parser_state_tos).paren_indents)
                            .offset(
                                i as isize,
                            ) = -(*((*parser_state_tos).paren_indents).offset(i as isize)
                            as libc::c_int + target_col) as libc::c_short;
                    } else {
                        let ref mut fresh2 = *((*parser_state_tos).paren_indents)
                            .offset(i as isize);
                        *fresh2 = (*fresh2 as libc::c_int - offset) as libc::c_short;
                    }
                }
                i += 1;
                i;
            }
            i = (*parser_state_tos).p_l_follow;
            while i < (*parser_state_tos).paren_indents_size {
                if *((*parser_state_tos).paren_indents).offset(i as isize)
                    as libc::c_long >= ptr.offset_from(s_code) as libc::c_long
                {
                    let ref mut fresh3 = *((*parser_state_tos).paren_indents)
                        .offset(i as isize);
                    *fresh3 = (*fresh3 as libc::c_int - offset) as libc::c_short;
                }
                i += 1;
                i;
            }
            output_substring(
                output,
                s_code,
                s_code.offset((*buf_break).offset as isize),
            );
            c = *s_code.offset((*buf_break).offset as isize);
            *s_code.offset((*buf_break).offset as isize) = '\0' as i32 as libc::c_char;
            *pcur_col = count_columns(*pcur_col, s_code, '\0' as i32);
            paren_level += count_parens(s_code);
            *s_code.offset((*buf_break).offset as isize) = c;
            *pnot_truncated = 0 as libc::c_int;
            len = (e_code.offset_from(ptr) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as libc::c_int;
            memmove(
                s_code as *mut libc::c_void,
                ptr.offset(1 as libc::c_int as isize) as *const libc::c_void,
                len as libc::c_ulong,
            );
            e_code = s_code.offset(len as isize);
            *e_code = '\0' as i32 as libc::c_char;
            s_code_corresponds_to = (*buf_break).corresponds_to;
            prev_target_col_break = (*buf_break).target_col;
            if (*buf_break).priority_newline == 0 {
                (*parser_state_tos).broken_at_non_nl = 1 as libc::c_int;
            }
            set_next_buf_break(target_col, (*buf_break).target_col, offset, pbreak_line);
            buf_break_used = 1 as libc::c_int;
            *pbreak_line = (!buf_break.is_null()
                && output_line_length() > settings.max_col) as libc::c_int as BOOLEAN;
        } else {
            i = 0 as libc::c_int;
            while i < (*parser_state_tos).p_l_follow {
                if *((*parser_state_tos).paren_indents).offset(i as isize) as libc::c_int
                    >= 0 as libc::c_int
                {
                    *((*parser_state_tos).paren_indents)
                        .offset(
                            i as isize,
                        ) = -(*((*parser_state_tos).paren_indents).offset(i as isize)
                        as libc::c_int + target_col) as libc::c_short;
                }
                i += 1;
                i;
            }
            output_substring(output, s_code, e_code);
            *pcur_col = count_columns(*pcur_col, s_code, '\0' as i32);
            clear_buf_break_list(pbreak_line);
        }
    }
}
pub unsafe extern "C" fn dump_line(
    mut force_nl: libc::c_int,
    mut paren_targ: *mut libc::c_int,
    mut pbreak_line: *mut BOOLEAN,
) {
    let mut cur_col: libc::c_int = 0;
    let mut not_truncated: libc::c_int = 1 as libc::c_int;
    let mut target_col_break: libc::c_int = -(1 as libc::c_int);
    if buf_break_used != 0 {
        buf_break_used = 0 as libc::c_int;
        target_col_break = prev_target_col_break;
    } else if force_nl != 0 {
        (*parser_state_tos).broken_at_non_nl = 0 as libc::c_int;
    }
    if *((*parser_state_tos).procname).offset(0 as libc::c_int as isize) as libc::c_int
        != 0 && *((*parser_state_tos).classname).offset(0 as libc::c_int as isize) == 0
        && s_code_corresponds_to == (*parser_state_tos).procname
    {
        (*parser_state_tos)
            .procname = b"\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if *((*parser_state_tos).procname).offset(0 as libc::c_int as isize)
        as libc::c_int != 0
        && *((*parser_state_tos).classname).offset(0 as libc::c_int as isize)
            as libc::c_int != 0 && s_code_corresponds_to == (*parser_state_tos).classname
    {
        (*parser_state_tos)
            .procname = b"\0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*parser_state_tos)
            .classname = b"\0\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if s_code == e_code && s_lab == e_lab && s_com == e_com {
        if (*parser_state_tos).use_ff != 0 {
            putc('\u{c}' as i32, output);
            (*parser_state_tos).use_ff = 0 as libc::c_int;
        } else {
            n_real_blanklines += 1;
            n_real_blanklines;
        }
    } else {
        if prefix_blankline_requested != 0 && n_real_blanklines == 0 as libc::c_int {
            if prefix_blankline_requested_code as libc::c_uint
                != decl as libc::c_int as libc::c_uint
                || (*parser_state_tos).decl_on_line == 0
            {
                n_real_blanklines = 1 as libc::c_int;
            }
        } else if settings.swallow_optional_blanklines != 0
            && n_real_blanklines > 1 as libc::c_int
        {
            n_real_blanklines = 1 as libc::c_int;
        }
        loop {
            n_real_blanklines -= 1;
            if !(n_real_blanklines >= 0 as libc::c_int) {
                break;
            }
            putc('\n' as i32, output);
        }
        n_real_blanklines = 0 as libc::c_int;
        if e_lab != s_lab || e_code != s_code {
            code_lines += 1;
            code_lines;
        }
        if e_lab != s_lab {
            cur_col = dump_line_label();
        } else {
            cur_col = 1 as libc::c_int;
        }
        (*parser_state_tos).pcase = 0 as libc::c_int;
        while *e_code.offset(-(1 as libc::c_int as isize)) as libc::c_int == ' ' as i32
            && e_code > s_code
        {
            e_code = e_code.offset(-1);
            *e_code = '\0' as i32 as libc::c_char;
        }
        dump_line_code(
            &mut cur_col,
            &mut not_truncated,
            *paren_targ,
            pbreak_line,
            target_col_break,
        );
        if s_com != e_com {
            let mut target: libc::c_int = (*parser_state_tos).com_col;
            let mut com_st: *mut libc::c_char = s_com;
            if cur_col > target {
                putc('\n' as i32, output);
                cur_col = 1 as libc::c_int;
                out_lines += 1;
                out_lines;
            }
            cur_col = pad_output(cur_col, target);
            fwrite(
                com_st as *const libc::c_void,
                e_com.offset_from(com_st) as libc::c_long as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                output,
            );
            cur_col = (cur_col as libc::c_long
                + e_com.offset_from(com_st) as libc::c_long) as libc::c_int;
            com_lines += 1;
            com_lines;
        } else if embedded_comment_on_line != 0 {
            com_lines += 1;
            com_lines;
        }
        embedded_comment_on_line = 0 as libc::c_int;
        if (*parser_state_tos).use_ff != 0 {
            putc('\u{c}' as i32, output);
            (*parser_state_tos).use_ff = 0 as libc::c_int;
        } else {
            putc('\n' as i32, output);
        }
        out_lines += 1;
        out_lines;
        if (*parser_state_tos).just_saw_decl == 1 as libc::c_int
            && settings.blanklines_after_declarations != 0
        {
            prefix_blankline_requested = 1 as libc::c_int;
            prefix_blankline_requested_code = decl;
            (*parser_state_tos).just_saw_decl = 0 as libc::c_int;
        } else {
            prefix_blankline_requested = postfix_blankline_requested;
            prefix_blankline_requested_code = postfix_blankline_requested_code;
        }
        postfix_blankline_requested = 0 as libc::c_int;
    }
    (*parser_state_tos).decl_on_line = (*parser_state_tos).in_decl as libc::c_int;
    (*parser_state_tos).ind_stmt = (*parser_state_tos).in_stmt;
    e_lab = s_lab;
    *s_lab = '\0' as i32 as libc::c_char;
    if not_truncated != 0 {
        e_code = s_code;
        *s_code = '\0' as i32 as libc::c_char;
        s_code_corresponds_to = 0 as *mut libc::c_char;
    }
    e_com = s_com;
    *s_com = '\0' as i32 as libc::c_char;
    (*parser_state_tos).ind_level = (*parser_state_tos).i_l_follow;
    (*parser_state_tos).paren_level = (*parser_state_tos).p_l_follow;
    if (*parser_state_tos).paren_level > 0 as libc::c_int {
        if not_truncated == 0
            && (*s_code as libc::c_int == '(' as i32
                || *s_code as libc::c_int == '[' as i32)
            && (*parser_state_tos).paren_level >= 2 as libc::c_int
        {
            *paren_targ = -(*((*parser_state_tos).paren_indents)
                .offset(((*parser_state_tos).paren_level - 2 as libc::c_int) as isize)
                as libc::c_int);
        } else {
            *paren_targ = -(*((*parser_state_tos).paren_indents)
                .offset(((*parser_state_tos).paren_level - 1 as libc::c_int) as isize)
                as libc::c_int);
        }
    } else {
        *paren_targ = 0 as libc::c_int;
    }
    if inhibited != 0 {
        let mut p: *mut libc::c_char = cur_line;
        loop {
            n_real_blanklines -= 1;
            if !(n_real_blanklines >= 0 as libc::c_int) {
                break;
            }
            putc('\n' as i32, output);
        }
        n_real_blanklines = 0 as libc::c_int;
        loop {
            while *p as libc::c_int != '\0' as i32 && *p as libc::c_int != '\n' as i32 {
                let fresh4 = p;
                p = p.offset(1);
                putc(*fresh4 as libc::c_int, output);
            }
            if *p as libc::c_int == '\0' as i32
                && p.offset_from((*current_input).data) as libc::c_long as libc::c_ulong
                    == (*current_input).size
            {
                in_prog_pos = p;
                buf_end = p;
                buf_ptr = p;
                had_eof = 1 as libc::c_int as BOOLEAN;
                return;
            }
            if *p as libc::c_int == '\n' as i32 {
                cur_line = p.offset(1 as libc::c_int as isize);
                line_no += 1;
                line_no;
            }
            let fresh5 = p;
            p = p.offset(1);
            putc(*fresh5 as libc::c_int, output);
            while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
                putc(*p as libc::c_int, output);
                p = p.offset(1);
                p;
            }
            if *p as libc::c_int == '/' as i32
                && (*p.offset(1 as libc::c_int as isize) as libc::c_int == '*' as i32
                    || *p.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
            {
                let fresh6 = p;
                p = p.offset(1);
                putc(*fresh6 as libc::c_int, output);
                let fresh7 = p;
                p = p.offset(1);
                putc(*fresh7 as libc::c_int, output);
                while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32
                {
                    putc(*p as libc::c_int, output);
                    p = p.offset(1);
                    p;
                }
                if strncmp(
                    p,
                    b"*INDENT-ON*\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    loop {
                        while *p as libc::c_int != '\0' as i32
                            && *p as libc::c_int != '\n' as i32
                        {
                            let fresh8 = p;
                            p = p.offset(1);
                            putc(*fresh8 as libc::c_int, output);
                        }
                        if *p as libc::c_int == '\0' as i32
                            && p.offset_from((*current_input).data) as libc::c_long
                                as libc::c_ulong == (*current_input).size
                        {
                            in_prog_pos = p;
                            buf_end = p;
                            buf_ptr = p;
                            had_eof = 1 as libc::c_int as BOOLEAN;
                            return;
                        } else {
                            if *p as libc::c_int == '\n' as i32 {
                                inhibited = 0 as libc::c_int as BOOLEAN;
                                cur_line = p.offset(1 as libc::c_int as isize);
                                line_no += 1;
                                line_no;
                            }
                            let fresh9 = p;
                            p = p.offset(1);
                            putc(*fresh9 as libc::c_int, output);
                        }
                        if !(inhibited != 0) {
                            break;
                        }
                    }
                }
            }
            if !(inhibited != 0) {
                break;
            }
        }
        in_prog_pos = cur_line;
        buf_end = cur_line;
        buf_ptr = cur_line;
        fill_buffer();
    }
    if buf_break_used != 0 && s_code != e_code && force_nl != 0 {
        prefix_blankline_requested = 0 as libc::c_int;
        dump_line(1 as libc::c_int, paren_targ, pbreak_line);
    }
}
pub unsafe extern "C" fn flush_output() {
    fflush(output);
}
pub unsafe extern "C" fn open_output(
    mut filename: *const libc::c_char,
    mut mode: *const libc::c_char,
) {
    if filename.is_null() {
        output = stdout;
    } else {
        output = fopen(filename, mode);
        if output.is_null() {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"indent: can't create %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
        }
    };
}
pub unsafe extern "C" fn reopen_output_trunc(mut filename: *const libc::c_char) {
    output = freopen(filename, b"w\0" as *const u8 as *const libc::c_char, output);
}
pub unsafe extern "C" fn close_output(
    mut file_stats: *mut stat,
    mut filename: *const libc::c_char,
) {
    if output != stdout {
        if fclose(output) != 0 as libc::c_int {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Can't close output file %s\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                filename,
            );
        } else if !file_stats.is_null() && !filename.is_null() {
            let mut buf: utimbuf = utimbuf { actime: 0, modtime: 0 };
            buf.actime = time(0 as *mut time_t);
            buf.modtime = (*file_stats).st_mtim.tv_sec;
            if utime(filename, &mut buf) != 0 as libc::c_int {
                message(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Warning\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Can't preserve modification time on output file %s\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    filename as *mut libc::c_char,
                    0 as *mut libc::c_char,
                );
            }
        }
    }
}
pub unsafe extern "C" fn inhibit_indenting(mut flag: BOOLEAN) {
    inhibited = flag;
}
pub unsafe extern "C" fn compute_code_target(
    mut paren_targ: libc::c_int,
) -> libc::c_int {
    let mut target_col: libc::c_int = 0;
    if buf_break_used != 0 {
        return prev_target_col_break;
    }
    if *((*parser_state_tos).procname).offset(0 as libc::c_int as isize) as libc::c_int
        != 0 && s_code_corresponds_to == (*parser_state_tos).procname
    {
        target_col = 1 as libc::c_int;
        if (*parser_state_tos).paren_level == 0 {
            return target_col;
        }
    } else {
        target_col = (*parser_state_tos).ind_level + 1 as libc::c_int;
    }
    if (*parser_state_tos).paren_level == 0 {
        if (*parser_state_tos).ind_stmt != 0 {
            target_col += settings.continuation_indent;
        }
        return target_col;
    }
    if settings.lineup_to_parens == 0 {
        return target_col + settings.continuation_indent
            + settings.paren_indent
                * ((*parser_state_tos).paren_level - 1 as libc::c_int);
    }
    return paren_targ;
}
pub unsafe extern "C" fn count_columns(
    mut column: libc::c_int,
    mut bp: *mut libc::c_char,
    mut stop_char: libc::c_int,
) -> libc::c_int {
    while *bp as libc::c_int != stop_char && *bp as libc::c_int != '\0' as i32 {
        let fresh10 = bp;
        bp = bp.offset(1);
        match *fresh10 as libc::c_int {
            10 | 12 => {
                column = 1 as libc::c_int;
            }
            9 => {
                column
                    += settings.tabsize - (column - 1 as libc::c_int) % settings.tabsize;
            }
            8 => {
                column -= 1;
                column;
            }
            _ => {
                column += 1;
                column;
            }
        }
    }
    return column;
}
pub unsafe extern "C" fn compute_label_target() -> libc::c_int {
    if *s_lab as libc::c_int == '#' as i32 {
        return 1 as libc::c_int;
    }
    if (*parser_state_tos).pcase != 0 {
        return *((*parser_state_tos).cstk).offset((*parser_state_tos).tos as isize)
            + 1 as libc::c_int;
    }
    if settings.c_plus_plus != 0 && (*parser_state_tos).in_decl as libc::c_int != 0 {
        return 1 as libc::c_int
    } else if settings.label_offset < 0 as libc::c_int {
        return (*parser_state_tos).ind_level + settings.label_offset + 1 as libc::c_int
    } else {
        return settings.label_offset + 1 as libc::c_int
    };
}
pub unsafe extern "C" fn output_line_length() -> libc::c_int {
    let mut code_length: libc::c_int = 0 as libc::c_int;
    let mut com_length: libc::c_int = 0 as libc::c_int;
    let mut length: libc::c_int = 0;
    if s_lab == e_lab {
        length = 0 as libc::c_int;
    } else {
        length = count_columns(compute_label_target(), s_lab, '\n' as i32)
            - 1 as libc::c_int;
    }
    if s_code != e_code {
        let mut code_col: libc::c_int = compute_code_target(paren_target);
        code_length = count_columns(code_col, s_code, '\n' as i32) - code_col;
    }
    if s_com != e_com {
        let mut com_col: libc::c_int = (*parser_state_tos).com_col;
        com_length = count_columns(com_col, s_com, '\n' as i32) - com_col;
    }
    if code_length != 0 as libc::c_int {
        length += compute_code_target(paren_target) - 1 as libc::c_int + code_length;
        if embedded_comment_on_line != 0 {
            length += com_length;
        }
    }
    return length;
}

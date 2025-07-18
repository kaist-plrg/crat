use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn addkey(key: *mut libc::c_char, val: rwcodes_ty);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(size: libc::c_uint) -> *mut libc::c_void;
    fn xfree(ptr: *mut libc::c_void);
    fn fatal(string: *const libc::c_char, a0: *const libc::c_char);
    fn message(
        kind: *mut libc::c_char,
        string: *mut libc::c_char,
        a0: *mut libc::c_char,
        a1: *mut libc::c_char,
    );
    fn DieError(errval: libc::c_int, fmt: *const libc::c_char, _: ...);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type size_t = libc::c_ulong;
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
pub type exit_values = libc::c_uint;
pub const system_error: exit_values = 5;
pub const indent_fatal: exit_values = 4;
pub const indent_punt: exit_values = 3;
pub const indent_error: exit_values = 2;
pub const invocation_error: exit_values = 64;
pub const total_success: exit_values = 0;
pub type BOOLEAN = libc::c_uchar;
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
pub struct pro_ty {
    pub p_name: *const libc::c_char,
    pub p_type: profile_ty,
    pub p_default: libc::c_int,
    pub p_special: on_or_off_ty,
    pub p_obj: *mut libc::c_void,
    pub p_explicit: *mut libc::c_int,
}
pub type on_or_off_ty = libc::c_uint;
pub const ON: on_or_off_ty = 2;
pub const OFF: on_or_off_ty = 1;
pub const ONOFF_NA: on_or_off_ty = 0;
pub type profile_ty = libc::c_uint;
pub const PRO_PRSTRING: profile_ty = 5;
pub const PRO_SETTINGS: profile_ty = 4;
pub const PRO_KEY: profile_ty = 3;
pub const PRO_IGN: profile_ty = 2;
pub const PRO_INT: profile_ty = 1;
pub const PRO_BOOL: profile_ty = 0;
pub type long_option_conversion_ty = long_option_conversion;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct long_option_conversion {
    pub long_name: *const libc::c_char,
    pub short_name: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
pub static mut settings_strings: [*const libc::c_char; 6] = [
    b"-nbad\0-bap\0-nbc\0-bbo\0-hnl\0-br\0-brs\0-c33\0-cd33\0-ncdb\0-ce\0-ci4\0-cli0\0-d0\0-di1\0-nfc1\0-i4\0-ip0\0-l75\0-lp\0-npcs\0-nprs\0-npsl\0-sai\0-saf\0-saw\0-cs\0-nsc\0-nsob\0-nfca\0-cp33\0-nss\0-par\0-sar\0\0"
        as *const u8 as *const libc::c_char,
    b"-nbad\0-bap\0-bbo\0-hnl\0-nbc\0-bl\0-bls\0-ncdb\0-cs\0-nce\0-di2\0-ndj\0-nfc1\0-i2\0-ip5\0-lp\0-pcs\0-nprs\0-psl\0-nsc\0-sai\0-saf\0-saw\0-nsob\0-bli2\0-cp1\0-nfca\0\0"
        as *const u8 as *const libc::c_char,
    b"-nbap\0-nbad\0-bbo\0-hnl\0-bc\0-br\0-brs\0-c33\0-cd33\0-cdb\0-ce\0-ci4\0-cli0\0-cp33\0-di16\0-fc1\0-fca\0-i4\0-l75\0-lp\0-npcs\0-nprs\0-psl\0-sc\0-sai\0-saf\0-saw\0-nsob\0-nss\0-ts8\0\0"
        as *const u8 as *const libc::c_char,
    b"-nbad\0-bap\0-nbc\0-bbo\0-hnl\0-br\0-brs\0-c33\0-cd33\0-ncdb\0-ce\0-ci4\0-cli0\0-d0\0-di1\0-nfc1\0-i8\0-ip0\0-l80\0-lp\0-npcs\0-nprs\0-npsl\0-sai\0-saf\0-saw\0-ncs\0-nsc\0-sob\0-nfca\0-cp33\0-ss\0-ts8\0-il1\0-nbs\0\0"
        as *const u8 as *const libc::c_char,
    b"-ip0\0\0" as *const u8 as *const libc::c_char,
    b"2.2.13\0" as *const u8 as *const libc::c_char,
];
static mut exp_T: libc::c_int = 0 as libc::c_int;
static mut exp_bacc: libc::c_int = 0 as libc::c_int;
static mut exp_badp: libc::c_int = 0 as libc::c_int;
static mut exp_bad: libc::c_int = 0 as libc::c_int;
static mut exp_bap: libc::c_int = 0 as libc::c_int;
static mut exp_bbb: libc::c_int = 0 as libc::c_int;
static mut exp_bbo: libc::c_int = 0 as libc::c_int;
static mut exp_bc: libc::c_int = 0 as libc::c_int;
static mut exp_bl: libc::c_int = 0 as libc::c_int;
static mut exp_blf: libc::c_int = 0 as libc::c_int;
static mut exp_bli: libc::c_int = 0 as libc::c_int;
static mut exp_bls: libc::c_int = 0 as libc::c_int;
static mut exp_bs: libc::c_int = 0 as libc::c_int;
static mut exp_c: libc::c_int = 0 as libc::c_int;
static mut exp_cbi: libc::c_int = 0 as libc::c_int;
static mut exp_cdb: libc::c_int = 0 as libc::c_int;
static mut exp_cd: libc::c_int = 0 as libc::c_int;
static mut exp_cdw: libc::c_int = 0 as libc::c_int;
static mut exp_ce: libc::c_int = 0 as libc::c_int;
static mut exp_ci: libc::c_int = 0 as libc::c_int;
static mut exp_cli: libc::c_int = 0 as libc::c_int;
static mut exp_cp: libc::c_int = 0 as libc::c_int;
static mut exp_cpp: libc::c_int = 0 as libc::c_int;
static mut exp_cs: libc::c_int = 0 as libc::c_int;
static mut exp_d: libc::c_int = 0 as libc::c_int;
static mut exp_bfda: libc::c_int = 0 as libc::c_int;
static mut exp_bfde: libc::c_int = 0 as libc::c_int;
static mut exp_di: libc::c_int = 0 as libc::c_int;
static mut exp_dj: libc::c_int = 0 as libc::c_int;
static mut exp_eei: libc::c_int = 0 as libc::c_int;
static mut exp_fc1: libc::c_int = 0 as libc::c_int;
static mut exp_fca: libc::c_int = 0 as libc::c_int;
static mut exp_fnc: libc::c_int = 0 as libc::c_int;
static mut exp_gnu: libc::c_int = 0 as libc::c_int;
static mut exp_gts: libc::c_int = 0 as libc::c_int;
static mut exp_hnl: libc::c_int = 0 as libc::c_int;
static mut exp_i: libc::c_int = 0 as libc::c_int;
static mut exp_il: libc::c_int = 0 as libc::c_int;
static mut exp_ip: libc::c_int = 0 as libc::c_int;
static mut exp_kr: libc::c_int = 0 as libc::c_int;
static mut exp_l: libc::c_int = 0 as libc::c_int;
static mut exp_lc: libc::c_int = 0 as libc::c_int;
static mut exp_linux: libc::c_int = 0 as libc::c_int;
static mut exp_lp: libc::c_int = 0 as libc::c_int;
static mut exp_lps: libc::c_int = 0 as libc::c_int;
static mut exp_nip: libc::c_int = 0 as libc::c_int;
static mut exp_o: libc::c_int = 0 as libc::c_int;
static mut exp_orig: libc::c_int = 0 as libc::c_int;
static mut exp_pcs: libc::c_int = 0 as libc::c_int;
static mut exp_pi: libc::c_int = 0 as libc::c_int;
static mut exp_pmt: libc::c_int = 0 as libc::c_int;
static mut exp_pro: libc::c_int = 0 as libc::c_int;
static mut exp_prs: libc::c_int = 0 as libc::c_int;
static mut exp_psl: libc::c_int = 0 as libc::c_int;
static mut exp_ppi: libc::c_int = 0 as libc::c_int;
static mut exp_sai: libc::c_int = 0 as libc::c_int;
static mut exp_saf: libc::c_int = 0 as libc::c_int;
static mut exp_saw: libc::c_int = 0 as libc::c_int;
static mut exp_sbi: libc::c_int = 0 as libc::c_int;
static mut exp_sc: libc::c_int = 0 as libc::c_int;
static mut exp_sob: libc::c_int = 0 as libc::c_int;
static mut exp_ss: libc::c_int = 0 as libc::c_int;
static mut exp_st: libc::c_int = 0 as libc::c_int;
static mut exp_ts: libc::c_int = 0 as libc::c_int;
static mut exp_ut: libc::c_int = 0 as libc::c_int;
static mut exp_v: libc::c_int = 0 as libc::c_int;
static mut exp_version: libc::c_int = 0 as libc::c_int;
static mut exp_par: libc::c_int = 0 as libc::c_int;
static mut exp_slc: libc::c_int = 0 as libc::c_int;
static mut exp_as: libc::c_int = 0 as libc::c_int;
static mut exp_sar: libc::c_int = 0 as libc::c_int;
static mut exp_ntac: libc::c_int = 0 as libc::c_int;
pub static mut settings: user_options_ty = user_options_ty {
    verbose: 0,
    use_tabs: 0,
    tabsize: 0,
    use_stdout: 0,
    space_sp_semicolon: 0,
    swallow_optional_blanklines: 0,
    star_comment_cont: 0,
    struct_brace_indent: 0,
    space_after_while: 0,
    space_after_if: 0,
    space_after_for: 0,
    procnames_start_line: 0,
    parentheses_space: 0,
    preserve_mtime: 0,
    paren_indent: 0,
    proc_calls_space: 0,
    leave_preproc_space: 0,
    force_preproc_width: 0,
    lineup_to_parens: 0,
    honour_newlines: 0,
    fix_nested_comments: 0,
    format_comments: 0,
    format_col1_comments: 0,
    extra_expression_indent: 0,
    ljust_decl: 0,
    cast_space: 0,
    cuddle_else: 0,
    cuddle_do_while: 0,
    comment_delimiter_on_blankline: 0,
    blank_after_sizeof: 0,
    break_function_decl_args: 0,
    break_function_decl_args_end: 0,
    leave_comma: 0,
    break_before_boolean_operator: 0,
    blanklines_before_blockcomments: 0,
    blanklines_after_declarations: 0,
    blanklines_after_procs: 0,
    blanklines_after_declarations_at_proctop: 0,
    blanklines_around_conditional_compilation: 0,
    comment_max_col: 0,
    max_col: 0,
    label_offset: 0,
    ind_size: 0,
    indent_parameters: 0,
    decl_indent: 0,
    unindent_displace: 0,
    else_endif_col: 0,
    case_indent: 0,
    continuation_indent: 0,
    decl_com_ind: 0,
    case_brace_indent: 0,
    c_plus_plus: 0,
    com_ind: 0,
    braces_on_struct_decl_line: 0,
    braces_on_func_def_line: 0,
    btype_2: 0,
    brace_indent: 0,
    expect_output_file: 0,
    pointer_align_right: 0,
    gettext_strings: 0,
    allow_single_line_conditionals: 0,
    align_with_spaces: 0,
    spaces_around_initializers: 0,
    dont_tab_align_comments: 0,
};
pub static mut pro: [pro_ty; 112] = [pro_ty {
    p_name: 0 as *const libc::c_char,
    p_type: PRO_BOOL,
    p_default: 0,
    p_special: ONOFF_NA,
    p_obj: 0 as *mut libc::c_void,
    p_explicit: 0 as *mut libc::c_int,
}; 112];
pub static mut option_conversions: [long_option_conversion_ty; 123] = [
    {
        let mut init = long_option_conversion {
            long_name: b"version\0" as *const u8 as *const libc::c_char,
            short_name: b"version\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"verbose\0" as *const u8 as *const libc::c_char,
            short_name: b"v\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"usage\0" as *const u8 as *const libc::c_char,
            short_name: b"h\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"use-tabs\0" as *const u8 as *const libc::c_char,
            short_name: b"ut\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"tab-size\0" as *const u8 as *const libc::c_char,
            short_name: b"ts\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"swallow-optional-blank-lines\0" as *const u8
                as *const libc::c_char,
            short_name: b"sob\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"struct-brace-indentation\0" as *const u8 as *const libc::c_char,
            short_name: b"sbi\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"start-left-side-of-comments\0" as *const u8
                as *const libc::c_char,
            short_name: b"sc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"standard-output\0" as *const u8 as *const libc::c_char,
            short_name: b"st\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"space-special-semicolon\0" as *const u8 as *const libc::c_char,
            short_name: b"ss\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"space-after-while\0" as *const u8 as *const libc::c_char,
            short_name: b"saw\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"space-after-procedure-calls\0" as *const u8
                as *const libc::c_char,
            short_name: b"pcs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"space-after-parentheses\0" as *const u8 as *const libc::c_char,
            short_name: b"prs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"space-after-if\0" as *const u8 as *const libc::c_char,
            short_name: b"sai\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"space-after-for\0" as *const u8 as *const libc::c_char,
            short_name: b"saf\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"space-after-cast\0" as *const u8 as *const libc::c_char,
            short_name: b"cs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"remove-preprocessor-space\0" as *const u8
                as *const libc::c_char,
            short_name: b"nlps\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"procnames-start-lines\0" as *const u8 as *const libc::c_char,
            short_name: b"psl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"preserve-mtime\0" as *const u8 as *const libc::c_char,
            short_name: b"pmt\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"paren-indentation\0" as *const u8 as *const libc::c_char,
            short_name: b"pi\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"parameter-indentation\0" as *const u8 as *const libc::c_char,
            short_name: b"ip\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"output-file\0" as *const u8 as *const libc::c_char,
            short_name: b"o\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"output\0" as *const u8 as *const libc::c_char,
            short_name: b"o\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"original-style\0" as *const u8 as *const libc::c_char,
            short_name: b"orig\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"original\0" as *const u8 as *const libc::c_char,
            short_name: b"orig\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-verbosity\0" as *const u8 as *const libc::c_char,
            short_name: b"nv\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-tabs\0" as *const u8 as *const libc::c_char,
            short_name: b"nut\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-space-after-while\0" as *const u8 as *const libc::c_char,
            short_name: b"nsaw\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-space-after-parentheses\0" as *const u8
                as *const libc::c_char,
            short_name: b"nprs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-space-after-if\0" as *const u8 as *const libc::c_char,
            short_name: b"nsai\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-space-after-function-call-names\0" as *const u8
                as *const libc::c_char,
            short_name: b"npcs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-space-after-for\0" as *const u8 as *const libc::c_char,
            short_name: b"nsaf\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-space-after-cast\0" as *const u8 as *const libc::c_char,
            short_name: b"ncs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-space-after-casts\0" as *const u8 as *const libc::c_char,
            short_name: b"ncs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-parameter-indentation\0" as *const u8 as *const libc::c_char,
            short_name: b"nip\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-extra-expression-indentation\0" as *const u8
                as *const libc::c_char,
            short_name: b"neei\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-gettext-strings\0" as *const u8 as *const libc::c_char,
            short_name: b"ngts\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-comment-delimiters-on-blank-lines\0" as *const u8
                as *const libc::c_char,
            short_name: b"ncdb\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-blank-lines-before-block-comments\0" as *const u8
                as *const libc::c_char,
            short_name: b"nbbb\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-blank-lines-after-procedures\0" as *const u8
                as *const libc::c_char,
            short_name: b"nbap\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-blank-lines-after-procedure-declarations\0" as *const u8
                as *const libc::c_char,
            short_name: b"nbadp\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-blank-lines-after-ifdefs\0" as *const u8
                as *const libc::c_char,
            short_name: b"nbacc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-blank-lines-after-declarations\0" as *const u8
                as *const libc::c_char,
            short_name: b"nbad\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-blank-lines-after-commas\0" as *const u8
                as *const libc::c_char,
            short_name: b"nbc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-blank-before-sizeof\0" as *const u8 as *const libc::c_char,
            short_name: b"nbs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"no-Bill-Shannon\0" as *const u8 as *const libc::c_char,
            short_name: b"nbs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"label-offset\0" as *const u8 as *const libc::c_char,
            short_name: b"il\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"line-length\0" as *const u8 as *const libc::c_char,
            short_name: b"l\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"line-comments-indentation\0" as *const u8
                as *const libc::c_char,
            short_name: b"d\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"linux-style\0" as *const u8 as *const libc::c_char,
            short_name: b"linux\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"left-justify-declarations\0" as *const u8
                as *const libc::c_char,
            short_name: b"dj\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"leave-preprocessor-space\0" as *const u8 as *const libc::c_char,
            short_name: b"lps\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"leave-optional-blank-lines\0" as *const u8
                as *const libc::c_char,
            short_name: b"nsob\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"kernighan-and-ritchie-style\0" as *const u8
                as *const libc::c_char,
            short_name: b"kr\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"kernighan-and-ritchie\0" as *const u8 as *const libc::c_char,
            short_name: b"kr\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"k-and-r-style\0" as *const u8 as *const libc::c_char,
            short_name: b"kr\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"indent-label\0" as *const u8 as *const libc::c_char,
            short_name: b"il\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"indentation-level\0" as *const u8 as *const libc::c_char,
            short_name: b"i\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"indent-level\0" as *const u8 as *const libc::c_char,
            short_name: b"i\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"ignore-profile\0" as *const u8 as *const libc::c_char,
            short_name: b"npro\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"ignore-newlines\0" as *const u8 as *const libc::c_char,
            short_name: b"nhnl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"honour-newlines\0" as *const u8 as *const libc::c_char,
            short_name: b"hnl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"help\0" as *const u8 as *const libc::c_char,
            short_name: b"h\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"gettext-strings\0" as *const u8 as *const libc::c_char,
            short_name: b"gts\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"gnu-style\0" as *const u8 as *const libc::c_char,
            short_name: b"gnu\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"fix-nested-comments\0" as *const u8 as *const libc::c_char,
            short_name: b"fnc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"format-first-column-comments\0" as *const u8
                as *const libc::c_char,
            short_name: b"fc1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"format-all-comments\0" as *const u8 as *const libc::c_char,
            short_name: b"fca\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"extra-expression-indentation\0" as *const u8
                as *const libc::c_char,
            short_name: b"eei\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"else-endif-column\0" as *const u8 as *const libc::c_char,
            short_name: b"cp\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-star-comments\0" as *const u8 as *const libc::c_char,
            short_name: b"nsc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-space-special-semicolon\0" as *const u8
                as *const libc::c_char,
            short_name: b"nss\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-line-up-parentheses\0" as *const u8 as *const libc::c_char,
            short_name: b"nlp\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-left-justify-declarations\0" as *const u8
                as *const libc::c_char,
            short_name: b"ndj\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-indent-parameters\0" as *const u8 as *const libc::c_char,
            short_name: b"nip\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-format-first-column-comments\0" as *const u8
                as *const libc::c_char,
            short_name: b"nfc1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-format-comments\0" as *const u8 as *const libc::c_char,
            short_name: b"nfca\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-cuddle-else\0" as *const u8 as *const libc::c_char,
            short_name: b"nce\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-cuddle-do-while\0" as *const u8 as *const libc::c_char,
            short_name: b"ncdw\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-break-procedure-type\0" as *const u8
                as *const libc::c_char,
            short_name: b"npsl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-break-function-decl-args\0" as *const u8
                as *const libc::c_char,
            short_name: b"nbfda\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-break-function-decl-args-end\0" as *const u8
                as *const libc::c_char,
            short_name: b"nbfde\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"declaration-indentation\0" as *const u8 as *const libc::c_char,
            short_name: b"di\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"declaration-comment-column\0" as *const u8
                as *const libc::c_char,
            short_name: b"cd\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"cuddle-else\0" as *const u8 as *const libc::c_char,
            short_name: b"ce\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"cuddle-do-while\0" as *const u8 as *const libc::c_char,
            short_name: b"cdw\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"continue-at-parentheses\0" as *const u8 as *const libc::c_char,
            short_name: b"lp\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"continuation-indentation\0" as *const u8 as *const libc::c_char,
            short_name: b"ci\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"comment-line-length\0" as *const u8 as *const libc::c_char,
            short_name: b"lc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"comment-indentation\0" as *const u8 as *const libc::c_char,
            short_name: b"c\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"comment-delimiters-on-blank-lines\0" as *const u8
                as *const libc::c_char,
            short_name: b"cdb\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"case-indentation\0" as *const u8 as *const libc::c_char,
            short_name: b"cli\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"case-brace-indentation\0" as *const u8 as *const libc::c_char,
            short_name: b"cbi\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"c-plus-plus\0" as *const u8 as *const libc::c_char,
            short_name: b"c++\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"break-function-decl-args\0" as *const u8 as *const libc::c_char,
            short_name: b"bfda\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"break-function-decl-args-end\0" as *const u8
                as *const libc::c_char,
            short_name: b"bfde\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"break-before-boolean-operator\0" as *const u8
                as *const libc::c_char,
            short_name: b"bbo\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"break-after-boolean-operator\0" as *const u8
                as *const libc::c_char,
            short_name: b"nbbo\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"braces-on-struct-decl-line\0" as *const u8
                as *const libc::c_char,
            short_name: b"brs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"braces-on-func-def-line\0" as *const u8 as *const libc::c_char,
            short_name: b"brf\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"braces-on-if-line\0" as *const u8 as *const libc::c_char,
            short_name: b"br\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"braces-after-struct-decl-line\0" as *const u8
                as *const libc::c_char,
            short_name: b"bls\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"braces-after-func-def-line\0" as *const u8
                as *const libc::c_char,
            short_name: b"blf\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"braces-after-if-line\0" as *const u8 as *const libc::c_char,
            short_name: b"bl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"brace-indent\0" as *const u8 as *const libc::c_char,
            short_name: b"bli\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"blank-lines-before-block-comments\0" as *const u8
                as *const libc::c_char,
            short_name: b"bbb\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"blank-lines-after-procedures\0" as *const u8
                as *const libc::c_char,
            short_name: b"bap\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"blank-lines-after-procedure-declarations\0" as *const u8
                as *const libc::c_char,
            short_name: b"badp\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"blank-lines-after-ifdefs\0" as *const u8 as *const libc::c_char,
            short_name: b"bacc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"blank-lines-after-declarations\0" as *const u8
                as *const libc::c_char,
            short_name: b"bad\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"blank-lines-after-commas\0" as *const u8 as *const libc::c_char,
            short_name: b"bc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"blank-before-sizeof\0" as *const u8 as *const libc::c_char,
            short_name: b"bs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"berkeley-style\0" as *const u8 as *const libc::c_char,
            short_name: b"orig\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"berkeley\0" as *const u8 as *const libc::c_char,
            short_name: b"orig\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"Bill-Shannon\0" as *const u8 as *const libc::c_char,
            short_name: b"bs\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"preprocessor-indentation\0" as *const u8 as *const libc::c_char,
            short_name: b"ppi\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"pointer-align-right\0" as *const u8 as *const libc::c_char,
            short_name: b"par\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"pointer-align-left\0" as *const u8 as *const libc::c_char,
            short_name: b"pal\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"single-line-conditionals\0" as *const u8 as *const libc::c_char,
            short_name: b"slc\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"align-with-spaces\0" as *const u8 as *const libc::c_char,
            short_name: b"as\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"spaces-around-initializers\0" as *const u8
                as *const libc::c_char,
            short_name: b"sar\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: b"dont-tab-align-comments\0" as *const u8 as *const libc::c_char,
            short_name: b"ntac\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = long_option_conversion {
            long_name: 0 as *const libc::c_char,
            short_name: 0 as *const libc::c_char,
        };
        init
    },
];
unsafe extern "C" fn usage() {
    DieError(
        invocation_error as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"usage: indent file [-o outfile ] [ options ]\n       indent file1 file2 ... fileN [ options ]\n\0"
                as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
}
unsafe extern "C" fn eqin(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
    mut start_param: *mut *const libc::c_char,
) -> BOOLEAN {
    let mut ret: BOOLEAN = 1 as libc::c_int as BOOLEAN;
    while *s1 != 0 {
        let fresh0 = s1;
        s1 = s1.offset(1);
        let fresh1 = s2;
        s2 = s2.offset(1);
        if *fresh0 as libc::c_int != *fresh1 as libc::c_int {
            ret = 0 as libc::c_int as BOOLEAN;
        }
    }
    *start_param = s2;
    return ret;
}
pub unsafe extern "C" fn set_defaults() {
    let mut p: *const pro_ty = 0 as *const pro_ty;
    p = pro.as_ptr();
    while !((*p).p_name).is_null() {
        if !((*p).p_obj).is_null()
            && ((*p).p_type as libc::c_uint == PRO_BOOL as libc::c_int as libc::c_uint
                && (*p).p_special as libc::c_uint == ON as libc::c_int as libc::c_uint
                || (*p).p_type as libc::c_uint == PRO_INT as libc::c_int as libc::c_uint)
        {
            *((*p).p_obj as *mut libc::c_int) = (*p).p_default;
        }
        p = p.offset(1);
        p;
    }
}
pub unsafe extern "C" fn set_defaults_after() {
    if exp_lc == 0 {
        settings.comment_max_col = settings.max_col;
    }
}
unsafe extern "C" fn arg_missing(
    mut option: *const libc::c_char,
    mut option_source: *const libc::c_char,
) {
    DieError(
        invocation_error as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"%s: missing argument to parameter %s\n\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        option_source,
        option,
    );
}
static mut option_prefixes: [*const libc::c_char; 4] = [
    b"--\0" as *const u8 as *const libc::c_char,
    b"-\0" as *const u8 as *const libc::c_char,
    b"+\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn option_prefix(mut arg: *const libc::c_char) -> size_t {
    let mut prefixes: *mut *const libc::c_char = option_prefixes.as_mut_ptr();
    let mut this_prefix: *const libc::c_char = *prefixes;
    let mut argp: *const libc::c_char = arg;
    let mut ret: size_t = 0 as libc::c_int as size_t;
    loop {
        this_prefix = *prefixes;
        argp = arg;
        while *this_prefix as libc::c_int == *argp as libc::c_int {
            this_prefix = this_prefix.offset(1);
            this_prefix;
            argp = argp.offset(1);
            argp;
        }
        if *this_prefix as libc::c_int == '\0' as i32 {
            ret = this_prefix.offset_from(*prefixes) as libc::c_long as size_t;
            break;
        } else {
            prefixes = prefixes.offset(1);
            if (*prefixes).is_null() {
                break;
            }
        }
    }
    return ret;
}
pub unsafe extern "C" fn set_option(
    mut option: *const libc::c_char,
    mut param: *const libc::c_char,
    mut explicit: libc::c_int,
    mut option_source: *const libc::c_char,
) -> libc::c_int {
    let mut p: *const pro_ty = pro.as_ptr();
    let mut param_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut option_length: size_t = option_prefix(option);
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut found: BOOLEAN = 0 as libc::c_int as BOOLEAN;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ctmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut param_len: size_t = 0 as libc::c_int as size_t;
    if option_length > 0 as libc::c_int as libc::c_ulong {
        if option_length == 1 as libc::c_int as libc::c_ulong
            && *option as libc::c_int == '-' as i32
        {
            option = option.offset(1);
            option;
            p = pro.as_ptr();
            while !((*p).p_name).is_null() {
                if *(*p).p_name as libc::c_int == *option as libc::c_int
                    && eqin((*p).p_name, option, &mut param_start) as libc::c_int != 0
                {
                    found = 1 as libc::c_int as BOOLEAN;
                    break;
                } else {
                    p = p.offset(1);
                    p;
                }
            }
        } else {
            let mut o: *const long_option_conversion_ty = option_conversions.as_ptr();
            option = option.offset(option_length as isize);
            while !((*o).short_name).is_null() {
                if eqin((*o).long_name, option, &mut param_start) != 0 {
                    break;
                }
                o = o.offset(1);
                o;
            }
            if !((*o).short_name).is_null() {
                p = pro.as_ptr();
                while !((*p).p_name).is_null() {
                    if strcmp((*p).p_name, (*o).short_name) == 0 {
                        found = 1 as libc::c_int as BOOLEAN;
                        break;
                    } else {
                        p = p.offset(1);
                        p;
                    }
                }
            }
        }
    }
    if found == 0 {
        DieError(
            invocation_error as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: unknown option \"%s\"\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            option_source,
            option.offset(-(1 as libc::c_int as isize)),
        );
    } else if strlen((*p).p_name) == 1 as libc::c_int as libc::c_ulong
        && *(*p).p_name as libc::c_int == 'h' as i32
    {
        usage();
    } else {
        if settings.verbose != 0 {
            fprintf(
                stderr,
                dcgettext(
                    0 as *const libc::c_char,
                    b"option: %s\n\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                (*p).p_name,
            );
        }
        if explicit != 0 || *(*p).p_explicit == 0 {
            if explicit != 0 {
                *(*p).p_explicit = 1 as libc::c_int;
            }
            match (*p).p_type as libc::c_uint {
                5 => {
                    printf(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"GNU indent %s\n\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        settings_strings[(*p).p_obj as size_t as usize],
                    );
                    exit(invocation_error as libc::c_int);
                }
                4 => {
                    ctmp = settings_strings[(*p).p_obj as size_t as usize];
                    loop {
                        set_option(
                            ctmp,
                            0 as *const libc::c_char,
                            0 as libc::c_int,
                            option_source,
                        );
                        loop {
                            let fresh2 = ctmp;
                            ctmp = ctmp.offset(1);
                            if !(*fresh2 != 0) {
                                break;
                            }
                        }
                        if !(*ctmp != 0) {
                            break;
                        }
                    }
                }
                2 => {}
                3 => {
                    if *param_start as libc::c_int == 0 as libc::c_int {
                        param_start = param;
                        if param_start.is_null() {
                            arg_missing(option, option_source);
                        } else {
                            val = 1 as libc::c_int;
                        }
                    }
                    param_len = strlen(param_start);
                    tmp = xmalloc(
                        param_len.wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_uint,
                    ) as *mut libc::c_char;
                    memcpy(
                        tmp as *mut libc::c_void,
                        param_start as *const libc::c_void,
                        param_len,
                    );
                    *tmp.offset(param_len as isize) = '\0' as i32 as libc::c_char;
                    addkey(tmp, rw_decl);
                }
                0 => {
                    if (*p).p_special as libc::c_uint
                        == OFF as libc::c_int as libc::c_uint
                    {
                        *((*p).p_obj as *mut libc::c_int) = 0 as libc::c_int;
                    } else {
                        *((*p).p_obj as *mut libc::c_int) = 1 as libc::c_int;
                    }
                }
                1 => {
                    if *param_start as libc::c_int == '\0' as i32 {
                        param_start = param;
                        if param_start.is_null() {
                            arg_missing(option, option_source);
                        } else {
                            val = 1 as libc::c_int;
                        }
                    }
                    if *(*__ctype_b_loc()).offset(*param_start as libc::c_int as isize)
                        as libc::c_int
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                        || *param_start as libc::c_int == '-' as i32
                            && *(*__ctype_b_loc())
                                .offset(
                                    *param_start.offset(1 as libc::c_int as isize)
                                        as libc::c_int as isize,
                                ) as libc::c_int
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                != 0
                    {
                        *((*p).p_obj as *mut libc::c_int) = atoi(param_start);
                    } else {
                        DieError(
                            invocation_error as libc::c_int,
                            dcgettext(
                                0 as *const libc::c_char,
                                b"%s: option ``%s'' requires a numeric parameter\n\0"
                                    as *const u8 as *const libc::c_char,
                                5 as libc::c_int,
                            ),
                            option_source,
                            option.offset(-(1 as libc::c_int as isize)),
                        );
                    }
                }
                _ => {
                    DieError(
                        invocation_error as libc::c_int,
                        dcgettext(
                            0 as *const libc::c_char,
                            b"set_option: internal error: p_type %d\n\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        (*p).p_type as libc::c_int,
                    );
                }
            }
        }
    }
    return val;
}
unsafe extern "C" fn skip_cpp_comment(mut f: *mut FILE) -> libc::c_int {
    let mut i: libc::c_int = 0;
    loop {
        i = getc(f);
        if !(i != -(1 as libc::c_int) && i != '\n' as i32) {
            break;
        }
    }
    if i == '\n' as i32 {
        i = getc(f);
    }
    return i;
}
unsafe extern "C" fn skip_c_comment(mut f: *mut FILE) -> libc::c_int {
    let mut i: libc::c_int = getc(f);
    loop {
        while i != -(1 as libc::c_int) && i != '*' as i32 {
            i = getc(f);
        }
        if i == -(1 as libc::c_int) {
            message(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Warning\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Profile contains an unterminated comment\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
            break;
        } else {
            i = getc(f);
            if !(i != '/' as i32) {
                break;
            }
        }
    }
    if i != -(1 as libc::c_int) {
        i = getc(f);
    }
    return i;
}
unsafe extern "C" fn skip_comment(mut f: *mut FILE) -> libc::c_int {
    let mut i: libc::c_int = getc(f);
    match i {
        47 => {
            i = skip_cpp_comment(f);
        }
        42 => {
            i = skip_c_comment(f);
        }
        _ => {
            message(
                dcgettext(
                    0 as *const libc::c_char,
                    b"Warning\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Profile contains unpalatable characters\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                0 as *mut libc::c_void as *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_char,
            );
        }
    }
    return i;
}
unsafe extern "C" fn skip_spaces(
    mut f: *mut FILE,
    mut first: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = first;
    while i <= ' ' as i32 && i != -(1 as libc::c_int) {
        i = getc(f);
    }
    return i;
}
unsafe extern "C" fn read_string(
    mut f: *mut FILE,
    mut buff: *mut libc::c_char,
    mut first: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = first;
    let mut p: *mut libc::c_char = buff;
    while i != -(1 as libc::c_int) && i > ' ' as i32 && i != '/' as i32
        && p < buff.offset(8192 as libc::c_int as isize)
    {
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = i as libc::c_char;
        i = getc(f);
    }
    *p = '\0' as i32 as libc::c_char;
    return i;
}
unsafe extern "C" fn scan_profile(
    mut f: *mut FILE,
    mut option_source: *const libc::c_char,
) {
    let mut b0: [libc::c_char; 8192] = [0; 8192];
    let mut b1: [libc::c_char; 8192] = [0; 8192];
    let mut current: *mut libc::c_char = b0.as_mut_ptr();
    let mut i: libc::c_int = skip_spaces(f, ' ' as i32);
    memset(
        b0.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        8192 as libc::c_int as libc::c_ulong,
    );
    memset(
        b1.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        8192 as libc::c_int as libc::c_ulong,
    );
    while i != -(1 as libc::c_int) {
        if i == '/' as i32 {
            i = skip_comment(f);
        } else {
            i = read_string(f, current, i);
            if current == b0.as_mut_ptr() {
                current = b1.as_mut_ptr();
            } else if set_option(
                b0.as_mut_ptr(),
                b1.as_mut_ptr(),
                1 as libc::c_int,
                option_source,
            ) == 1 as libc::c_int
            {
                current = b0.as_mut_ptr();
            } else {
                strcpy(b0.as_mut_ptr(), b1.as_mut_ptr());
                current = b1.as_mut_ptr();
            }
        }
        i = skip_spaces(f, i);
    }
    if current != b0.as_mut_ptr() {
        set_option(
            b0.as_mut_ptr(),
            0 as *const libc::c_char,
            1 as libc::c_int,
            option_source,
        );
    }
}
pub unsafe extern "C" fn set_profile() -> *mut libc::c_char {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut prof: [libc::c_char; 12] = unsafe {
        *::std::mem::transmute::<&[u8; 12], &mut [libc::c_char; 12]>(b".indent.pro\0")
    };
    let mut homedir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut envname: *const libc::c_char = getenv(
        b"INDENT_PROFILE\0" as *const u8 as *const libc::c_char,
    );
    if !envname.is_null() {
        f = fopen(envname, b"r\0" as *const u8 as *const libc::c_char);
        if f.is_null() {
            fatal(
                dcgettext(
                    0 as *const libc::c_char,
                    b"File named by environment variable %s does not exist or is not readable\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"INDENT_PROFILE\0" as *const u8 as *const libc::c_char,
            );
        } else {
            scan_profile(f, envname);
            fclose(f);
            fname = strdup(envname);
        }
    } else {
        f = fopen(
            b".indent.pro\0" as *const u8 as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if !f.is_null() {
            let mut len: libc::c_int = (strlen(
                b".indent.pro\0" as *const u8 as *const libc::c_char,
            ))
                .wrapping_add(3 as libc::c_int as libc::c_ulong) as libc::c_int;
            scan_profile(f, b".indent.pro\0" as *const u8 as *const libc::c_char);
            fclose(f);
            fname = xmalloc(len as libc::c_uint) as *mut libc::c_char;
            strcpy(fname, b"./\0" as *const u8 as *const libc::c_char);
            strcat(fname, b".indent.pro\0" as *const u8 as *const libc::c_char);
        } else {
            homedir = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
            if !homedir.is_null() {
                fname = xmalloc(
                    (strlen(homedir))
                        .wrapping_add(
                            strlen(b"%s/%s\0" as *const u8 as *const libc::c_char),
                        )
                        .wrapping_add(
                            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                        ) as libc::c_uint,
                ) as *mut libc::c_char;
                sprintf(
                    fname,
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    homedir,
                    prof.as_mut_ptr(),
                );
                f = fopen(fname, b"r\0" as *const u8 as *const libc::c_char);
                if !f.is_null() {
                    scan_profile(f, fname);
                    fclose(f);
                } else {
                    xfree(fname as *mut libc::c_void);
                    fname = 0 as *mut libc::c_char;
                }
            }
        }
    }
    return fname;
}
unsafe extern "C" fn run_static_initializers() {
    pro = [
        {
            let mut init = pro_ty {
                p_name: b"version\0" as *const u8 as *const libc::c_char,
                p_type: PRO_PRSTRING,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: 5 as libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_version,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"v\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.verbose as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_v,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ut\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.use_tabs as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_ut,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ts\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 8 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.tabsize as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_ts,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"st\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.use_stdout as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_st,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ss\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.space_sp_semicolon as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_ss,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"sob\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.swallow_optional_blanklines as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_sob,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"sc\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.star_comment_cont as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_sc,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"sbi\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.struct_brace_indent as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_sbi,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"saw\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.space_after_while as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_saw,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"sai\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.space_after_if as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_sai,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"saf\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.space_after_for as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_saf,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"psl\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.procnames_start_line as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_psl,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"prs\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.parentheses_space as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_prs,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"pmt\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.preserve_mtime as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_pmt,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"pi\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: -(1 as libc::c_int),
                p_special: ONOFF_NA,
                p_obj: &mut settings.paren_indent as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_pi,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"pcs\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.proc_calls_space as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_pcs,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"orig\0" as *const u8 as *const libc::c_char,
                p_type: PRO_SETTINGS,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: 2 as libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_orig,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"o\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.expect_output_file as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_o,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nv\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.verbose as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_v,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nut\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.use_tabs as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_ut,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nss\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.space_sp_semicolon as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_ss,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nsob\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.swallow_optional_blanklines as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_sob,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nsc\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.star_comment_cont as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_sc,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nsaw\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.space_after_while as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_saw,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nsai\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.space_after_if as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_sai,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nsaf\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.space_after_for as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_saf,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"npsl\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.procnames_start_line as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_psl,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nprs\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.parentheses_space as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_prs,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"npro\0" as *const u8 as *const libc::c_char,
                p_type: PRO_IGN,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: 0 as *mut libc::c_void,
                p_explicit: &mut exp_pro,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"npmt\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.preserve_mtime as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_pmt,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"npcs\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.proc_calls_space as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_pcs,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nlps\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.leave_preproc_space as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_lps,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nlp\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.lineup_to_parens as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_lp,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nip\0" as *const u8 as *const libc::c_char,
                p_type: PRO_SETTINGS,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: 4 as libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_nip,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nhnl\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.honour_newlines as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_hnl,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ngts\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.gettext_strings as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_gts,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nfca\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.format_comments as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_fca,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nfc1\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.format_col1_comments as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_fc1,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"neei\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.extra_expression_indent as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_eei,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ndj\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.ljust_decl as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_dj,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ncs\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.cast_space as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_cs,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nce\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.cuddle_else as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_ce,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ncdw\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.cuddle_do_while as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_cdw,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ncdb\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.comment_delimiter_on_blankline as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_cdb,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nbs\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.blank_after_sizeof as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bs,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nbfda\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.break_function_decl_args as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bfda,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nbfde\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.break_function_decl_args_end as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bfde,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nbc\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.leave_comma as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bc,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nbbo\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.break_before_boolean_operator as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bbo,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nbbb\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.blanklines_before_blockcomments as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bbb,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nbap\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.blanklines_after_procs as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bap,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nbadp\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.blanklines_after_declarations_at_proctop
                    as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_badp,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nbad\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.blanklines_after_declarations as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bad,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"nbacc\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.blanklines_around_conditional_compilation
                    as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_bacc,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"linux\0" as *const u8 as *const libc::c_char,
                p_type: PRO_SETTINGS,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: 3 as libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_linux,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"lps\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.leave_preproc_space as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_lps,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"lp\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.lineup_to_parens as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_lp,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"lc\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 78 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.comment_max_col as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_lc,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"l\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 78 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.max_col as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_l,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"kr\0" as *const u8 as *const libc::c_char,
                p_type: PRO_SETTINGS,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: 0 as *mut libc::c_void,
                p_explicit: &mut exp_kr,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"il\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: -(2 as libc::c_int),
                p_special: ONOFF_NA,
                p_obj: &mut settings.label_offset as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_il,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ip\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 5 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.indent_parameters as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_ip,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"i\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 2 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.ind_size as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_i,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"hnl\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.honour_newlines as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_hnl,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"h\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: 0 as *mut libc::c_void,
                p_explicit: 0 as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"gts\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.gettext_strings as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_gts,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"gnu\0" as *const u8 as *const libc::c_char,
                p_type: PRO_SETTINGS,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: 1 as libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_gnu,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"fnc\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.fix_nested_comments as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_fnc,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"fca\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.format_comments as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_fca,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"fc1\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.format_col1_comments as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_fc1,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"eei\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.extra_expression_indent as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_eei,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"dj\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.ljust_decl as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_dj,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"di\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 2 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.decl_indent as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_di,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"d\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.unindent_displace as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_d,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"cs\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.cast_space as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_cs,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"cp\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 1 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.else_endif_col as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_cp,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"cli\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.case_indent as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_cli,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ci\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.continuation_indent as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_ci,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ce\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.cuddle_else as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_ce,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"cdw\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.cuddle_do_while as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_cdw,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"cdb\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.comment_delimiter_on_blankline as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_cdb,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"cd\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 33 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.decl_com_ind as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_cd,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"cbi\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: -(1 as libc::c_int),
                p_special: ONOFF_NA,
                p_obj: &mut settings.case_brace_indent as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_cbi,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"c++\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.c_plus_plus as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_cpp,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"c\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 33 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.com_ind as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_c,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bs\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.blank_after_sizeof as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bs,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"brs\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.braces_on_struct_decl_line as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bls,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bls\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.braces_on_struct_decl_line as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bls,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"brf\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.braces_on_func_def_line as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_blf,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"blf\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.braces_on_func_def_line as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_blf,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bli\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 2 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.brace_indent as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bli,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"br\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.btype_2 as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_bl,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bl\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.btype_2 as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_bl,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bfda\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.break_function_decl_args as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bfda,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bfde\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.break_function_decl_args_end as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bfde,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bc\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.leave_comma as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bc,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bbo\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.break_before_boolean_operator as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bbo,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bbb\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.blanklines_before_blockcomments as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bbb,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bap\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.blanklines_after_procs as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bap,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"badp\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.blanklines_after_declarations_at_proctop
                    as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_badp,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bad\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.blanklines_after_declarations as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_bad,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"bacc\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.blanklines_around_conditional_compilation
                    as *mut libc::c_int as *mut libc::c_void,
                p_explicit: &mut exp_bacc,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"T\0" as *const u8 as *const libc::c_char,
                p_type: PRO_KEY,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: 0 as *mut libc::c_void,
                p_explicit: &mut exp_T,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ppi\0" as *const u8 as *const libc::c_char,
                p_type: PRO_INT,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: &mut settings.force_preproc_width as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_ppi,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"pal\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: OFF,
                p_obj: &mut settings.pointer_align_right as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_par,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"par\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 1 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.pointer_align_right as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_par,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"slc\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.allow_single_line_conditionals as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_slc,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"as\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.align_with_spaces as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_as,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"sar\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.spaces_around_initializers as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_sar,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: b"ntac\0" as *const u8 as *const libc::c_char,
                p_type: PRO_BOOL,
                p_default: 0 as libc::c_int,
                p_special: ON,
                p_obj: &mut settings.dont_tab_align_comments as *mut libc::c_int
                    as *mut libc::c_void,
                p_explicit: &mut exp_ntac,
            };
            init
        },
        {
            let mut init = pro_ty {
                p_name: 0 as *const libc::c_char,
                p_type: PRO_IGN,
                p_default: 0 as libc::c_int,
                p_special: ONOFF_NA,
                p_obj: 0 as *mut libc::c_void,
                p_explicit: 0 as *mut libc::c_int,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

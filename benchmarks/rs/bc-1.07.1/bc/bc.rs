use ::libc;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut functions: *mut bc_function;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut break_label: libc::c_int;
    static mut if_label: libc::c_int;
    static mut continue_label: libc::c_int;
    static mut next_label: libc::c_int;
    static mut genstr: *mut libc::c_char;
    static mut genlen: libc::c_int;
    static mut interactive: libc::c_char;
    static mut quiet: libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn yylex() -> libc::c_int;
    fn nextarg(
        args: *mut arg_list,
        val: libc::c_int,
        is_var: libc::c_int,
    ) -> *mut arg_list;
    fn arg_str(args: *mut arg_list) -> *mut libc::c_char;
    fn call_str(args: *mut arg_list) -> *mut libc::c_char;
    fn free_args(args: *mut arg_list);
    fn check_params(params: *mut arg_list, autos: *mut arg_list);
    fn set_genstr_size(_: libc::c_int);
    fn init_gen();
    fn generate(str: *const libc::c_char);
    fn run_code();
    fn lookup(name: *mut libc::c_char, namekind: libc::c_int) -> libc::c_int;
    fn welcome();
    fn warranty(_: *const libc::c_char);
    fn show_bc_version();
    fn limits();
    fn yyerror(str: *const libc::c_char, _: ...);
    fn ct_warn(mesg: *const libc::c_char, _: ...);
    fn bc_exit(_: libc::c_int);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_label_group {
    pub l_adrs: [libc::c_ulong; 64],
    pub l_next: *mut bc_label_group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_list {
    pub av_name: libc::c_int,
    pub arg_is_var: libc::c_int,
    pub next: *mut arg_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_function {
    pub f_defined: libc::c_char,
    pub f_void: libc::c_char,
    pub f_body: *mut libc::c_char,
    pub f_body_size: size_t,
    pub f_code_size: size_t,
    pub f_label: *mut bc_label_group,
    pub f_params: *mut arg_list,
    pub f_autos: *mut arg_list,
}
pub type yytype_int16 = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub union YYSTYPE {
    pub s_value: *mut libc::c_char,
    pub c_value: libc::c_char,
    pub i_value: libc::c_int,
    pub a_value: *mut arg_list,
}
pub type yytype_uint8 = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub union yyalloc {
    pub yyss_alloc: yytype_int16,
    pub yyvs_alloc: YYSTYPE,
}
pub static mut cur_func: libc::c_int = -(1 as libc::c_int);
static mut yytranslate: [yytype_uint8; 293] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
];
static mut yypact: [yytype_int16; 198] = [
    -(144 as libc::c_int) as yytype_int16,
    188 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    392 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(36 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    484 as libc::c_int as yytype_int16,
    -(31 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(32 as libc::c_int) as yytype_int16,
    595 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(11 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(10 as libc::c_int) as yytype_int16,
    -(7 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(6 as libc::c_int) as yytype_int16,
    -(5 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    595 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    222 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    2 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    676 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    131 as libc::c_int as yytype_int16,
    621 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    -(27 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    54 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    676 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    569 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    425 as libc::c_int as yytype_int16,
    535 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    318 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    595 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(23 as libc::c_int) as yytype_int16,
    41 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    676 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    436 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    445 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    485 as libc::c_int as yytype_int16,
    494 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    40 as libc::c_int as yytype_int16,
    676 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    270 as libc::c_int as yytype_int16,
    535 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    595 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    647 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    53 as libc::c_int as yytype_int16,
    676 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    676 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    569 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    131 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    -(22 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(20 as libc::c_int) as yytype_int16,
    676 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    102 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    352 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    535 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    67 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    535 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    132 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    7 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    92 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    595 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    99 as libc::c_int as yytype_int16,
    535 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    222 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    9 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    535 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
];
static mut yydefact: [yytype_uint8; 198] = [
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    93 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    105 as libc::c_int as yytype_uint8,
    106 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    109 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    108 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    92 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    107 as libc::c_int as yytype_uint8,
    96 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    91 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    97 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    101 as libc::c_int as yytype_uint8,
    102 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    94 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    89 as libc::c_int as yytype_uint8,
    90 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    95 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    104 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    98 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    35 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    99 as libc::c_int as yytype_uint8,
    100 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    103 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    110 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    111 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    112 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
];
static mut yypgoto: [yytype_int16; 36] = [
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(143 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(39 as libc::c_int) as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(3 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(29 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(141 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(2 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
    145 as libc::c_int as yytype_int16,
    -(144 as libc::c_int) as yytype_int16,
];
static mut yydefgoto: [yytype_int16; 36] = [
    -(1 as libc::c_int) as yytype_int16,
    1 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    150 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    64 as libc::c_int as yytype_int16,
    65 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    53 as libc::c_int as yytype_int16,
    148 as libc::c_int as yytype_int16,
    178 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    139 as libc::c_int as yytype_int16,
    55 as libc::c_int as yytype_int16,
    140 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    94 as libc::c_int as yytype_int16,
    95 as libc::c_int as yytype_int16,
    171 as libc::c_int as yytype_int16,
    179 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    136 as libc::c_int as yytype_int16,
    182 as libc::c_int as yytype_int16,
    137 as libc::c_int as yytype_int16,
    80 as libc::c_int as yytype_int16,
    81 as libc::c_int as yytype_int16,
    118 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    174 as libc::c_int as yytype_int16,
];
static mut yytable: [yytype_int16; 720] = [
    41 as libc::c_int as yytype_int16,
    35 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    156 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    66 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    160 as libc::c_int as yytype_int16,
    162 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    180 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    191 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    77 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    78 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    61 as libc::c_int as yytype_int16,
    62 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    133 as libc::c_int as yytype_int16,
    157 as libc::c_int as yytype_int16,
    181 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    186 as libc::c_int as yytype_int16,
    185 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    58 as libc::c_int as yytype_int16,
    59 as libc::c_int as yytype_int16,
    82 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    85 as libc::c_int as yytype_int16,
    100 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    196 as libc::c_int as yytype_int16,
    87 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    89 as libc::c_int as yytype_int16,
    90 as libc::c_int as yytype_int16,
    192 as libc::c_int as yytype_int16,
    195 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    91 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    147 as libc::c_int as yytype_int16,
    84 as libc::c_int as yytype_int16,
    134 as libc::c_int as yytype_int16,
    158 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    101 as libc::c_int as yytype_int16,
    104 as libc::c_int as yytype_int16,
    105 as libc::c_int as yytype_int16,
    106 as libc::c_int as yytype_int16,
    107 as libc::c_int as yytype_int16,
    108 as libc::c_int as yytype_int16,
    109 as libc::c_int as yytype_int16,
    110 as libc::c_int as yytype_int16,
    135 as libc::c_int as yytype_int16,
    159 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    92 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    121 as libc::c_int as yytype_int16,
    113 as libc::c_int as yytype_int16,
    116 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    115 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    143 as libc::c_int as yytype_int16,
    126 as libc::c_int as yytype_int16,
    138 as libc::c_int as yytype_int16,
    125 as libc::c_int as yytype_int16,
    144 as libc::c_int as yytype_int16,
    127 as libc::c_int as yytype_int16,
    128 as libc::c_int as yytype_int16,
    145 as libc::c_int as yytype_int16,
    146 as libc::c_int as yytype_int16,
    147 as libc::c_int as yytype_int16,
    149 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    153 as libc::c_int as yytype_int16,
    154 as libc::c_int as yytype_int16,
    129 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    132 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    155 as libc::c_int as yytype_int16,
    165 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    163 as libc::c_int as yytype_int16,
    164 as libc::c_int as yytype_int16,
    166 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    96 as libc::c_int as yytype_int16,
    167 as libc::c_int as yytype_int16,
    168 as libc::c_int as yytype_int16,
    169 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    170 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    173 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    175 as libc::c_int as yytype_int16,
    176 as libc::c_int as yytype_int16,
    177 as libc::c_int as yytype_int16,
    83 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    183 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    189 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    161 as libc::c_int as yytype_int16,
    184 as libc::c_int as yytype_int16,
    193 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    141 as libc::c_int as yytype_int16,
    187 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    130 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    172 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    119 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    190 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    2 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(9 as libc::c_int) as yytype_int16,
    0 as libc::c_int as yytype_int16,
    197 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    -(13 as libc::c_int) as yytype_int16,
    0 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    -(9 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(13 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    -(13 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(15 as libc::c_int) as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(15 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    -(15 as libc::c_int) as yytype_int16,
    63 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    -(12 as libc::c_int) as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    -(12 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    152 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    97 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    117 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    120 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    122 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    123 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    93 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    79 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    131 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    68 as libc::c_int as yytype_int16,
    69 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    0 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
];
static mut yycheck: [yytype_int16; 720] = [
    3 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    146 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    37 as libc::c_int as yytype_int16,
    148 as libc::c_int as yytype_int16,
    151 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    10 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    26 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    179 as libc::c_int as yytype_int16,
    178 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    194 as libc::c_int as yytype_int16,
    54 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    56 as libc::c_int as yytype_int16,
    57 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    60 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    63 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    67 as libc::c_int as yytype_int16,
    70 as libc::c_int as yytype_int16,
    71 as libc::c_int as yytype_int16,
    72 as libc::c_int as yytype_int16,
    73 as libc::c_int as yytype_int16,
    74 as libc::c_int as yytype_int16,
    75 as libc::c_int as yytype_int16,
    76 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    52 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    86 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    88 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    99 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    98 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    102 as libc::c_int as yytype_int16,
    103 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    49 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    111 as libc::c_int as yytype_int16,
    112 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    114 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    22 as libc::c_int as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    50 as libc::c_int as yytype_int16,
    142 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    148 as libc::c_int as yytype_int16,
    150 as libc::c_int as yytype_int16,
    51 as libc::c_int as yytype_int16,
    188 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    124 as libc::c_int as yytype_int16,
    181 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    162 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    178 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    186 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    0 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    196 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    13 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    3 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    47 as libc::c_int as yytype_int16,
    48 as libc::c_int as yytype_int16,
    1 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    44 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    3 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    45 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    51 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    11 as libc::c_int as yytype_int16,
    45 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    47 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    11 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    11 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    46 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    11 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    11 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    46 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    14 as libc::c_int as yytype_int16,
    15 as libc::c_int as yytype_int16,
    16 as libc::c_int as yytype_int16,
    17 as libc::c_int as yytype_int16,
    18 as libc::c_int as yytype_int16,
    19 as libc::c_int as yytype_int16,
    20 as libc::c_int as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    29 as libc::c_int as yytype_int16,
    30 as libc::c_int as yytype_int16,
    31 as libc::c_int as yytype_int16,
    32 as libc::c_int as yytype_int16,
    33 as libc::c_int as yytype_int16,
    34 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    7 as libc::c_int as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    45 as libc::c_int as yytype_int16,
    12 as libc::c_int as yytype_int16,
    47 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    45 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    45 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    6 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    8 as libc::c_int as yytype_int16,
    9 as libc::c_int as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    12 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    16 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    45 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    21 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    23 as libc::c_int as yytype_int16,
    24 as libc::c_int as yytype_int16,
    25 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    27 as libc::c_int as yytype_int16,
    28 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    31 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    4 as libc::c_int as yytype_int16,
    5 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    36 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    39 as libc::c_int as yytype_int16,
    11 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    45 as libc::c_int as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    -(1 as libc::c_int) as yytype_int16,
    38 as libc::c_int as yytype_int16,
    39 as libc::c_int as yytype_int16,
    40 as libc::c_int as yytype_int16,
    41 as libc::c_int as yytype_int16,
    42 as libc::c_int as yytype_int16,
    43 as libc::c_int as yytype_int16,
];
static mut yystos: [yytype_uint8; 198] = [
    0 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    9 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    15 as libc::c_int as yytype_uint8,
    16 as libc::c_int as yytype_uint8,
    17 as libc::c_int as yytype_uint8,
    18 as libc::c_int as yytype_uint8,
    19 as libc::c_int as yytype_uint8,
    20 as libc::c_int as yytype_uint8,
    21 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    24 as libc::c_int as yytype_uint8,
    25 as libc::c_int as yytype_uint8,
    27 as libc::c_int as yytype_uint8,
    28 as libc::c_int as yytype_uint8,
    29 as libc::c_int as yytype_uint8,
    30 as libc::c_int as yytype_uint8,
    31 as libc::c_int as yytype_uint8,
    32 as libc::c_int as yytype_uint8,
    33 as libc::c_int as yytype_uint8,
    34 as libc::c_int as yytype_uint8,
    36 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    23 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    37 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    11 as libc::c_int as yytype_uint8,
    38 as libc::c_int as yytype_uint8,
    39 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    41 as libc::c_int as yytype_uint8,
    42 as libc::c_int as yytype_uint8,
    43 as libc::c_int as yytype_uint8,
    10 as libc::c_int as yytype_uint8,
    12 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    7 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    45 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    49 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    40 as libc::c_int as yytype_uint8,
    52 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    47 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    22 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    50 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    26 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    51 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    46 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    44 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    48 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
];
static mut yyr1: [yytype_uint8; 113] = [
    0 as libc::c_int as yytype_uint8,
    53 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    54 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    55 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    56 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    57 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    58 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    59 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    61 as libc::c_int as yytype_uint8,
    62 as libc::c_int as yytype_uint8,
    63 as libc::c_int as yytype_uint8,
    64 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    65 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    66 as libc::c_int as yytype_uint8,
    67 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    68 as libc::c_int as yytype_uint8,
    60 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    69 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    70 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    72 as libc::c_int as yytype_uint8,
    71 as libc::c_int as yytype_uint8,
    74 as libc::c_int as yytype_uint8,
    73 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    75 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    76 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    77 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    78 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    79 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    80 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    81 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    82 as libc::c_int as yytype_uint8,
    84 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    85 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    86 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    83 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    87 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
    88 as libc::c_int as yytype_uint8,
];
static mut yyr2: [yytype_uint8; 113] = [
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    14 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    8 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    13 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    6 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    5 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    3 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    4 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    0 as libc::c_int as yytype_uint8,
    1 as libc::c_int as yytype_uint8,
    2 as libc::c_int as yytype_uint8,
];
unsafe extern "C" fn yydestruct(
    mut yymsg: *const libc::c_char,
    mut yytype: libc::c_int,
    mut yyvaluep: *mut YYSTYPE,
) {
    if yymsg.is_null() {
        yymsg = b"Deleting\0" as *const u8 as *const libc::c_char;
    }
}
pub static mut yychar: libc::c_int = 0;
pub static mut yylval: YYSTYPE = YYSTYPE {
    s_value: 0 as *const libc::c_char as *mut libc::c_char,
};
pub static mut yynerrs: libc::c_int = 0;
pub unsafe extern "C" fn yyparse() -> libc::c_int {
    let mut current_block: u64;
    let mut yystate: libc::c_int = 0;
    let mut yyerrstatus: libc::c_int = 0;
    let mut yyssa: [yytype_int16; 200] = [0; 200];
    let mut yyss: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyssp: *mut yytype_int16 = 0 as *mut yytype_int16;
    let mut yyvsa: [YYSTYPE; 200] = [YYSTYPE {
        s_value: 0 as *const libc::c_char as *mut libc::c_char,
    }; 200];
    let mut yyvs: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yyvsp: *mut YYSTYPE = 0 as *mut YYSTYPE;
    let mut yystacksize: libc::c_ulong = 0;
    let mut yyn: libc::c_int = 0;
    let mut yyresult: libc::c_int = 0;
    let mut yytoken: libc::c_int = 0 as libc::c_int;
    let mut yyval: YYSTYPE = YYSTYPE {
        s_value: 0 as *const libc::c_char as *mut libc::c_char,
    };
    let mut yylen: libc::c_int = 0 as libc::c_int;
    yyss = yyssa.as_mut_ptr();
    yyssp = yyss;
    yyvs = yyvsa.as_mut_ptr();
    yyvsp = yyvs;
    yystacksize = 200 as libc::c_int as libc::c_ulong;
    yystate = 0 as libc::c_int;
    yyerrstatus = 0 as libc::c_int;
    yynerrs = 0 as libc::c_int;
    yychar = -(2 as libc::c_int);
    '_yysetstate: loop {
        *yyssp = yystate as yytype_int16;
        if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
            <= yyssp
        {
            let mut yysize: libc::c_ulong = (yyssp.offset_from(yyss) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong;
            if 10000 as libc::c_int as libc::c_ulong <= yystacksize {
                current_block = 17887119663423605163;
                break;
            }
            yystacksize = yystacksize.wrapping_mul(2 as libc::c_int as libc::c_ulong);
            if (10000 as libc::c_int as libc::c_ulong) < yystacksize {
                yystacksize = 10000 as libc::c_int as libc::c_ulong;
            }
            let mut yyss1: *mut yytype_int16 = yyss;
            let mut yyptr: *mut yyalloc = malloc(
                yystacksize
                    .wrapping_mul(
                        (::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                            .wrapping_add(
                                ::std::mem::size_of::<YYSTYPE>() as libc::c_ulong,
                            ),
                    )
                    .wrapping_add(
                        (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ),
            ) as *mut yyalloc;
            if yyptr.is_null() {
                current_block = 17887119663423605163;
                break;
            }
            let mut yynewbytes: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyss_alloc as *mut yytype_int16 as *mut libc::c_void,
                yyss as *const libc::c_void,
                yysize
                    .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyss = &mut (*yyptr).yyss_alloc;
            yynewbytes = yystacksize
                .wrapping_mul(::std::mem::size_of::<yytype_int16>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr
                .offset(
                    yynewbytes
                        .wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        as isize,
                );
            let mut yynewbytes_0: libc::c_ulong = 0;
            libc::memcpy(
                &mut (*yyptr).yyvs_alloc as *mut YYSTYPE as *mut libc::c_void,
                yyvs as *const libc::c_void,
                yysize.wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                    as libc::size_t,
            );
            yyvs = &mut (*yyptr).yyvs_alloc;
            yynewbytes_0 = yystacksize
                .wrapping_mul(::std::mem::size_of::<YYSTYPE>() as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            yyptr = yyptr
                .offset(
                    yynewbytes_0
                        .wrapping_div(::std::mem::size_of::<yyalloc>() as libc::c_ulong)
                        as isize,
                );
            if yyss1 != yyssa.as_mut_ptr() {
                free(yyss1 as *mut libc::c_void);
            }
            yyssp = yyss.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            yyvsp = yyvs.offset(yysize as isize).offset(-(1 as libc::c_int as isize));
            if yyss.offset(yystacksize as isize).offset(-(1 as libc::c_int as isize))
                <= yyssp
            {
                current_block = 12636511953179831793;
                break;
            }
        }
        if yystate == 2 as libc::c_int {
            yyresult = 0 as libc::c_int;
            current_block = 7716844613298590902;
            break;
        } else {
            yyn = yypact[yystate as usize] as libc::c_int;
            if yyn == -(144 as libc::c_int) {
                current_block = 3864098584368094764;
            } else {
                if yychar == -(2 as libc::c_int) {
                    yychar = yylex();
                }
                if yychar <= 0 as libc::c_int {
                    yytoken = 0 as libc::c_int;
                    yychar = yytoken;
                } else {
                    yytoken = if yychar as libc::c_uint
                        <= 292 as libc::c_int as libc::c_uint
                    {
                        yytranslate[yychar as usize] as libc::c_int
                    } else {
                        2 as libc::c_int
                    };
                }
                yyn += yytoken;
                if yyn < 0 as libc::c_int || (719 as libc::c_int) < yyn
                    || yycheck[yyn as usize] as libc::c_int != yytoken
                {
                    current_block = 3864098584368094764;
                } else {
                    yyn = yytable[yyn as usize] as libc::c_int;
                    if yyn <= 0 as libc::c_int {
                        yyn = -yyn;
                        current_block = 309064319803281694;
                    } else {
                        if yyerrstatus != 0 {
                            yyerrstatus -= 1;
                            yyerrstatus;
                        }
                        yychar = -(2 as libc::c_int);
                        yystate = yyn;
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yylval;
                        current_block = 7519392569660095677;
                    }
                }
            }
            match current_block {
                3864098584368094764 => {
                    yyn = yydefact[yystate as usize] as libc::c_int;
                    if yyn == 0 as libc::c_int {
                        yytoken = if yychar == -(2 as libc::c_int) {
                            -(2 as libc::c_int)
                        } else if yychar as libc::c_uint
                            <= 292 as libc::c_int as libc::c_uint
                        {
                            yytranslate[yychar as usize] as libc::c_int
                        } else {
                            2 as libc::c_int
                        };
                        if yyerrstatus == 0 {
                            yynerrs += 1;
                            yynerrs;
                            yyerror(
                                b"syntax error\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if yyerrstatus == 3 as libc::c_int {
                            if yychar <= 0 as libc::c_int {
                                if yychar == 0 as libc::c_int {
                                    current_block = 12636511953179831793;
                                    break;
                                }
                            } else {
                                yydestruct(
                                    b"Error: discarding\0" as *const u8 as *const libc::c_char,
                                    yytoken,
                                    &mut yylval,
                                );
                                yychar = -(2 as libc::c_int);
                            }
                        }
                        yyerrstatus = 3 as libc::c_int;
                        loop {
                            yyn = yypact[yystate as usize] as libc::c_int;
                            if !(yyn == -(144 as libc::c_int)) {
                                yyn += 1 as libc::c_int;
                                if 0 as libc::c_int <= yyn && yyn <= 719 as libc::c_int
                                    && yycheck[yyn as usize] as libc::c_int == 1 as libc::c_int
                                {
                                    yyn = yytable[yyn as usize] as libc::c_int;
                                    if (0 as libc::c_int) < yyn {
                                        break;
                                    }
                                }
                            }
                            if yyssp == yyss {
                                current_block = 12636511953179831793;
                                break '_yysetstate;
                            }
                            yydestruct(
                                b"Error: popping\0" as *const u8 as *const libc::c_char,
                                yystos[yystate as usize] as libc::c_int,
                                yyvsp,
                            );
                            yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
                            yyssp = yyssp.offset(-(1 as libc::c_int as isize));
                            yystate = *yyssp as libc::c_int;
                        }
                        yyvsp = yyvsp.offset(1);
                        *yyvsp = yylval;
                        yystate = yyn;
                        current_block = 7519392569660095677;
                    } else {
                        current_block = 309064319803281694;
                    }
                }
                _ => {}
            }
            match current_block {
                309064319803281694 => {
                    yylen = yyr2[yyn as usize] as libc::c_int;
                    yyval = *yyvsp.offset((1 as libc::c_int - yylen) as isize);
                    match yyn {
                        2 => {
                            yyval.i_value = 0 as libc::c_int;
                            if interactive as libc::c_int != 0 && quiet == 0 {
                                show_bc_version();
                                welcome();
                            }
                        }
                        4 => {
                            run_code();
                        }
                        5 => {
                            run_code();
                        }
                        6 => {
                            yyerrstatus = 0 as libc::c_int;
                            init_gen();
                        }
                        8 => {
                            ct_warn(
                                b"newline not allowed\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        9 => {
                            yyval.i_value = 0 as libc::c_int;
                        }
                        13 => {
                            yyval.i_value = 0 as libc::c_int;
                        }
                        20 => {
                            yyval
                                .i_value = (*yyvsp.offset(0 as libc::c_int as isize))
                                .i_value;
                        }
                        21 => {
                            warranty(b"\0" as *const u8 as *const libc::c_char);
                        }
                        22 => {
                            limits();
                        }
                        23 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 2 as libc::c_int != 0
                            {
                                ct_warn(
                                    b"comparison in expression\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 1 as libc::c_int != 0
                            {
                                generate(b"W\0" as *const u8 as *const libc::c_char);
                            } else {
                                generate(b"p\0" as *const u8 as *const libc::c_char);
                            }
                        }
                        24 => {
                            yyval.i_value = 0 as libc::c_int;
                            generate(b"w\0" as *const u8 as *const libc::c_char);
                            generate((*yyvsp.offset(0 as libc::c_int as isize)).s_value);
                            free(
                                (*yyvsp.offset(0 as libc::c_int as isize)).s_value
                                    as *mut libc::c_void,
                            );
                        }
                        25 => {
                            if break_label == 0 as libc::c_int {
                                yyerror(
                                    b"Break outside a for/while\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"J%1d:\0" as *const u8 as *const libc::c_char,
                                    break_label,
                                );
                                generate(genstr);
                            }
                        }
                        26 => {
                            ct_warn(
                                b"Continue statement\0" as *const u8 as *const libc::c_char,
                            );
                            if continue_label == 0 as libc::c_int {
                                yyerror(
                                    b"Continue outside a for\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"J%1d:\0" as *const u8 as *const libc::c_char,
                                    continue_label,
                                );
                                generate(genstr);
                            }
                        }
                        27 => {
                            bc_exit(0 as libc::c_int);
                        }
                        28 => {
                            generate(b"h\0" as *const u8 as *const libc::c_char);
                        }
                        29 => {
                            generate(b"R\0" as *const u8 as *const libc::c_char);
                        }
                        30 => {
                            (*yyvsp.offset(0 as libc::c_int as isize))
                                .i_value = break_label;
                            let fresh0 = next_label;
                            next_label = next_label + 1;
                            break_label = fresh0;
                        }
                        31 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 2 as libc::c_int != 0
                            {
                                ct_warn(
                                    b"Comparison in first for expression\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"first expression is void\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 16 as libc::c_int == 0
                            {
                                generate(b"p\0" as *const u8 as *const libc::c_char);
                            }
                            let fresh1 = next_label;
                            next_label = next_label + 1;
                            (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .i_value = fresh1;
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"N%1d:\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value,
                            );
                            generate(genstr);
                        }
                        32 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"second expression is void\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 16 as libc::c_int != 0
                            {
                                generate(b"1\0" as *const u8 as *const libc::c_char);
                            }
                            let fresh2 = next_label;
                            next_label = next_label + 1;
                            (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .i_value = fresh2;
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"B%1d:J%1d:\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value,
                                break_label,
                            );
                            generate(genstr);
                            yyval.i_value = continue_label;
                            let fresh3 = next_label;
                            next_label = next_label + 1;
                            continue_label = fresh3;
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"N%1d:\0" as *const u8 as *const libc::c_char,
                                continue_label,
                            );
                            generate(genstr);
                        }
                        33 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 2 as libc::c_int != 0
                            {
                                ct_warn(
                                    b"Comparison in third for expression\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"third expression is void\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 16 as libc::c_int != 0
                            {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"J%1d:N%1d:\0" as *const u8 as *const libc::c_char,
                                    (*yyvsp.offset(-(7 as libc::c_int) as isize)).i_value,
                                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).i_value,
                                );
                            } else {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"pJ%1d:N%1d:\0" as *const u8 as *const libc::c_char,
                                    (*yyvsp.offset(-(7 as libc::c_int) as isize)).i_value,
                                    (*yyvsp.offset(-(4 as libc::c_int) as isize)).i_value,
                                );
                            }
                            generate(genstr);
                        }
                        34 => {
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"J%1d:N%1d:\0" as *const u8 as *const libc::c_char,
                                continue_label,
                                break_label,
                            );
                            generate(genstr);
                            break_label = (*yyvsp.offset(-(13 as libc::c_int) as isize))
                                .i_value;
                            continue_label = (*yyvsp
                                .offset(-(5 as libc::c_int) as isize))
                                .i_value;
                        }
                        35 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .i_value = if_label;
                            let fresh4 = next_label;
                            next_label = next_label + 1;
                            if_label = fresh4;
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"Z%1d:\0" as *const u8 as *const libc::c_char,
                                if_label,
                            );
                            generate(genstr);
                        }
                        36 => {
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"N%1d:\0" as *const u8 as *const libc::c_char,
                                if_label,
                            );
                            generate(genstr);
                            if_label = (*yyvsp.offset(-(5 as libc::c_int) as isize))
                                .i_value;
                        }
                        37 => {
                            (*yyvsp.offset(0 as libc::c_int as isize))
                                .i_value = continue_label;
                            let fresh5 = next_label;
                            next_label = next_label + 1;
                            continue_label = fresh5;
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"N%1d:\0" as *const u8 as *const libc::c_char,
                                continue_label,
                            );
                            generate(genstr);
                        }
                        38 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            (*yyvsp.offset(0 as libc::c_int as isize))
                                .i_value = break_label;
                            let fresh6 = next_label;
                            next_label = next_label + 1;
                            break_label = fresh6;
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"Z%1d:\0" as *const u8 as *const libc::c_char,
                                break_label,
                            );
                            generate(genstr);
                        }
                        39 => {
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"J%1d:N%1d:\0" as *const u8 as *const libc::c_char,
                                continue_label,
                                break_label,
                            );
                            generate(genstr);
                            break_label = (*yyvsp.offset(-(4 as libc::c_int) as isize))
                                .i_value;
                            continue_label = (*yyvsp
                                .offset(-(7 as libc::c_int) as isize))
                                .i_value;
                        }
                        40 => {
                            yyval.i_value = 0 as libc::c_int;
                        }
                        41 => {
                            ct_warn(
                                b"print statement\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        45 => {
                            generate(b"O\0" as *const u8 as *const libc::c_char);
                            generate((*yyvsp.offset(0 as libc::c_int as isize)).s_value);
                            free(
                                (*yyvsp.offset(0 as libc::c_int as isize)).s_value
                                    as *mut libc::c_void,
                            );
                        }
                        46 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression in print\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"P\0" as *const u8 as *const libc::c_char);
                        }
                        48 => {
                            ct_warn(
                                b"else clause in if statement\0" as *const u8
                                    as *const libc::c_char,
                            );
                            let fresh7 = next_label;
                            next_label = next_label + 1;
                            (*yyvsp.offset(0 as libc::c_int as isize)).i_value = fresh7;
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"J%d:N%1d:\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                if_label,
                            );
                            generate(genstr);
                            if_label = (*yyvsp.offset(0 as libc::c_int as isize))
                                .i_value;
                        }
                        50 => {
                            let mut params: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut autos: *mut libc::c_char = 0 as *mut libc::c_char;
                            check_params(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).a_value,
                                (*yyvsp.offset(0 as libc::c_int as isize)).a_value,
                            );
                            params = arg_str(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).a_value,
                            );
                            autos = arg_str(
                                (*yyvsp.offset(0 as libc::c_int as isize)).a_value,
                            );
                            set_genstr_size(
                                (30 as libc::c_int as libc::c_ulong)
                                    .wrapping_add(strlen(params))
                                    .wrapping_add(strlen(autos)) as libc::c_int,
                            );
                            cur_func = lookup(
                                (*yyvsp.offset(-(7 as libc::c_int) as isize)).s_value,
                                3 as libc::c_int,
                            );
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"F%d,%s.%s[\0" as *const u8 as *const libc::c_char,
                                cur_func,
                                params,
                                autos,
                            );
                            generate(genstr);
                            (*functions.offset(cur_func as isize))
                                .f_void = (*yyvsp.offset(-(8 as libc::c_int) as isize))
                                .i_value as libc::c_char;
                            free_args(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).a_value,
                            );
                            free_args(
                                (*yyvsp.offset(0 as libc::c_int as isize)).a_value,
                            );
                            (*yyvsp.offset(-(9 as libc::c_int) as isize))
                                .i_value = next_label;
                            next_label = 1 as libc::c_int;
                        }
                        51 => {
                            generate(b"0R]\0" as *const u8 as *const libc::c_char);
                            next_label = (*yyvsp.offset(-(12 as libc::c_int) as isize))
                                .i_value;
                            cur_func = -(1 as libc::c_int);
                        }
                        52 => {
                            yyval.i_value = 0 as libc::c_int;
                        }
                        53 => {
                            yyval.i_value = 1 as libc::c_int;
                            ct_warn(
                                b"void functions\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        54 => {
                            yyval.a_value = 0 as *mut arg_list;
                        }
                        56 => {
                            yyval.a_value = 0 as *mut arg_list;
                        }
                        57 => {
                            yyval
                                .a_value = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .a_value;
                        }
                        58 => {
                            yyval
                                .a_value = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .a_value;
                        }
                        59 => {
                            yyval
                                .a_value = nextarg(
                                0 as *mut arg_list,
                                lookup(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).s_value,
                                    0 as libc::c_int,
                                ),
                                0 as libc::c_int,
                            );
                        }
                        60 => {
                            yyval
                                .a_value = nextarg(
                                0 as *mut arg_list,
                                lookup(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).s_value,
                                    1 as libc::c_int,
                                ),
                                0 as libc::c_int,
                            );
                        }
                        61 => {
                            yyval
                                .a_value = nextarg(
                                0 as *mut arg_list,
                                lookup(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).s_value,
                                    1 as libc::c_int,
                                ),
                                1 as libc::c_int,
                            );
                            ct_warn(
                                b"Call by variable arrays\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        62 => {
                            yyval
                                .a_value = nextarg(
                                0 as *mut arg_list,
                                lookup(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).s_value,
                                    1 as libc::c_int,
                                ),
                                1 as libc::c_int,
                            );
                            ct_warn(
                                b"Call by variable arrays\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        63 => {
                            yyval
                                .a_value = nextarg(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).a_value,
                                lookup(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).s_value,
                                    0 as libc::c_int,
                                ),
                                0 as libc::c_int,
                            );
                        }
                        64 => {
                            yyval
                                .a_value = nextarg(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).a_value,
                                lookup(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).s_value,
                                    1 as libc::c_int,
                                ),
                                0 as libc::c_int,
                            );
                        }
                        65 => {
                            yyval
                                .a_value = nextarg(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).a_value,
                                lookup(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).s_value,
                                    1 as libc::c_int,
                                ),
                                1 as libc::c_int,
                            );
                            ct_warn(
                                b"Call by variable arrays\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        66 => {
                            yyval
                                .a_value = nextarg(
                                (*yyvsp.offset(-(5 as libc::c_int) as isize)).a_value,
                                lookup(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).s_value,
                                    1 as libc::c_int,
                                ),
                                1 as libc::c_int,
                            );
                            ct_warn(
                                b"Call by variable arrays\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        67 => {
                            yyval.a_value = 0 as *mut arg_list;
                        }
                        69 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 2 as libc::c_int != 0
                            {
                                ct_warn(
                                    b"comparison in argument\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void argument\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .a_value = nextarg(
                                0 as *mut arg_list,
                                0 as libc::c_int,
                                0 as libc::c_int,
                            );
                        }
                        70 => {
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"K%d:\0" as *const u8 as *const libc::c_char,
                                -lookup(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).s_value,
                                    1 as libc::c_int,
                                ),
                            );
                            generate(genstr);
                            yyval
                                .a_value = nextarg(
                                0 as *mut arg_list,
                                1 as libc::c_int,
                                0 as libc::c_int,
                            );
                        }
                        71 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 2 as libc::c_int != 0
                            {
                                ct_warn(
                                    b"comparison in argument\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void argument\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            yyval
                                .a_value = nextarg(
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).a_value,
                                0 as libc::c_int,
                                0 as libc::c_int,
                            );
                        }
                        72 => {
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"K%d:\0" as *const u8 as *const libc::c_char,
                                -lookup(
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).s_value,
                                    1 as libc::c_int,
                                ),
                            );
                            generate(genstr);
                            yyval
                                .a_value = nextarg(
                                (*yyvsp.offset(-(4 as libc::c_int) as isize)).a_value,
                                1 as libc::c_int,
                                0 as libc::c_int,
                            );
                        }
                        73 => {
                            yyval.i_value = 16 as libc::c_int;
                            ct_warn(
                                b"Missing expression in for statement\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        75 => {
                            yyval.i_value = 0 as libc::c_int;
                            generate(b"0\0" as *const u8 as *const libc::c_char);
                            if cur_func == -(1 as libc::c_int) {
                                yyerror(
                                    b"Return outside of a function.\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                        76 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 2 as libc::c_int != 0
                            {
                                ct_warn(
                                    b"comparison in return expresion\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 4 as libc::c_int == 0
                            {
                                ct_warn(
                                    b"return expression requires parenthesis\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"return requires non-void expression\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if cur_func == -(1 as libc::c_int) {
                                yyerror(
                                    b"Return outside of a function.\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else if (*functions.offset(cur_func as isize)).f_void != 0
                            {
                                yyerror(
                                    b"Return expression in a void function.\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                        }
                        77 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).c_value
                                as libc::c_int != '=' as i32
                            {
                                if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                    < 0 as libc::c_int
                                {
                                    snprintf(
                                        genstr,
                                        genlen as libc::c_ulong,
                                        b"DL%d:\0" as *const u8 as *const libc::c_char,
                                        -(*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value,
                                    );
                                } else {
                                    snprintf(
                                        genstr,
                                        genlen as libc::c_ulong,
                                        b"l%d:\0" as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value,
                                    );
                                }
                                generate(genstr);
                            }
                        }
                        78 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 0 as libc::c_int != 0
                            {
                                ct_warn(
                                    b"comparison in assignment\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"Assignment of a void expression\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(-(2 as libc::c_int) as isize)).c_value
                                as libc::c_int != '=' as i32
                            {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"%c\0" as *const u8 as *const libc::c_char,
                                    (*yyvsp.offset(-(2 as libc::c_int) as isize)).c_value
                                        as libc::c_int,
                                );
                                generate(genstr);
                            }
                            if (*yyvsp.offset(-(3 as libc::c_int) as isize)).i_value
                                < 0 as libc::c_int
                            {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"S%d:\0" as *const u8 as *const libc::c_char,
                                    -(*yyvsp.offset(-(3 as libc::c_int) as isize)).i_value,
                                );
                            } else {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"s%d:\0" as *const u8 as *const libc::c_char,
                                    (*yyvsp.offset(-(3 as libc::c_int) as isize)).i_value,
                                );
                            }
                            generate(genstr);
                            yyval.i_value = 0 as libc::c_int;
                        }
                        79 => {
                            ct_warn(
                                b"&& operator\0" as *const u8 as *const libc::c_char,
                            );
                            let fresh8 = next_label;
                            next_label = next_label + 1;
                            (*yyvsp.offset(0 as libc::c_int as isize)).i_value = fresh8;
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"DZ%d:p\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                            );
                            generate(genstr);
                        }
                        80 => {
                            if (*yyvsp.offset(-(3 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                                || (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                    & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with &&\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"DZ%d:p1N%d:\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value,
                            );
                            generate(genstr);
                            yyval
                                .i_value = ((*yyvsp.offset(-(3 as libc::c_int) as isize))
                                .i_value
                                | (*yyvsp.offset(0 as libc::c_int as isize)).i_value)
                                & !(4 as libc::c_int);
                        }
                        81 => {
                            ct_warn(
                                b"|| operator\0" as *const u8 as *const libc::c_char,
                            );
                            let fresh9 = next_label;
                            next_label = next_label + 1;
                            (*yyvsp.offset(0 as libc::c_int as isize)).i_value = fresh9;
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"B%d:\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                            );
                            generate(genstr);
                        }
                        82 => {
                            let mut tmplab: libc::c_int = 0;
                            if (*yyvsp.offset(-(3 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                                || (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                    & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with ||\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            let fresh10 = next_label;
                            next_label = next_label + 1;
                            tmplab = fresh10;
                            snprintf(
                                genstr,
                                genlen as libc::c_ulong,
                                b"B%d:0J%d:N%d:1N%d:\0" as *const u8 as *const libc::c_char,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value,
                                tmplab,
                                (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value,
                                tmplab,
                            );
                            generate(genstr);
                            yyval
                                .i_value = ((*yyvsp.offset(-(3 as libc::c_int) as isize))
                                .i_value
                                | (*yyvsp.offset(0 as libc::c_int as isize)).i_value)
                                & !(4 as libc::c_int);
                        }
                        83 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with !\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            yyval
                                .i_value = (*yyvsp.offset(0 as libc::c_int as isize))
                                .i_value & !(4 as libc::c_int);
                            ct_warn(b"! operator\0" as *const u8 as *const libc::c_char);
                            generate(b"!\0" as *const u8 as *const libc::c_char);
                        }
                        84 => {
                            if (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                                || (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                    & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with comparison\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            yyval.i_value = 1 as libc::c_int | 2 as libc::c_int;
                            match *(*yyvsp.offset(-(1 as libc::c_int) as isize)).s_value
                                as libc::c_int
                            {
                                61 => {
                                    generate(b"=\0" as *const u8 as *const libc::c_char);
                                }
                                33 => {
                                    generate(b"#\0" as *const u8 as *const libc::c_char);
                                }
                                60 => {
                                    if *((*yyvsp.offset(-(1 as libc::c_int) as isize)).s_value)
                                        .offset(1 as libc::c_int as isize) as libc::c_int
                                        == '=' as i32
                                    {
                                        generate(b"{\0" as *const u8 as *const libc::c_char);
                                    } else {
                                        generate(b"<\0" as *const u8 as *const libc::c_char);
                                    }
                                }
                                62 => {
                                    if *((*yyvsp.offset(-(1 as libc::c_int) as isize)).s_value)
                                        .offset(1 as libc::c_int as isize) as libc::c_int
                                        == '=' as i32
                                    {
                                        generate(b"}\0" as *const u8 as *const libc::c_char);
                                    } else {
                                        generate(b">\0" as *const u8 as *const libc::c_char);
                                    }
                                }
                                _ => {}
                            }
                            free(
                                (*yyvsp.offset(-(1 as libc::c_int) as isize)).s_value
                                    as *mut libc::c_void,
                            );
                        }
                        85 => {
                            if (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                                || (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                    & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with +\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"+\0" as *const u8 as *const libc::c_char);
                            yyval
                                .i_value = ((*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .i_value
                                | (*yyvsp.offset(0 as libc::c_int as isize)).i_value)
                                & !(4 as libc::c_int);
                        }
                        86 => {
                            if (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                                || (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                    & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with -\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"-\0" as *const u8 as *const libc::c_char);
                            yyval
                                .i_value = ((*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .i_value
                                | (*yyvsp.offset(0 as libc::c_int as isize)).i_value)
                                & !(4 as libc::c_int);
                        }
                        87 => {
                            if (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                                || (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                    & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with *\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"*\0" as *const u8 as *const libc::c_char);
                            yyval
                                .i_value = ((*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .i_value
                                | (*yyvsp.offset(0 as libc::c_int as isize)).i_value)
                                & !(4 as libc::c_int);
                        }
                        88 => {
                            if (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                                || (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                    & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with /\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"/\0" as *const u8 as *const libc::c_char);
                            yyval
                                .i_value = ((*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .i_value
                                | (*yyvsp.offset(0 as libc::c_int as isize)).i_value)
                                & !(4 as libc::c_int);
                        }
                        89 => {
                            if (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                                || (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                    & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with %\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"%\0" as *const u8 as *const libc::c_char);
                            yyval
                                .i_value = ((*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .i_value
                                | (*yyvsp.offset(0 as libc::c_int as isize)).i_value)
                                & !(4 as libc::c_int);
                        }
                        90 => {
                            if (*yyvsp.offset(-(2 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                                || (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                    & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with ^\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"^\0" as *const u8 as *const libc::c_char);
                            yyval
                                .i_value = ((*yyvsp.offset(-(2 as libc::c_int) as isize))
                                .i_value
                                | (*yyvsp.offset(0 as libc::c_int as isize)).i_value)
                                & !(4 as libc::c_int);
                        }
                        91 => {
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression with unary -\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"n\0" as *const u8 as *const libc::c_char);
                            yyval
                                .i_value = (*yyvsp.offset(0 as libc::c_int as isize))
                                .i_value & !(4 as libc::c_int);
                        }
                        92 => {
                            yyval.i_value = 1 as libc::c_int;
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                < 0 as libc::c_int
                            {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"L%d:\0" as *const u8 as *const libc::c_char,
                                    -(*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                );
                            } else {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"l%d:\0" as *const u8 as *const libc::c_char,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                );
                            }
                            generate(genstr);
                        }
                        93 => {
                            let mut len: libc::c_int = strlen(
                                (*yyvsp.offset(0 as libc::c_int as isize)).s_value,
                            ) as libc::c_int;
                            yyval.i_value = 1 as libc::c_int;
                            if len == 1 as libc::c_int
                                && *(*yyvsp.offset(0 as libc::c_int as isize)).s_value
                                    as libc::c_int == '0' as i32
                            {
                                generate(b"0\0" as *const u8 as *const libc::c_char);
                            } else if len == 1 as libc::c_int
                                && *(*yyvsp.offset(0 as libc::c_int as isize)).s_value
                                    as libc::c_int == '1' as i32
                            {
                                generate(b"1\0" as *const u8 as *const libc::c_char);
                            } else {
                                generate(b"K\0" as *const u8 as *const libc::c_char);
                                generate(
                                    (*yyvsp.offset(0 as libc::c_int as isize)).s_value,
                                );
                                generate(b":\0" as *const u8 as *const libc::c_char);
                            }
                            free(
                                (*yyvsp.offset(0 as libc::c_int as isize)).s_value
                                    as *mut libc::c_void,
                            );
                        }
                        94 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression in parenthesis\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            yyval
                                .i_value = (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .i_value | 1 as libc::c_int | 4 as libc::c_int;
                        }
                        95 => {
                            let mut fn_0: libc::c_int = 0;
                            fn_0 = lookup(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).s_value,
                                2 as libc::c_int,
                            );
                            if (*functions.offset(fn_0 as isize)).f_void != 0 {
                                yyval.i_value = 8 as libc::c_int;
                            } else {
                                yyval.i_value = 1 as libc::c_int;
                            }
                            if !((*yyvsp.offset(-(1 as libc::c_int) as isize)).a_value)
                                .is_null()
                            {
                                let mut params_0: *mut libc::c_char = call_str(
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).a_value,
                                );
                                set_genstr_size(
                                    (20 as libc::c_int as libc::c_ulong)
                                        .wrapping_add(strlen(params_0)) as libc::c_int,
                                );
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"C%d,%s:\0" as *const u8 as *const libc::c_char,
                                    fn_0,
                                    params_0,
                                );
                                free_args(
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).a_value,
                                );
                            } else {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"C%d:\0" as *const u8 as *const libc::c_char,
                                    fn_0,
                                );
                            }
                            generate(genstr);
                        }
                        96 => {
                            yyval.i_value = 1 as libc::c_int;
                            if (*yyvsp.offset(0 as libc::c_int as isize)).i_value
                                < 0 as libc::c_int
                            {
                                if (*yyvsp.offset(-(1 as libc::c_int) as isize)).c_value
                                    as libc::c_int == '+' as i32
                                {
                                    snprintf(
                                        genstr,
                                        genlen as libc::c_ulong,
                                        b"DA%d:L%d:\0" as *const u8 as *const libc::c_char,
                                        -(*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                        -(*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                    );
                                } else {
                                    snprintf(
                                        genstr,
                                        genlen as libc::c_ulong,
                                        b"DM%d:L%d:\0" as *const u8 as *const libc::c_char,
                                        -(*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                        -(*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                    );
                                }
                            } else if (*yyvsp.offset(-(1 as libc::c_int) as isize))
                                .c_value as libc::c_int == '+' as i32
                            {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"i%d:l%d:\0" as *const u8 as *const libc::c_char,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                );
                            } else {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"d%d:l%d:\0" as *const u8 as *const libc::c_char,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                    (*yyvsp.offset(0 as libc::c_int as isize)).i_value,
                                );
                            }
                            generate(genstr);
                        }
                        97 => {
                            yyval.i_value = 1 as libc::c_int;
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                < 0 as libc::c_int
                            {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"DL%d:x\0" as *const u8 as *const libc::c_char,
                                    -(*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value,
                                );
                                generate(genstr);
                                if (*yyvsp.offset(0 as libc::c_int as isize)).c_value
                                    as libc::c_int == '+' as i32
                                {
                                    snprintf(
                                        genstr,
                                        genlen as libc::c_ulong,
                                        b"A%d:\0" as *const u8 as *const libc::c_char,
                                        -(*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value,
                                    );
                                } else {
                                    snprintf(
                                        genstr,
                                        genlen as libc::c_ulong,
                                        b"M%d:\0" as *const u8 as *const libc::c_char,
                                        -(*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value,
                                    );
                                }
                            } else {
                                snprintf(
                                    genstr,
                                    genlen as libc::c_ulong,
                                    b"l%d:\0" as *const u8 as *const libc::c_char,
                                    (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value,
                                );
                                generate(genstr);
                                if (*yyvsp.offset(0 as libc::c_int as isize)).c_value
                                    as libc::c_int == '+' as i32
                                {
                                    snprintf(
                                        genstr,
                                        genlen as libc::c_ulong,
                                        b"i%d:\0" as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value,
                                    );
                                } else {
                                    snprintf(
                                        genstr,
                                        genlen as libc::c_ulong,
                                        b"d%d:\0" as *const u8 as *const libc::c_char,
                                        (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value,
                                    );
                                }
                            }
                            generate(genstr);
                        }
                        98 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression in length()\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"cL\0" as *const u8 as *const libc::c_char);
                            yyval.i_value = 1 as libc::c_int;
                        }
                        99 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression in sqrt()\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"cR\0" as *const u8 as *const libc::c_char);
                            yyval.i_value = 1 as libc::c_int;
                        }
                        100 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression in scale()\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            generate(b"cS\0" as *const u8 as *const libc::c_char);
                            yyval.i_value = 1 as libc::c_int;
                        }
                        101 => {
                            ct_warn(
                                b"read function\0" as *const u8 as *const libc::c_char,
                            );
                            generate(b"cI\0" as *const u8 as *const libc::c_char);
                            yyval.i_value = 1 as libc::c_int;
                        }
                        102 => {
                            ct_warn(
                                b"random function\0" as *const u8 as *const libc::c_char,
                            );
                            generate(b"cX\0" as *const u8 as *const libc::c_char);
                            yyval.i_value = 1 as libc::c_int;
                        }
                        103 => {
                            yyval
                                .i_value = lookup(
                                (*yyvsp.offset(0 as libc::c_int as isize)).s_value,
                                0 as libc::c_int,
                            );
                        }
                        104 => {
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 8 as libc::c_int != 0
                            {
                                yyerror(
                                    b"void expression as subscript\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            if (*yyvsp.offset(-(1 as libc::c_int) as isize)).i_value
                                & 2 as libc::c_int != 0
                            {
                                ct_warn(
                                    b"comparison in subscript\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            yyval
                                .i_value = lookup(
                                (*yyvsp.offset(-(3 as libc::c_int) as isize)).s_value,
                                1 as libc::c_int,
                            );
                        }
                        105 => {
                            yyval.i_value = 0 as libc::c_int;
                        }
                        106 => {
                            yyval.i_value = 1 as libc::c_int;
                        }
                        107 => {
                            yyval.i_value = 2 as libc::c_int;
                        }
                        108 => {
                            yyval.i_value = 3 as libc::c_int;
                            ct_warn(
                                b"History variable\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        109 => {
                            yyval.i_value = 4 as libc::c_int;
                            ct_warn(
                                b"Last variable\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        110 => {
                            ct_warn(
                                b"End of line required\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        112 => {
                            ct_warn(
                                b"Too many end of lines\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        _ => {}
                    }
                    yyvsp = yyvsp.offset(-(yylen as isize));
                    yyssp = yyssp.offset(-(yylen as isize));
                    yylen = 0 as libc::c_int;
                    yyvsp = yyvsp.offset(1);
                    *yyvsp = yyval;
                    yyn = yyr1[yyn as usize] as libc::c_int;
                    yystate = yypgoto[(yyn - 53 as libc::c_int) as usize] as libc::c_int
                        + *yyssp as libc::c_int;
                    if 0 as libc::c_int <= yystate && yystate <= 719 as libc::c_int
                        && yycheck[yystate as usize] as libc::c_int
                            == *yyssp as libc::c_int
                    {
                        yystate = yytable[yystate as usize] as libc::c_int;
                    } else {
                        yystate = yydefgoto[(yyn - 53 as libc::c_int) as usize]
                            as libc::c_int;
                    }
                }
                _ => {}
            }
            yyssp = yyssp.offset(1);
            yyssp;
        }
    }
    match current_block {
        17887119663423605163 => {
            yyerror(b"memory exhausted\0" as *const u8 as *const libc::c_char);
            yyresult = 2 as libc::c_int;
        }
        12636511953179831793 => {
            yyresult = 1 as libc::c_int;
        }
        _ => {}
    }
    if yychar != -(2 as libc::c_int) {
        yytoken = if yychar as libc::c_uint <= 292 as libc::c_int as libc::c_uint {
            yytranslate[yychar as usize] as libc::c_int
        } else {
            2 as libc::c_int
        };
        yydestruct(
            b"Cleanup: discarding lookahead\0" as *const u8 as *const libc::c_char,
            yytoken,
            &mut yylval,
        );
    }
    yyvsp = yyvsp.offset(-(yylen as isize));
    yyssp = yyssp.offset(-(yylen as isize));
    while yyssp != yyss {
        yydestruct(
            b"Cleanup: popping\0" as *const u8 as *const libc::c_char,
            yystos[*yyssp as usize] as libc::c_int,
            yyvsp,
        );
        yyvsp = yyvsp.offset(-(1 as libc::c_int as isize));
        yyssp = yyssp.offset(-(1 as libc::c_int as isize));
    }
    if yyss != yyssa.as_mut_ptr() {
        free(yyss as *mut libc::c_void);
    }
    return yyresult;
}

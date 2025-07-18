use ::libc;
pub type size_t = libc::c_ulong;
pub type sign = libc::c_uint;
pub const MINUS: sign = 1;
pub const PLUS: sign = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_struct {
    pub n_sign: sign,
    pub n_len: libc::c_int,
    pub n_scale: libc::c_int,
    pub n_refs: libc::c_int,
    pub n_next: bc_num,
    pub n_ptr: *mut libc::c_char,
    pub n_value: *mut libc::c_char,
}
pub type bc_num = *mut bc_struct;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program_counter {
    pub pc_func: libc::c_uint,
    pub pc_addr: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_var {
    pub v_value: bc_num,
    pub v_next: *mut bc_var,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_array_node {
    pub n_items: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub n_num: [bc_num; 64],
    pub n_down: [*mut bc_array_node; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_array {
    pub a_tree: *mut bc_array_node,
    pub a_depth: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bc_var_array {
    pub a_value: *mut bc_array,
    pub a_param: libc::c_char,
    pub a_next: *mut bc_var_array,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct estack_rec {
    pub s_num: bc_num,
    pub s_next: *mut estack_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fstack_rec {
    pub s_val: libc::c_int,
    pub s_next: *mut fstack_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct id_rec {
    pub id: *mut libc::c_char,
    pub a_name: libc::c_int,
    pub f_name: libc::c_int,
    pub v_name: libc::c_int,
    pub balance: libc::c_short,
    pub left: *mut id_rec,
    pub right: *mut id_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_node {
    pub name: *mut libc::c_char,
    pub next: *mut file_node,
}
pub static mut break_label: libc::c_int = 0;
pub static mut if_label: libc::c_int = 0;
pub static mut continue_label: libc::c_int = 0;
pub static mut next_label: libc::c_int = 0;
pub static mut genstr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
pub static mut genlen: libc::c_int = 0 as libc::c_int;
pub static mut out_count: libc::c_int = 0;
pub static mut did_gen: libc::c_char = 0;
pub static mut interactive: libc::c_char = 0 as libc::c_int as libc::c_char;
pub static mut compile_only: libc::c_int = 0 as libc::c_int;
pub static mut use_math: libc::c_int = 0 as libc::c_int;
pub static mut warn_not_std: libc::c_int = 0 as libc::c_int;
pub static mut std_only: libc::c_int = 0 as libc::c_int;
pub static mut quiet: libc::c_int = 0 as libc::c_int;
pub static mut file_names: *mut file_node = 0 as *const file_node as *mut file_node;
pub static mut file_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut is_std_in: libc::c_char = 0;
pub static mut functions: *mut bc_function = 0 as *const bc_function as *mut bc_function;
pub static mut f_names: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub static mut f_count: libc::c_int = 0;
pub static mut variables: *mut *mut bc_var = 0 as *const *mut bc_var as *mut *mut bc_var;
pub static mut v_names: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub static mut v_count: libc::c_int = 0;
pub static mut arrays: *mut *mut bc_var_array = 0 as *const *mut bc_var_array
    as *mut *mut bc_var_array;
pub static mut a_names: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub static mut a_count: libc::c_int = 0;
pub static mut ex_stack: *mut estack_rec = 0 as *const estack_rec as *mut estack_rec;
pub static mut fn_stack: *mut fstack_rec = 0 as *const fstack_rec as *mut fstack_rec;
pub static mut i_base: libc::c_int = 0;
pub static mut o_base: libc::c_int = 0;
pub static mut scale: libc::c_int = 0;
pub static mut c_code: libc::c_char = 0;
pub static mut runtime_error: libc::c_char = 0;
pub static mut pc: program_counter = program_counter {
    pc_func: 0,
    pc_addr: 0,
};
pub static mut out_col: libc::c_int = 0;
pub static mut line_size: libc::c_int = 0;
pub static mut line_no: libc::c_int = 0;
pub static mut had_error: libc::c_int = 0;
pub static mut next_array: libc::c_int = 0;
pub static mut next_func: libc::c_int = 0;
pub static mut next_var: libc::c_int = 0;
pub static mut name_tree: *mut id_rec = 0 as *const id_rec as *mut id_rec;
pub static mut libmath: [*const libc::c_char; 47] = [
    b"@iK20:s2:p@r\0" as *const u8 as *const libc::c_char,
    b"@iF1,5.6,7,8,9,10,11,12,13,14,15[l0:KA:#Z1:l0:s7:pKA:s0:pl5:C1,0:\0" as *const u8
        as *const libc::c_char,
    b"s14:pl7:s0:pl14:RN1:l5:0<Z2:1s12:pl5:ns5:pN2:l2:s15:pK6:l15:+\0" as *const u8
        as *const libc::c_char,
    b"K.44:l5:*+s13:pl5:cS1+s2:pN3:l5:1>Z4:l10:1+s10:pl5:K2:/s5:pl2:\0" as *const u8
        as *const libc::c_char,
    b"1+s2:pJ3:N4:l13:s2:p1l5:+s14:pl5:s6:p1s8:pK2:s11:pN6:1B7:J5:N8:\0" as *const u8
        as *const libc::c_char,
    b"l11:i11:pJ6:N7:l6:l5:*s6:l8:l11:*s8:/s9:pl9:0=Z9:l10:0>Z10:N11:\0" as *const u8
        as *const libc::c_char,
    b"l10:d10:Z12:l14:l14:*s14:pJ11:N12:N10:l15:s2:pl12:Z13:1l14:/R\0" as *const u8
        as *const libc::c_char,
    b"N13:l14:1/RN9:l14:l9:+s14:pJ8:N5:0R]@r\0" as *const u8 as *const libc::c_char,
    b"@iF2,5.7,9,10,11,12,13,14,15[l0:KA:#Z1:l0:s7:pKA:s0:pl5:C2,0:\0" as *const u8
        as *const libc::c_char,
    b"s14:pl7:s0:pl14:RN1:l5:0{Z2:1K10:l2:^-1/RN2:l2:s15:pK6:l2:+s2:\0" as *const u8
        as *const libc::c_char,
    b"pK2:s10:p0s11:pN3:l5:K2:}Z4:l10:K2:*s10:pl5:cRs5:pJ3:N4:N5:l5:\0" as *const u8
        as *const libc::c_char,
    b"K.5:{Z6:l10:K2:*s10:pl5:cRs5:pJ5:N6:l5:1-l5:1+/s13:s14:pl13:l13:\0" as *const u8
        as *const libc::c_char,
    b"*s12:pK3:s11:pN8:1B9:J7:N10:l11:K2:+s11:pJ8:N9:l13:l12:*s13:l11:\0" as *const u8
        as *const libc::c_char,
    b"/s9:pl9:0=Z11:l10:l14:*s14:pl15:s2:pl14:1/RN11:l14:l9:+s14:pJ10:N7:\0" as *const u8
        as *const libc::c_char,
    b"0R]@r\0" as *const u8 as *const libc::c_char,
    b"@iF3,5.7,9,11,12,13,16,14,15[l0:KA:#Z1:l0:s7:pKA:s0:pl5:C3,0:\0" as *const u8
        as *const libc::c_char,
    b"s14:pl7:s0:pl14:RN1:l2:s15:pK1.1:l15:*K2:+s2:p1C4,0:s14:pl5:0\0" as *const u8
        as *const libc::c_char,
    b"<Z2:1s12:pl5:ns5:pN2:0s2:pl5:l14:/K2:+K4:/s13:pl5:K4:l13:*l14:\0" as *const u8
        as *const libc::c_char,
    b"*-s5:pl13:K2:%Z3:l5:ns5:pN3:l15:K2:+s2:pl5:s9:s14:pl5:nl5:*s16:\0" as *const u8
        as *const libc::c_char,
    b"pK3:s11:pN5:1B6:J4:N7:l11:K2:+s11:pJ5:N6:l9:l16:l11:l11:1-*/*\0" as *const u8
        as *const libc::c_char,
    b"s9:pl9:0=Z8:l15:s2:pl12:Z9:l14:n1/RN9:l14:1/RN8:l14:l9:+s14:p\0" as *const u8
        as *const libc::c_char,
    b"J7:N4:0R]@r\0" as *const u8 as *const libc::c_char,
    b"@iF5,5.7,14,15[l0:KA:#Z1:l0:s7:pKA:s0:pl5:C5,0:s14:pl7:s0:pl14:\0" as *const u8
        as *const libc::c_char,
    b"RN1:l2:s15:pl2:K1.2:*s2:pl5:1C4,0:K2:*+C3,0:s14:pl15:s2:pl14:\0" as *const u8
        as *const libc::c_char,
    b"1/R0R]@r\0" as *const u8 as *const libc::c_char,
    b"@iF4,5.6,7,9,10,11,12,13,16,14,15[l0:KA:#Z1:l0:s7:pKA:s0:pl5:\0" as *const u8
        as *const libc::c_char,
    b"C4,0:s14:pl7:s0:pl14:RN1:1s12:pl5:0<Z2:1ns12:pl5:ns5:pN2:l5:1\0" as *const u8
        as *const libc::c_char,
    b"=Z3:l2:K25:{Z4:K.7853981633974483096156608:l12:/RN4:l2:K40:{Z5:\0" as *const u8
        as *const libc::c_char,
    b"K.7853981633974483096156608458198757210492:l12:/RN5:l2:K60:{Z6:\0" as *const u8
        as *const libc::c_char,
    b"K.785398163397448309615660845819875721049292349843776455243736\0" as *const u8
        as *const libc::c_char,
    b":l12:/RN6:N3:l5:K.2:=Z7:l2:K25:{Z8:K.1973955598498807583700497\0" as *const u8
        as *const libc::c_char,
    b":l12:/RN8:l2:K40:{Z9:K.1973955598498807583700497651947902934475\0" as *const u8
        as *const libc::c_char,
    b":l12:/RN9:l2:K60:{Z10:K.197395559849880758370049765194790293447585103787852101517688\0"
        as *const u8 as *const libc::c_char,
    b":l12:/RN10:N7:l2:s15:pl5:K.2:>Z11:l15:K5:+s2:pK.2:C4,0:s6:pN11:\0" as *const u8
        as *const libc::c_char,
    b"l15:K3:+s2:pN12:l5:K.2:>Z13:l10:1+s10:pl5:K.2:-1l5:K.2:*+/s5:\0" as *const u8
        as *const libc::c_char,
    b"pJ12:N13:l5:s13:s14:pl5:nl5:*s16:pK3:s11:pN15:1B16:J14:N17:l11:\0" as *const u8
        as *const libc::c_char,
    b"K2:+s11:pJ15:N16:l13:l16:*s13:l11:/s9:pl9:0=Z18:l15:s2:pl10:l6:\0" as *const u8
        as *const libc::c_char,
    b"*l14:+l12:/RN18:l14:l9:+s14:pJ17:N14:0R]@r\0" as *const u8 as *const libc::c_char,
    b"@iF6,13,5.6,7,8,9,10,11,12,16,14,15[l0:KA:#Z1:l0:s7:pKA:s0:pl13:\0" as *const u8
        as *const libc::c_char,
    b"l5:C6,00:s14:pl7:s0:pl14:RN1:l2:s15:p0s2:pl13:1/s13:pl13:0<Z2:\0" as *const u8
        as *const libc::c_char,
    b"l13:ns13:pl13:K2:%1=Z3:1s12:pN3:N2:1s10:pK2:s11:pN5:l11:l13:{\0" as *const u8
        as *const libc::c_char,
    b"B6:J4:N7:l11:i11:pJ5:N6:l10:l11:*s10:pJ7:N4:K1.5:l15:*s2:pl5:\0" as *const u8
        as *const libc::c_char,
    b"l13:^K2:l13:^/l10:/s10:p1s9:s14:pl5:nl5:*K4:/s16:pK1.5:l15:*l10:\0" as *const u8
        as *const libc::c_char,
    b"cL+l10:cS-s2:p1s11:pN9:1B10:J8:N11:l11:i11:pJ9:N10:l9:l16:*l11:\0" as *const u8
        as *const libc::c_char,
    b"/l13:l11:+/s9:pl9:0=Z12:l15:s2:pl12:Z13:l10:nl14:*1/RN13:l10:\0" as *const u8
        as *const libc::c_char,
    b"l14:*1/RN12:l14:l9:+s14:pJ11:N8:0R]@r\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

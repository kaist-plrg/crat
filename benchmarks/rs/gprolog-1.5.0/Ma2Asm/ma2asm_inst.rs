use ::libc;
extern "C" {
    static mut pic_code: Bool;
    static mut mi: MapperInf;
    static mut lg_cont: LabelGen;
    fn Label_Gen_New(g: *mut LabelGen) -> *mut libc::c_char;
    fn Label_Gen_Get(g: *mut LabelGen) -> *mut libc::c_char;
    fn Label_Printf(label: *mut libc::c_char, _: ...);
    fn Inst_Printf(op: *mut libc::c_char, operands: *mut libc::c_char, _: ...);
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type intptr_t = libc::c_long;
pub type PlLong = intptr_t;
pub type Bool = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MapperInf {
    pub can_produce_pic_code: Bool,
    pub needs_pre_pass: Bool,
    pub comment_prefix: *mut libc::c_char,
    pub local_symb_prefix: *mut libc::c_char,
    pub string_symb_prefix: *mut libc::c_char,
    pub double_symb_prefix: *mut libc::c_char,
    pub strings_need_null: Bool,
    pub call_c_reverse_args: Bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CodeInf {
    pub name: *mut libc::c_char,
    pub prolog: Bool,
    pub initializer: Bool,
    pub global: Bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StringInf {
    pub no: libc::c_int,
    pub str_0: *mut libc::c_char,
    pub symb: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DoubleInf {
    pub no: libc::c_int,
    pub ma_str: *mut libc::c_char,
    pub is_ma_str_human: Bool,
    pub cmt_str: *mut libc::c_char,
    pub symb: *mut libc::c_char,
    pub v: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub dbl: libc::c_double,
    pub i64_0: int64_t,
    pub i32_0: [int32_t; 2],
}
pub type VType = libc::c_uint;
pub const ARRAY_SIZE: VType = 2;
pub const INITIAL_VALUE: VType = 1;
pub const NONE: VType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LongInf {
    pub name: *mut libc::c_char,
    pub global: Bool,
    pub vtype: VType,
    pub value: PlLong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LabelGen {
    pub prefix: *mut libc::c_char,
    pub no: libc::c_int,
    pub label: [libc::c_char; 256],
}
pub unsafe extern "C" fn Dico_Double(mut d: *mut DoubleInf) {
    Label_Printf(
        b"%s:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*d).symb,
    );
    Inst_Printf(
        b".quad\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%ld\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*d).v.i64_0,
    );
}
pub unsafe extern "C" fn Data_Stop(mut initializer_fct: *mut libc::c_char) {}
pub unsafe extern "C" fn Data_Start(mut initializer_fct: *mut libc::c_char) {
    if initializer_fct.is_null() {
        return;
    }
    Inst_Printf(
        b".section\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".ctors,\"aw\",@progbits\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Inst_Printf(
        b".align\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b".quad\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        initializer_fct,
    );
}
pub unsafe extern "C" fn Dico_Long_Stop(mut nb: libc::c_int) {}
pub unsafe extern "C" fn Dico_Long(mut l: *mut LongInf) {
    let mut size_bytes: PlLong = 0;
    match (*l).vtype as libc::c_uint {
        0 | 2 => {
            size_bytes = (*l).value * 8 as libc::c_int as libc::c_long;
            if (*l).global == 0 {
                Inst_Printf(
                    b".local\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*l).name,
                );
            }
            Inst_Printf(
                b".comm\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"%s,%ld,8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*l).name,
                size_bytes,
            );
        }
        1 => {
            if (*l).global != 0 {
                Inst_Printf(
                    b".globl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*l).name,
                );
            }
            Inst_Printf(
                b".align\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            Inst_Printf(
                b".size\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"%s,8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*l).name,
            );
            Label_Printf(
                b"%s:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*l).name,
            );
            Inst_Printf(
                b".quad\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"%ld\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*l).value,
            );
        }
        _ => {}
    };
}
pub unsafe extern "C" fn Dico_Long_Start(mut nb: libc::c_int) {
    Label_Printf(b".data\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    Inst_Printf(
        b".align\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn Dico_Double_Stop(mut nb: libc::c_int) {}
pub unsafe extern "C" fn Move_Ret_To_Foreign_D(mut index: libc::c_int) {
    if pic_code != 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"pl_foreign_double@GOTPCREL(%%rip), %%r10\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        Inst_Printf(
            b"movsd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%%xmm0, %d(%%r10)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index * 8 as libc::c_int,
        );
    } else {
        Inst_Printf(
            b"movsd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%%xmm0, pl_foreign_double+%d(%%rip)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index * 8 as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn Dico_Double_Start(mut nb: libc::c_int) {
    Inst_Printf(
        b".section\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".rodata.cst8,\"aM\",@progbits,8\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Inst_Printf(
        b".align\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn Dico_String_Stop(mut nb: libc::c_int) {}
pub unsafe extern "C" fn Dico_String(mut s: *mut StringInf) {
    Label_Printf(
        b"%s:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*s).symb,
    );
    Inst_Printf(
        b".string\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*s).str_0,
    );
}
pub unsafe extern "C" fn Dico_String_Start(mut nb: libc::c_int) {
    Inst_Printf(
        b".section\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".rodata.str1.1,\"aMS\",@progbits,1\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn C_Ret() {
    Inst_Printf(
        b"addq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"$%d, %%rsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        32 as libc::c_int * 8 as libc::c_int,
    );
    Inst_Printf(
        b"popq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%%rbx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b"ret\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn Jump_If_Greater(mut label: *mut libc::c_char) {
    Inst_Printf(
        b"jg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label,
    );
}
pub unsafe extern "C" fn Jump_If_Equal(mut label: *mut libc::c_char) {
    Inst_Printf(
        b"je\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label,
    );
}
pub unsafe extern "C" fn Cmp_Ret_And_Int(mut int_val: PlLong) {
    if int_val == 0 as libc::c_int as libc::c_long {
        Inst_Printf(
            b"testq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%%rax, %%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if int_val
        >= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
        && int_val <= 2147483647 as libc::c_int as libc::c_long
    {
        Inst_Printf(
            b"cmpq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"$%ld, %%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            int_val,
        );
    } else {
        Inst_Printf(
            b"movabsq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"$%ld, %%rdx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            int_val,
        );
        Inst_Printf(
            b"cmpq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%%rdx, %%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    };
}
pub unsafe extern "C" fn Asm_Start() {
    strcpy(asm_reg_e.as_mut_ptr(), b"%rbx\0" as *const u8 as *const libc::c_char);
    sprintf(
        asm_reg_b.as_mut_ptr(),
        b"%%%s\0" as *const u8 as *const libc::c_char,
        b"r14\0" as *const u8 as *const libc::c_char,
    );
    strcpy(
        asm_reg_cp.as_mut_ptr(),
        Off_Reg_Bank((256 as libc::c_int + 1 as libc::c_int) * 8 as libc::c_int),
    );
    Label_Printf(b".text\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    Label(b"fail\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    Pl_Fail();
}
pub unsafe extern "C" fn Pl_Fail() {
    Inst_Printf(
        b"jmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"*-8(%s)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        asm_reg_b.as_mut_ptr(),
    );
}
pub static mut asm_reg_b: [libc::c_char; 32] = [0; 32];
pub unsafe extern "C" fn Label(mut label: *mut libc::c_char) {
    Label_Printf(
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    Label_Printf(
        b"%s:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label,
    );
}
unsafe extern "C" fn Off_Reg_Bank(mut offset: libc::c_int) -> *mut libc::c_char {
    static mut str: [libc::c_char; 32] = [0; 32];
    sprintf(
        str.as_mut_ptr(),
        b"%d(%s)\0" as *const u8 as *const libc::c_char,
        offset,
        b"%r12\0" as *const u8 as *const libc::c_char,
    );
    return str.as_mut_ptr();
}
pub static mut asm_reg_cp: [libc::c_char; 32] = [0; 32];
pub static mut asm_reg_e: [libc::c_char; 32] = [0; 32];
pub unsafe extern "C" fn Asm_Stop() {
    Inst_Printf(
        b".section\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".note.GNU-stack,\"\",@progbits\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn Code_Start(mut c: *mut CodeInf) {
    Label_Printf(
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    Inst_Printf(
        b".p2align\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"4,,15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b".type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s,@function\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*c).name,
    );
    if (*c).global != 0 {
        Inst_Printf(
            b".globl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*c).name,
        );
    }
    Label((*c).name);
    if (*c).prolog == 0 {
        Inst_Printf(
            b"pushq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%%rbx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        Inst_Printf(
            b"subq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"$%d, %%rsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            32 as libc::c_int * 8 as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn Code_Stop(mut c: *mut CodeInf) {}
pub unsafe extern "C" fn Reload_E_In_Register() {
    Inst_Printf(
        b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Off_Reg_Bank((256 as libc::c_int + 2 as libc::c_int) * 8 as libc::c_int),
        asm_reg_e.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn Pl_Jump(mut label: *mut libc::c_char) {
    if pic_code != 0 {
        Inst_Printf(
            b"jmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s@PLT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label,
        );
    } else {
        Inst_Printf(
            b"jmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label,
        );
    };
}
pub unsafe extern "C" fn Prep_CP() {
    if 1 as libc::c_int != 0 || pic_code != 0 {
        Inst_Printf(
            b"leaq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s(%%rip), %%r10\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            Label_Gen_New(&mut lg_cont),
        );
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%%r10, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            asm_reg_cp.as_mut_ptr(),
        );
    } else {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"$%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Label_Gen_New(&mut lg_cont),
            asm_reg_cp.as_mut_ptr(),
        );
    };
}
pub unsafe extern "C" fn Here_CP() {
    Label_Printf(
        b"%s:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Label_Gen_Get(&mut lg_cont),
    );
}
pub unsafe extern "C" fn Pl_Call(mut label: *mut libc::c_char) {
    Prep_CP();
    Pl_Jump(label);
    Here_CP();
}
pub unsafe extern "C" fn Pl_Ret() {
    Inst_Printf(
        b"jmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"*%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        asm_reg_cp.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn Jump(mut label: *mut libc::c_char) {
    if pic_code != 0 {
        Inst_Printf(
            b"jmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s@PLT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label,
        );
    } else {
        Inst_Printf(
            b"jmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            label,
        );
    };
}
pub unsafe extern "C" fn Move_From_Reg_X(mut index: libc::c_int) {
    Inst_Printf(
        b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s, %%rdx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Off_Reg_Bank(index * 8 as libc::c_int),
    );
}
pub unsafe extern "C" fn Move_From_Reg_Y(mut index: libc::c_int) {
    Inst_Printf(
        b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%d(%s), %%rdx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ((-(3 as libc::c_int) - 1 as libc::c_int - index) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<PlLong>() as libc::c_ulong)
            as libc::c_int,
        asm_reg_e.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn Move_To_Reg_X(mut index: libc::c_int) {
    Inst_Printf(
        b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%%rdx, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Off_Reg_Bank(index * 8 as libc::c_int),
    );
}
pub unsafe extern "C" fn Move_To_Reg_Y(mut index: libc::c_int) {
    Inst_Printf(
        b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%%rdx, %d(%s)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ((-(3 as libc::c_int) - 1 as libc::c_int - index) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<PlLong>() as libc::c_ulong)
            as libc::c_int,
        asm_reg_e.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn Call_C_Start(
    mut fct_name: *mut libc::c_char,
    mut fc: Bool,
    mut nb_args: libc::c_int,
    mut nb_args_in_words: libc::c_int,
) {
    arg_reg_no = 0 as libc::c_int;
    arg_dbl_reg_no = 0 as libc::c_int;
}
pub static mut arg_dbl_reg_no: libc::c_int = 0;
pub static mut arg_reg_no: libc::c_int = 0;
pub unsafe extern "C" fn Call_C_Arg_Int(
    mut offset: libc::c_int,
    mut int_val: PlLong,
) -> libc::c_int {
    let mut r: [libc::c_char; 32] = [0; 32];
    let mut r_aux: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r_eq_r_aux: Bool = 0 as libc::c_int;
    if arg_reg_no < 6 as libc::c_int {
        let fresh0 = arg_reg_no;
        arg_reg_no = arg_reg_no + 1;
        strcpy(r.as_mut_ptr(), arg_reg_name[fresh0 as usize]);
        r_aux = r.as_mut_ptr();
        r_eq_r_aux = 1 as libc::c_int;
    } else {
        let mut nwords: libc::c_int = offset - arg_reg_no - arg_dbl_reg_no;
        sprintf(
            r.as_mut_ptr(),
            b"%d(%%rsp)\0" as *const u8 as *const libc::c_char,
            nwords * 8 as libc::c_int,
        );
        r_aux = b"%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if int_val >= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
        && int_val <= 2147483647 as libc::c_int as libc::c_long
    {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"$%ld, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            int_val,
            r.as_mut_ptr(),
        );
    } else {
        Inst_Printf(
            b"movabsq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"$%ld, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            int_val,
            r_aux,
        );
        if r_eq_r_aux == 0 {
            Inst_Printf(
                b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                r_aux,
                r.as_mut_ptr(),
            );
        }
    }
    return 1 as libc::c_int;
}
pub static mut arg_reg_name: [*mut libc::c_char; 6] = [
    b"%rdi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%rsi\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%rdx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%rcx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%r8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%r9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub unsafe extern "C" fn Call_C_Arg_Double(
    mut offset: libc::c_int,
    mut d: *mut DoubleInf,
) -> libc::c_int {
    let mut r: [libc::c_char; 32] = [0; 32];
    let mut r_aux: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r_eq_r_aux: Bool = 0 as libc::c_int;
    if arg_dbl_reg_no < 8 as libc::c_int {
        let fresh1 = arg_dbl_reg_no;
        arg_dbl_reg_no = arg_dbl_reg_no + 1;
        strcpy(r.as_mut_ptr(), arg_dbl_reg_name[fresh1 as usize]);
        r_aux = r.as_mut_ptr();
        r_eq_r_aux = 1 as libc::c_int;
    } else {
        let mut nwords: libc::c_int = offset - arg_reg_no - arg_dbl_reg_no;
        sprintf(
            r.as_mut_ptr(),
            b"%d(%%rsp)\0" as *const u8 as *const libc::c_char,
            nwords * 8 as libc::c_int,
        );
        r_aux = b"%xmm8\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    Inst_Printf(
        b"movsd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%s(%%rip), %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*d).symb,
        r_aux,
    );
    if r_eq_r_aux == 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            r_aux,
            r.as_mut_ptr(),
        );
    }
    return 1 as libc::c_int;
}
pub static mut arg_dbl_reg_name: [*mut libc::c_char; 8] = [
    b"%xmm0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%xmm1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%xmm2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%xmm3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%xmm4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%xmm5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%xmm6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"%xmm7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
pub unsafe extern "C" fn Call_C_Arg_String(
    mut offset: libc::c_int,
    mut s: *mut StringInf,
) -> libc::c_int {
    let mut r: [libc::c_char; 32] = [0; 32];
    let mut r_aux: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r_eq_r_aux: Bool = 0 as libc::c_int;
    if arg_reg_no < 6 as libc::c_int {
        let fresh2 = arg_reg_no;
        arg_reg_no = arg_reg_no + 1;
        strcpy(r.as_mut_ptr(), arg_reg_name[fresh2 as usize]);
        r_aux = r.as_mut_ptr();
        r_eq_r_aux = 1 as libc::c_int;
    } else {
        let mut nwords: libc::c_int = offset - arg_reg_no - arg_dbl_reg_no;
        sprintf(
            r.as_mut_ptr(),
            b"%d(%%rsp)\0" as *const u8 as *const libc::c_char,
            nwords * 8 as libc::c_int,
        );
        r_aux = b"%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if 1 as libc::c_int != 0 || pic_code != 0 {
        Inst_Printf(
            b"leaq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s(%%rip), %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*s).symb,
            r_aux,
        );
    } else {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"$%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*s).symb,
            r_aux,
        );
    }
    if r_eq_r_aux == 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            r_aux,
            r.as_mut_ptr(),
        );
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Call_C_Arg_Mem_L(
    mut offset: libc::c_int,
    mut adr_of: Bool,
    mut name: *mut libc::c_char,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut r: [libc::c_char; 32] = [0; 32];
    let mut r_aux: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r_eq_r_aux: Bool = 0 as libc::c_int;
    if arg_reg_no < 6 as libc::c_int {
        let fresh3 = arg_reg_no;
        arg_reg_no = arg_reg_no + 1;
        strcpy(r.as_mut_ptr(), arg_reg_name[fresh3 as usize]);
        r_aux = r.as_mut_ptr();
        r_eq_r_aux = 1 as libc::c_int;
    } else {
        let mut nwords: libc::c_int = offset - arg_reg_no - arg_dbl_reg_no;
        sprintf(
            r.as_mut_ptr(),
            b"%d(%%rsp)\0" as *const u8 as *const libc::c_char,
            nwords * 8 as libc::c_int,
        );
        r_aux = b"%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if pic_code != 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s@GOTPCREL(%%rip), %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
            r_aux,
        );
        if adr_of != 0 {
            if index != 0 as libc::c_int {
                Inst_Printf(
                    b"addq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"$%d, %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    index * 8 as libc::c_int,
                    r_aux,
                );
            }
        } else {
            Inst_Printf(
                b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"%d(%s), %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                index * 8 as libc::c_int,
                r_aux,
                r_aux,
            );
        }
    } else if adr_of != 0 {
        Inst_Printf(
            b"leaq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s+%d(%%rip), %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
            index * 8 as libc::c_int,
            r_aux,
        );
    } else {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s+%d(%%rip), %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
            index * 8 as libc::c_int,
            r_aux,
        );
    }
    if r_eq_r_aux == 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            r_aux,
            r.as_mut_ptr(),
        );
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Call_C_Arg_Reg_X(
    mut offset: libc::c_int,
    mut adr_of: Bool,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut r: [libc::c_char; 32] = [0; 32];
    let mut r_aux: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r_eq_r_aux: Bool = 0 as libc::c_int;
    if arg_reg_no < 6 as libc::c_int {
        let fresh4 = arg_reg_no;
        arg_reg_no = arg_reg_no + 1;
        strcpy(r.as_mut_ptr(), arg_reg_name[fresh4 as usize]);
        r_aux = r.as_mut_ptr();
        r_eq_r_aux = 1 as libc::c_int;
    } else {
        let mut nwords: libc::c_int = offset - arg_reg_no - arg_dbl_reg_no;
        sprintf(
            r.as_mut_ptr(),
            b"%d(%%rsp)\0" as *const u8 as *const libc::c_char,
            nwords * 8 as libc::c_int,
        );
        r_aux = b"%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if adr_of != 0 {
        if r_eq_r_aux == 0 && index == 0 as libc::c_int {
            Inst_Printf(
                b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"%r12\0" as *const u8 as *const libc::c_char,
                r.as_mut_ptr(),
            );
            current_block = 13982081572620738789;
        } else {
            Inst_Printf(
                b"leaq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                Off_Reg_Bank(index * 8 as libc::c_int),
                r_aux,
            );
            current_block = 1841672684692190573;
        }
    } else {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Off_Reg_Bank(index * 8 as libc::c_int),
            r_aux,
        );
        current_block = 1841672684692190573;
    }
    match current_block {
        1841672684692190573 => {
            if r_eq_r_aux == 0 {
                Inst_Printf(
                    b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    r_aux,
                    r.as_mut_ptr(),
                );
            }
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Call_C_Arg_Reg_Y(
    mut offset: libc::c_int,
    mut adr_of: Bool,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut r: [libc::c_char; 32] = [0; 32];
    let mut r_aux: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r_eq_r_aux: Bool = 0 as libc::c_int;
    if arg_reg_no < 6 as libc::c_int {
        let fresh5 = arg_reg_no;
        arg_reg_no = arg_reg_no + 1;
        strcpy(r.as_mut_ptr(), arg_reg_name[fresh5 as usize]);
        r_aux = r.as_mut_ptr();
        r_eq_r_aux = 1 as libc::c_int;
    } else {
        let mut nwords: libc::c_int = offset - arg_reg_no - arg_dbl_reg_no;
        sprintf(
            r.as_mut_ptr(),
            b"%d(%%rsp)\0" as *const u8 as *const libc::c_char,
            nwords * 8 as libc::c_int,
        );
        r_aux = b"%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if adr_of != 0 {
        Inst_Printf(
            b"leaq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%d(%s), %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ((-(3 as libc::c_int) - 1 as libc::c_int - index) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<PlLong>() as libc::c_ulong)
                as libc::c_int,
            asm_reg_e.as_mut_ptr(),
            r_aux,
        );
    } else {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%d(%s), %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ((-(3 as libc::c_int) - 1 as libc::c_int - index) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<PlLong>() as libc::c_ulong)
                as libc::c_int,
            asm_reg_e.as_mut_ptr(),
            r_aux,
        );
    }
    if r_eq_r_aux == 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            r_aux,
            r.as_mut_ptr(),
        );
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Call_C_Arg_Foreign_L(
    mut offset: libc::c_int,
    mut adr_of: Bool,
    mut index: libc::c_int,
) -> libc::c_int {
    let mut r: [libc::c_char; 32] = [0; 32];
    let mut r_aux: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r_eq_r_aux: Bool = 0 as libc::c_int;
    if arg_reg_no < 6 as libc::c_int {
        let fresh6 = arg_reg_no;
        arg_reg_no = arg_reg_no + 1;
        strcpy(r.as_mut_ptr(), arg_reg_name[fresh6 as usize]);
        r_aux = r.as_mut_ptr();
        r_eq_r_aux = 1 as libc::c_int;
    } else {
        let mut nwords: libc::c_int = offset - arg_reg_no - arg_dbl_reg_no;
        sprintf(
            r.as_mut_ptr(),
            b"%d(%%rsp)\0" as *const u8 as *const libc::c_char,
            nwords * 8 as libc::c_int,
        );
        r_aux = b"%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if pic_code != 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"pl_foreign_long@GOTPCREL(%%rip), %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            r_aux,
        );
        if adr_of != 0 {
            if index != 0 as libc::c_int {
                Inst_Printf(
                    b"addq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"$%d, %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    index * 8 as libc::c_int,
                    r_aux,
                );
            }
        } else {
            Inst_Printf(
                b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"%d(%s), %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                index * 8 as libc::c_int,
                r_aux,
                r_aux,
            );
        }
    } else if adr_of != 0 {
        Inst_Printf(
            b"leaq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"pl_foreign_long+%d(%%rip), %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index * 8 as libc::c_int,
            r_aux,
        );
    } else {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"pl_foreign_long+%d(%%rip), %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index * 8 as libc::c_int,
            r_aux,
        );
    }
    if r_eq_r_aux == 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            r_aux,
            r.as_mut_ptr(),
        );
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Call_C_Arg_Foreign_D(
    mut offset: libc::c_int,
    mut adr_of: Bool,
    mut index: libc::c_int,
) -> libc::c_int {
    if adr_of != 0 {
        let mut r: [libc::c_char; 32] = [0; 32];
        let mut r_aux: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut r_eq_r_aux: Bool = 0 as libc::c_int;
        if arg_reg_no < 6 as libc::c_int {
            let fresh7 = arg_reg_no;
            arg_reg_no = arg_reg_no + 1;
            strcpy(r.as_mut_ptr(), arg_reg_name[fresh7 as usize]);
            r_aux = r.as_mut_ptr();
            r_eq_r_aux = 1 as libc::c_int;
        } else {
            let mut nwords: libc::c_int = offset - arg_reg_no - arg_dbl_reg_no;
            sprintf(
                r.as_mut_ptr(),
                b"%d(%%rsp)\0" as *const u8 as *const libc::c_char,
                nwords * 8 as libc::c_int,
            );
            r_aux = b"%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if pic_code != 0 {
            Inst_Printf(
                b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"pl_foreign_double@GOTPCREL(%%rip), %s\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                r_aux,
            );
            if index != 0 as libc::c_int {
                Inst_Printf(
                    b"addq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    b"$%d, %s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    index * 8 as libc::c_int,
                    r_aux,
                );
            }
        } else {
            Inst_Printf(
                b"leaq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"pl_foreign_double+%d(%%rip), %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                index * 8 as libc::c_int,
                r_aux,
            );
        }
        if r_eq_r_aux == 0 {
            Inst_Printf(
                b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                r_aux,
                r.as_mut_ptr(),
            );
        }
        return 1 as libc::c_int;
    }
    let mut r_0: [libc::c_char; 32] = [0; 32];
    let mut r_aux_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r_eq_r_aux_0: Bool = 0 as libc::c_int;
    if arg_dbl_reg_no < 8 as libc::c_int {
        let fresh8 = arg_dbl_reg_no;
        arg_dbl_reg_no = arg_dbl_reg_no + 1;
        strcpy(r_0.as_mut_ptr(), arg_dbl_reg_name[fresh8 as usize]);
        r_aux_0 = r_0.as_mut_ptr();
        r_eq_r_aux_0 = 1 as libc::c_int;
    } else {
        let mut nwords_0: libc::c_int = offset - arg_reg_no - arg_dbl_reg_no;
        sprintf(
            r_0.as_mut_ptr(),
            b"%d(%%rsp)\0" as *const u8 as *const libc::c_char,
            nwords_0 * 8 as libc::c_int,
        );
        r_aux_0 = b"%xmm8\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if pic_code != 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"pl_foreign_double@GOTPCREL(%%rip), %%r10\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        Inst_Printf(
            b"movsd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%d(%%r10), %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            index * 8 as libc::c_int,
            r_aux_0,
        );
    } else {
        Inst_Printf(
            b"movsd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"pl_foreign_double+%d(%%rip), %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index * 8 as libc::c_int,
            r_aux_0,
        );
    }
    if r_eq_r_aux_0 == 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            r_aux_0,
            r_0.as_mut_ptr(),
        );
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn Call_C_Invoke(
    mut fct_name: *mut libc::c_char,
    mut fc: Bool,
    mut nb_args: libc::c_int,
    mut nb_args_in_words: libc::c_int,
) {
    if pic_code != 0 {
        Inst_Printf(
            b"call\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s@PLT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            fct_name,
        );
    } else {
        Inst_Printf(
            b"call\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            fct_name,
        );
    };
}
pub unsafe extern "C" fn Call_C_Stop(
    mut fct_name: *mut libc::c_char,
    mut nb_args: libc::c_int,
) {}
pub unsafe extern "C" fn Jump_Ret() {
    Inst_Printf(
        b"jmp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"*%%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn Fail_Ret() {
    Inst_Printf(
        b"test\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%%rax, %%rax\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Inst_Printf(
        b"je\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"fail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn Move_Ret_To_Mem_L(
    mut name: *mut libc::c_char,
    mut index: libc::c_int,
) {
    if pic_code != 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%s@GOTPCREL(%%rip), %%r10\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%%rax, %d(%%r10)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index * 8 as libc::c_int,
        );
    } else {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%%rax, %s+%d(%%rip)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
            index * 8 as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn Move_Ret_To_Reg_X(mut index: libc::c_int) {
    Inst_Printf(
        b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%%rax, %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Off_Reg_Bank(index * 8 as libc::c_int),
    );
}
pub unsafe extern "C" fn Move_Ret_To_Reg_Y(mut index: libc::c_int) {
    Inst_Printf(
        b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"%%rax, %d(%s)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ((-(3 as libc::c_int) - 1 as libc::c_int - index) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<PlLong>() as libc::c_ulong)
            as libc::c_int,
        asm_reg_e.as_mut_ptr(),
    );
}
pub unsafe extern "C" fn Move_Ret_To_Foreign_L(mut index: libc::c_int) {
    if pic_code != 0 {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"pl_foreign_long@GOTPCREL(%%rip), %%r10\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%%rax, %d(%%r10)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index * 8 as libc::c_int,
        );
    } else {
        Inst_Printf(
            b"movq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"%%rax, pl_foreign_long+%d(%%rip)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index * 8 as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn Init_Mapper() {
    mi.needs_pre_pass = 0 as libc::c_int;
    mi.can_produce_pic_code = 1 as libc::c_int;
    mi.comment_prefix = b"#\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    mi
        .local_symb_prefix = b".L\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    mi
        .string_symb_prefix = b".LC\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    mi
        .double_symb_prefix = b".LCD\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    mi.strings_need_null = 0 as libc::c_int;
    mi.call_c_reverse_args = 0 as libc::c_int;
}

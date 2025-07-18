use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut _zero_: bc_num;
    static mut _one_: bc_num;
    fn bc_new_num(length: libc::c_int, scale_0: libc::c_int) -> bc_num;
    fn bc_free_num(num: *mut bc_num);
    fn bc_copy_num(num: bc_num) -> bc_num;
    fn bc_init_num(num: *mut bc_num);
    fn bc_int2num(num: *mut bc_num, val: libc::c_int);
    fn bc_compare(n1: bc_num, n2: bc_num) -> libc::c_int;
    fn bc_is_zero(num: bc_num) -> libc::c_char;
    fn bc_is_neg(num: bc_num) -> libc::c_char;
    fn bc_add(n1: bc_num, n2: bc_num, result: *mut bc_num, scale_min: libc::c_int);
    fn bc_sub(n1: bc_num, n2: bc_num, result: *mut bc_num, scale_min: libc::c_int);
    fn bc_multiply(n1: bc_num, n2: bc_num, prod: *mut bc_num, scale_0: libc::c_int);
    fn bc_divide(
        n1: bc_num,
        n2: bc_num,
        quot: *mut bc_num,
        scale_0: libc::c_int,
    ) -> libc::c_int;
    fn bc_modulo(
        num1: bc_num,
        num2: bc_num,
        result: *mut bc_num,
        scale_0: libc::c_int,
    ) -> libc::c_int;
    fn bc_raise(num1: bc_num, num2: bc_num, result: *mut bc_num, scale_0: libc::c_int);
    fn bc_sqrt(num: *mut bc_num, scale_0: libc::c_int) -> libc::c_int;
    fn bc_out_num(
        num: bc_num,
        o_base_0: libc::c_int,
        out_char_0: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
        leading_zero: libc::c_int,
    );
    static mut interactive: libc::c_char;
    static mut std_only: libc::c_int;
    static mut functions: *mut bc_function;
    static mut f_names: *mut *mut libc::c_char;
    static mut ex_stack: *mut estack_rec;
    static mut fn_stack: *mut fstack_rec;
    static mut i_base: libc::c_int;
    static mut o_base: libc::c_int;
    static mut scale: libc::c_int;
    static mut c_code: libc::c_char;
    static mut runtime_error: libc::c_char;
    static mut pc: program_counter;
    static mut out_col: libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn random() -> libc::c_long;
    fn incr_array(var_name: libc::c_int);
    fn process_params(_pc_: *mut program_counter, func: libc::c_int);
    fn auto_var(name: libc::c_int);
    fn fpush(val: libc::c_int);
    fn load_array(var_name: libc::c_int);
    fn decr_array(var_name: libc::c_int);
    fn store_array(var_name: libc::c_int);
    fn out_char(ch: libc::c_int);
    fn decr_var(var_name: libc::c_int);
    fn bc_exit(_: libc::c_int);
    fn incr_var(var_name: libc::c_int);
    fn load_var(var_name: libc::c_int);
    fn store_var(var_name: libc::c_int);
    fn out_schar(ch: libc::c_int);
    fn push_copy(num: bc_num);
    fn push_num(num: bc_num);
    fn check_stack(depth: libc::c_int) -> libc::c_char;
    fn rt_error(mesg: *const libc::c_char, _: ...);
    fn pop_vars(list: *mut arg_list);
    fn fpop() -> libc::c_int;
    fn pop();
    fn use_quit(_: libc::c_int);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[inline]
unsafe extern "C" fn getchar() -> libc::c_int {
    return getc(stdin);
}
pub static mut had_sigint: libc::c_int = 0;
pub unsafe extern "C" fn stop_execution(mut sig: libc::c_int) {
    had_sigint = 1 as libc::c_int;
}
pub unsafe extern "C" fn byte(mut p: *mut program_counter) -> libc::c_uchar {
    let fresh0 = (*p).pc_addr;
    (*p).pc_addr = ((*p).pc_addr).wrapping_add(1);
    return *((*functions.offset((*p).pc_func as isize)).f_body).offset(fresh0 as isize)
        as libc::c_uchar;
}
pub unsafe extern "C" fn execute() {
    let mut label_num: libc::c_ulong = 0;
    let mut l_gp: libc::c_ulong = 0;
    let mut l_off: libc::c_ulong = 0;
    let mut gp: *mut bc_label_group = 0 as *mut bc_label_group;
    let mut inst: libc::c_char = 0;
    let mut ch: libc::c_char = 0;
    let mut new_func: libc::c_long = 0;
    let mut var_name: libc::c_long = 0;
    let mut const_base: libc::c_long = 0;
    let mut temp_num: bc_num = 0 as *mut bc_struct;
    let mut auto_list: *mut arg_list = 0 as *mut arg_list;
    pc.pc_func = 0 as libc::c_int as libc::c_uint;
    pc.pc_addr = 0 as libc::c_int as libc::c_uint;
    runtime_error = 0 as libc::c_int as libc::c_char;
    bc_init_num(&mut temp_num);
    if interactive != 0 {
        signal(
            2 as libc::c_int,
            Some(stop_execution as unsafe extern "C" fn(libc::c_int) -> ()),
        );
    }
    had_sigint = 0 as libc::c_int;
    while (pc.pc_addr as libc::c_ulong)
        < (*functions.offset(pc.pc_func as isize)).f_code_size && runtime_error == 0
        && had_sigint == 0
    {
        inst = byte(&mut pc) as libc::c_char;
        let mut current_block_215: u64;
        match inst as libc::c_int {
            65 => {
                var_name = byte(&mut pc) as libc::c_long;
                if var_name & 0x80 as libc::c_int as libc::c_long
                    != 0 as libc::c_int as libc::c_long
                {
                    var_name = ((var_name & 0x7f as libc::c_int as libc::c_long)
                        << 8 as libc::c_int) + byte(&mut pc) as libc::c_long;
                }
                incr_array(var_name as libc::c_int);
                current_block_215 = 12343738388509029619;
            }
            66 | 90 => {
                c_code = (bc_is_zero((*ex_stack).s_num) == 0) as libc::c_int
                    as libc::c_char;
                pop();
                current_block_215 = 2519562090548833035;
            }
            74 => {
                current_block_215 = 2519562090548833035;
            }
            67 => {
                new_func = byte(&mut pc) as libc::c_long;
                if new_func & 0x80 as libc::c_int as libc::c_long
                    != 0 as libc::c_int as libc::c_long
                {
                    new_func = ((new_func & 0x7f as libc::c_int as libc::c_long)
                        << 8 as libc::c_int) + byte(&mut pc) as libc::c_long;
                }
                if (*functions.offset(new_func as isize)).f_defined == 0 {
                    rt_error(
                        b"Function %s not defined.\0" as *const u8
                            as *const libc::c_char,
                        *f_names.offset(new_func as isize),
                    );
                } else {
                    process_params(&mut pc, new_func as libc::c_int);
                    auto_list = (*functions.offset(new_func as isize)).f_autos;
                    while !auto_list.is_null() {
                        auto_var((*auto_list).av_name);
                        auto_list = (*auto_list).next;
                    }
                    fpush(pc.pc_func as libc::c_int);
                    fpush(pc.pc_addr as libc::c_int);
                    fpush(i_base);
                    pc.pc_func = new_func as libc::c_uint;
                    pc.pc_addr = 0 as libc::c_int as libc::c_uint;
                }
                current_block_215 = 12343738388509029619;
            }
            68 => {
                push_copy((*ex_stack).s_num);
                current_block_215 = 12343738388509029619;
            }
            75 => {
                if pc.pc_func == 0 as libc::c_int as libc::c_uint {
                    const_base = i_base as libc::c_long;
                } else {
                    const_base = (*fn_stack).s_val as libc::c_long;
                }
                if const_base == 10 as libc::c_int as libc::c_long {
                    push_b10_const(&mut pc);
                } else {
                    push_constant(
                        Some(prog_char as unsafe extern "C" fn() -> libc::c_int),
                        const_base as libc::c_int,
                    );
                }
                current_block_215 = 12343738388509029619;
            }
            76 => {
                var_name = byte(&mut pc) as libc::c_long;
                if var_name & 0x80 as libc::c_int as libc::c_long
                    != 0 as libc::c_int as libc::c_long
                {
                    var_name = ((var_name & 0x7f as libc::c_int as libc::c_long)
                        << 8 as libc::c_int) + byte(&mut pc) as libc::c_long;
                }
                load_array(var_name as libc::c_int);
                current_block_215 = 12343738388509029619;
            }
            77 => {
                var_name = byte(&mut pc) as libc::c_long;
                if var_name & 0x80 as libc::c_int as libc::c_long
                    != 0 as libc::c_int as libc::c_long
                {
                    var_name = ((var_name & 0x7f as libc::c_int as libc::c_long)
                        << 8 as libc::c_int) + byte(&mut pc) as libc::c_long;
                }
                decr_array(var_name as libc::c_int);
                current_block_215 = 12343738388509029619;
            }
            79 => {
                loop {
                    ch = byte(&mut pc) as libc::c_char;
                    if !(ch as libc::c_int != '"' as i32) {
                        break;
                    }
                    if ch as libc::c_int != '\\' as i32 {
                        out_schar(ch as libc::c_int);
                    } else {
                        ch = byte(&mut pc) as libc::c_char;
                        if ch as libc::c_int == '"' as i32 {
                            break;
                        }
                        match ch as libc::c_int {
                            97 => {
                                out_schar(0o7 as libc::c_int);
                            }
                            98 => {
                                out_schar('\u{8}' as i32);
                            }
                            102 => {
                                out_schar('\u{c}' as i32);
                            }
                            110 => {
                                out_schar('\n' as i32);
                            }
                            113 => {
                                out_schar('"' as i32);
                            }
                            114 => {
                                out_schar('\r' as i32);
                            }
                            116 => {
                                out_schar('\t' as i32);
                            }
                            92 => {
                                out_schar('\\' as i32);
                            }
                            _ => {}
                        }
                    }
                }
                fflush(stdout);
                current_block_215 = 12343738388509029619;
            }
            82 => {
                if pc.pc_func != 0 as libc::c_int as libc::c_uint {
                    pop_vars((*functions.offset(pc.pc_func as isize)).f_autos);
                    pop_vars((*functions.offset(pc.pc_func as isize)).f_params);
                    fpop();
                    pc.pc_addr = fpop() as libc::c_uint;
                    pc.pc_func = fpop() as libc::c_uint;
                } else {
                    rt_error(
                        b"Return from main program.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block_215 = 12343738388509029619;
            }
            83 => {
                var_name = byte(&mut pc) as libc::c_long;
                if var_name & 0x80 as libc::c_int as libc::c_long
                    != 0 as libc::c_int as libc::c_long
                {
                    var_name = ((var_name & 0x7f as libc::c_int as libc::c_long)
                        << 8 as libc::c_int) + byte(&mut pc) as libc::c_long;
                }
                store_array(var_name as libc::c_int);
                current_block_215 = 12343738388509029619;
            }
            84 => {
                c_code = bc_is_zero((*ex_stack).s_num);
                assign(c_code);
                current_block_215 = 12343738388509029619;
            }
            87 | 80 => {
                bc_out_num(
                    (*ex_stack).s_num,
                    o_base,
                    Some(out_char as unsafe extern "C" fn(libc::c_int) -> ()),
                    std_only,
                );
                if inst as libc::c_int == 'W' as i32 {
                    out_char('\n' as i32);
                }
                store_var(4 as libc::c_int);
                fflush(stdout);
                pop();
                current_block_215 = 12343738388509029619;
            }
            99 => {
                new_func = byte(&mut pc) as libc::c_long;
                match new_func {
                    76 => {
                        if (*(*ex_stack).s_num).n_len == 1 as libc::c_int
                            && (*(*ex_stack).s_num).n_scale != 0 as libc::c_int
                            && *((*(*ex_stack).s_num).n_value)
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                == 0 as libc::c_int
                        {
                            bc_int2num(
                                &mut (*ex_stack).s_num,
                                (*(*ex_stack).s_num).n_scale,
                            );
                        } else {
                            bc_int2num(
                                &mut (*ex_stack).s_num,
                                (*(*ex_stack).s_num).n_len + (*(*ex_stack).s_num).n_scale,
                            );
                        }
                    }
                    83 => {
                        bc_int2num(&mut (*ex_stack).s_num, (*(*ex_stack).s_num).n_scale);
                    }
                    82 => {
                        if bc_sqrt(&mut (*ex_stack).s_num, scale) == 0 {
                            rt_error(
                                b"Square root of a negative number\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    }
                    73 => {
                        push_constant(
                            Some(input_char as unsafe extern "C" fn() -> libc::c_int),
                            i_base,
                        );
                    }
                    88 => {
                        push_copy(_zero_);
                        bc_int2num(&mut (*ex_stack).s_num, random() as libc::c_int);
                    }
                    _ => {}
                }
                current_block_215 = 12343738388509029619;
            }
            100 => {
                var_name = byte(&mut pc) as libc::c_long;
                if var_name & 0x80 as libc::c_int as libc::c_long
                    != 0 as libc::c_int as libc::c_long
                {
                    var_name = ((var_name & 0x7f as libc::c_int as libc::c_long)
                        << 8 as libc::c_int) + byte(&mut pc) as libc::c_long;
                }
                decr_var(var_name as libc::c_int);
                current_block_215 = 12343738388509029619;
            }
            104 => {
                bc_exit(0 as libc::c_int);
                current_block_215 = 12343738388509029619;
            }
            105 => {
                var_name = byte(&mut pc) as libc::c_long;
                if var_name & 0x80 as libc::c_int as libc::c_long
                    != 0 as libc::c_int as libc::c_long
                {
                    var_name = ((var_name & 0x7f as libc::c_int as libc::c_long)
                        << 8 as libc::c_int) + byte(&mut pc) as libc::c_long;
                }
                incr_var(var_name as libc::c_int);
                current_block_215 = 12343738388509029619;
            }
            108 => {
                var_name = byte(&mut pc) as libc::c_long;
                if var_name & 0x80 as libc::c_int as libc::c_long
                    != 0 as libc::c_int as libc::c_long
                {
                    var_name = ((var_name & 0x7f as libc::c_int as libc::c_long)
                        << 8 as libc::c_int) + byte(&mut pc) as libc::c_long;
                }
                load_var(var_name as libc::c_int);
                current_block_215 = 12343738388509029619;
            }
            110 => {
                bc_sub(
                    _zero_,
                    (*ex_stack).s_num,
                    &mut (*ex_stack).s_num,
                    0 as libc::c_int,
                );
                current_block_215 = 12343738388509029619;
            }
            112 => {
                pop();
                current_block_215 = 12343738388509029619;
            }
            115 => {
                var_name = byte(&mut pc) as libc::c_long;
                if var_name & 0x80 as libc::c_int as libc::c_long
                    != 0 as libc::c_int as libc::c_long
                {
                    var_name = ((var_name & 0x7f as libc::c_int as libc::c_long)
                        << 8 as libc::c_int) + byte(&mut pc) as libc::c_long;
                }
                store_var(var_name as libc::c_int);
                current_block_215 = 12343738388509029619;
            }
            119 => {
                loop {
                    ch = byte(&mut pc) as libc::c_char;
                    if !(ch as libc::c_int != '"' as i32) {
                        break;
                    }
                    out_schar(ch as libc::c_int);
                }
                fflush(stdout);
                current_block_215 = 12343738388509029619;
            }
            120 => {
                if check_stack(2 as libc::c_int) != 0 {
                    let mut temp: bc_num = (*ex_stack).s_num;
                    (*ex_stack).s_num = (*(*ex_stack).s_next).s_num;
                    (*(*ex_stack).s_next).s_num = temp;
                }
                current_block_215 = 12343738388509029619;
            }
            48 => {
                push_copy(_zero_);
                current_block_215 = 12343738388509029619;
            }
            49 => {
                push_copy(_one_);
                current_block_215 = 12343738388509029619;
            }
            33 => {
                c_code = bc_is_zero((*ex_stack).s_num);
                assign(c_code);
                current_block_215 = 12343738388509029619;
            }
            38 => {
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_is_zero((*(*ex_stack).s_next).s_num) == 0
                        && bc_is_zero((*ex_stack).s_num) == 0) as libc::c_int
                        as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 12343738388509029619;
            }
            124 => {
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_is_zero((*(*ex_stack).s_next).s_num) == 0
                        || bc_is_zero((*ex_stack).s_num) == 0) as libc::c_int
                        as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 12343738388509029619;
            }
            43 => {
                if check_stack(2 as libc::c_int) != 0 {
                    bc_add(
                        (*(*ex_stack).s_next).s_num,
                        (*ex_stack).s_num,
                        &mut temp_num,
                        0 as libc::c_int,
                    );
                    pop();
                    pop();
                    push_num(temp_num);
                    bc_init_num(&mut temp_num);
                }
                current_block_215 = 12343738388509029619;
            }
            45 => {
                if check_stack(2 as libc::c_int) != 0 {
                    bc_sub(
                        (*(*ex_stack).s_next).s_num,
                        (*ex_stack).s_num,
                        &mut temp_num,
                        0 as libc::c_int,
                    );
                    pop();
                    pop();
                    push_num(temp_num);
                    bc_init_num(&mut temp_num);
                }
                current_block_215 = 12343738388509029619;
            }
            42 => {
                if check_stack(2 as libc::c_int) != 0 {
                    bc_multiply(
                        (*(*ex_stack).s_next).s_num,
                        (*ex_stack).s_num,
                        &mut temp_num,
                        scale,
                    );
                    pop();
                    pop();
                    push_num(temp_num);
                    bc_init_num(&mut temp_num);
                }
                current_block_215 = 12343738388509029619;
            }
            47 => {
                if check_stack(2 as libc::c_int) != 0 {
                    if bc_divide(
                        (*(*ex_stack).s_next).s_num,
                        (*ex_stack).s_num,
                        &mut temp_num,
                        scale,
                    ) == 0 as libc::c_int
                    {
                        pop();
                        pop();
                        push_num(temp_num);
                        bc_init_num(&mut temp_num);
                    } else {
                        rt_error(
                            b"Divide by zero\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
                current_block_215 = 12343738388509029619;
            }
            37 => {
                if check_stack(2 as libc::c_int) != 0 {
                    if bc_is_zero((*ex_stack).s_num) != 0 {
                        rt_error(
                            b"Modulo by zero\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        bc_modulo(
                            (*(*ex_stack).s_next).s_num,
                            (*ex_stack).s_num,
                            &mut temp_num,
                            scale,
                        );
                        pop();
                        pop();
                        push_num(temp_num);
                        bc_init_num(&mut temp_num);
                    }
                }
                current_block_215 = 12343738388509029619;
            }
            94 => {
                if check_stack(2 as libc::c_int) != 0 {
                    bc_raise(
                        (*(*ex_stack).s_next).s_num,
                        (*ex_stack).s_num,
                        &mut temp_num,
                        scale,
                    );
                    if bc_is_zero((*(*ex_stack).s_next).s_num) as libc::c_int != 0
                        && bc_is_neg((*ex_stack).s_num) as libc::c_int != 0
                    {
                        rt_error(
                            b"divide by zero\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    pop();
                    pop();
                    push_num(temp_num);
                    bc_init_num(&mut temp_num);
                }
                current_block_215 = 12343738388509029619;
            }
            61 => {
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        == 0 as libc::c_int) as libc::c_int as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 12343738388509029619;
            }
            35 => {
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        != 0 as libc::c_int) as libc::c_int as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 12343738388509029619;
            }
            60 => {
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        == -(1 as libc::c_int)) as libc::c_int as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 12343738388509029619;
            }
            123 => {
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        <= 0 as libc::c_int) as libc::c_int as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 12343738388509029619;
            }
            62 => {
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        == 1 as libc::c_int) as libc::c_int as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 12343738388509029619;
            }
            125 => {
                if check_stack(2 as libc::c_int) != 0 {
                    c_code = (bc_compare((*(*ex_stack).s_next).s_num, (*ex_stack).s_num)
                        >= 0 as libc::c_int) as libc::c_int as libc::c_char;
                    pop();
                    assign(c_code);
                }
                current_block_215 = 12343738388509029619;
            }
            _ => {
                rt_error(
                    b"bad instruction: inst=%c\0" as *const u8 as *const libc::c_char,
                    inst as libc::c_int,
                );
                current_block_215 = 12343738388509029619;
            }
        }
        match current_block_215 {
            2519562090548833035 => {
                label_num = byte(&mut pc) as libc::c_ulong;
                label_num = label_num
                    .wrapping_add(
                        ((byte(&mut pc) as libc::c_int) << 8 as libc::c_int)
                            as libc::c_ulong,
                    );
                if inst as libc::c_int == 'J' as i32
                    || inst as libc::c_int == 'B' as i32 && c_code as libc::c_int != 0
                    || inst as libc::c_int == 'Z' as i32 && c_code == 0
                {
                    gp = (*functions.offset(pc.pc_func as isize)).f_label;
                    l_gp = label_num >> 6 as libc::c_int;
                    l_off = label_num.wrapping_rem(64 as libc::c_int as libc::c_ulong);
                    loop {
                        let fresh1 = l_gp;
                        l_gp = l_gp.wrapping_sub(1);
                        if !(fresh1 > 0 as libc::c_int as libc::c_ulong) {
                            break;
                        }
                        gp = (*gp).l_next;
                    }
                    if !gp.is_null() {
                        pc.pc_addr = (*gp).l_adrs[l_off as usize] as libc::c_uint;
                    } else {
                        rt_error(
                            b"Internal error.\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
            _ => {}
        }
    }
    while pc.pc_func != 0 as libc::c_int as libc::c_uint {
        pop_vars((*functions.offset(pc.pc_func as isize)).f_autos);
        pop_vars((*functions.offset(pc.pc_func as isize)).f_params);
        fpop();
        pc.pc_addr = fpop() as libc::c_uint;
        pc.pc_func = fpop() as libc::c_uint;
    }
    while !ex_stack.is_null() {
        pop();
    }
    if interactive != 0 {
        signal(
            2 as libc::c_int,
            Some(use_quit as unsafe extern "C" fn(libc::c_int) -> ()),
        );
        if had_sigint != 0 {
            printf(b"\ninterrupted execution.\n\0" as *const u8 as *const libc::c_char);
        }
    }
}
pub unsafe extern "C" fn prog_char() -> libc::c_int {
    return byte(&mut pc) as libc::c_int;
}
pub unsafe extern "C" fn input_char() -> libc::c_int {
    let mut in_ch: libc::c_int = 0;
    in_ch = getchar();
    if in_ch == '\\' as i32 {
        in_ch = getchar();
        if in_ch == '\n' as i32 {
            in_ch = getchar();
            out_col = 0 as libc::c_int;
        }
    }
    if *(*__ctype_b_loc()).offset(in_ch as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        return in_ch - '0' as i32;
    }
    if in_ch >= 'A' as i32 && in_ch <= 'Z' as i32 {
        return in_ch + 10 as libc::c_int - 'A' as i32;
    }
    if in_ch >= 'a' as i32 && in_ch <= 'z' as i32 {
        return in_ch + 10 as libc::c_int - 'a' as i32;
    }
    if in_ch == '.' as i32 || in_ch == '+' as i32 || in_ch == '-' as i32 {
        return in_ch;
    }
    if in_ch == '~' as i32 {
        return ':' as i32;
    }
    if in_ch <= ' ' as i32 {
        return '~' as i32;
    }
    return ':' as i32;
}
pub unsafe extern "C" fn push_constant(
    mut in_char: Option::<unsafe extern "C" fn() -> libc::c_int>,
    mut conv_base: libc::c_int,
) {
    let mut digits: libc::c_int = 0;
    let mut build: bc_num = 0 as *mut bc_struct;
    let mut temp: bc_num = 0 as *mut bc_struct;
    let mut result: bc_num = 0 as *mut bc_struct;
    let mut mult: bc_num = 0 as *mut bc_struct;
    let mut divisor: bc_num = 0 as *mut bc_struct;
    let mut in_ch: libc::c_int = 0;
    let mut first_ch: libc::c_int = 0;
    let mut negative: libc::c_char = 0;
    bc_init_num(&mut temp);
    bc_init_num(&mut result);
    bc_init_num(&mut mult);
    build = bc_copy_num(_zero_);
    negative = 0 as libc::c_int as libc::c_char;
    bc_int2num(&mut mult, conv_base);
    in_ch = in_char.unwrap()();
    while in_ch == '~' as i32 {
        in_ch = in_char.unwrap()();
    }
    if in_ch == '+' as i32 {
        in_ch = in_char.unwrap()();
    } else if in_ch == '-' as i32 {
        negative = 1 as libc::c_int as libc::c_char;
        in_ch = in_char.unwrap()();
    }
    if in_ch < 36 as libc::c_int {
        first_ch = in_ch;
        in_ch = in_char.unwrap()();
        if in_ch < 36 as libc::c_int && first_ch >= conv_base {
            first_ch = conv_base - 1 as libc::c_int;
        }
        bc_int2num(&mut build, first_ch);
    }
    while in_ch < 36 as libc::c_int {
        if in_ch < 36 as libc::c_int && in_ch >= conv_base {
            in_ch = conv_base - 1 as libc::c_int;
        }
        bc_multiply(build, mult, &mut result, 0 as libc::c_int);
        bc_int2num(&mut temp, in_ch);
        bc_add(result, temp, &mut build, 0 as libc::c_int);
        in_ch = in_char.unwrap()();
    }
    if in_ch == '.' as i32 {
        in_ch = in_char.unwrap()();
        if in_ch >= conv_base {
            in_ch = conv_base - 1 as libc::c_int;
        }
        bc_free_num(&mut result);
        bc_free_num(&mut temp);
        divisor = bc_copy_num(_one_);
        result = bc_copy_num(_zero_);
        digits = 0 as libc::c_int;
        while in_ch < 36 as libc::c_int {
            bc_multiply(result, mult, &mut result, 0 as libc::c_int);
            bc_int2num(&mut temp, in_ch);
            bc_add(result, temp, &mut result, 0 as libc::c_int);
            bc_multiply(divisor, mult, &mut divisor, 0 as libc::c_int);
            digits += 1;
            digits;
            in_ch = in_char.unwrap()();
            if in_ch < 36 as libc::c_int && in_ch >= conv_base {
                in_ch = conv_base - 1 as libc::c_int;
            }
        }
        bc_divide(result, divisor, &mut result, digits);
        bc_add(build, result, &mut build, 0 as libc::c_int);
    }
    if negative != 0 {
        bc_sub(_zero_, build, &mut build, 0 as libc::c_int);
    }
    push_num(build);
    bc_free_num(&mut temp);
    bc_free_num(&mut result);
    bc_free_num(&mut mult);
}
pub unsafe extern "C" fn push_b10_const(mut progctr: *mut program_counter) {
    let mut build: bc_num = 0 as *mut bc_struct;
    let mut look_pc: program_counter = program_counter {
        pc_func: 0,
        pc_addr: 0,
    };
    let mut kdigits: libc::c_int = 0;
    let mut kscale: libc::c_int = 0;
    let mut inchar: libc::c_uchar = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    look_pc = *progctr;
    kdigits = 0 as libc::c_int;
    kscale = 0 as libc::c_int;
    inchar = byte(&mut look_pc);
    while inchar as libc::c_int != '.' as i32 && inchar as libc::c_int != ':' as i32 {
        kdigits += 1;
        kdigits;
        inchar = byte(&mut look_pc);
    }
    if inchar as libc::c_int == '.' as i32 {
        inchar = byte(&mut look_pc);
        while inchar as libc::c_int != ':' as i32 {
            kscale += 1;
            kscale;
            inchar = byte(&mut look_pc);
        }
    }
    inchar = byte(progctr);
    if kdigits == 1 as libc::c_int && kscale == 0 as libc::c_int {
        if inchar as libc::c_int == 0 as libc::c_int {
            push_copy(_zero_);
            inchar = byte(progctr);
            return;
        }
        if inchar as libc::c_int == 1 as libc::c_int {
            push_copy(_one_);
            inchar = byte(progctr);
            return;
        }
        if inchar as libc::c_int > 9 as libc::c_int {
            bc_init_num(&mut build);
            bc_int2num(&mut build, inchar as libc::c_int);
            push_num(build);
            inchar = byte(progctr);
            return;
        }
    }
    if kdigits == 0 as libc::c_int {
        build = bc_new_num(1 as libc::c_int, kscale);
        ptr = (*build).n_value;
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        *fresh2 = 0 as libc::c_int as libc::c_char;
    } else {
        build = bc_new_num(kdigits, kscale);
        ptr = (*build).n_value;
    }
    while inchar as libc::c_int != ':' as i32 {
        if inchar as libc::c_int != '.' as i32 {
            if inchar as libc::c_int > 9 as libc::c_int {
                let fresh3 = ptr;
                ptr = ptr.offset(1);
                *fresh3 = 9 as libc::c_int as libc::c_char;
            } else {
                let fresh4 = ptr;
                ptr = ptr.offset(1);
                *fresh4 = inchar as libc::c_char;
            }
        }
        inchar = byte(progctr);
    }
    push_num(build);
}
pub unsafe extern "C" fn assign(mut code: libc::c_char) {
    bc_free_num(&mut (*ex_stack).s_num);
    if code != 0 {
        (*ex_stack).s_num = bc_copy_num(_one_);
    } else {
        (*ex_stack).s_num = bc_copy_num(_zero_);
    };
}

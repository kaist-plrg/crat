use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut warn_not_std: libc::c_int;
    static mut std_only: libc::c_int;
    static mut functions: *mut bc_function;
    static mut had_error: libc::c_int;
    fn execute();
    fn nextarg(
        args: *mut arg_list,
        val: libc::c_int,
        is_var: libc::c_int,
    ) -> *mut arg_list;
    fn bc_malloc(_: size_t) -> *mut libc::c_void;
    fn yyerror(str: *const libc::c_char, _: ...);
    fn ct_warn(mesg: *const libc::c_char, _: ...);
    fn bc_exit(_: libc::c_int);
    fn clear_func(func: libc::c_int);
    fn free(__ptr: *mut libc::c_void);
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
pub static mut load_adr: program_counter = program_counter {
    pc_func: 0,
    pc_addr: 0,
};
pub static mut load_str: libc::c_char = 0;
pub static mut load_const: libc::c_char = 0;
pub unsafe extern "C" fn init_load() {
    clear_func(0 as libc::c_int);
    load_adr.pc_func = 0 as libc::c_int as libc::c_uint;
    load_adr.pc_addr = 0 as libc::c_int as libc::c_uint;
    load_str = 0 as libc::c_int as libc::c_char;
    load_const = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn addbyte(mut thebyte: libc::c_uchar) {
    let mut prog_addr: libc::c_ulong = 0;
    let mut f: *mut bc_function = 0 as *mut bc_function;
    let mut new_body: *mut libc::c_char = 0 as *mut libc::c_char;
    if had_error != 0 {
        return;
    }
    let fresh0 = load_adr.pc_addr;
    load_adr.pc_addr = (load_adr.pc_addr).wrapping_add(1);
    prog_addr = fresh0 as libc::c_ulong;
    f = &mut *functions.offset(load_adr.pc_func as isize) as *mut bc_function;
    if prog_addr >= (*f).f_body_size {
        (*f)
            .f_body_size = ((*f).f_body_size as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        new_body = bc_malloc((*f).f_body_size) as *mut libc::c_char;
        memcpy(
            new_body as *mut libc::c_void,
            (*f).f_body as *const libc::c_void,
            ((*f).f_body_size).wrapping_div(2 as libc::c_int as libc::c_ulong),
        );
        free((*f).f_body as *mut libc::c_void);
        (*f).f_body = new_body;
    }
    *((*f).f_body)
        .offset(
            prog_addr as isize,
        ) = (thebyte as libc::c_int & 0xff as libc::c_int) as libc::c_char;
    (*f).f_code_size = ((*f).f_code_size).wrapping_add(1);
    (*f).f_code_size;
}
pub unsafe extern "C" fn def_label(mut lab: libc::c_ulong) {
    let mut temp: *mut bc_label_group = 0 as *mut bc_label_group;
    let mut group: libc::c_ulong = 0;
    let mut offset: libc::c_ulong = 0;
    let mut func: libc::c_ulong = 0;
    group = lab >> 6 as libc::c_int;
    offset = lab.wrapping_rem(64 as libc::c_int as libc::c_ulong);
    func = load_adr.pc_func as libc::c_ulong;
    if ((*functions.offset(func as isize)).f_label).is_null() {
        let ref mut fresh1 = (*functions.offset(func as isize)).f_label;
        *fresh1 = bc_malloc(::std::mem::size_of::<bc_label_group>() as libc::c_ulong)
            as *mut bc_label_group;
        let ref mut fresh2 = (*(*functions.offset(func as isize)).f_label).l_next;
        *fresh2 = 0 as *mut bc_label_group;
    }
    temp = (*functions.offset(func as isize)).f_label;
    while group > 0 as libc::c_int as libc::c_ulong {
        if ((*temp).l_next).is_null() {
            (*temp)
                .l_next = bc_malloc(
                ::std::mem::size_of::<bc_label_group>() as libc::c_ulong,
            ) as *mut bc_label_group;
            (*(*temp).l_next).l_next = 0 as *mut bc_label_group;
        }
        temp = (*temp).l_next;
        group = group.wrapping_sub(1);
        group;
    }
    (*temp).l_adrs[offset as usize] = load_adr.pc_addr as libc::c_ulong;
}
pub unsafe extern "C" fn long_val(mut str: *mut *const libc::c_char) -> libc::c_long {
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut neg: libc::c_char = 0 as libc::c_int as libc::c_char;
    if **str as libc::c_int == '-' as i32 {
        neg = 1 as libc::c_int as libc::c_char;
        *str = (*str).offset(1);
        *str;
    }
    while *(*__ctype_b_loc()).offset(**str as libc::c_int as isize) as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        let fresh3 = *str;
        *str = (*str).offset(1);
        val = val * 10 as libc::c_int + *fresh3 as libc::c_int - '0' as i32;
    }
    if neg != 0 { return -val as libc::c_long } else { return val as libc::c_long };
}
pub unsafe extern "C" fn load_code(mut code: *const libc::c_char) {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap_name: libc::c_ulong = 0;
    let mut label_no: libc::c_ulong = 0;
    let mut vaf_name: libc::c_ulong = 0;
    let mut func: libc::c_ulong = 0;
    static mut save_adr: program_counter = program_counter {
        pc_func: 0,
        pc_addr: 0,
    };
    str = code;
    while *str as libc::c_int != 0 as libc::c_int {
        if had_error != 0 {
            return;
        }
        if load_str != 0 {
            if *str as libc::c_int == '"' as i32 {
                load_str = 0 as libc::c_int as libc::c_char;
            }
            let fresh4 = str;
            str = str.offset(1);
            addbyte(*fresh4 as libc::c_uchar);
        } else if load_const != 0 {
            if *str as libc::c_int == '\n' as i32 {
                str = str.offset(1);
                str;
            } else if *str as libc::c_int == ':' as i32 {
                load_const = 0 as libc::c_int as libc::c_char;
                let fresh5 = str;
                str = str.offset(1);
                addbyte(*fresh5 as libc::c_uchar);
            } else if *str as libc::c_int == '.' as i32 {
                let fresh6 = str;
                str = str.offset(1);
                addbyte(*fresh6 as libc::c_uchar);
            } else {
                if *str as libc::c_int > 'F' as i32
                    && (warn_not_std != 0 || std_only != 0)
                {
                    if std_only != 0 {
                        yyerror(
                            b"Error in numeric constant\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        ct_warn(
                            b"Non-standard base in numeric constant\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                if *str as libc::c_int >= 'A' as i32 {
                    let fresh7 = str;
                    str = str.offset(1);
                    addbyte(
                        (*fresh7 as libc::c_int + 10 as libc::c_int - 'A' as i32)
                            as libc::c_uchar,
                    );
                } else {
                    let fresh8 = str;
                    str = str.offset(1);
                    addbyte((*fresh8 as libc::c_int - '0' as i32) as libc::c_uchar);
                }
            }
        } else {
            let mut current_block_88: u64;
            match *str as libc::c_int {
                34 => {
                    load_str = 1 as libc::c_int as libc::c_char;
                    current_block_88 = 10393716428851982524;
                }
                78 => {
                    str = str.offset(1);
                    str;
                    label_no = long_val(&mut str) as libc::c_ulong;
                    def_label(label_no);
                    current_block_88 = 10393716428851982524;
                }
                66 => {
                    current_block_88 = 11489031855839515445;
                }
                74 | 90 => {
                    current_block_88 = 11489031855839515445;
                }
                70 => {
                    str = str.offset(1);
                    str;
                    func = long_val(&mut str) as libc::c_ulong;
                    clear_func(func as libc::c_int);
                    loop {
                        let fresh10 = str;
                        str = str.offset(1);
                        if !(*fresh10 as libc::c_int != '.' as i32) {
                            break;
                        }
                        if *str as libc::c_int == '.' as i32 {
                            str = str.offset(1);
                            str;
                            break;
                        } else if *str as libc::c_int == '*' as i32 {
                            str = str.offset(1);
                            str;
                            ap_name = long_val(&mut str) as libc::c_ulong;
                            let ref mut fresh11 = (*functions
                                .offset(func as libc::c_int as isize))
                                .f_params;
                            *fresh11 = nextarg(
                                (*functions.offset(func as libc::c_int as isize)).f_params,
                                ap_name as libc::c_int,
                                1 as libc::c_int,
                            );
                        } else {
                            ap_name = long_val(&mut str) as libc::c_ulong;
                            let ref mut fresh12 = (*functions
                                .offset(func as libc::c_int as isize))
                                .f_params;
                            *fresh12 = nextarg(
                                (*functions.offset(func as libc::c_int as isize)).f_params,
                                ap_name as libc::c_int,
                                0 as libc::c_int,
                            );
                        }
                    }
                    while *str as libc::c_int != '[' as i32 {
                        if *str as libc::c_int == ',' as i32 {
                            str = str.offset(1);
                            str;
                        }
                        ap_name = long_val(&mut str) as libc::c_ulong;
                        let ref mut fresh13 = (*functions
                            .offset(func as libc::c_int as isize))
                            .f_autos;
                        *fresh13 = nextarg(
                            (*functions.offset(func as libc::c_int as isize)).f_autos,
                            ap_name as libc::c_int,
                            0 as libc::c_int,
                        );
                    }
                    save_adr = load_adr;
                    load_adr.pc_func = func as libc::c_uint;
                    load_adr.pc_addr = 0 as libc::c_int as libc::c_uint;
                    current_block_88 = 10393716428851982524;
                }
                93 => {
                    (*functions.offset(load_adr.pc_func as isize))
                        .f_defined = 1 as libc::c_int as libc::c_char;
                    load_adr = save_adr;
                    current_block_88 = 10393716428851982524;
                }
                67 => {
                    let fresh14 = str;
                    str = str.offset(1);
                    addbyte(*fresh14 as libc::c_uchar);
                    func = long_val(&mut str) as libc::c_ulong;
                    if func < 128 as libc::c_int as libc::c_ulong {
                        addbyte(func as libc::c_char as libc::c_uchar);
                    } else {
                        addbyte(
                            (func >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong
                                | 0x80 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                        );
                        addbyte(
                            (func & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar,
                        );
                    }
                    if *str as libc::c_int == ',' as i32 {
                        str = str.offset(1);
                        str;
                    }
                    while *str as libc::c_int != ':' as i32 {
                        let fresh15 = str;
                        str = str.offset(1);
                        addbyte(*fresh15 as libc::c_uchar);
                    }
                    addbyte(':' as i32 as libc::c_uchar);
                    current_block_88 = 10393716428851982524;
                }
                99 => {
                    let fresh16 = str;
                    str = str.offset(1);
                    addbyte(*fresh16 as libc::c_uchar);
                    addbyte(*str as libc::c_uchar);
                    current_block_88 = 10393716428851982524;
                }
                75 => {
                    addbyte(*str as libc::c_uchar);
                    load_const = 1 as libc::c_int as libc::c_char;
                    current_block_88 = 10393716428851982524;
                }
                100 => {
                    current_block_88 = 12097614021719101263;
                }
                105 => {
                    current_block_88 = 12097614021719101263;
                }
                108 => {
                    current_block_88 = 7276789952063056912;
                }
                115 => {
                    current_block_88 = 17825897184406634955;
                }
                65 => {
                    current_block_88 = 10906814863437986970;
                }
                77 => {
                    current_block_88 = 5945114928109262113;
                }
                76 | 83 => {
                    current_block_88 = 16858144124982468868;
                }
                64 => {
                    str = str.offset(1);
                    match *str as libc::c_int {
                        105 => {
                            init_load();
                        }
                        114 => {
                            execute();
                        }
                        _ => {}
                    }
                    current_block_88 = 10393716428851982524;
                }
                10 => {
                    current_block_88 = 10393716428851982524;
                }
                _ => {
                    addbyte(*str as libc::c_uchar);
                    current_block_88 = 10393716428851982524;
                }
            }
            match current_block_88 {
                11489031855839515445 => {
                    let fresh9 = str;
                    str = str.offset(1);
                    addbyte(*fresh9 as libc::c_uchar);
                    label_no = long_val(&mut str) as libc::c_ulong;
                    if label_no > 65535 as libc::c_long as libc::c_ulong {
                        fprintf(
                            stderr,
                            b"Program too big.\n\0" as *const u8 as *const libc::c_char,
                        );
                        bc_exit(1 as libc::c_int);
                    }
                    addbyte(
                        (label_no & 0xff as libc::c_int as libc::c_ulong) as libc::c_char
                            as libc::c_uchar,
                    );
                    addbyte(
                        (label_no >> 8 as libc::c_int) as libc::c_char as libc::c_uchar,
                    );
                    current_block_88 = 10393716428851982524;
                }
                12097614021719101263 => {
                    current_block_88 = 7276789952063056912;
                }
                _ => {}
            }
            match current_block_88 {
                7276789952063056912 => {
                    current_block_88 = 17825897184406634955;
                }
                _ => {}
            }
            match current_block_88 {
                17825897184406634955 => {
                    current_block_88 = 10906814863437986970;
                }
                _ => {}
            }
            match current_block_88 {
                10906814863437986970 => {
                    current_block_88 = 5945114928109262113;
                }
                _ => {}
            }
            match current_block_88 {
                5945114928109262113 => {
                    current_block_88 = 16858144124982468868;
                }
                _ => {}
            }
            match current_block_88 {
                16858144124982468868 => {
                    let fresh17 = str;
                    str = str.offset(1);
                    addbyte(*fresh17 as libc::c_uchar);
                    vaf_name = long_val(&mut str) as libc::c_ulong;
                    if vaf_name < 128 as libc::c_int as libc::c_ulong {
                        addbyte(vaf_name as libc::c_uchar);
                    } else {
                        addbyte(
                            (vaf_name >> 8 as libc::c_int
                                & 0xff as libc::c_int as libc::c_ulong
                                | 0x80 as libc::c_int as libc::c_ulong) as libc::c_uchar,
                        );
                        addbyte(
                            (vaf_name & 0xff as libc::c_int as libc::c_ulong)
                                as libc::c_uchar,
                        );
                    }
                }
                _ => {}
            }
            str = str.offset(1);
            str;
        }
    }
}

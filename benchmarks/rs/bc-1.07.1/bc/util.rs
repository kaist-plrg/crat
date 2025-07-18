use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut break_label: libc::c_int;
    static mut continue_label: libc::c_int;
    static mut next_label: libc::c_int;
    static mut genstr: *mut libc::c_char;
    static mut genlen: libc::c_int;
    static mut out_count: libc::c_int;
    static mut did_gen: libc::c_char;
    static mut compile_only: libc::c_int;
    static mut use_math: libc::c_int;
    static mut warn_not_std: libc::c_int;
    static mut std_only: libc::c_int;
    static mut file_name: *mut libc::c_char;
    static mut is_std_in: libc::c_char;
    static mut f_names: *mut *mut libc::c_char;
    static mut f_count: libc::c_int;
    static mut v_names: *mut *mut libc::c_char;
    static mut v_count: libc::c_int;
    static mut a_names: *mut *mut libc::c_char;
    static mut a_count: libc::c_int;
    static mut runtime_error: libc::c_char;
    static mut pc: program_counter;
    static mut out_col: libc::c_int;
    static mut line_size: libc::c_int;
    static mut line_no: libc::c_int;
    static mut had_error: libc::c_int;
    static mut next_array: libc::c_int;
    static mut next_func: libc::c_int;
    static mut next_var: libc::c_int;
    static mut name_tree: *mut id_rec;
    fn execute();
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn init_load();
    fn load_code(code: *const libc::c_char);
    fn more_variables();
    fn more_functions();
    fn more_arrays();
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arg_list {
    pub av_name: libc::c_int,
    pub arg_is_var: libc::c_int,
    pub next: *mut arg_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program_counter {
    pub pc_func: libc::c_uint,
    pub pc_addr: libc::c_uint,
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
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
pub unsafe extern "C" fn strcopyof(mut str: *const libc::c_char) -> *mut libc::c_char {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    temp = bc_malloc((strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    return strcpy(temp, str);
}
pub unsafe extern "C" fn nextarg(
    mut args: *mut arg_list,
    mut val: libc::c_int,
    mut is_var: libc::c_int,
) -> *mut arg_list {
    let mut temp: *mut arg_list = 0 as *mut arg_list;
    temp = bc_malloc(::std::mem::size_of::<arg_list>() as libc::c_ulong)
        as *mut arg_list;
    (*temp).av_name = val;
    (*temp).arg_is_var = is_var;
    (*temp).next = args;
    return temp;
}
static mut arglist1: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut arglist2: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
unsafe extern "C" fn make_arg_str(
    mut args: *mut arg_list,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sval: [libc::c_char; 30] = [0; 30];
    if !args.is_null() {
        temp = make_arg_str((*args).next, len + 12 as libc::c_int);
    } else {
        temp = bc_malloc(len as size_t) as *mut libc::c_char;
        *temp = 0 as libc::c_int as libc::c_char;
        return temp;
    }
    if (*args).arg_is_var != 0 {
        if len != 1 as libc::c_int {
            snprintf(
                sval.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
                b"*%d,\0" as *const u8 as *const libc::c_char,
                (*args).av_name,
            );
        } else {
            snprintf(
                sval.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
                b"*%d\0" as *const u8 as *const libc::c_char,
                (*args).av_name,
            );
        }
    } else if len != 1 as libc::c_int {
        snprintf(
            sval.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
            b"%d,\0" as *const u8 as *const libc::c_char,
            (*args).av_name,
        );
    } else {
        snprintf(
            sval.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            (*args).av_name,
        );
    }
    temp = strcat(temp, sval.as_mut_ptr());
    return temp;
}
pub unsafe extern "C" fn arg_str(mut args: *mut arg_list) -> *mut libc::c_char {
    if !arglist2.is_null() {
        free(arglist2 as *mut libc::c_void);
    }
    arglist2 = arglist1;
    arglist1 = make_arg_str(args, 1 as libc::c_int);
    return arglist1;
}
pub unsafe extern "C" fn call_str(mut args: *mut arg_list) -> *mut libc::c_char {
    let mut temp: *mut arg_list = 0 as *mut arg_list;
    let mut arg_count: libc::c_int = 0;
    let mut ix: libc::c_int = 0;
    if !arglist2.is_null() {
        free(arglist2 as *mut libc::c_void);
    }
    arglist2 = arglist1;
    temp = args;
    arg_count = 0 as libc::c_int;
    while !temp.is_null() {
        arg_count += 1;
        arg_count;
        temp = (*temp).next;
    }
    arglist1 = bc_malloc((arg_count + 1 as libc::c_int) as size_t) as *mut libc::c_char;
    temp = args;
    ix = 0 as libc::c_int;
    while !temp.is_null() {
        let fresh0 = ix;
        ix = ix + 1;
        *arglist1
            .offset(
                fresh0 as isize,
            ) = (if (*temp).av_name != 0 { '1' as i32 } else { '0' as i32 })
            as libc::c_char;
        temp = (*temp).next;
    }
    *arglist1.offset(ix as isize) = 0 as libc::c_int as libc::c_char;
    return arglist1;
}
pub unsafe extern "C" fn free_args(mut args: *mut arg_list) {
    let mut temp: *mut arg_list = 0 as *mut arg_list;
    temp = args;
    while !temp.is_null() {
        args = (*args).next;
        free(temp as *mut libc::c_void);
        temp = args;
    }
}
pub unsafe extern "C" fn check_params(
    mut params: *mut arg_list,
    mut autos: *mut arg_list,
) {
    let mut tmp1: *mut arg_list = 0 as *mut arg_list;
    let mut tmp2: *mut arg_list = 0 as *mut arg_list;
    if !params.is_null() {
        tmp1 = params;
        while !tmp1.is_null() {
            tmp2 = (*tmp1).next;
            while !tmp2.is_null() {
                if (*tmp2).av_name == (*tmp1).av_name {
                    yyerror(
                        b"duplicate parameter names\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                tmp2 = (*tmp2).next;
            }
            if (*tmp1).arg_is_var != 0 {
                ct_warn(
                    b"Variable array parameter\0" as *const u8 as *const libc::c_char,
                );
            }
            tmp1 = (*tmp1).next;
        }
    }
    if !autos.is_null() {
        tmp1 = autos;
        while !tmp1.is_null() {
            tmp2 = (*tmp1).next;
            while !tmp2.is_null() {
                if (*tmp2).av_name == (*tmp1).av_name {
                    yyerror(
                        b"duplicate auto variable names\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                tmp2 = (*tmp2).next;
            }
            if (*tmp1).arg_is_var != 0 {
                yyerror(b"* not allowed here\0" as *const u8 as *const libc::c_char);
            }
            tmp1 = (*tmp1).next;
        }
    }
    if !params.is_null() && !autos.is_null() {
        tmp1 = params;
        while !tmp1.is_null() {
            tmp2 = autos;
            while !tmp2.is_null() {
                if (*tmp2).av_name == (*tmp1).av_name {
                    yyerror(
                        b"variable in both parameter and auto lists\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                tmp2 = (*tmp2).next;
            }
            tmp1 = (*tmp1).next;
        }
    }
}
pub unsafe extern "C" fn set_genstr_size(mut size: libc::c_int) {
    if size > genlen {
        if !genstr.is_null() {
            free(genstr as *mut libc::c_void);
        }
        genstr = bc_malloc(size as size_t) as *mut libc::c_char;
        genlen = size;
    }
}
pub unsafe extern "C" fn init_gen() {
    break_label = 0 as libc::c_int;
    continue_label = 0 as libc::c_int;
    next_label = 1 as libc::c_int;
    out_count = 2 as libc::c_int;
    if compile_only != 0 {
        printf(b"@i\0" as *const u8 as *const libc::c_char);
    } else {
        init_load();
    }
    had_error = 0 as libc::c_int;
    did_gen = 0 as libc::c_int as libc::c_char;
    set_genstr_size(64 as libc::c_int);
}
pub unsafe extern "C" fn generate(mut str: *const libc::c_char) {
    did_gen = 1 as libc::c_int as libc::c_char;
    if compile_only != 0 {
        printf(b"%s\0" as *const u8 as *const libc::c_char, str);
        out_count = (out_count as libc::c_ulong).wrapping_add(strlen(str)) as libc::c_int
            as libc::c_int;
        if out_count > 60 as libc::c_int {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            out_count = 0 as libc::c_int;
        }
    } else {
        load_code(str);
    };
}
pub unsafe extern "C" fn run_code() {
    if had_error == 0 && did_gen as libc::c_int != 0 {
        if compile_only != 0 {
            printf(b"@r\n\0" as *const u8 as *const libc::c_char);
            out_count = 0 as libc::c_int;
        } else {
            execute();
        }
    }
    if did_gen != 0 {
        init_gen();
    } else {
        had_error = 0 as libc::c_int;
    };
}
pub unsafe extern "C" fn out_char(mut ch: libc::c_int) {
    if ch == '\n' as i32 {
        out_col = 0 as libc::c_int;
        putchar('\n' as i32);
    } else {
        out_col += 1;
        out_col;
        if out_col == line_size - 1 as libc::c_int && line_size != 0 as libc::c_int {
            putchar('\\' as i32);
            putchar('\n' as i32);
            out_col = 1 as libc::c_int;
        }
        putchar(ch);
    };
}
pub unsafe extern "C" fn out_schar(mut ch: libc::c_int) {
    if ch == '\n' as i32 {
        out_col = 0 as libc::c_int;
        putchar('\n' as i32);
    } else {
        if std_only == 0 {
            out_col += 1;
            out_col;
            if out_col == line_size - 1 as libc::c_int && line_size != 0 as libc::c_int {
                putchar('\\' as i32);
                putchar('\n' as i32);
                out_col = 1 as libc::c_int;
            }
        }
        putchar(ch);
    };
}
pub unsafe extern "C" fn find_id(
    mut tree: *mut id_rec,
    mut id: *const libc::c_char,
) -> *mut id_rec {
    let mut cmp_result: libc::c_int = 0;
    if tree.is_null() {
        return 0 as *mut id_rec;
    }
    cmp_result = strcmp(id, (*tree).id);
    if cmp_result == 0 as libc::c_int {
        return tree
    } else if cmp_result < 0 as libc::c_int {
        return find_id((*tree).left, id)
    } else {
        return find_id((*tree).right, id)
    };
}
pub unsafe extern "C" fn insert_id_rec(
    mut root: *mut *mut id_rec,
    mut new_id: *mut id_rec,
) -> libc::c_int {
    let mut A: *mut id_rec = 0 as *mut id_rec;
    let mut B: *mut id_rec = 0 as *mut id_rec;
    if (*root).is_null() {
        *root = new_id;
        (*new_id).left = 0 as *mut id_rec;
        (*new_id).right = 0 as *mut id_rec;
        (*new_id).balance = 0 as libc::c_int as libc::c_short;
        return 1 as libc::c_int;
    }
    if strcmp((*new_id).id, (**root).id) < 0 as libc::c_int {
        if insert_id_rec(&mut (**root).left, new_id) != 0 {
            (**root).balance -= 1;
            (**root).balance;
            match (**root).balance as libc::c_int {
                0 => return 0 as libc::c_int,
                -1 => return 1 as libc::c_int,
                -2 => {
                    A = *root;
                    B = (**root).left;
                    if (*B).balance as libc::c_int <= 0 as libc::c_int {
                        (*A).left = (*B).right;
                        (*B).right = A;
                        *root = B;
                        (*A).balance = 0 as libc::c_int as libc::c_short;
                        (*B).balance = 0 as libc::c_int as libc::c_short;
                    } else {
                        *root = (*B).right;
                        (*B).right = (**root).left;
                        (*A).left = (**root).right;
                        (**root).left = B;
                        (**root).right = A;
                        match (**root).balance as libc::c_int {
                            -1 => {
                                (*A).balance = 1 as libc::c_int as libc::c_short;
                                (*B).balance = 0 as libc::c_int as libc::c_short;
                            }
                            0 => {
                                (*A).balance = 0 as libc::c_int as libc::c_short;
                                (*B).balance = 0 as libc::c_int as libc::c_short;
                            }
                            1 => {
                                (*A).balance = 0 as libc::c_int as libc::c_short;
                                (*B).balance = -(1 as libc::c_int) as libc::c_short;
                            }
                            _ => {}
                        }
                        (**root).balance = 0 as libc::c_int as libc::c_short;
                    }
                }
                _ => {}
            }
        }
    } else if insert_id_rec(&mut (**root).right, new_id) != 0 {
        (**root).balance += 1;
        (**root).balance;
        match (**root).balance as libc::c_int {
            0 => return 0 as libc::c_int,
            1 => return 1 as libc::c_int,
            2 => {
                A = *root;
                B = (**root).right;
                if (*B).balance as libc::c_int >= 0 as libc::c_int {
                    (*A).right = (*B).left;
                    (*B).left = A;
                    *root = B;
                    (*A).balance = 0 as libc::c_int as libc::c_short;
                    (*B).balance = 0 as libc::c_int as libc::c_short;
                } else {
                    *root = (*B).left;
                    (*B).left = (**root).right;
                    (*A).right = (**root).left;
                    (**root).left = A;
                    (**root).right = B;
                    match (**root).balance as libc::c_int {
                        -1 => {
                            (*A).balance = 0 as libc::c_int as libc::c_short;
                            (*B).balance = 1 as libc::c_int as libc::c_short;
                        }
                        0 => {
                            (*A).balance = 0 as libc::c_int as libc::c_short;
                            (*B).balance = 0 as libc::c_int as libc::c_short;
                        }
                        1 => {
                            (*A).balance = -(1 as libc::c_int) as libc::c_short;
                            (*B).balance = 0 as libc::c_int as libc::c_short;
                        }
                        _ => {}
                    }
                    (**root).balance = 0 as libc::c_int as libc::c_short;
                }
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn init_tree() {
    name_tree = 0 as *mut id_rec;
    next_array = 1 as libc::c_int;
    next_func = 1 as libc::c_int;
    next_var = 5 as libc::c_int;
}
pub unsafe extern "C" fn lookup(
    mut name: *mut libc::c_char,
    mut namekind: libc::c_int,
) -> libc::c_int {
    let mut id: *mut id_rec = 0 as *mut id_rec;
    if strlen(name) != 1 as libc::c_int as libc::c_ulong {
        ct_warn(
            b"multiple letter name - %s\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    id = find_id(name_tree, name);
    if id.is_null() {
        id = bc_malloc(::std::mem::size_of::<id_rec>() as libc::c_ulong) as *mut id_rec;
        (*id).id = strcopyof(name);
        (*id).a_name = 0 as libc::c_int;
        (*id).f_name = 0 as libc::c_int;
        (*id).v_name = 0 as libc::c_int;
        insert_id_rec(&mut name_tree, id);
    }
    let mut current_block_51: u64;
    match namekind {
        1 => {
            if (*id).a_name != 0 as libc::c_int {
                free(name as *mut libc::c_void);
                return -(*id).a_name;
            }
            let fresh1 = next_array;
            next_array = next_array + 1;
            (*id).a_name = fresh1;
            if (*id).a_name < 32767 as libc::c_int {
                if (*id).a_name >= a_count {
                    more_arrays();
                }
                let ref mut fresh2 = *a_names.offset((*id).a_name as isize);
                *fresh2 = name;
                return -(*id).a_name;
            }
            yyerror(b"Too many array variables\0" as *const u8 as *const libc::c_char);
            bc_exit(1 as libc::c_int);
            current_block_51 = 3604972210128459352;
        }
        2 | 3 => {
            current_block_51 = 3604972210128459352;
        }
        0 => {
            current_block_51 = 9480739220481158223;
        }
        _ => {
            current_block_51 = 4090602189656566074;
        }
    }
    match current_block_51 {
        3604972210128459352 => {
            if (*id).f_name != 0 as libc::c_int {
                free(name as *mut libc::c_void);
                if use_math != 0 && namekind == 3 as libc::c_int
                    && (*id).f_name <= 6 as libc::c_int
                {
                    let fresh3 = next_func;
                    next_func = next_func + 1;
                    (*id).f_name = fresh3;
                }
                return (*id).f_name;
            }
            let fresh4 = next_func;
            next_func = next_func + 1;
            (*id).f_name = fresh4;
            if (*id).f_name < 32767 as libc::c_int {
                if (*id).f_name >= f_count {
                    more_functions();
                }
                let ref mut fresh5 = *f_names.offset((*id).f_name as isize);
                *fresh5 = name;
                return (*id).f_name;
            }
            yyerror(b"Too many functions\0" as *const u8 as *const libc::c_char);
            bc_exit(1 as libc::c_int);
            current_block_51 = 9480739220481158223;
        }
        _ => {}
    }
    match current_block_51 {
        9480739220481158223 => {
            if (*id).v_name != 0 as libc::c_int {
                free(name as *mut libc::c_void);
                return (*id).v_name;
            }
            let fresh6 = next_var;
            next_var = next_var + 1;
            (*id).v_name = fresh6;
            if (*id).v_name <= 32767 as libc::c_int {
                if (*id).v_name >= v_count {
                    more_variables();
                }
                let ref mut fresh7 = *v_names
                    .offset(((*id).v_name - 1 as libc::c_int) as isize);
                *fresh7 = name;
                return (*id).v_name;
            }
            yyerror(b"Too many variables\0" as *const u8 as *const libc::c_char);
            bc_exit(1 as libc::c_int);
        }
        _ => {}
    }
    yyerror(
        b"End of util.c/lookup() reached.  Please report this bug.\0" as *const u8
            as *const libc::c_char,
    );
    bc_exit(1 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn limits() {
    printf(
        b"BC_BASE_MAX     = %d\n\0" as *const u8 as *const libc::c_char,
        2147483647 as libc::c_int,
    );
    printf(
        b"BC_DIM_MAX      = %ld\n\0" as *const u8 as *const libc::c_char,
        16777215 as libc::c_int as libc::c_long,
    );
    printf(
        b"BC_SCALE_MAX    = %d\n\0" as *const u8 as *const libc::c_char,
        2147483647 as libc::c_int,
    );
    printf(
        b"BC_STRING_MAX   = %d\n\0" as *const u8 as *const libc::c_char,
        2147483647 as libc::c_int,
    );
    printf(
        b"MAX Exponent    = %ld\n\0" as *const u8 as *const libc::c_char,
        9223372036854775807 as libc::c_long,
    );
    printf(
        b"Number of vars  = %ld\n\0" as *const u8 as *const libc::c_char,
        32767 as libc::c_int as libc::c_long,
    );
}
pub unsafe extern "C" fn bc_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = malloc(size);
    if ptr.is_null() {
        out_of_memory();
    }
    return ptr;
}
pub unsafe extern "C" fn out_of_memory() {
    fprintf(
        stderr,
        b"Fatal error: Out of memory for malloc.\n\0" as *const u8 as *const libc::c_char,
    );
    bc_exit(1 as libc::c_int);
}
pub unsafe extern "C" fn yyerror(mut str: *const libc::c_char, mut args: ...) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    if is_std_in != 0 {
        name = b"(standard_in)\0" as *const u8 as *const libc::c_char;
    } else {
        name = file_name;
    }
    fprintf(stderr, b"%s %d: \0" as *const u8 as *const libc::c_char, name, line_no);
    vfprintf(stderr, str, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    had_error = 1 as libc::c_int;
}
pub unsafe extern "C" fn ct_warn(mut mesg: *const libc::c_char, mut args: ...) {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    if std_only != 0 {
        if is_std_in != 0 {
            name = b"(standard_in)\0" as *const u8 as *const libc::c_char;
        } else {
            name = file_name;
        }
        fprintf(
            stderr,
            b"%s %d: Error: \0" as *const u8 as *const libc::c_char,
            name,
            line_no,
        );
        vfprintf(stderr, mesg, args_0.as_va_list());
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
        had_error = 1 as libc::c_int;
    } else if warn_not_std != 0 {
        if is_std_in != 0 {
            name = b"(standard_in)\0" as *const u8 as *const libc::c_char;
        } else {
            name = file_name;
        }
        fprintf(
            stderr,
            b"%s %d: (Warning) \0" as *const u8 as *const libc::c_char,
            name,
            line_no,
        );
        vfprintf(stderr, mesg, args_0.as_va_list());
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn rt_error(mut mesg: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    fprintf(
        stderr,
        b"Runtime error (func=%s, adr=%d): \0" as *const u8 as *const libc::c_char,
        *f_names.offset(pc.pc_func as isize),
        pc.pc_addr,
    );
    args_0 = args.clone();
    vfprintf(stderr, mesg, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    runtime_error = 1 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn rt_warn(mut mesg: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    fprintf(
        stderr,
        b"Runtime warning (func=%s, adr=%d): \0" as *const u8 as *const libc::c_char,
        *f_names.offset(pc.pc_func as isize),
        pc.pc_addr,
    );
    args_0 = args.clone();
    vfprintf(stderr, mesg, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn bc_exit(mut val: libc::c_int) {
    exit(val);
}
